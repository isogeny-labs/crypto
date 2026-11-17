#!/usr/bin/env python3
"""Independent validator recompute for research/ic_tableb_binary_ksum_20261115.

Stdlib only.  Written by the validator without reading the study's
analysis/*.mjs, results.json or summary.json.  Reads only:
  - sessions/<B..-r.>/{records.jsonl.gz, session.json, spec.json}
  - specs/<B..-r.>.json
  - docs/ic/calibration.json (pinned char2 ratios)
  - docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json (curve parameters)

Usage: recompute.py STUDY_DIR CALIBRATION_JSON LEDGER_JSON OUT_JSON [extra session dirs...]

Pricing (AGENTS.md section 5a; semantics as documented in
src/cryptanalysis/ecbench/band.rs, re-implemented here):
  priced_gae(w) = G - C + w * (U + n_C),  S = priced_gae / sqrt(r)
Every recovered log is re-checked with this file's own GF(2^n) affine
arithmetic: [recovered] G == target.
"""
import gzip, hashlib, json, math, os, random, sys
from collections import defaultdict

W = [0.0, 0.1, 1.0]
WKEY = {0.0: "0", 0.1: "0.1", 1.0: "1"}
BINARY_SHA = "bc96a44a3342d24569e7f2bbe7e24e55400eda47d955960e766ae66df3e08f8d"
ROWS = {
    15: ("random-binary-n15-b524b", "icv1-f2m15-t131-ed7256f0", "secondary"),
    18: ("random-binary-n18-b6507", "icv1-f2m18-t393-d2c04152", "secondary"),
    21: ("random-binary-n21-b1b6f3b", "icv1-f2m21-t2591-de54d72e", "primary"),
    24: ("random-binary-n24-b5fc9da", "icv1-f2m24-tm3047-2d0c0e8c", "primary"),
    27: ("random-binary-n27-b845462", "icv1-f2m27-t12543-569dca8b", "primary"),
}
# methods.rs PRICED_NATIVE and solver_units.rs PINNABLE_NATIVE (counter, unit)
PRICED_NATIVE = [("sqrt_solves", "ns_per_sqrt"), ("as_solves", "ns_per_as_solve"),
                 ("s4_pairs", "ns_per_s4_pair"), ("lookups", "ns_per_lookup"),
                 ("row_ops", "ns_per_row_op"), ("legendre_symbols", "ns_per_legendre"),
                 ("inversions", "ns_per_inversion"), ("frobenius_maps", "ns_per_frobenius"),
                 ("canonicalisations", "ns_per_canon"), ("target_guard_probes", "ns_per_lookup"),
                 ("hashed_tensor_steps", "ns_per_hashed_tensor_step")]
TABLE_WORK = ["table_insert_attempts", "sibling_second_points", "sibling_lift_products"]
NON_OP = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged",
          "native_table_bytes_uncharged"}
RESAMPLES = 4000
BOOT_SEED = 20261002


# ---------------------------------------------------------------- GF(2^n)
class Field:
    def __init__(self, n, modulus):
        self.n, self.mod = n, modulus

    def mul(self, a, b):
        n, mod, r = self.n, self.mod, 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if (a >> n) & 1:
                a ^= mod
        return r

    def sqr(self, a):
        return self.mul(a, a)

    def inv(self, a):
        if a == 0:
            raise ZeroDivisionError
        u, v, g1, g2 = a, self.mod, 1, 0
        while u != 1:
            j = u.bit_length() - v.bit_length()
            if j < 0:
                u, v, g1, g2, j = v, u, g2, g1, -j
            u ^= v << j
            g1 ^= g2 << j
        return g1

    def trace(self, a):
        t, s = 0, a
        for _ in range(self.n):
            t ^= s
            s = self.sqr(s)
        return t


class Curve:
    """y^2 + xy = x^3 + a x^2 + b over GF(2^n); None is the point at infinity."""

    def __init__(self, n, modulus, a, b):
        self.F, self.a, self.b = Field(n, modulus), a, b

    def on_curve(self, P):
        if P is None:
            return True
        F, (x, y) = self.F, P
        return F.sqr(y) ^ F.mul(x, y) == F.mul(F.sqr(x), x) ^ F.mul(self.a, F.sqr(x)) ^ self.b

    def neg(self, P):
        return None if P is None else (P[0], P[0] ^ P[1])

    def dbl(self, P):
        if P is None or P[0] == 0:
            return None
        F, (x, y) = self.F, P
        lam = x ^ F.mul(y, F.inv(x))
        x3 = F.sqr(lam) ^ lam ^ self.a
        y3 = F.sqr(x) ^ F.mul(lam ^ 1, x3)
        return (x3, y3)

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if y1 == y2:
                return self.dbl(P)
            return None  # Q = -P
        F = self.F
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sqr(lam) ^ lam ^ x1 ^ x2 ^ self.a
        y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)

    def mul(self, k, P):
        R = None
        for bit in bin(k)[2:] if k > 0 else "":
            R = self.dbl(R)
            if bit == "1":
                R = self.add(R, P)
        return R


def is_prime(n):
    if n < 2:
        return False
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % p == 0:
            return n == p
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def selftest_curve():
    """Brute-force group-law check on a small curve over GF(2^7)."""
    n, mod = 7, (1 << 7) | 0b11  # x^7 + x + 1
    E = Curve(n, mod, 1, 0x29)
    F = E.F
    for a in range(1, 1 << n):
        assert F.mul(a, F.inv(a)) == 1
    pts = [None]
    for x in range(1 << n):
        for y in range(1 << n):
            if E.on_curve((x, y)):
                pts.append((x, y))
    N = len(pts)
    rnd = random.Random(7)
    for _ in range(3000):
        P, Q, R = rnd.choice(pts), rnd.choice(pts), rnd.choice(pts)
        assert E.on_curve(E.add(P, Q))
        assert E.add(P, Q) == E.add(Q, P)
        assert E.add(E.add(P, Q), R) == E.add(P, E.add(Q, R))
        assert E.add(P, E.neg(P)) is None
    for P in pts:
        assert E.mul(N, P) is None
    return {"field": "GF(2^7), x^7+x+1", "a": 1, "b": "0x29", "points": N,
            "checks": "closure, commutativity, associativity on 3000 random triples, P + (-P) = O, [#E]P = O for every P"}


# ---------------------------------------------------------------- pricing
def unpriced_counts(r):
    out = {}
    counters = r.get("counters") or {}
    for name in r["cost"].get("unpriced") or []:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in counters:
            out[name] = counters[name]
            continue
        tot, found = 0, False
        for p in r["phases"]:
            nat = p.get("native") or {}
            v = nat.get(name, nat.get(base))
            if v is not None:
                tot += v
                found = True
        if not found and base.startswith("solver_") and r.get("solver"):
            out[name] = r["solver"]["ops"]
            continue
        out[name] = tot if found else None
    return out


def table_work(r):
    out = {}
    for name in TABLE_WORK:
        marker = "counted:" + name
        if not any(marker in (p.get("work_native") or {}) for p in r["phases"]):
            continue
        out[name] = sum((p.get("work_native") or {}).get(name, 0) for p in r["phases"])
    return out


def calibrated(r, ratios):
    pinned = (r.get("detail") or {}).get("calibration_pinned_units") or []
    if not pinned:
        return 0.0, 0
    gae, ops = 0.0, 0
    for p in r["phases"]:
        nat = p.get("native") or {}
        for counter, unit in PRICED_NATIVE:
            n = nat.get(counter, 0)
            if n > 0 and unit in pinned:
                gae += n * ratios[unit]
                ops += n
    s = r.get("solver")
    if s and s.get("ops", 0) > 0:
        raise SystemExit(f"{r['record_id']}: solver ops present; not handled")
    return gae, ops


def priced(r, ratios):
    G = r["cost"]["total_gae"]
    up = unpriced_counts(r)
    up = {k: v for k, v in up.items() if k not in NON_OP}
    tw = table_work(r)
    missing = any(v is None for v in up.values())
    U = sum(v for v in up.values() if v is not None) + sum(tw.values())
    C, nC = calibrated(r, ratios)
    return G, C, nC, U, missing, up, tw


# ---------------------------------------------------------------- stats
def t975(df):
    # Student t quantile by bisection on a Simpson-integrated CDF.
    c = math.exp(math.lgamma((df + 1) / 2) - math.lgamma(df / 2)) / math.sqrt(df * math.pi)
    pdf = lambda x: c * (1 + x * x / df) ** (-(df + 1) / 2)

    def cdf(x, m=2000):
        h = x / m
        s = pdf(0) + pdf(x) + sum((4 if i % 2 else 2) * pdf(i * h) for i in range(1, m))
        return 0.5 + s * h / 3

    lo, hi = 1.0, 3.0
    for _ in range(60):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if cdf(mid) < 0.975 else (lo, mid)
    return (lo + hi) / 2


T_CACHE = {}


def delta_ci(c, b):
    n = len(c)
    R = sum(c) / sum(b)
    mb = sum(b) / n
    z = [(ci - R * bi) / mb for ci, bi in zip(c, b)]
    mz = sum(z) / n
    sd = math.sqrt(sum((x - mz) ** 2 for x in z) / (n - 1))
    if n - 1 not in T_CACHE:
        T_CACHE[n - 1] = t975(n - 1)
    t = T_CACHE[n - 1]
    se = sd / math.sqrt(n)
    return R, R - t * se, R + t * se


def quantile(xs, q):
    xs = sorted(xs)
    h = (len(xs) - 1) * q
    lo = math.floor(h)
    return xs[lo] + (h - lo) * (xs[min(lo + 1, len(xs) - 1)] - xs[lo])


def boot(strata, seed, resamples=RESAMPLES):
    """strata: list of (c_list, b_list) per stratum; resample units within each stratum."""
    rng = random.Random(seed)
    out = []
    for _ in range(resamples):
        sc = sb = 0.0
        for c, b in strata:
            n = len(c)
            idx = [rng.randrange(n) for _ in range(n)]
            sc += sum(map(c.__getitem__, idx))
            sb += sum(map(b.__getitem__, idx))
        out.append(sc / sb)
    return quantile(out, 0.025), quantile(out, 0.975)


def cluster_boot(strata, seed, resamples=RESAMPLES):
    """strata: list of dict target -> (sum_c, sum_b) per stratum; resample targets."""
    rng = random.Random(seed)
    prepared = [(list(d.values())) for d in strata]
    out = []
    for _ in range(resamples):
        sc = sb = 0.0
        for units in prepared:
            n = len(units)
            for _ in range(n):
                u = units[rng.randrange(n)]
                sc += u[0]
                sb += u[1]
        out.append(sc / sb)
    return quantile(out, 0.025), quantile(out, 0.975)


def sha256_bytes(b):
    return hashlib.sha256(b).hexdigest()


def sha256_file(p):
    with open(p, "rb") as f:
        return sha256_bytes(f.read())


# ---------------------------------------------------------------- main
def load_session(d):
    raw = gzip.open(os.path.join(d, "records.jsonl.gz")).read() if os.path.exists(
        os.path.join(d, "records.jsonl.gz")) else open(os.path.join(d, "records.jsonl"), "rb").read()
    recs = [json.loads(l) for l in raw.splitlines() if l.strip()]
    sess = json.load(open(os.path.join(d, "session.json")))
    return raw, recs, sess


def main():
    study, calib_p, ledger_p, out_p = sys.argv[1:5]
    extra = sys.argv[5:]
    out = {"schema": "validator.recompute/v1", "selftest": selftest_curve()}
    calib = json.load(open(calib_p))["instances"]
    ledger = {i["curve"]["name"]: i for i in json.load(open(ledger_p))["ledger"]["instances"]}

    curves = {}
    for bits, (name, slug, role) in ROWS.items():
        i = ledger[name]
        c = i["curve"]
        n = c["field"]["degree"]
        mod = (1 << n) | sum(1 << t for t in c["field"]["polynomial_low_terms"])
        a = int(c["a"], 16) if isinstance(c["a"], str) and c["a"].startswith("0x") else int(c["a"])
        b = int(c["b"], 16)
        E = Curve(n, mod, a, b)
        G = (int(c["generator"]["x"], 16), int(c["generator"]["y"], 16))
        r = i["r"]
        N = c["group_order"]
        info = {"name": name, "slug": slug, "n": n, "modulus": mod, "a": a, "b": b, "r": r,
                "group_order": N, "G": G, "G_on_curve": E.on_curve(G), "rG_is_O": E.mul(r, G) is None,
                "G_not_O": G is not None, "r_prime": is_prime(r), "N_div_r": N % r == 0,
                "hasse": abs(N - (1 << n) - 1) <= 2 * math.isqrt(1 << n) + 2,
                "NG_is_O": E.mul(N, G) is None}
        curves[bits] = (E, info)
    out["curves"] = {str(k): {kk: (vv if not isinstance(vv, tuple) else [hex(x) for x in vv])
                              for kk, vv in v[1].items()} for k, v in curves.items()}

    sessions = sorted(x for x in os.listdir(os.path.join(study, "sessions")) if x.startswith("B"))
    dirs = [(s, os.path.join(study, "sessions", s)) for s in sessions] + [
        (os.path.basename(os.path.normpath(e)), e) for e in extra]
    # data[bits][sess][arm][(round, tidx)] = record facts
    data = defaultdict(lambda: defaultdict(lambda: defaultdict(dict)))
    sess_meta = {}
    verify_cache = {}
    cert = defaultdict(int)
    pricing_audit = defaultdict(lambda: defaultdict(int))
    integrity = {}
    ksum_build = defaultdict(lambda: defaultdict(int))
    for sname, d in dirs:
        raw, recs, sess = load_session(d)
        bits = int(sname[1:3])
        E, cinfo = curves[bits]
        name, slug, role = ROWS[bits]
        ratios = calib["char2/" + name]
        spec_session = open(os.path.join(d, "spec.json"), "rb").read()
        spec_reg = os.path.join(study, "specs", sname + ".json")
        sm = {
            "session_id": sess["session_id"], "status": sess["status"],
            "records": len(recs), "executions_planned": sess["executions_planned"],
            "records_written": sess["records_written"],
            "records_sha256_matches": sha256_bytes(raw) == sess["records_sha256"],
            "session_binary_sha256": sess["binary_sha256"], "git_commit": sess["git_commit"],
            "git_dirty": sess["git_dirty"], "started_unix_ms": sess["started_unix_ms"],
            "finished_unix_ms": sess["finished_unix_ms"],
            "spec_json_equals_registered_spec": (os.path.exists(spec_reg) and open(spec_reg, "rb").read() == spec_session),
            "spec_sha256": sha256_bytes(spec_session),
        }
        spec = json.loads(spec_session)
        sc = spec["workloads"]["curves"][0]
        sm["spec_curve_matches_ledger"] = (sc["n"] == cinfo["n"] and sc["modulus"] == cinfo["modulus"] and sc["a"] == cinfo["a"]
                                           and sc["b"] == cinfo["b"] and sc["r"] == cinfo["r"] and sc["group_order"] == cinfo["group_order"]
                                           and (sc["gx"], sc["gy"]) == cinfo["G"])
        sm["target_seed"] = spec["workloads"]["target_seed"]
        sm["measurement_seed"] = spec["measurement"]["seed"]
        sm["targets_per_curve"] = spec["workloads"]["targets_per_curve"]
        sm["rounds"] = spec["measurement"]["rounds"]
        sm["arms"] = [a["name"] + "=" + json.dumps(a["method"], sort_keys=True) for a in spec["arms"]]
        statuses = defaultdict(int)
        bins = set()
        cells = defaultdict(dict)
        for r in recs:
            statuses[r["outcome"]["status"]] += 1
            bins.add(r.get("binary_sha256"))
            wl = r["workload"]
            assert wl["curve"]["slug"] == slug, (sname, wl["curve"]["slug"])
            assert wl["curve"]["r"] == cinfo["r"]
            T = (int(wl["target"][0], 16), int(wl["target"][1], 16))
            rec = r["outcome"].get("recovered")
            key = (T, rec)
            if key not in verify_cache:
                ok = rec is not None and E.on_curve(T) and 0 <= int(rec) < cinfo["r"] and E.mul(int(rec), cinfo["G"]) == T
                verify_cache[key] = ok
            cert["records_checked"] += 1
            if not verify_cache[key]:
                cert["failures"] += 1
            if r["outcome"]["status"] != "verified" or r["outcome"].get("matches_target") is not True:
                cert["not_verified_by_ecbench"] += 1
            cell = (r["round"], r["warmup"], wl["target_index"])
            arm = r["arm"]
            if arm in cells[cell]:
                cert["duplicate_arm_in_cell"] += 1
            cells[cell][arm] = (T, r["algorithm_seed"], rec)
            if r["warmup"]:
                continue
            G, C, nC, U, missing, up, tw = priced(r, ratios)
            fam = r["method"]["family"]
            pa = pricing_audit[f"B{bits}"]
            pa[f"{arm}:runs"] += 1
            if missing:
                pa[f"{arm}:missing_unpriced_count"] += 1
            if fam == "ic":
                addsdbl = sum(p["adds"] + p["doubles"] for p in r["phases"])
                if abs(G - (addsdbl + C)) > 1e-6 * max(1, G):
                    pa[f"{arm}:G_ne_adds_doubles_plus_C"] += 1
                if abs((G - C) - round(G - C)) > 1e-6:
                    pa[f"{arm}:G_minus_C_nonintegral"] += 1
                if not tw:
                    pa[f"{arm}:no_table_work_marker"] += 1
                # silent natives: nonzero native counters neither calibrated nor unpriced
                pinned = set((r.get("detail") or {}).get("calibration_pinned_units") or [])
                for p in r["phases"]:
                    for k, v in (p.get("native") or {}).items():
                        units = [u for c_, u in PRICED_NATIVE if c_ == k]
                        if v and units and not any(u in pinned for u in units):
                            pa[f"{arm}:priced_native_without_pinned_unit:{k}"] += 1
                fb = r.get("factor_base") or {}
                if r["method"]["params"]["oracle"].startswith("ksum"):
                    k = int(r["method"]["params"]["oracle"].split("m=")[1])
                    A = fb["abscissae"]
                    setup = [p for p in r["phases"] if p["name"] == "oracle_setup"][0]
                    want_adds = sum(math.comb(A, dd) * 2 ** (dd - 1) for dd in range(2, k + 1))
                    want_ins = math.comb(A, k) * 2 ** (k - 1)
                    ins = (setup.get("work_native") or {}).get("table_insert_attempts")
                    kb = ksum_build[f"B{bits}:{arm}"]
                    kb["runs"] += 1
                    kb["abscissae"] = A
                    kb["signed_points"] = fb["signed_points"]
                    kb["expected_build_adds"] = want_adds
                    kb["expected_insert_attempts"] = want_ins
                    if setup["adds"] != want_adds:
                        kb["build_adds_mismatch"] += 1
                    if ins != want_ins:
                        kb["insert_attempts_mismatch"] += 1
                    rel = [p for p in r["phases"] if p["name"] == "relations"][0]
                    nat = rel.get("native") or {}
                    if rel["adds"] < nat.get("walk_steps", 0) + (k - 1) * nat.get("relations", 0):
                        kb["relations_adds_below_walk_plus_confirm"] += 1
            elif fam == "rho":
                if abs(G - sum(p["gae"] for p in r["phases"])) > 1e-9:
                    pa[f"{arm}:G_ne_phase_sum"] += 1
            sqrt_r = math.sqrt(cinfo["r"])
            if abs(r["cost"]["s"] - G / sqrt_r) > 1e-9 * max(1, r["cost"]["s"]):
                pa[f"{arm}:recorded_s_ne_G_over_sqrt_r"] += 1
            S = {w: (G - C + w * (U + nC)) / sqrt_r for w in W}
            data[bits][sname][arm][(r["round"], wl["target_index"])] = {
                "S": S, "T": T, "seed": r["algorithm_seed"], "rec": rec,
                "counts_key": (G, U, C, nC)}
        sm["status_counts"] = dict(statuses)
        sm["record_binary_sha256"] = sorted(b for b in bins if b)
        sm["all_records_bound_to_binary"] = sm["record_binary_sha256"] == [BINARY_SHA] or None
        # pairing
        miss = more_t = more_seed = 0
        arms = {a["name"] for a in spec["arms"]}
        for cell, m in cells.items():
            if set(m) != arms:
                miss += 1
            if len({v[0] for v in m.values()}) > 1:
                more_t += 1
            if len({v[1] for v in m.values()}) > 1:
                more_seed += 1
            if len({v[2] for v in m.values()}) > 1:
                cert["cells_with_disagreeing_recovered_logs"] += 1
        sm["pairing"] = {"cells": len(cells), "cells_missing_an_arm": miss,
                         "cells_with_more_than_one_target": more_t,
                         "cells_with_more_than_one_algorithm_seed": more_seed}
        sess_meta[sname] = sm
    out["sessions"] = sess_meta
    out["certificates"] = dict(cert, distinct_target_log_pairs=len(verify_cache),
                               method="own affine GF(2^n) double-and-add in Python: [recovered]G == target, target on curve, 0 <= recovered < r; generator on curve, [r]G = O, r prime")
    out["pricing_audit"] = {k: dict(v) for k, v in pricing_audit.items()}
    out["ksum_build_audit"] = {k: dict(v) for k, v in ksum_build.items()}

    # ---------------- A/A, recompute, intervals, L1
    rows_out = {}
    for bits in sorted(data):
        name, slug, role = ROWS[bits]
        sd = data[bits]
        snames = sorted(s for s in sd if s.startswith(f"B{bits}-r"))
        if not snames:
            continue
        arms = sorted(sd[snames[0]])
        incs = [a for a in arms if a.startswith("inc-")]
        units = {s: sorted(sd[s][arms[0]]) for s in snames}
        for s in snames:
            for a in arms:
                assert sorted(sd[s][a]) == units[s], (s, a)
        ro = {"slug": slug, "role": role, "sessions": snames, "pairs": sum(len(u) for u in units.values()),
              "incumbents": incs}
        # A/A control
        aa_diff = sum(1 for s in snames for u in units[s]
                      if sd[s]["rho-neg"][u]["counts_key"] != sd[s]["rho-neg-aa"][u]["counts_key"])
        ro["aa_rho_neg_vs_rho_neg_aa_pairs_differing"] = aa_diff

        def vals(s, a, w):
            return [sd[s][a][u]["S"][w] for u in units[s]]

        mean_pooled = {a: {WKEY[w]: sum(sum(vals(s, a, w)) for s in snames) / ro["pairs"] for w in W} for a in arms}
        mean_sess = {a: {s: {WKEY[w]: sum(vals(s, a, w)) / len(units[s]) for w in W} for s in snames} for a in arms}
        ro["mean_S_pooled"] = mean_pooled
        ro["mean_S_by_session"] = mean_sess
        spread = {a: max(mean_sess[a][s]["0"] for s in snames) / min(mean_sess[a][s]["0"] for s in snames)
                  for a in ["inc-tb", "rho-neg", "ksum5", "ksum4"]}
        delta = max(spread.values())
        ro["replicate_spread_w0"] = spread
        ro["delta"] = delta
        ro["inc_tb_over_strictest_w0"] = mean_pooled["inc-tb"]["0"] / min(mean_pooled[a]["0"] for a in incs)
        cand_out = {}
        for cand in ["ksum5", "ksum4"]:
            co = {"by_weight": {}}
            max_pooled_upper = 0.0
            max_any_upper = 0.0
            max_pooled_upper_cluster = 0.0
            max_any_upper_cluster = 0.0
            max_proto_pooled = 0.0
            max_proto_any = 0.0
            all_below_1 = True
            for w in W:
                I = min(incs, key=lambda a: mean_pooled[a][WKEY[w]])
                c_all = [x for s in snames for x in vals(s, cand, w)]
                b_all = [x for s in snames for x in vals(s, I, w)]
                R, dlo, dhi = delta_ci(c_all, b_all)
                seed = BOOT_SEED
                blo, bhi = boot([(vals(s, cand, w), vals(s, I, w)) for s in snames], seed)
                cl = []
                for s in snames:
                    dct = defaultdict(lambda: [0.0, 0.0])
                    for u in units[s]:
                        dct[u[1]][0] += sd[s][cand][u]["S"][w]
                        dct[u[1]][1] += sd[s][I][u]["S"][w]
                    cl.append(dct)
                clo, chi = cluster_boot(cl, seed)
                # protocol-faithful reading: pairs are (session, target) means over rounds,
                # linearised t on those, plus the target-cluster bootstrap
                tc = [v[0] for dct in cl for v in dct.values()]
                tb = [v[1] for dct in cl for v in dct.values()]
                _, tlo, thi = delta_ci(tc, tb)
                up_proto = max(thi, chi)
                max_proto_pooled = max(max_proto_pooled, up_proto)
                up = max(dhi, bhi)
                per = []
                for s in snames:
                    Ii = min(incs, key=lambda a: mean_sess[a][s][WKEY[w]])
                    cc, bb = vals(s, cand, w), vals(s, Ii, w)
                    Ri, dlo_i, dhi_i = delta_ci(cc, bb)
                    blo_i, bhi_i = boot([(cc, bb)], seed)
                    dct = defaultdict(lambda: [0.0, 0.0])
                    for u in units[s]:
                        dct[u[1]][0] += sd[s][cand][u]["S"][w]
                        dct[u[1]][1] += sd[s][Ii][u]["S"][w]
                    clo_i, chi_i = cluster_boot([dct], seed)
                    _, tlo_i, thi_i = delta_ci([v[0] for v in dct.values()], [v[1] for v in dct.values()])
                    up_proto_i = max(thi_i, chi_i)
                    max_proto_any = max(max_proto_any, up_proto_i)
                    up_i = max(dhi_i, bhi_i)
                    per.append({"session": s, "incumbent": Ii, "ratio": Ri, "delta_ci": [dlo_i, dhi_i],
                                "pair_boot_ci": [blo_i, bhi_i], "cluster_boot_ci": [clo_i, chi_i],
                                "delta_ci_target_level": [tlo_i, thi_i], "upper_protocol_reading": up_proto_i,
                                "upper_max_delta_pairboot": up_i})
                    max_any_upper = max(max_any_upper, up_i)
                    max_any_upper_cluster = max(max_any_upper_cluster, chi_i, up_i)
                    all_below_1 &= up_i < 1
                all_below_1 &= up < 1
                max_pooled_upper = max(max_pooled_upper, up)
                max_pooled_upper_cluster = max(max_pooled_upper_cluster, chi, up)
                co["by_weight"][WKEY[w]] = {
                    "strictest_incumbent_pooled": I, "S_incumbent": mean_pooled[I][WKEY[w]],
                    "S_candidate": mean_pooled[cand][WKEY[w]], "ratio": R,
                    "delta_ci": [dlo, dhi], "pair_boot_ci": [blo, bhi], "cluster_boot_ci": [clo, chi],
                    "delta_ci_target_level": [tlo, thi], "upper_protocol_reading": up_proto,
                    "upper_max_delta_pairboot": up, "per_replicate": per,
                    "gap_to_rho_neg": mean_pooled[cand][WKEY[w]] / mean_pooled["rho-neg"][WKEY[w]],
                    "over_bsgs_neg": mean_pooled[cand][WKEY[w]] / mean_pooled["bsgs-neg"][WKEY[w]],
                }
            noise_ok = (1 - max_pooled_upper) > (delta - 1)
            co["max_upper_pooled"] = max_pooled_upper
            co["max_upper_any_replicate"] = max_any_upper
            co["max_upper_pooled_incl_cluster"] = max_pooled_upper_cluster
            co["max_upper_any_replicate_incl_cluster"] = max_any_upper_cluster
            co["all_uppers_below_1"] = all_below_1
            co["noise_condition_pooled_upper"] = noise_ok
            co["noise_condition_any_replicate_upper"] = (1 - max_any_upper) > (delta - 1)
            co["L1_holds"] = bool(all_below_1 and noise_ok)
            co["L1_holds_with_cluster_bootstrap_too"] = bool(
                all_below_1 and max_any_upper_cluster < 1 and (1 - max_pooled_upper_cluster) > (delta - 1))
            co["max_upper_pooled_protocol_reading"] = max_proto_pooled
            co["max_upper_any_replicate_protocol_reading"] = max_proto_any
            co["L1_holds_protocol_reading"] = bool(
                max_proto_pooled < 1 and max_proto_any < 1 and (1 - max_proto_pooled) > (delta - 1))
            cand_out[cand] = co
        ro["candidates"] = cand_out
        # null-object control for the interval machinery: the two cheapest incumbents
        # (same oracle, same family, neighbouring base sizes) are near-equal arms; report
        # their paired ratio and intervals, and the A/A arm against itself across replicates.
        nulls = {}
        for w in W:
            order = sorted(incs, key=lambda a: mean_pooled[a][WKEY[w]])
            a1, a2 = order[0], order[1]
            cl = []
            for s in snames:
                dct = defaultdict(lambda: [0.0, 0.0])
                for u in units[s]:
                    dct[u[1]][0] += sd[s][a2][u]["S"][w]
                    dct[u[1]][1] += sd[s][a1][u]["S"][w]
                cl.append(dct)
            tc = [v[0] for dct in cl for v in dct.values()]
            tb = [v[1] for dct in cl for v in dct.values()]
            Rn, lo, hi = delta_ci(tc, tb)
            clo, chi = cluster_boot(cl, BOOT_SEED)
            nulls[WKEY[w]] = {"pair": f"{a2}/{a1}", "ratio": Rn, "delta_ci_target_level": [lo, hi],
                              "cluster_boot_ci": [clo, chi]}
        ro["null_control_neighbouring_incumbents"] = nulls
        ro["band"] = {WKEY[w]: {"ksum5": mean_pooled["ksum5"][WKEY[w]], "rho_neg": mean_pooled["rho-neg"][WKEY[w]],
                                "ksum5_over_rho_neg": mean_pooled["ksum5"][WKEY[w]] / mean_pooled["rho-neg"][WKEY[w]],
                                "ksum5_total_gae_mean": mean_pooled["ksum5"][WKEY[w]] * math.sqrt(curves[bits][1]["r"]),
                                "rho_neg_total_gae_mean": mean_pooled["rho-neg"][WKEY[w]] * math.sqrt(curves[bits][1]["r"])}
                      for w in W}
        ro["band_order_rho_below_ksum5_all_w"] = all(ro["band"][k]["rho_neg"] < ro["band"][k]["ksum5"] for k in ro["band"])
        rows_out[str(bits)] = ro
    out["rows"] = rows_out
    # extra (fresh) sessions: per-session ratios only
    fresh = {}
    for sname, d in dirs:
        if sname.startswith("B") and sname[3:5] == "-r":
            continue
        bits = int(sname[1:3])
        sd = data[bits][sname]
        arms = sorted(sd)
        incs = [a for a in arms if a.startswith("inc-")]
        units = sorted(sd[arms[0]])
        fo = {"arms": arms, "pairs": len(units)}
        for w in W:
            means = {a: sum(sd[a][u]["S"][w] for u in units) / len(units) for a in arms}
            I = min(incs, key=lambda a: means[a])
            entry = {"mean_S": means, "cheapest_incumbent": I}
            for cand in [a for a in arms if a.startswith("ksum")]:
                cc = [sd[cand][u]["S"][w] for u in units]
                bb = [sd[I][u]["S"][w] for u in units]
                R, lo, hi = delta_ci(cc, bb)
                blo, bhi = boot([(cc, bb)], BOOT_SEED)
                entry[cand] = {"ratio": R, "delta_ci": [lo, hi], "pair_boot_ci": [blo, bhi],
                               "gap_to_rho_neg": means[cand] / means["rho-neg"]}
            fo[WKEY[w]] = entry
        fresh[sname] = fo
    out["fresh_sessions"] = fresh
    with open(out_p, "w") as f:
        json.dump(out, f, indent=1, sort_keys=True)
        f.write("\n")


if __name__ == "__main__":
    main()
