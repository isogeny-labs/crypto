#!/usr/bin/env python3
"""Validator's independent recomputation for XP-20261029-srenew-deferred-guard-v3.

Stdlib only; imports nothing from ecbench and runs no ecbench code.  Reads one
session's records.jsonl and plan.json (plus the frozen spec, the worker's
input-spec.json and l2-gate.json for the instrument checks) and recomputes from
the raw records:

  * completeness per arm (records, warm-ups, measured, statuses, matches_target,
    censored) and pairing on (workload, round) with equal algorithm seeds;
  * the section 5a priced cost S(w) = (G - C + w (U + n_C)) / sqrt(r) with
    C = n_C = 0 (checked: no record names a pinned calibration unit) and U the
    counts behind the record's `cost.unpriced` names (top-level counter of that
    name, else the sum over phases of native[name] or native[name minus
    "_uncharged"]; the definition written in band.rs);
  * paired ratios of totals  sum S_b / sum S_a  pooled and per curve at
    w = 0, 0.1, 1, summed in ecbench's order (workload id, then round) so the
    point ratios can be compared bitwise;
  * intervals: (i) a replica, written from the documented source (stats.rs
    `cluster_bootstrap_ci`, `Resampler`, canonical.rs `splitmix64`,
    `quantile`), of ecbench's two-stage cluster bootstrap, 4000 resamples,
    seed 20261002; (ii) the validator's own two-stage bootstrap with Python's
    Mersenne Twister, seed 29102026 and ten further seeds for the
    decision-bearing upper bound; (iii) a one-stage bootstrap over targets;
  * A/A exactness pair by pair;
  * band means per curve and arm (record order, as band.rs sums) and the
    ordering at every weight;
  * the v3 section 6.1 instrument recomputed from the records (isolation level
    of every execution, warm-ups included; the L2 blockers on every execution
    below L2; the solve wall times of those executions), cross-checked against
    l2-gate.json; spec JSON-equality to the frozen spec;
  * that the candidate and the baseline differ only in `targets` and
    `factor_base`;
  * the deferred-guard ledger, pair by pair, against the eager guard at the
    same N (dg vs srenew at N_inc, dg-small vs srenew-small at N_b);
  * GAE bookkeeping: phase gae = adds + doubles, total = sum of phases,
    oracle_setup adds = N (N + 1), s = total / sqrt(r);
  * an independent elliptic-curve check: a, b derived from plan.json points;
    p, r prime; points on the curve; [r]G = O; r the only multiple of r in the
    Hasse interval (so #E = r); j-invariant equal to the ICV1 string's; and
    [d]G = Q in affine coordinates for every record (warm-ups included).

Usage: python3 -I recompute.py <session_dir> <label> <frozen_spec> <input_spec> <l2_gate> <baseline_arm>
Prints JSON.
"""
import json
import math
import random
import sys
from collections import defaultdict

WEIGHTS = (0.0, 0.1, 1.0)
RESAMPLES = 4000
ECBENCH_SEED = 20261002
OWN_SEED = 29102026
M64 = (1 << 64) - 1


# ---------------------------------------------------------------- arithmetic
def is_prime(n):
    if n < 2:
        return False
    small = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for q in small:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in small:
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


class Curve:
    """Short Weierstrass y^2 = x^3 + a x + b over F_p, affine, None = O."""

    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a, b

    def on(self, P):
        if P is None:
            return True
        x, y = P
        return (y * y - (x * x * x + self.a * x + self.b)) % self.p == 0

    def add(self, P, Q):
        p = self.p
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return None
            lam = (3 * x1 * x1 + self.a) * pow(2 * y1, -1, p) % p
        else:
            lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (lam * lam - x1 - x2) % p
        return (x3, (lam * (x1 - x3) - y1) % p)

    def mul(self, k, P):
        R = None
        A = P
        while k:
            if k & 1:
                R = self.add(R, A)
            A = self.add(A, A)
            k >>= 1
        return R


def pt(v):
    return (int(v[0], 16), int(v[1], 16))


# ------------------------------------------------- ecbench bootstrap replica
def splitmix64(x):
    x = (x + 0x9E3779B97F4A7C15) & M64
    z = x
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
    return z ^ (z >> 31)


def quantile(xs, q):
    v = sorted(xs)
    pos = min(max(q, 0.0), 1.0) * float(len(v) - 1)
    lo, hi = math.floor(pos), math.ceil(pos)
    return v[lo] + (v[hi] - v[lo]) * (pos - lo)


def ratio_of_sums(pairs):
    sa = sb = 0.0
    n = 0
    for a, b in pairs:
        sa += a
        sb += b
        n += 1
    return sb / sa if n > 0 and sa > 0.0 else None


def boot_ecbench(strata, resamples=RESAMPLES, seed=ECBENCH_SEED):
    """stats.rs `resample(.., clusters=true, ..)` with `Resampler`: draw
    len(strata) strata with replacement, then len(s) items inside each drawn
    stratum; statistic = ratio of sums over the flattened draw; percentile
    interval by linear interpolation (`quantile`)."""
    state = seed & M64
    k = len(strata)
    vals = []
    for _ in range(resamples):
        picked = []
        for _ in range(k):
            state = splitmix64(state)
            picked.append(strata[state % k])
        sa = sb = 0.0
        n = 0
        for s in picked:
            m = len(s)
            for _ in range(m):
                state = splitmix64(state)
                a, b = s[state % m]
                sa += a
                sb += b
                n += 1
        if n > 0 and sa > 0.0:
            v = sb / sa
            if math.isfinite(v):
                vals.append(v)
    if len(vals) < resamples * 9 // 10:
        return None
    return [quantile(vals, 0.025), quantile(vals, 0.975)]


def boot_own_two_stage(strata, seed, resamples=RESAMPLES):
    """Validator's own RNG (Mersenne Twister), same two-stage scheme,
    nearest-rank percentiles."""
    rng = random.Random(seed)
    k = len(strata)
    vals = []
    for _ in range(resamples):
        sa = sb = 0.0
        for _ in range(k):
            s = strata[rng.randrange(k)]
            m = len(s)
            for _ in range(m):
                a, b = s[rng.randrange(m)]
                sa += a
                sb += b
        vals.append(sb / sa)
    vals.sort()
    return [vals[int(0.025 * resamples)], vals[int(0.975 * resamples) - 1]]


def boot_one_stage(strata, seed, resamples=RESAMPLES):
    """Targets resampled with replacement, all pairs of a drawn target kept."""
    rng = random.Random(seed)
    tot = [(sum(a for a, _ in s), sum(b for _, b in s)) for s in strata]
    k = len(tot)
    vals = []
    for _ in range(resamples):
        sa = sb = 0.0
        for _ in range(k):
            a, b = tot[rng.randrange(k)]
            sa += a
            sb += b
        vals.append(sb / sa)
    vals.sort()
    return [vals[int(0.025 * resamples)], vals[int(0.975 * resamples) - 1]]


# ---------------------------------------------------------------- records
def unpriced_count(rec):
    total = 0
    detail = {}
    for name in rec["cost"].get("unpriced", []):
        base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
        if name in rec.get("counters", {}):
            v = rec["counters"][name]
            detail[name] = v
            total += v
            continue
        found = False
        sub = 0
        for ph in rec.get("phases", []):
            nat = ph.get("native", {}) or {}
            if name in nat:
                sub += nat[name]
                found = True
            elif base in nat:
                sub += nat[base]
                found = True
        if not found:
            raise SystemExit(f"{rec['record_id']}: unpriced {name} has no count")
        detail[name] = sub
        total += sub
    return total, detail


def phase(rec, name):
    for ph in rec["phases"]:
        if ph["name"] == name:
            return ph
    return None


def nat(rec, ph, key):
    p = phase(rec, ph)
    return (p.get("native") or {}).get(key, 0) if p else 0


def fb_size(rec):
    fbp = rec["method"]["params"].get("factor_base", "")
    return int(fbp.split("size=")[1]) if "size=" in fbp else None


def main():
    sdir, label, frozen_path, input_spec_path, l2_path, baseline = sys.argv[1:7]
    plan = json.load(open(f"{sdir}/plan.json"))
    session = json.load(open(f"{sdir}/session.json"))
    recs = [json.loads(l) for l in open(f"{sdir}/records.jsonl")]
    frozen = json.load(open(frozen_path))
    out = {"label": label, "session_dir": sdir, "records": len(recs), "baseline_arm": baseline}
    out["session"] = {k: session.get(k) for k in (
        "session_id", "status", "executions_planned", "records_written", "grading_version",
        "binary_sha256", "git_commit", "git_dirty", "env_class_id", "status_counts")}
    out["binary_sha256_in_records"] = sorted({r.get("binary_sha256") for r in recs})
    out["session_ids_in_records"] = sorted({r["session_id"] for r in recs})

    # ---- spec equality
    sess_spec = json.load(open(f"{sdir}/spec.json"))
    inp_spec = json.load(open(input_spec_path))
    out["spec"] = {
        "input_spec_equals_frozen": inp_spec == frozen,
        "session_spec_equals_frozen": sess_spec == frozen,
        "frozen_arms": [a["name"] for a in frozen["arms"]],
        "frozen_measurement": frozen["measurement"],
        "frozen_curves": frozen["workloads"]["curves"],
    }
    arm_role = {a["name"]: a["role"] for a in frozen["arms"]}
    arm_method = {a["name"]: a["method"]["id"] for a in frozen["arms"]}

    # ---- curves from plan.json, independent EC checks
    curves = {}
    for wl in plan["workloads"]:
        c = wl["curve"]
        slug = c["slug"]
        icv = c["icv1"].split(":")
        cur = curves.setdefault(slug, {"p": int(icv[1].split("-")[1]), "t": int(icv[2]),
                                       "r_icv1": int(icv[3]), "j_icv1": icv[4],
                                       "r": int(c["r"]), "G": pt(c["generator"]),
                                       "targets": {}, "icv1": c["icv1"]})
        cur["targets"][wl["workload_id"]] = (wl["target_index"], pt(wl["target"]))
    ec = {}
    curve_checks = {}
    for slug, c in sorted(curves.items()):
        p, r, G = c["p"], c["r"], c["G"]
        pts = [G] + [q for _, q in c["targets"].values()]
        (x1, y1) = pts[0]
        (x2, y2) = next(q for q in pts if q[0] != x1)
        a = ((y1 * y1 - x1 ** 3) - (y2 * y2 - x2 ** 3)) * pow(x1 - x2, -1, p) % p
        b = (y1 * y1 - x1 ** 3 - a * x1) % p
        E = Curve(p, a, b)
        ec[slug] = E
        disc = (4 * a ** 3 + 27 * b ** 2) % p
        j = 1728 * 4 * a ** 3 * pow(disc, -1, p) % p if disc else None
        lo = p + 1 - 2 * math.isqrt(p) - 2
        hi = p + 1 + 2 * math.isqrt(p) + 2
        multiples = [m for m in range(r * max(1, lo // r), hi + 1, r) if lo <= m <= hi]
        curve_checks[slug] = {
            "p": p, "a": a, "b": b, "r": r, "trace": c["t"], "icv1": c["icv1"],
            "r_equals_icv1": r == c["r_icv1"],
            "r_equals_p_plus_1_minus_t": r == p + 1 - c["t"],
            "p_bits": p.bit_length(),
            "p_prime": is_prime(p), "r_prime": is_prime(r), "nonsingular": disc != 0,
            "j_invariant": j, "j_equals_icv1": str(j) == c["j_icv1"],
            "j_not_0_or_1728": j not in (0, 1728 % p),
            "all_points_on_curve": all(E.on(q) for q in pts),
            "rG_is_identity": E.mul(r, G) is None,
            "r_only_multiple_in_hasse_interval": multiples == [r],
            "targets": len(c["targets"]),
        }
    out["curves"] = curve_checks

    # ---- completeness, [d]G = Q for every record, isolation
    by_arm = defaultdict(list)
    for rec in recs:
        by_arm[rec["arm"]].append(rec)
    comp = {}
    dg_fail = []
    dg_checked = defaultdict(int)
    calib = 0
    for arm, rs in sorted(by_arm.items()):
        m = [x for x in rs if not x["warmup"]]
        st = defaultdict(int)
        for x in m:
            st[x["outcome"]["status"]] += 1
        comp[arm] = {
            "role": arm_role.get(arm), "method": arm_method.get(arm),
            "records": len(rs), "warmup": len(rs) - len(m), "measured": len(m),
            "statuses_measured": dict(st),
            "matches_target_true_measured": sum(1 for x in m if x["outcome"].get("matches_target") is True),
            "censored_measured": sum(1 for x in m if x["outcome"]["status"] != "verified"
                                     or x["cost"].get("total_gae") is None),
            "statuses_all": dict(sorted(defaultdict(int, {s: sum(1 for x in rs if x["outcome"]["status"] == s)
                                                          for s in {x["outcome"]["status"] for x in rs}}).items())),
        }
        for x in rs:
            if x.get("detail") and x["detail"].get("calibration_pinned_units"):
                calib += 1
            wl = x["workload"]
            slug = wl["curve"]["slug"]
            E = ec[slug]
            G = pt(wl["curve"]["generator"])
            Q = pt(wl["target"])
            d = int(x["outcome"]["recovered"])
            dg_checked["warmup" if x["warmup"] else "measured"] += 1
            if E.mul(d, G) != Q or Q != curves[slug]["targets"][wl["workload_id"]][1] or G != curves[slug]["G"]:
                dg_fail.append(x["record_id"])
    out["completeness"] = comp
    out["records_with_pinned_calibration"] = calib
    out["dG_eq_Q"] = {"checked": dict(dg_checked), "failures": dg_fail,
                      "by_arm_measured": {a: sum(1 for x in rs if not x["warmup"]) for a, rs in sorted(by_arm.items())}}

    # ---- v3 section 6.1 instrument, recomputed from records
    BL = "L2: solve too short to admit wall time"
    lev = defaultdict(lambda: defaultdict(int))
    below = []
    l3_blockers = set()
    ic_below_l2 = []
    rho_bad = []
    rho_waived = defaultdict(list)
    non_l2_blockers_on_l2 = set()
    for x in recs:
        L = x["isolation"]["level"]
        lev[x["arm"]][L] += 1
        bl = x["isolation"].get("blockers", [])
        for b_ in bl:
            if b_.startswith("L3: "):
                l3_blockers.add(b_)
        is_ic = x["method"]["id"] == "ic.pipeline"
        if L in ("L2", "L3"):
            for b_ in bl:
                if b_.startswith("L2: "):
                    non_l2_blockers_on_l2.add(b_)
            continue
        below.append(x["seq"])
        if is_ic:
            ic_below_l2.append(x["seq"])
            continue
        l2b = [b_ for b_ in bl if b_.startswith("L2: ")]
        ok = (L == "L1" and l2b and all(b_.startswith(BL) for b_ in l2b)
              and x["arm"] in ("rho-neg", "rho-neg-aa"))
        if ok:
            rho_waived[x["arm"]].append((x["seq"], x["time"]["solve_wall_ns"], x["warmup"]))
        else:
            rho_bad.append({"seq": x["seq"], "arm": x["arm"], "level": L, "blockers": bl})
    # solve time threshold consistency for the rho arms
    rho_solve_lt_1ms_l2 = [x["seq"] for x in recs if x["arm"] in ("rho-neg", "rho-neg-aa")
                           and x["time"]["solve_wall_ns"] < 1_000_000 and x["isolation"]["level"] in ("L2", "L3")]
    rho_solve_ge_1ms_l1 = [x["seq"] for x in recs if x["arm"] in ("rho-neg", "rho-neg-aa")
                           and x["time"]["solve_wall_ns"] >= 1_000_000 and x["isolation"]["level"] == "L1"]
    gate = json.load(open(l2_path))
    gate_below = sorted(e["seq"] for e in gate["below_l2"])
    gate_below_arms = sorted({e["arm"] for e in gate["below_l2"]})
    gate_below_l2_blockers_ok = all(
        e["isolation"]["level"] == "L1"
        and all(b_.startswith(BL) for b_ in e["isolation"]["blockers"] if b_.startswith("L2: "))
        and any(b_.startswith("L2: ") for b_ in e["isolation"]["blockers"])
        for e in gate["below_l2"])
    out["gate_v3"] = {
        "levels_by_arm_all_executions": {a: dict(v) for a, v in sorted(lev.items())},
        "executions": len(recs),
        "below_l2_from_records": len(below),
        "condition4_ic_executions_below_l2": ic_below_l2,
        "condition4_holds": not ic_below_l2,
        "condition5_rho_waived_by_arm": {
            a: {"count": len(v), "measured": sum(1 for _, _, w in v if not w),
                "warmup": sum(1 for _, _, w in v if w),
                "solve_wall_ns_min": min(s for _, s, _ in v), "solve_wall_ns_max": max(s for _, s, _ in v)}
            for a, v in sorted(rho_waived.items())},
        "condition5_rho_not_waivable": rho_bad,
        "condition5_holds": not rho_bad,
        "l3_blockers_seen": sorted(l3_blockers),
        "l2_blockers_on_l2_or_l3_records": sorted(non_l2_blockers_on_l2),
        "rho_records_solve_lt_1ms_but_graded_L2": rho_solve_lt_1ms_l2,
        "rho_records_solve_ge_1ms_but_graded_L1": rho_solve_ge_1ms_l1,
        "condition6_non_verified_records": sum(1 for x in recs if x["outcome"]["status"] != "verified"),
        "l2_gate_json": {"executions": gate["executions"], "below_l2": len(gate["below_l2"]),
                         "below_l2_arms": gate_below_arms, "grades": gate["grades"],
                         "invalid_outcomes": len(gate["invalid_outcomes"]), "accepted": gate["accepted"],
                         "below_l2_seqs_equal_records": gate_below == sorted(below),
                         "below_l2_entries_all_waivable": gate_below_l2_blockers_ok},
    }

    # ---- measured map
    meas = defaultdict(dict)
    for x in recs:
        if x["warmup"]:
            continue
        wl = x["workload"]
        U, Ud = unpriced_count(x)
        meas[x["arm"]][(wl["workload_id"], x["round"])] = {
            "G": float(x["cost"]["total_gae"]), "U": U, "Ud": Ud, "r": int(wl["curve"]["r"]),
            "slug": wl["curve"]["slug"], "seed": x["algorithm_seed"], "s_rec": x["cost"]["s"], "rec": x}
    keys = sorted(next(iter(meas.values())).keys())
    out["pairing"] = {
        "pairs_per_arm": len(keys),
        "same_keys_all_arms": all(sorted(a.keys()) == keys for a in meas.values()),
        "algorithm_seed_mismatches": sum(1 for k in keys if len({meas[a][k]["seed"] for a in meas}) != 1),
        "workloads": len({k[0] for k in keys}), "rounds": sorted({k[1] for k in keys}),
    }
    out["record_s_vs_G_over_sqrt_r_max_rel_dev"] = max(
        abs(v["s_rec"] - v["G"] / math.sqrt(v["r"])) / (v["G"] / math.sqrt(v["r"]))
        for a in meas.values() for v in a.values())
    out["unpriced_names_by_arm"] = {a: sorted({n for v in d.values() for n in v["Ud"]}) for a, d in sorted(meas.items())}

    def S(v, w):
        return (v["G"] - 0.0 + w * float(v["U"])) / math.sqrt(v["r"])

    # ---- method params
    def params(arm, k):
        return meas[arm][k]["rec"]["method"]["params"]

    def param_diff(a, b):
        diffs = set()
        for k in keys:
            pa, pb = params(a, k), params(b, k)
            for kk in set(pa) | set(pb):
                if pa.get(kk) != pb.get(kk):
                    diffs.add((kk, pa.get(kk), pb.get(kk)))
            if meas[a][k]["rec"]["method"]["id"] != meas[b][k]["rec"]["method"]["id"]:
                diffs.add(("method.id", None, None))
        return sorted(map(list, diffs))

    out["param_differences"] = {
        f"{baseline}__dg-small": param_diff(baseline, "dg-small"),
        f"{baseline}__{baseline}-aa": param_diff(baseline, f"{baseline}-aa"),
        "srenew__dg": param_diff("srenew", "dg"),
        "srenew-small__dg-small": param_diff("srenew-small", "dg-small"),
        "srenew__srenew-small": param_diff("srenew", "srenew-small"),
    }
    fz = {a["name"]: a["method"].get("params", {}) for a in frozen["arms"]}
    out["frozen_spec_param_differences_candidate_vs_baseline"] = sorted(
        [kk, fz[baseline].get(kk), fz["dg-small"].get(kk)] for kk in set(fz[baseline]) | set(fz["dg-small"])
        if fz[baseline].get(kk) != fz["dg-small"].get(kk))

    # ---- ratios and bootstraps
    other = "walk" if baseline == "srenew" else "srenew"
    PAIRS = [(baseline, "dg-small"), (baseline, f"{baseline}-aa"), ("rho-neg", "rho-neg-aa"),
             ("rho-neg", baseline), ("rho-neg", "dg-small"), ("srenew", "dg"), ("srenew", "srenew-small"),
             ("srenew-small", "dg-small"), ("walk", "srenew") if baseline == "walk" else ("srenew", "walk"),
             ("srenew", "dg-small") if baseline == "walk" else ("walk", "dg-small")]
    decision = f"{baseline}__dg-small"

    def strata_for(a, b, w, slug=None):
        st = defaultdict(list)
        for k in keys:  # sorted by (workload id, round): ecbench's BTreeMap order
            if slug and meas[a][k]["slug"] != slug:
                continue
            st[k[0]].append((S(meas[a][k], w), S(meas[b][k], w)))
        return [st[wid] for wid in sorted(st)]

    ratios = {}
    for a, b in PAIRS:
        name = f"{a}__{b}"
        ratios[name] = {}
        for w in WEIGHTS:
            strata = strata_for(a, b, w)
            flat = [pr for s in strata for pr in s]
            e = {"ratio_b_over_a": ratio_of_sums(flat),
                 "mean_s_a": sum(x for x, _ in flat) / len(flat), "mean_s_b": sum(y for _, y in flat) / len(flat),
                 "pairs": len(flat), "workloads": len(strata)}
            e["ci95_ecbench_replica"] = boot_ecbench(strata)
            e["ci95_own_two_stage"] = boot_own_two_stage(strata, OWN_SEED)
            e["ci95_own_one_stage_targets"] = boot_one_stage(strata, OWN_SEED)
            if name == decision:
                ups = [boot_own_two_stage(strata, OWN_SEED + i)[1] for i in range(1, 11)]
                e["own_two_stage_upper_over_10_more_seeds"] = {"min": min(ups), "max": max(ups),
                                                               "all_below_1": all(u < 1 for u in ups)}
            pc = {}
            for slug in sorted(curves):
                cs = strata_for(a, b, w, slug)
                pc[slug] = {"ratio_b_over_a": ratio_of_sums([pr for s in cs for pr in s]),
                            "ci95_ecbench_replica": boot_ecbench(cs)}
            e["per_curve"] = pc
            ratios[name][str(w)] = e
    out["ratios"] = ratios

    # ---- section 6 reading (validator's own)
    d = ratios[decision]
    out["section6"] = {
        "decision_pair": decision,
        "upper_bound_below_1_every_weight_ecbench_replica": all(d[str(w)]["ci95_ecbench_replica"][1] < 1 for w in WEIGHTS),
        "upper_bound_below_1_every_weight_own_two_stage": all(d[str(w)]["ci95_own_two_stage"][1] < 1 for w in WEIGHTS),
        "point_below_1_every_weight": all(d[str(w)]["ratio_b_over_a"] < 1 for w in WEIGHTS),
        "upper_bounds": {str(w): d[str(w)]["ci95_ecbench_replica"][1] for w in WEIGHTS},
        "regression_test_upper_of_inverse_below_1_every_weight": all(
            1.0 / d[str(w)]["ci95_ecbench_replica"][0] < 1 for w in WEIGHTS),
    }

    # ---- A/A exactness
    def same(x, y):
        if x["cost"]["total_gae"] != y["cost"]["total_gae"]:
            return False
        if unpriced_count(x)[1] != unpriced_count(y)[1]:
            return False
        px = [(p["name"], p["adds"], p["doubles"], p["scalar_mults"], p["gae"], p.get("native")) for p in x["phases"]]
        py = [(p["name"], p["adds"], p["doubles"], p["scalar_mults"], p["gae"], p.get("native")) for p in y["phases"]]
        return px == py and x["outcome"]["recovered"] == y["outcome"]["recovered"]

    out["aa_exact"] = {
        f"{baseline}-aa_vs_{baseline}_pairs_identical": sum(
            1 for k in keys if same(meas[baseline][k]["rec"], meas[f"{baseline}-aa"][k]["rec"])),
        "rho-neg-aa_vs_rho-neg_pairs_identical": sum(
            1 for k in keys if same(meas["rho-neg"][k]["rec"], meas["rho-neg-aa"][k]["rec"])),
        "pairs": len(keys),
        "ratios_exactly_1": {n: {str(w): ratios[n][str(w)]["ratio_b_over_a"] == 1.0 for w in WEIGHTS}
                             for n in (f"{baseline}__{baseline}-aa", "rho-neg__rho-neg-aa")},
    }

    # ---- band means per curve and arm, record order
    band = {}
    for slug in sorted(curves):
        band[slug] = {}
        for arm in sorted(by_arm):
            vals = [x for x in by_arm[arm] if not x["warmup"] and x["workload"]["curve"]["slug"] == slug]
            row = {}
            for w in WEIGHTS:
                tot = 0.0
                for x in vals:
                    v = meas[arm][(x["workload"]["workload_id"], x["round"])]
                    tot += S(v, w)
                row[str(w)] = tot / len(vals)
            band[slug][arm] = row
        band[slug]["dg-small_lt_baseline_every_weight"] = all(
            band[slug]["dg-small"][str(w)] < band[slug][baseline][str(w)] for w in WEIGHTS)
        band[slug]["dg-small_gt_baseline_every_weight"] = all(
            band[slug]["dg-small"][str(w)] > band[slug][baseline][str(w)] for w in WEIGHTS)
        band[slug]["rho_lt_dg-small_every_weight"] = all(
            band[slug]["rho-neg"][str(w)] < band[slug]["dg-small"][str(w)] for w in WEIGHTS)
    out["band_mean_s"] = band
    out["band_dg-small_lt_baseline_on_every_curve"] = all(
        band[s]["dg-small_lt_baseline_every_weight"] for s in curves)

    # ---- GAE bookkeeping, every measured IC record
    book = {"ic_records": 0, "phase_gae_eq_adds_plus_doubles": 0, "total_gae_eq_sum_phase_gae": 0,
            "oracle_setup_adds_eq_N_N_plus_1": 0, "fb_abscissae_eq_N": 0, "oracle_setup_doubles_0": 0,
            "eager_guard_probes_eq_trials": 0, "eager_records": 0,
            "deferred_probes_eq_relations_plus_repeats": 0, "deferred_records": 0}
    for arm, dd in meas.items():
        for k, v in dd.items():
            x = v["rec"]
            if x["method"]["id"] != "ic.pipeline":
                continue
            book["ic_records"] += 1
            if all(abs(p["gae"] - (p["adds"] + p["doubles"])) < 1e-9 for p in x["phases"]):
                book["phase_gae_eq_adds_plus_doubles"] += 1
            if abs(sum(p["gae"] for p in x["phases"]) - x["cost"]["total_gae"]) < 1e-9:
                book["total_gae_eq_sum_phase_gae"] += 1
            N = fb_size(x)
            os_ = phase(x, "oracle_setup")
            if os_["adds"] == N * (N + 1):
                book["oracle_setup_adds_eq_N_N_plus_1"] += 1
            if os_["doubles"] == 0:
                book["oracle_setup_doubles_0"] += 1
            if x["factor_base"]["abscissae"] == N:
                book["fb_abscissae_eq_N"] += 1
            t = x["method"]["params"]["targets"]
            if t == "stride-renew":
                book["eager_records"] += 1
                if nat(x, "relations", "target_guard_probes") == nat(x, "relations", "trials"):
                    book["eager_guard_probes_eq_trials"] += 1
            if t == "stride-renew-deferred-guard":
                book["deferred_records"] += 1
                if nat(x, "relations", "target_guard_probes") == (
                        nat(x, "relations", "relations") + nat(x, "relations", "repeated_targets_skipped")):
                    book["deferred_probes_eq_relations_plus_repeats"] += 1
    out["gae_bookkeeping"] = book

    # ---- deferred-guard ledger against the eager guard at the same N
    dgl = {}
    for eager, deferred in (("srenew", "dg"), ("srenew-small", "dg-small")):
        c = defaultdict(int)
        extra_adds = []
        for k in keys:
            e, f = meas[eager][k]["rec"], meas[deferred][k]["rec"]
            c["pairs"] += 1
            same_keys = ("trials", "relations", "stride_steps", "stride_restarts", "stride_renewals")
            if all(nat(e, "relations", q) == nat(f, "relations", q) for q in same_keys):
                c["trials_relations_stride_counters_equal"] += 1
            if e["counters"]["targets_tried"] == f["counters"]["targets_tried"]:
                c["targets_tried_equal"] += 1
            if e["outcome"]["recovered"] == f["outcome"]["recovered"]:
                c["recovered_equal"] += 1
            re_, rf = phase(e, "relations"), phase(f, "relations")
            if re_["scalar_mults"] == rf["scalar_mults"] and re_["doubles"] == rf["doubles"]:
                c["rel_scalar_mults_and_doubles_equal"] += 1
            eager_skips = nat(e, "relations", "repeated_targets_skipped")
            dec_rep = nat(f, "relations", "repeated_targets_skipped")
            if nat(f, "relations", "lookups") == nat(e, "relations", "lookups") + eager_skips:
                c["lookups_deferred_eq_eager_plus_eager_skips"] += 1
            da = rf["adds"] - re_["adds"]
            extra_adds.append((da, dec_rep, eager_skips))
            if dec_rep <= da <= eager_skips:
                c["extra_adds_between_decomposing_repeats_and_eager_skips"] += 1
            if da == dec_rep:
                c["extra_adds_eq_decomposing_repeats"] += 1
            if all(phase(e, pn)["adds"] == phase(f, pn)["adds"] and phase(e, pn)["doubles"] == phase(f, pn)["doubles"]
                   for pn in ("factor_base", "oracle_setup", "linear_algebra", "verify")):
                c["other_phases_charged_equal"] += 1
            if e["counters"].get("matrix_rows") == f["counters"].get("matrix_rows") and \
               e["counters"].get("matrix_rank") == f["counters"].get("matrix_rank"):
                c["matrix_rows_rank_equal"] += 1
            c["sum_extra_adds"] += da
            c["sum_decomposing_repeats"] += dec_rep
            c["sum_eager_skips"] += eager_skips
            c["sum_eager_probes"] += nat(e, "relations", "target_guard_probes")
            c["sum_deferred_probes"] += nat(f, "relations", "target_guard_probes")
            c["sum_deferred_minus_eager_lookups"] += nat(f, "relations", "lookups") - nat(e, "relations", "lookups")
            c["sum_eager_total_gae"] += e["cost"]["total_gae"]
            c["sum_deferred_total_gae"] += f["cost"]["total_gae"]
            c["sum_eager_U"] += meas[eager][k]["U"]
            c["sum_deferred_U"] += meas[deferred][k]["U"]
        dgl[f"{eager}__{deferred}"] = dict(c)
    out["deferred_guard_ledger"] = dgl
    print(json.dumps(out, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
