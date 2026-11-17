#!/usr/bin/env python3
"""Independent recomputation for IC-LINE-LAYOUT-E2E-FP28-V6-20261106 (+ V6-AMENDMENT-1).

Validator-owned code. It shares nothing with analyze-v6.sh or ecbench: it reads the raw
`records.jsonl` of each session and recomputes, from first principles,

  * the paired candidate/incumbent total solve-wall ratio and its linearised-t 95% CI,
  * the incumbent-aa/incumbent A/A ratio and CI,
  * the A/A-corrected ratio, its Welch-Satterthwaite dof and CI,
  * the pc-fb2x positive control (wall and counted),
  * counted (GAE) parity, ops ratio, record-level parity, cross-session parity,
  * the counted IC/rho S ratio at unpriced-work weights w = 0, 0.1, 1 and the per-curve band,
  * each arm's block positions under alternate_measured.

Only after computing does it open the producer's analysis JSON (paths given on the command
line) to compare. Run with `python3 -I`; every input path is an argument.

usage: recompute.py OUT_JSON --wall NAME=SESSION_DIR ... --counted SESSION_DIR
                    [--producer NAME=ANALYSIS_DIR:INDEX ...] [--producer-counted ANALYSIS_DIR ...]
"""
import json
import math
import sys
from collections import defaultdict

# ---------------------------------------------------------------- Student t quantile

def _betacf(a, b, x):
    """Continued fraction for the regularised incomplete beta (modified Lentz)."""
    tiny = 1e-300
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = 1.0 / (d if abs(d) > tiny else tiny)
    h = d
    for m in range(1, 10000):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > tiny else tiny)
        c = 1.0 + aa / c
        c = c if abs(c) > tiny else tiny
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > tiny else tiny)
        c = 1.0 + aa / c
        c = c if abs(c) > tiny else tiny
        delta = d * c
        h *= delta
        if abs(delta - 1.0) < 1e-16:
            break
    return h


def betainc(a, b, x):
    if x <= 0.0:
        return 0.0
    if x >= 1.0:
        return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log1p(-x)
    bt = math.exp(lbt)
    if x < (a + 1.0) / (a + b + 2.0):
        return bt * _betacf(a, b, x) / a
    return 1.0 - bt * _betacf(b, a, 1.0 - x) / b


def t_cdf(t, nu):
    x = nu / (nu + t * t)
    tail = 0.5 * betainc(nu / 2.0, 0.5, x)
    return 1.0 - tail if t >= 0 else tail


def t_quantile(p, nu):
    """Inverse CDF by bisection (p in (0.5, 1))."""
    lo, hi = 0.0, 1000.0
    for _ in range(300):
        mid = 0.5 * (lo + hi)
        if t_cdf(mid, nu) < p:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)

# ---------------------------------------------------------------- statistics

def mean(xs):
    return math.fsum(xs) / len(xs)


def sd(xs):
    m = mean(xs)
    return math.sqrt(math.fsum((x - m) ** 2 for x in xs) / (len(xs) - 1))


def paired_ratio(a, b):
    """Ratio of totals R = sum(b)/sum(a) for paired samples, with the delta-method
    (linearised) standard error of log R: se = sd(b_i/mean(b) - a_i/mean(a)) / sqrt(n),
    i.e. sd(b_i - R a_i) / (sqrt(n) mean(b)). dof = n - 1.
    Returns the point ratio, se_log, dof, and two CI forms:
      log  : R * exp(+-t se_log)       (symmetric on the log scale)
      lin  : R * (1 +- t se_log)       (symmetric on the ratio scale)"""
    n = len(a)
    assert n == len(b) and n >= 2
    sa, sb = math.fsum(a), math.fsum(b)
    R = sb / sa
    ma, mb = sa / n, sb / n
    z = [bi / mb - ai / ma for ai, bi in zip(a, b)]
    se = sd(z) / math.sqrt(n)
    dof = n - 1
    t = t_quantile(0.975, dof)
    return {
        "n": n, "ratio": R, "se_log": se, "dof": dof, "t975": t,
        "ci95_log": [R * math.exp(-t * se), R * math.exp(t * se)],
        "ci95_lin": [R * (1 - t * se), R * (1 + t * se)],
    }


def corrected(rr, aa):
    """A/A-corrected ratio C = R / AA with independent-variance combination on the log
    scale and a Welch-Satterthwaite dof."""
    C = rr["ratio"] / aa["ratio"]
    v1, v2 = rr["se_log"] ** 2, aa["se_log"] ** 2
    se = math.sqrt(v1 + v2)
    dof = (v1 + v2) ** 2 / (v1 ** 2 / rr["dof"] + v2 ** 2 / aa["dof"]) if (v1 + v2) > 0 else float("inf")
    t = t_quantile(0.975, dof) if math.isfinite(dof) else 1.959963984540054
    return {"ratio": C, "se_log": se, "dof": dof, "t975": t,
            "ci95_log": [C * math.exp(-t * se), C * math.exp(t * se)]}


def median(xs):
    s = sorted(xs)
    n = len(s)
    return s[n // 2] if n % 2 else 0.5 * (s[n // 2 - 1] + s[n // 2])

# ---------------------------------------------------------------- records

def load(path):
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def cell(r):
    return (r["workload"]["workload_id"], r["round"], r["warmup"])


def by_arm(recs, measured_only=True):
    out = defaultdict(dict)
    for r in recs:
        if measured_only and r["warmup"]:
            continue
        k = cell(r)
        assert k not in out[r["arm"]], ("duplicate cell", r["arm"], k)
        out[r["arm"]][k] = r
    return out


def pair(arms, a, b):
    ka, kb = set(arms[a]), set(arms[b])
    keys = sorted(ka & kb)
    return keys, len(ka ^ kb)


def unpriced_counts(r):
    """Per-record counts of the operations named in cost.unpriced (each name ends in
    `_uncharged`). Source: counters[name] if present (rho), else the sum over phases of
    native[name without the suffix] (IC)."""
    out = {}
    for name in r["cost"]["unpriced"]:
        if name in (r.get("counters") or {}):
            out[name] = int(r["counters"][name])
        else:
            base = name[: -len("_uncharged")]
            out[name] = sum(int((p.get("native") or {}).get(base, 0)) for p in r["phases"])
    return out


def s_value(r, w):
    rr = int(r["workload"]["curve"]["r"])
    u = sum(unpriced_counts(r).values())
    return (float(r["cost"]["total_gae"]) + w * u) / math.sqrt(rr)


def proj(r):
    """Fields that must agree record by record (PROTOCOL-V6 section 6), no counter allowed
    to differ (ALLOWED_NATIVE = [])."""
    return {
        "recovered": r["outcome"]["recovered"],
        "total_gae": r["cost"]["total_gae"],
        "unpriced": r["cost"]["unpriced"],
        "phases": [{k: p.get(k) for k in ("name", "adds", "doubles", "scalar_mults", "gae")}
                   | {"native": p.get("native") or {}, "work_native": p.get("work_native") or {}}
                   for p in r["phases"]],
    }


def deep_diff_keys(x, y, prefix=""):
    """Paths at which two JSON values differ."""
    if isinstance(x, dict) and isinstance(y, dict):
        out = []
        for k in sorted(set(x) | set(y)):
            out += deep_diff_keys(x.get(k), y.get(k), f"{prefix}.{k}")
        return out
    if isinstance(x, list) and isinstance(y, list) and len(x) == len(y):
        out = []
        for i, (a, b) in enumerate(zip(x, y)):
            out += deep_diff_keys(a, b, f"{prefix}[]")
        return out
    return [] if x == y else [prefix]

# ---------------------------------------------------------------- per-session analyses

WALL = lambda r: int(r["time"]["solve_wall_ns"])


def wall_session(recs):
    res = {}
    arms_all = by_arm(recs, measured_only=False)
    arms = by_arm(recs, measured_only=True)
    res["records"] = len(recs)
    res["arms"] = {a: {"cells": len(arms_all[a]), "measured": len(arms[a])} for a in sorted(arms_all)}
    res["outcomes_not_verified"] = sum(1 for r in recs if r["outcome"]["status"] != "verified"
                                       or r["outcome"]["matches_target"] is not True)
    res["isolation_levels"] = dict(sorted(
        {lvl: sum(1 for r in recs if r["isolation"]["level"] == lvl)
         for lvl in {r["isolation"]["level"] for r in recs}}.items()))
    res["binary_sha256_set"] = sorted({r["binary_sha256"] for r in recs})
    res["min_solve_wall_ns"] = min(WALL(r) for r in recs)

    def cmp(a, b):
        keys, unpaired = pair(arms, a, b)
        xa = [WALL(arms[a][k]) for k in keys]
        xb = [WALL(arms[b][k]) for k in keys]
        out = paired_ratio(xa, xb)
        out["unpaired_runs"] = unpaired
        out["median_pair_ratio"] = median([y / x for x, y in zip(xa, xb)])
        out["pairs_below_L2"] = sum(1 for k in keys
                                    if arms[a][k]["isolation"]["level"] not in ("L2", "L3")
                                    or arms[b][k]["isolation"]["level"] not in ("L2", "L3"))
        out["same_algorithm_seed"] = all(arms[a][k]["algorithm_seed"] == arms[b][k]["algorithm_seed"] for k in keys)
        ga = [float(arms[a][k]["cost"]["total_gae"]) for k in keys]
        gb = [float(arms[b][k]["cost"]["total_gae"]) for k in keys]
        sa = [s_value(arms[a][k], 0.0) for k in keys]
        sb = [s_value(arms[b][k], 0.0) for k in keys]
        out["gae_ratio_of_totals"] = math.fsum(gb) / math.fsum(ga)
        out["ops_S_ratio_w0"] = math.fsum(sb) / math.fsum(sa)
        out["ns_per_gae_a"] = math.fsum(xa) / math.fsum(ga)
        out["ns_per_gae_b"] = math.fsum(xb) / math.fsum(gb)
        out["ns_per_gae_ratio"] = out["ns_per_gae_b"] / out["ns_per_gae_a"]
        out["gae_equal_every_pair"] = all(x == y for x, y in zip(ga, gb))
        out["s_record_field_matches_recomputed"] = all(
            abs(arms[c][k]["cost"]["s"] - s_value(arms[c][k], 0.0)) <= 1e-12 * abs(arms[c][k]["cost"]["s"])
            for c in (a, b) for k in keys)
        return out

    res["candidate"] = cmp("incumbent", "candidate")
    res["aa"] = cmp("incumbent", "incumbent-aa")
    res["positive"] = cmp("incumbent", "pc-fb2x")
    res["control"] = cmp("pc-fb2x", "incumbent-aa")
    res["candidate_corrected"] = corrected(res["candidate"], res["aa"])
    # A/A-corrected ns/GAE for the positive control (descriptive; no gate reads it).
    pc = res["positive"]
    res["positive_corrected_ns_per_gae"] = pc["ns_per_gae_ratio"] / res["aa"]["ratio"]
    # Exact counted parity, candidate vs incumbent, measured pairs (ops gate).
    c = res["candidate"]
    res["ops_parity"] = {"ratio_S": c["ops_S_ratio_w0"], "gae_equal_every_pair": c["gae_equal_every_pair"],
                         "unpaired_runs": c["unpaired_runs"], "same_algorithm_seed": c["same_algorithm_seed"]}
    # Record-level parity over all 144 cells (warm-ups included).
    inc, cand = arms_all["incumbent"], arms_all["candidate"]
    mism = [k for k in sorted(set(inc) | set(cand)) if k not in inc or k not in cand or proj(inc[k]) != proj(cand[k])]
    # Which record paths ever differ between the twins (outside time/isolation/identity)?
    ignore = {"record_id", "run_id", "seq", "arm", "role", "method", "time", "isolation", "online", "allocator"}
    paths = set()
    for k in sorted(set(inc) & set(cand)):
        x = {kk: v for kk, v in inc[k].items() if kk not in ignore}
        y = {kk: v for kk, v in cand[k].items() if kk not in ignore}
        paths.update(deep_diff_keys(x, y))
    res["record_parity"] = {"cells_incumbent": len(inc), "cells_candidate": len(cand),
                            "mismatched_cells": len(mism), "first_mismatches": [list(m) for m in mism[:5]],
                            "differing_paths_outside_timing": sorted(paths)}
    # Block positions under alternate_measured: position of each arm inside its
    # (workload, round, warmup) block, by seq.
    blocks = defaultdict(list)
    for r in recs:
        blocks[cell(r)].append(r)
    pos = defaultdict(lambda: defaultdict(int))
    for k, rs in blocks.items():
        for i, r in enumerate(sorted(rs, key=lambda r: r["seq"])):
            pos[("warmup" if k[2] else "measured")][f"{r['arm']}@{i}"] += 1
    res["block_positions"] = {kind: dict(sorted(d.items())) for kind, d in pos.items()}
    return res


def counted_session(recs, wall_sessions):
    res = {}
    arms_all = by_arm(recs, measured_only=False)
    arms = by_arm(recs, measured_only=True)
    res["records"] = len(recs)
    res["arms"] = {a: {"cells": len(arms_all[a]), "measured": len(arms[a])} for a in sorted(arms_all)}
    res["outcomes_not_verified"] = sum(1 for r in recs if r["outcome"]["status"] != "verified"
                                       or r["outcome"]["matches_target"] is not True)
    res["isolation_levels_by_arm"] = {a: dict(sorted(
        {lvl: sum(1 for r in arms_all[a].values() if r["isolation"]["level"] == lvl)
         for lvl in {r["isolation"]["level"] for r in arms_all[a].values()}}.items())) for a in sorted(arms_all)}
    res["deterministic_all"] = all(r["cost"]["deterministic"] for r in recs)

    def ops(a, b, w):
        keys, unpaired = pair(arms, a, b)
        sa = [s_value(arms[a][k], w) for k in keys]
        sb = [s_value(arms[b][k], w) for k in keys]
        R = math.fsum(sb) / math.fsum(sa)
        # cluster (workload) linearised-t interval, log scale
        wl = defaultdict(lambda: [0.0, 0.0])
        for k, x, y in zip(keys, sa, sb):
            wl[k[0]][0] += x
            wl[k[0]][1] += y
        A = [v[0] for v in wl.values()]
        Bv = [v[1] for v in wl.values()]
        cl = paired_ratio(A, Bv)
        ga = [float(arms[a][k]["cost"]["total_gae"]) + w * sum(unpriced_counts(arms[a][k]).values()) for k in keys]
        gb = [float(arms[b][k]["cost"]["total_gae"]) + w * sum(unpriced_counts(arms[b][k]).values()) for k in keys]
        return {"pairs": len(keys), "unpaired_runs": unpaired, "ratio_S": R,
                "ratio_raw_cost_totals": math.fsum(gb) / math.fsum(ga),
                "ci95_cluster_log": cl["ci95_log"], "cluster_dof": cl["dof"], "workloads": cl["n"],
                "exact_equal_every_pair": all(x == y for x, y in zip(sa, sb)),
                "mean_S_a": mean(sa), "mean_S_b": mean(sb)}

    res["ops"] = {}
    for w in (0.0, 0.1, 1.0):
        res["ops"][str(w)] = {f"{a}__{b}": ops(a, b, w) for a, b in
                              (("incumbent", "candidate"), ("rho-neg", "incumbent"),
                               ("rho-neg", "candidate"), ("rho-neg", "rho-neg-aa"))}
    # per-curve band: mean S at w for each arm (measured runs)
    band = defaultdict(dict)
    for a, d in arms.items():
        per_curve = defaultdict(list)
        for k, r in d.items():
            per_curve[r["workload"]["curve"]["slug"]].append(r)
        for slug, rs in per_curve.items():
            band[slug][a] = {"runs": len(rs),
                             "mean_S": [mean([s_value(r, w) for r in rs]) for w in (0.0, 0.1, 1.0)],
                             "charged_sum": math.fsum(float(r["cost"]["total_gae"]) for r in rs),
                             "native_sum": sum(sum(unpriced_counts(r).values()) for r in rs)}
    res["band"] = {slug: dict(sorted(v.items())) for slug, v in sorted(band.items())}
    # order across the band per curve: rank IC vs rho only if order holds at every w
    orders = {}
    for slug, v in res["band"].items():
        for x, y in (("incumbent", "rho-neg"), ("candidate", "rho-neg"), ("incumbent", "candidate")):
            d = [v[x]["mean_S"][i] - v[y]["mean_S"][i] for i in range(3)]
            orders[f"{slug}:{x}_vs_{y}"] = ("{} < {}".format(y, x) if all(e > 0 for e in d) else
                                            "{} < {}".format(x, y) if all(e < 0 for e in d) else "unresolved")
    res["band_orders"] = orders
    # record parity candidate vs incumbent inside the counted session (all 288 cells)
    inc, cand = arms_all["incumbent"], arms_all["candidate"]
    mism = [k for k in sorted(set(inc) | set(cand)) if k not in inc or k not in cand or proj(inc[k]) != proj(cand[k])]
    res["record_parity_in_counted"] = {"cells": len(inc), "mismatched_cells": len(mism)}
    # cross-session parity: counted IC records equal each wall session's on the projection
    cross = {}
    for name, wrecs in wall_sessions.items():
        wa = by_arm(wrecs, measured_only=False)
        bad = 0
        total = 0
        for arm in ("incumbent", "candidate"):
            ks = set(wa[arm]) | set(arms_all[arm])
            for k in ks:
                total += 1
                if k not in wa[arm] or k not in arms_all[arm] or proj(wa[arm][k]) != proj(arms_all[arm][k]):
                    bad += 1
        cross[name] = {"cells_compared": total, "mismatched_cells": bad}
    res["cross_session_parity"] = cross
    return res

# ---------------------------------------------------------------- comparison with producer output

def rel(x, y):
    if x is None or y is None:
        return None
    if x == y:
        return 0.0
    return abs(x - y) / max(abs(x), abs(y))


def compare_wall(mine, prod_dir, idx):
    def J(k):
        with open(f"{prod_dir}/session-{idx}-{k}.json") as f:
            return json.load(f)
    cand, pos, ctl = J("candidate"), J("positive"), J("control-comparison")
    c, aa, cc = mine["candidate"], mine["aa"], mine["candidate_corrected"]
    rows = [
        ("candidate.total_wall_ratio", c["ratio"], cand["wall"]["total_wall_ratio"]),
        ("candidate.total_wall_ci95[0]", c["ci95_log"][0], cand["wall"]["total_wall_ci95"][0]),
        ("candidate.total_wall_ci95[1]", c["ci95_log"][1], cand["wall"]["total_wall_ci95"][1]),
        ("candidate.median_ratio", c["median_pair_ratio"], cand["wall"]["median_ratio"]),
        ("candidate.ns_per_gae_a", c["ns_per_gae_a"], cand["wall"]["ns_per_gae_a"]),
        ("candidate.ns_per_gae_b", c["ns_per_gae_b"], cand["wall"]["ns_per_gae_b"]),
        ("aa.ratio", aa["ratio"], cand["wall"]["aa_ratio"]),
        ("aa.ci95[0]", aa["ci95_log"][0], cand["wall"]["aa_ci95"][0]),
        ("aa.ci95[1]", aa["ci95_log"][1], cand["wall"]["aa_ci95"][1]),
        ("aa.median_ratio", aa["median_pair_ratio"], cand["wall"]["aa_median_ratio"]),
        ("corrected.ratio", cc["ratio"], cand["wall"]["corrected_ratio"]),
        ("corrected.ci95[0]", cc["ci95_log"][0], cand["wall"]["corrected_ci95"][0]),
        ("corrected.ci95[1]", cc["ci95_log"][1], cand["wall"]["corrected_ci95"][1]),
        ("corrected.dof", cc["dof"], cand["wall"]["corrected_dof"]),
        ("candidate.ops_ratio", c["ops_S_ratio_w0"], cand["ops"]["ratio_b_over_a"]),
        ("control.aa_ci95[0]", aa["ci95_log"][0], ctl["wall"]["aa_ci95"][0]),
        ("control.aa_ci95[1]", aa["ci95_log"][1], ctl["wall"]["aa_ci95"][1]),
        ("control.total_wall_ratio", mine["control"]["ratio"], ctl["wall"]["total_wall_ratio"]),
        ("control.total_wall_ci95[0]", mine["control"]["ci95_log"][0], ctl["wall"]["total_wall_ci95"][0]),
        ("control.total_wall_ci95[1]", mine["control"]["ci95_log"][1], ctl["wall"]["total_wall_ci95"][1]),
        ("positive.total_wall_ratio", mine["positive"]["ratio"], pos["wall"]["total_wall_ratio"]),
        ("positive.total_wall_ci95[0]", mine["positive"]["ci95_log"][0], pos["wall"]["total_wall_ci95"][0]),
        ("positive.total_wall_ci95[1]", mine["positive"]["ci95_log"][1], pos["wall"]["total_wall_ci95"][1]),
        ("positive.ops_ratio", mine["positive"]["ops_S_ratio_w0"], pos["ops"]["ratio_b_over_a"]),
        ("positive.corrected_ns_per_gae", mine["positive_corrected_ns_per_gae"], pos["wall"]["corrected_ratio"]),
    ]
    out = [{"field": f, "mine": m, "producer": p, "rel_diff": rel(m, p)} for f, m, p in rows]
    flags = {
        "producer_aa_arms": cand["wall"]["aa_arms"], "producer_self_aa": cand["wall"]["self_aa"],
        "producer_outside_noise": cand["wall"]["outside_noise"], "producer_wall_status": cand["wall"]["status"],
        "producer_pairs": cand["wall"]["pairs"], "producer_pairs_below_level": cand["wall"]["pairs_below_level"],
        "producer_ops_ci95": cand["ops"]["ci95"], "producer_ops_status": cand["ops"]["status"],
        "producer_ops_same_seeds": cand["ops"]["same_seeds"], "producer_ops_unpaired": cand["ops"]["unpaired_runs"],
        "producer_positive_status": pos["wall"]["status"], "producer_positive_ops_status": pos["ops"]["status"],
        "producer_control_aa_arms": ctl["wall"]["aa_arms"], "producer_control_a_b": [ctl["a"]["arm"], ctl["b"]["arm"]],
    }
    return out, flags


def compare_counted(mine, prod_dir):
    out = []
    for w, tag in ((0.0, "0"), (0.1, "0.1"), (1.0, "1")):
        for pr in ("incumbent__candidate", "rho-neg__incumbent", "rho-neg__candidate", "rho-neg__rho-neg-aa"):
            with open(f"{prod_dir}/counted-compare-{pr}-w{tag}.json") as f:
                p = json.load(f)
            m = mine["ops"][str(w)][pr]
            out.append({"field": f"{pr}@w{tag}.ratio", "mine": m["ratio_S"], "producer": p["ops"]["ratio_b_over_a"],
                        "rel_diff": rel(m["ratio_S"], p["ops"]["ratio_b_over_a"])})
            for i in (0, 1):
                out.append({"field": f"{pr}@w{tag}.ci95[{i}]", "mine": m["ci95_cluster_log"][i],
                            "producer": p["ops"]["ci95"][i], "rel_diff": rel(m["ci95_cluster_log"][i], p["ops"]["ci95"][i])})
    with open(f"{prod_dir}/counted-band.json") as f:
        band = json.load(f)
    for row in band["rows"]:
        m = mine["band"][row["curve"]][row["arm"]]
        for i in range(3):
            out.append({"field": f"band:{row['curve']}:{row['arm']}:S[{i}]", "mine": m["mean_S"][i],
                        "producer": row["mean_s"][i], "rel_diff": rel(m["mean_S"][i], row["mean_s"][i])})
        out.append({"field": f"band:{row['curve']}:{row['arm']}:native_sum", "mine": m["native_sum"],
                    "producer": row["native_sum"], "rel_diff": rel(m["native_sum"], row["native_sum"])})
        out.append({"field": f"band:{row['curve']}:{row['arm']}:charged_sum", "mine": m["charged_sum"],
                    "producer": row["charged_sum"], "rel_diff": rel(m["charged_sum"], row["charged_sum"])})
    return out

# ---------------------------------------------------------------- main

def main(argv):
    out_path = argv[1]
    walls, counted, producers, producer_counted = {}, None, [], []
    i = 2
    while i < len(argv):
        if argv[i] == "--wall":
            name, d = argv[i + 1].split("=", 1)
            walls[name] = d
        elif argv[i] == "--counted":
            counted = argv[i + 1]
        elif argv[i] == "--producer":
            name, rest = argv[i + 1].split("=", 1)
            d, idx = rest.rsplit(":", 1)
            producers.append((name, d, int(idx)))
        elif argv[i] == "--producer-counted":
            producer_counted.append(argv[i + 1])
        else:
            raise SystemExit(f"unknown arg {argv[i]}")
        i += 2
    recs = {n: load(f"{d}/records.jsonl") for n, d in walls.items()}
    result = {"schema": "validator.recompute/v1", "t975_dof127": t_quantile(0.975, 127),
              "t975_dof15": t_quantile(0.975, 15), "wall": {}, "counted": None,
              "comparisons": {}}
    for n in walls:
        result["wall"][n] = wall_session(recs[n])
    if counted:
        result["counted"] = counted_session(load(f"{counted}/records.jsonl"), recs)
    for name, d, idx in producers:
        rows, flags = compare_wall(result["wall"][name], d, idx)
        result["comparisons"][f"{name}@{d}:{idx}"] = {"rows": rows, "producer_flags": flags,
                                                     "max_rel_diff": max(r["rel_diff"] for r in rows)}
    for d in producer_counted:
        rows = compare_counted(result["counted"], d)
        result["comparisons"][f"counted@{d}"] = {"rows": rows, "max_rel_diff": max(r["rel_diff"] for r in rows)}
    with open(out_path, "w") as f:
        json.dump(result, f, indent=1, sort_keys=True)
    print("wrote", out_path)


if __name__ == "__main__":
    main(sys.argv)
