#!/usr/bin/env python3
"""Independent validator recompute for research/ic_ksum_prune_20261115 (VAL-20261115-276156).

Written by the validator from the frozen PROTOCOL.md, the record format and the documented band
formula (src/cryptanalysis/ecbench/band.rs module comment: priced_gae(w) = G - C + w (U + n_C);
compare.rs: ratio of totals over (workload, round) pairs, linearised t on workload clusters with the
log-scale interval, two-stage cluster bootstrap). It does not import or read the producer's
analysis/*.mjs, analysis/correctness.py, summary.json or results.json; the claimed numbers are
compared only by mkreports.py, after this file has written own-numbers.json.

Standard library only. usage:
  python3 -I recompute.py <repo root> <out own-numbers.json> [fresh session dir ...]
"""
import gzip, hashlib, json, math, os, random, sys
from collections import defaultdict, Counter

ROOT = os.path.abspath(sys.argv[1])
OUT = sys.argv[2]
FRESH = sys.argv[3:]
STUDY = os.path.join(ROOT, "research/ic_ksum_prune_20261115")
SIBLINGS = {"P": os.path.join(ROOT, "research/ic_tableb_ksum_20261115"),
            "B": os.path.join(ROOT, "research/ic_tableb_binary_ksum_20261115")}
ROWS = ["P20", "P22", "P24", "B15", "B18", "B21", "B24", "B27"]
WEIGHTS = [0.0, 0.1, 1.0]
WKEY = {0.0: "0", 0.1: "0.1", 1.0: "1"}
ARMS = ["rho-neg", "rho-neg-aa", "ksum4", "ksum4-p", "ksum5", "ksum5-p"]
RESAMPLES, BOOT_SEED = 4000, 20261002
import datetime
REGISTERED_AT_MS = int(datetime.datetime.strptime(open(os.path.join(STUDY, "registered-at.txt")).read().strip(),
                                                  "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=datetime.timezone.utc).timestamp() * 1000)

# ---- band pricing (band.rs, methods.rs PRICED_NATIVE, solver_units.rs PINNABLE_NATIVE) ----------
PRICED_NATIVE = [("sqrt_solves", "ns_per_sqrt"), ("as_solves", "ns_per_as_solve"),
                 ("s4_pairs", "ns_per_s4_pair"), ("lookups", "ns_per_lookup"),
                 ("row_ops", "ns_per_row_op"), ("legendre_symbols", "ns_per_legendre"),
                 ("inversions", "ns_per_inversion"), ("frobenius_maps", "ns_per_frobenius"),
                 ("canonicalisations", "ns_per_canon"), ("target_guard_probes", "ns_per_lookup")]
PINNABLE = ["hashed_tensor_steps", "pkm_system_fp_mults_uncharged", "pkm_grid_fp_mults_uncharged",
            "pkm_shared_poly_fp_mults_uncharged", "pkm_trace_learn_spec_fp_mults_uncharged",
            "pkm_t0_base_map_fp_mults_uncharged", "pkm_base_check_fp_mults_uncharged",
            "pkm_t0_target_fp_mults_uncharged"]
NON_OPERATION = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged",
                 "native_table_bytes_uncharged"}
TABLE_WORK = ["table_insert_attempts", "sibling_second_points", "sibling_lift_products"]
CAL = json.load(open(os.path.join(ROOT, "docs/ic/calibration.json")))["instances"]
PRIME_CAL = {1048291: "prime/bench-20bit", 3290411: "prime/generated-22bit-3290411",
             10935329: "prime/generated-24bit-10935329"}


def cal_row(rec):
    c = rec["workload"]["curve_spec"]
    if c["kind"] == "prime_explicit":
        return PRIME_CAL[c["p"]]
    return "char2/random-binary-n%d-b%x" % (c["n"], c["b"])


def price_parts(rec):
    """(G, C, n_C, U, C_check): C from the pinned table; C_check = sum of phase gae - adds - doubles."""
    G = rec["cost"]["total_gae"]
    pinned = rec.get("detail", {}).get("calibration_pinned_units") or []
    C, nC = 0.0, 0
    if pinned:
        table = CAL[cal_row(rec)]
        for p in rec["phases"]:
            for cnt, unit in PRICED_NATIVE:
                n = p.get("native", {}).get(cnt, 0)
                if n > 0 and unit in pinned:
                    C += n * table[unit]
                    nC += n
            for cnt in PINNABLE:
                assert p.get("native", {}).get(cnt, 0) == 0, "pinnable counter present"
    assert rec.get("solver") in (None, {}), "solver block present"
    U = 0
    for name in rec["cost"]["unpriced"]:
        if name in NON_OPERATION:
            continue
        if name in rec["counters"]:
            U += rec["counters"][name]
            continue
        base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
        found = False
        for p in rec["phases"]:
            nat = p.get("native", {})
            if name in nat or base in nat:
                U += nat.get(name, nat.get(base)); found = True
        if not found:
            raise ValueError("uncounted unpriced counter " + name)
    for name in TABLE_WORK:
        if any(("counted:" + name) in p.get("work_native", {}) for p in rec["phases"]):
            U += sum(p.get("work_native", {}).get(name, 0) for p in rec["phases"])
    C_check = sum(p["gae"] - p["adds"] - p["doubles"] for p in rec["phases"]) if pinned else 0.0
    return G, C, nC, U, C_check


def priced(parts, w):
    G, C, nC, U, _ = parts
    return G - C + w * (U + nC)


# ---- statistics ------------------------------------------------------------------------------
def betacf(a, b, x):
    tiny = 1e-300
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = tiny if abs(d) < tiny else d
    d = 1.0 / d; h = d
    for m in range(1, 10000):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d; d = tiny if abs(d) < tiny else d
        c = 1.0 + aa / c; c = tiny if abs(c) < tiny else c
        d = 1.0 / d; h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d; d = tiny if abs(d) < tiny else d
        c = 1.0 + aa / c; c = tiny if abs(c) < tiny else c
        d = 1.0 / d; de = d * c; h *= de
        if abs(de - 1.0) < 1e-16:
            break
    return h


def betai(a, b, x):
    if x <= 0: return 0.0
    if x >= 1: return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log(1 - x)
    if x < (a + 1) / (a + b + 2):
        return math.exp(lbt) * betacf(a, b, x) / a
    return 1.0 - math.exp(lbt) * betacf(b, a, 1 - x) / b


def t_cdf(t, df):
    x = df / (df + t * t)
    tail = 0.5 * betai(df / 2.0, 0.5, x)
    return 1.0 - tail if t > 0 else tail


def t975(df):
    lo, hi = 0.0, 64.0
    for _ in range(300):
        mid = 0.5 * (lo + hi)
        if t_cdf(mid, df) < 0.975: lo = mid
        else: hi = mid
    return 0.5 * (lo + hi)


def lin_ratio(clusters):
    """Ratio of totals with the delta-method se over clusters, t(K-1), log-scale 95% interval."""
    K = len(clusters)
    sa = sum(a for a, _ in clusters); sb = sum(b for _, b in clusters)
    R = sb / sa
    u = [(b - R * a) / sa for a, b in clusters]
    ub = sum(u) / K
    se = math.sqrt(K / (K - 1) * sum((x - ub) ** 2 for x in u))
    q = t975(K - 1)
    half = q * se / R
    return R, (R * math.exp(-half), R * math.exp(half)), se, K - 1


def cluster_boot(strata, resamples, seed):
    """Two-stage percentile bootstrap of sum(b)/sum(a): clusters with replacement, then pairs in each."""
    rng = random.Random(seed)
    rnd = rng.random
    vals = []
    K = len(strata)
    for _ in range(resamples):
        sa = sb = 0.0
        for _ in range(K):
            s = strata[int(rnd() * K)]
            n = len(s)
            for _ in range(n):
                a, b = s[int(rnd() * n)]
                sa += a; sb += b
        vals.append(sb / sa)
    vals.sort()
    def q(p):
        x = p * (len(vals) - 1); i = int(math.floor(x)); f = x - i
        return vals[i] if i + 1 >= len(vals) else vals[i] * (1 - f) + vals[i + 1] * f
    return q(0.025), q(0.975)


# ---- elliptic-curve arithmetic for the scalar certificates -----------------------------------
def is_prime(n):
    if n < 2: return False
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % p == 0: return n == p
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else:
            return False
    return True


class PrimeCurve:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b
    def on(s, P): return P is None or (P[1] * P[1] - (P[0] ** 3 + s.a * P[0] + s.b)) % s.p == 0
    def add(s, P, Q):
        p = s.p
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            l = (3 * P[0] * P[0] + s.a) * pow(2 * P[1], -1, p) % p
        else:
            l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
        x = (l * l - P[0] - Q[0]) % p
        return (x, (l * (P[0] - x) - P[1]) % p)


class BinaryCurve:
    """y^2 + xy = x^3 + a x^2 + b over GF(2^n) = GF(2)[t]/(modulus)."""
    def __init__(s, n, modulus, a, b): s.n, s.m, s.a, s.b = n, modulus, a, b
    def mul(s, x, y):
        r = 0
        while y:
            if y & 1: r ^= x
            y >>= 1; x <<= 1
            if x >> s.n & 1: x ^= s.m
        return r
    def inv(s, x):
        assert x
        r, e, base = 1, (1 << s.n) - 2, x
        while e:
            if e & 1: r = s.mul(r, base)
            base = s.mul(base, base); e >>= 1
        return r
    def on(s, P):
        if P is None: return True
        x, y = P
        return s.mul(y, y) ^ s.mul(x, y) == s.mul(s.mul(x, x), x) ^ s.mul(s.a, s.mul(x, x)) ^ s.b
    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y2 == x1 ^ y1 or x1 == 0: return None  # Q = -P (or doubling a 2-torsion point)
            l = x1 ^ s.mul(y1, s.inv(x1))
            x3 = s.mul(l, l) ^ l ^ s.a
            return (x3, s.mul(x1, x1) ^ s.mul(l, x3) ^ x3)
        l = s.mul(y1 ^ y2, s.inv(x1 ^ x2))
        x3 = s.mul(l, l) ^ l ^ x1 ^ x2 ^ s.a
        return (x3, s.mul(l, x1 ^ x3) ^ x3 ^ y1)


def smul(E, k, P):
    R = None
    while k:
        if k & 1: R = E.add(R, P)
        P = E.add(P, P); k >>= 1
    return R


def curve_of(spec):
    if spec["kind"] == "prime_explicit":
        return PrimeCurve(spec["p"], spec["a"], spec["b"])
    return BinaryCurve(spec["n"], spec["modulus"], spec["a"], spec["b"])


def curve_facts(spec):
    E = curve_of(spec); G = (spec["gx"], spec["gy"])
    return {"G_on_curve": E.on(G), "rG_is_O": smul(E, spec["r"], G) is None,
            "NG_is_O": smul(E, spec["group_order"], G) is None, "r_prime": is_prime(spec["r"]),
            "G_not_O": G is not None, "N_div_r": spec["group_order"] % spec["r"] == 0}


# ---- k-sum build: closed-form add counts and an exact re-enumeration -------------------------
def C(n, k):
    return math.comb(n, k) if 0 <= k <= n else 0


def build_adds_formula(a, k, prune):
    return sum((1 << (j - 1)) * C(a - (k - j) if prune else a, j) for j in range(2, k + 1))


def deep_diff(x, y, path, out):
    if isinstance(x, dict) and isinstance(y, dict):
        for k in set(x) | set(y):
            deep_diff(x.get(k, "<absent>"), y.get(k, "<absent>"), path + "." + k, out)
    elif isinstance(x, list) and isinstance(y, list) and len(x) == len(y):
        for i, (u, v) in enumerate(zip(x, y)):
            nm = u.get("name") if isinstance(u, dict) and "name" in u else str(i)
            deep_diff(u, v, path + "[" + nm + "]", out)
    elif x != y:
        out.append(path)


# Paths in which a -p record may legitimately differ from its unpruned twin.
ALLOWED_DIFF = {
    ".record_id", ".run_id", ".seq", ".arm", ".method.params.oracle", ".method.method_id",
    ".method.method_sha256", ".cost.total_gae", ".cost.s", ".boundaries.ratio_to_floor",
    ".phases[oracle_setup].adds", ".phases[oracle_setup].gae", ".detail.label",
    ".detail.framework_total_gae_before_solver_removal",
    ".work.phases[oracle_setup].prims.group_add", ".work.phases[oracle_setup].bitops",
    ".work.phases[oracle_setup].log2_bitops", ".work.bitops", ".work.log2_bitops",
    ".work.bitops_per_sqrt_r", ".work.ratio_to_floor", ".work.reusable_bitops",
    ".fbm_id",  # FBM1 id: hash of a preimage that includes the oracle string (ecbench/fbm.rs)
}
VOLATILE_PREFIX = (".time.", ".isolation.", ".online.wall_ns", ".online.phases_ns", ".allocator.")


def volatile(p):
    return p.startswith(VOLATILE_PREFIX) or p.endswith(".wall_ns")


# ---- session loading ---------------------------------------------------------------------------
def load_session(d):
    f = os.path.join(d, "records.jsonl")
    raw = open(f, "rb").read() if os.path.exists(f) else gzip.open(f + ".gz", "rb").read()
    lines = raw.decode().splitlines()
    recs = [json.loads(l) for l in lines]
    seals_ok = 0
    for l, r in zip(lines, recs):
        rid = r["record_id"]; fld = '"record_id":"%s"' % rid
        blank = l.replace(fld, '"record_id":""', 1)
        seals_ok += ("ECR1h" + hashlib.sha256(blank.encode()).hexdigest()[:12] == rid)
    sess = json.load(open(os.path.join(d, "session.json")))
    return sess, recs, hashlib.sha256(raw).hexdigest(), seals_ok


def analyse_session(name, d, spec_file, binary_sha, out_sessions, certs, build_audit, corr, pairs_store):
    sess, recs, rec_sha, seals_ok = load_session(d)
    spec = json.load(open(os.path.join(d, "spec.json")))
    info = {"session_id": sess["session_id"], "records": len(recs), "records_sha256": rec_sha,
            "records_sha256_matches_session": rec_sha == sess["records_sha256"],
            "record_seals_ok": seals_ok, "status": sess["status"],
            "git_commit": sess.get("git_commit"), "git_dirty": sess.get("git_dirty"),
            "session_binary_sha256": sess.get("binary_sha256"),
            "all_records_bound_to_binary": all(r.get("binary_sha256") == binary_sha for r in recs),
            "binary_matches": sess.get("binary_sha256") == binary_sha,
            "started_after_registration": sess["started_unix_ms"] > REGISTERED_AT_MS,
            "status_counts": dict(Counter(r["outcome"]["status"] for r in recs)),
            "all_verified": all(r["outcome"]["status"] == "verified" for r in recs),
            "matches_target_all": all(r["outcome"]["matches_target"] for r in recs),
            "arms": sorted(set(r["arm"] for r in recs)),
            "target_seed": spec["workloads"]["target_seed"], "measurement_seed": spec["measurement"]["seed"]}
    if spec_file:
        info["spec_identical_to_registered"] = json.load(open(spec_file)) == spec
    # scalar certificates, each distinct (curve, target, recovered) once
    cs = recs[0]["workload"]["curve_spec"]
    E = curve_of(cs); G = (cs["gx"], cs["gy"])
    seen = {}
    bad = 0
    for r in recs:
        T = tuple(int(v, 16) for v in r["workload"]["target"])
        key = (T, r["outcome"]["recovered"])
        if key not in seen:
            k = int(r["outcome"]["recovered"])
            seen[key] = E.on(T) and 0 <= k < cs["r"] and smul(E, k, G) == T
        bad += not seen[key]
    certs["records_checked"] += len(recs); certs["failures"] += bad
    certs["distinct_target_log_pairs"] += len(seen)
    info["scalar_certificate_failures"] = bad
    # pricing and pairing
    by = defaultdict(dict)  # arm -> (workload_id, round) -> record
    for r in recs:
        by[r["arm"]][(r["workload"]["workload_id"], r["round"], r["warmup"])] = r
    parts = {}
    cal_dev = 0.0
    for r in recs:
        pp = price_parts(r)
        parts[r["record_id"]] = pp
        if pp[1] or pp[4]:
            cal_dev = max(cal_dev, abs(pp[1] - pp[4]) / max(1.0, abs(pp[4])))
    info["calibration_C_vs_phase_residual_max_rel_dev"] = cal_dev
    # A/A and per-arm means
    S = {}
    for arm in by:
        meas = [r for k, r in by[arm].items() if not k[2] and r["outcome"]["status"] == "verified"]
        sq = math.sqrt(cs["r"])
        S[arm] = {WKEY[w]: sum(priced(parts[r["record_id"]], w) / sq for r in meas) / len(meas) for w in WEIGHTS}
        S[arm]["runs"] = len(meas)
    info["mean_S"] = S
    # pairs for every comparison the protocol names, measured runs only, keyed (workload, round)
    for (a, b) in [("ksum5", "ksum5-p"), ("ksum4", "ksum4-p"), ("rho-neg", "rho-neg-aa"),
                   ("rho-neg", "ksum5-p"), ("rho-neg", "ksum5"), ("rho-neg", "ksum4-p"), ("rho-neg", "ksum4")]:
        for w in WEIGHTS:
            strata = defaultdict(list)
            same_seed = True
            for k, ra in by[a].items():
                if k[2]: continue
                rb = by[b].get(k)
                if rb is None: continue
                same_seed &= ra["algorithm_seed"] == rb["algorithm_seed"]
                strata[k[0]].append((priced(parts[ra["record_id"]], w), priced(parts[rb["record_id"]], w)))
            pairs_store[(name, a, b, WKEY[w])] = (list(strata.values()), same_seed)
    # A/A identity, count-level
    aa_diff = 0
    for k, ra in by["rho-neg"].items():
        rb = by["rho-neg-aa"][k]
        aa_diff += (ra["cost"]["total_gae"], ra["counters"], ra["outcome"]["recovered"]) != \
                   (rb["cost"]["total_gae"], rb["counters"], rb["outcome"]["recovered"])
    info["aa_pairs_differing"] = aa_diff
    # correctness: every round incl. warm-up, every target
    for kk in (4, 5):
        u, p = "ksum%d" % kk, "ksum%d-p" % kk
        diffs = Counter(); disallowed = Counter(); saved = Counter(); n = 0
        gae_minus_adds = 0.0
        for key, ru in by[u].items():
            rp = by[p].get(key)
            if rp is None:
                disallowed["missing -p record"] += 1; continue
            n += 1
            out = []
            deep_diff(ru, rp, "", out)
            for path in out:
                if volatile(path): continue
                diffs[path] += 1
                if path not in ALLOWED_DIFF: disallowed[path] += 1
            su = {q["name"]: q for q in ru["phases"]}["oracle_setup"]
            sp = {q["name"]: q for q in rp["phases"]}["oracle_setup"]
            if sp["adds"] > su["adds"]: disallowed["oracle_setup adds increased"] += 1
            saved[su["adds"] - sp["adds"]] += 1
            gae_minus_adds = max(gae_minus_adds, abs((ru["cost"]["total_gae"] - rp["cost"]["total_gae"]) - (su["adds"] - sp["adds"])))
            # build audit: formula vs record
            fb = ru["factor_base"]
            a_, sp_ = fb["abscissae"], fb["signed_points"]
            bkey = "%s:%s" % (name[:3], u)
            ba = build_audit.setdefault(bkey, {"abscissae": a_, "signed_points": sp_,
                                               "formula_unpruned": build_adds_formula(a_, kk, False),
                                               "formula_pruned": build_adds_formula(a_, kk, True),
                                               "recorded_unpruned_setup_adds": Counter(),
                                               "recorded_pruned_setup_adds": Counter(),
                                               "insert_attempts": Counter(), "expected_insert_attempts": (1 << (kk - 1)) * C(a_, kk)})
            ba["recorded_unpruned_setup_adds"][su["adds"]] += 1
            ba["recorded_pruned_setup_adds"][sp["adds"]] += 1
            ba["insert_attempts"][su["work_native"]["table_insert_attempts"]] += 1
        corr[name + ":" + u] = {"pairs": n, "differing_paths": dict(diffs), "disallowed": dict(disallowed),
                               "saved_adds_values": {str(k): v for k, v in saved.items()},
                               "max_abs_total_gae_diff_minus_saved_adds": gae_minus_adds}
    out_sessions[name] = info
    return recs


def stringify(o):
    if isinstance(o, dict): return {str(k): stringify(v) for k, v in o.items()}
    if isinstance(o, list): return [stringify(v) for v in o]
    return o


def main():
    binary_sha = open(os.path.join(STUDY, "BINARY.sha256")).read().split()[0]
    own = {"schema": "validator.recompute/v1", "binary_sha256_registered": binary_sha,
           "sessions": {}, "certificates": {"records_checked": 0, "failures": 0, "distinct_target_log_pairs": 0},
           "curves": {}, "ksum_build_audit": {}, "correctness": {}, "rows": {}, "fresh_sessions": {},
           "default_off_vs_prebuild_siblings": {}}
    pairs = {}
    for row in ROWS:
        for rep in (1, 2, 3):
            name = "%s-r%d" % (row, rep)
            recs = analyse_session(name, os.path.join(STUDY, "sessions", name), os.path.join(STUDY, "specs", name + ".json"),
                                   binary_sha, own["sessions"], own["certificates"], own["ksum_build_audit"],
                                   own["correctness"], pairs)
            if rep == 1:
                own["curves"][row] = dict(recs[0]["workload"]["curve_spec"], slug=recs[0]["workload"]["curve"]["slug"],
                                          **curve_facts(recs[0]["workload"]["curve_spec"]))
            # default-off path: this study's unpruned arms against the sibling (pre-prune binary) records
            sib = os.path.join(SIBLINGS[row[0]], "sessions", name)
            if os.path.isdir(sib):
                _, srecs, _, _ = load_session(sib)
                sb = {(r["arm"], r["round"], r["workload"]["target_index"]): r for r in srecs if r["arm"] in ("ksum4", "ksum5", "rho-neg")}
                res = Counter(); n = 0
                for r in recs:
                    if r["arm"] not in ("ksum4", "ksum5", "rho-neg"): continue
                    s = sb.get((r["arm"], r["round"], r["workload"]["target_index"]))
                    if s is None: res["missing in sibling"] += 1; continue
                    n += 1
                    out = []
                    deep_diff(s, r, "", out)
                    for path in out:
                        if not volatile(path): res[path] += 1
                own["default_off_vs_prebuild_siblings"][name] = {"sibling": os.path.relpath(sib, ROOT), "pairs": n,
                                                                 "sibling_binary_sha256": srecs[0].get("binary_sha256"),
                                                                 "differing_paths": dict(res)}
            del recs
    # ---- rows -------------------------------------------------------------------------------
    for row in ROWS:
        names = ["%s-r%d" % (row, rep) for rep in (1, 2, 3)]
        R = {"mean_S": {}, "ratios": {}}
        for arm in ARMS:
            R["mean_S"][arm] = {WKEY[w]: sum(own["sessions"][n]["mean_S"][arm][WKEY[w]] for n in names) / 3 for w in WEIGHTS}
        for (a, b) in [("ksum5", "ksum5-p"), ("ksum4", "ksum4-p"), ("rho-neg", "rho-neg-aa"), ("rho-neg", "ksum5-p"),
                       ("rho-neg", "ksum5"), ("rho-neg", "ksum4-p"), ("rho-neg", "ksum4")]:
            key = "%s_over_%s" % (b, a)
            R["ratios"][key] = {}
            for w in WEIGHTS:
                wk = WKEY[w]
                allstrata = []; per = {}; same = True
                for n in names:
                    st, ss = pairs[(n, a, b, wk)]
                    same &= ss
                    allstrata.extend(st)
                    clusters = [(sum(x for x, _ in s), sum(y for _, y in s)) for s in st]
                    rr, ci, se, df = lin_ratio(clusters)
                    entry = {"ratio": rr, "lin_ci95": ci, "df": df, "pairs": sum(len(s) for s in st)}
                    if a.startswith("ksum"):
                        entry["boot_ci95"] = cluster_boot(st, RESAMPLES, BOOT_SEED)
                    per[n] = entry
                clusters = [(sum(x for x, _ in s), sum(y for _, y in s)) for s in allstrata]
                rr, ci, se, df = lin_ratio(clusters)
                pooled = {"ratio": rr, "lin_ci95": ci, "se": se, "df": df, "pairs": sum(len(s) for s in allstrata),
                          "clusters": len(allstrata), "same_algorithm_seeds": same}
                if a.startswith("ksum"):
                    pooled["boot_ci95"] = cluster_boot(allstrata, RESAMPLES, BOOT_SEED)
                R["ratios"][key][wk] = {"pooled": pooled, "per_replicate": per}
        # registered rule, both noise readings
        for kk in (5, 4):
            key = "ksum%d-p_over_ksum%d" % (kk, kk)
            ups_pooled = [max(R["ratios"][key][WKEY[w]]["pooled"]["lin_ci95"][1], R["ratios"][key][WKEY[w]]["pooled"]["boot_ci95"][1]) for w in WEIGHTS]
            ups_rep = [max(v["lin_ci95"][1], v["boot_ci95"][1]) for w in WEIGHTS for v in R["ratios"][key][WKEY[w]]["per_replicate"].values()]
            aa = max(abs(R["ratios"]["rho-neg-aa_over_rho-neg"][WKEY[w]]["pooled"]["ratio"] - 1) for w in WEIGHTS)
            mx = max(ups_pooled + ups_rep)
            delta = 1.0
            for arm in ("rho-neg", "ksum%d" % kk, "ksum%d-p" % kk):
                v = [own["sessions"][n]["mean_S"][arm]["0"] for n in names]
                delta = max(delta, max(v) / min(v))
            R["rule_k%d" % kk] = {"max_upper_pooled": max(ups_pooled), "max_upper_replicate": max(ups_rep),
                                  "aa_max_dev": aa, "holds_paired_aa_gate": (1 - mx) > aa and mx < 1,
                                  "sibling_delta": delta, "holds_sibling_delta_gate": (1 - mx) > (delta - 1)}
            R["gap_to_rho_neg_k%d" % kk] = {WKEY[w]: {"unpruned": R["mean_S"]["ksum%d" % kk][WKEY[w]] / R["mean_S"]["rho-neg"][WKEY[w]],
                                                      "pruned": R["mean_S"]["ksum%d-p" % kk][WKEY[w]] / R["mean_S"]["rho-neg"][WKEY[w]]} for w in WEIGHTS}
        own["rows"][row] = R
    # ---- fresh sessions ------------------------------------------------------------------------
    for d in FRESH:
        name = os.path.basename(d.rstrip("/"))
        fs = {}; fcorr = {}; fpairs = {}; fba = {}; fcert = {"records_checked": 0, "failures": 0, "distinct_target_log_pairs": 0}
        analyse_session(name, d, None, binary_sha, fs, fcert, fba, fcorr, fpairs)
        F = {"session": fs[name], "certificates": fcert, "correctness": fcorr, "build_audit": fba, "ratios": {}}
        for (a, b) in [("ksum5", "ksum5-p"), ("ksum4", "ksum4-p"), ("rho-neg", "ksum5-p"), ("rho-neg", "ksum5"), ("rho-neg", "rho-neg-aa")]:
            key = "%s_over_%s" % (b, a); F["ratios"][key] = {}
            for w in WEIGHTS:
                st, same = fpairs[(name, a, b, WKEY[w])]
                clusters = [(sum(x for x, _ in s), sum(y for _, y in s)) for s in st]
                rr, ci, se, df = lin_ratio(clusters)
                e = {"ratio": rr, "lin_ci95": ci, "df": df, "same_algorithm_seeds": same}
                if a.startswith("ksum"): e["boot_ci95"] = cluster_boot(st, RESAMPLES, BOOT_SEED)
                F["ratios"][key][WKEY[w]] = e
        own["fresh_sessions"][name] = F
    json.dump(stringify(own), open(OUT, "w"), indent=1, sort_keys=True)
    print("wrote", OUT)


if __name__ == "__main__":
    main()
