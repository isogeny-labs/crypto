"""J1 (TASK-20261029-654f73): the decision statistic of PROTOCOL section 5 from
the wall session's records.jsonl, by the validator's own code.

Pairs are keyed by (curve slug, target index, round), measured rounds only; a
pair is admitted when both runs are at L2 or L3 and verified. For arms A, B:
  ns_per_gae_X = sum solve_wall_ns(X) / sum total_gae(X)
  ratio        = ns_per_gae_B / ns_per_gae_A
with a linearised (delta-method) standard error of ln ratio, t with n-1 dof.
corrected = ratio(dgb->dgbf) / ratio(dgb->dgb-aa), the two log SEs combined as
independent, Welch-Satterthwaite dof. Descriptive: per-curve intervals,
target-clustered bootstrap, per-round estimates and a label-swap null.

usage: j1_decision.py B OUT.json
"""
import math
import random
import sys

import vcommon as v
import vstats as s

ADMIT = {"L2", "L3"}


def pairs_for(arms, a, b, filt=None):
    A, B = arms[a], arms[b]
    if set(A) != set(B):
        raise SystemExit(f"STOP: key sets differ between {a} and {b}")
    out, below, keys = [], 0, []
    for k in sorted(A):
        ra, rb = A[k], B[k]
        if filt and not filt(k):
            continue
        ok = (ra["isolation"]["level"] in ADMIT and rb["isolation"]["level"] in ADMIT
              and ra["outcome"]["status"] == "verified" and rb["outcome"]["status"] == "verified")
        if not ok:
            below += 1
            continue
        out.append((ra["time"]["solve_wall_ns"], ra["cost"]["total_gae"],
                    rb["time"]["solve_wall_ns"], rb["cost"]["total_gae"]))
        keys.append(k)
    return out, below, keys


def summarize(pairs, below):
    r = s.ratio_of_sums(pairs)
    r["pairs_below_level"] = below
    r["ci95_log"] = s.ci_log(r["ratio"], r["se_log"], r["dof"])
    r["ci95_linear"] = s.ci_linear(r["ratio"], r["se_log"], r["dof"])
    return r


def decision(arms, filt=None):
    d, bd, kd = pairs_for(arms, "dgb", "dgbf", filt)
    a, ba, ka = pairs_for(arms, "dgb", "dgb-aa", filt)
    rd, ra = summarize(d, bd), summarize(a, ba)
    c = s.corrected(rd, ra)
    return rd, ra, c


def main():
    b = int(sys.argv[1])
    recs = v.load_wall(b)
    arms = v.by_arm(recs)
    res = {"size": b, "arms": {k: len(x) for k, x in arms.items()}}
    rd, ra, c = decision(arms)
    res["decision"] = {"dgb->dgbf": rd, "aa dgb->dgb-aa": ra, "corrected": c}
    # condition 9: direct dgb-aa -> dgbf
    p9, b9, _ = pairs_for(arms, "dgb-aa", "dgbf")
    r9 = summarize(p9, b9)
    res["cond9"] = {"dgb-aa->dgbf": r9,
                    "rel_diff_vs_corrected": abs(r9["ratio"] - c["ratio"]) / c["ratio"],
                    "sum_wall_ratio_dgbf_over_dgbaa": r9["sum_wall_b"] / r9["sum_wall_a"],
                    "gae_sums_equal": r9["sum_gae_a"] == r9["sum_gae_b"]}
    # descriptive contrasts
    for a_, b_ in (("dgb", "dgbf-edge"), ("dgbf-edge", "dgbf"), ("dgbf-edge", "dgb-aa")):
        pp, bb, _ = pairs_for(arms, a_, b_)
        res[f"{a_}->{b_}"] = summarize(pp, bb)
    # per curve
    curves = sorted({k[0] for k in arms["dgb"]})
    res["per_curve"] = {}
    for cv in curves:
        rdc, rac, cc = decision(arms, lambda k, cv=cv: k[0] == cv)
        pe, be, _ = pairs_for(arms, "dgb", "dgbf-edge", lambda k, cv=cv: k[0] == cv)
        res["per_curve"][cv] = {"ratio": rdc["ratio"], "ci95_log": rdc["ci95_log"],
                                "aa_ratio": rac["ratio"], "aa_ci95_log": rac["ci95_log"],
                                "corrected": cc["ratio"], "corrected_ci95_log": cc["ci95_log"],
                                "corrected_dof": cc["dof"], "pairs": rdc["n"],
                                "position_control": summarize(pe, be)["ratio"]}
    # per round (descriptive stability)
    rounds = sorted({k[2] for k in arms["dgb"]})
    res["per_round_corrected"] = {}
    for rd_ in rounds:
        _, _, cr = decision(arms, lambda k, rd_=rd_: k[2] == rd_)
        res["per_round_corrected"][str(rd_)] = cr["ratio"]
    # target-clustered bootstrap of the corrected ratio (cluster = curve, target)
    clusters = {}
    for k in arms["dgb"]:
        t = (k[0], k[1])
        x = clusters.setdefault(t, [0, 0, 0, 0, 0, 0])
        for i, arm in enumerate(("dgb", "dgb-aa", "dgbf")):
            r = arms[arm][k]
            x[2 * i] += r["time"]["solve_wall_ns"]; x[2 * i + 1] += r["cost"]["total_gae"]
    cl = list(clusters.values())

    def stat(sample):
        w = [sum(c_[i] for c_ in sample) for i in range(6)]
        return (w[4] / w[5]) / (w[2] / w[3])  # (dgbf ns/GAE) / (dgb-aa ns/GAE)

    def stat_raw(sample):
        w = [sum(c_[i] for c_ in sample) for i in range(6)]
        return (w[4] / w[5]) / (w[0] / w[1])
    ci_c, _ = s.cluster_bootstrap(cl, stat)
    ci_r, _ = s.cluster_bootstrap(cl, stat_raw)
    res["target_clustered_bootstrap"] = {"clusters": len(cl), "B": 4000, "seed": 654733,
                                         "corrected_point": stat(cl), "corrected_ci95_pct": ci_c,
                                         "ratio_point": stat_raw(cl), "ratio_ci95_pct": ci_r}
    res["target_clustered_bootstrap_per_curve"] = {}
    for cv in curves:
        sub = [clusters[t] for t in clusters if t[0] == cv]
        ci_cc, _ = s.cluster_bootstrap(sub, stat)
        res["target_clustered_bootstrap_per_curve"][cv] = {"clusters": len(sub), "corrected_point": stat(sub),
                                                           "corrected_ci95_pct": ci_cc}
    # label-swap null on the matched-position pair (dgb-aa, dgbf): swapping
    # arm labels within a pair destroys the arm effect; the null distribution
    # must centre on 1 and its width must shrink like 1/sqrt(n).
    rng = random.Random(20261030)
    null = {}
    for n_sub in (72, 144, 288):
        sub = p9[:n_sub] if n_sub < len(p9) else p9
        vals = []
        for _ in range(4000):
            sa = sb = ga = gb = 0
            for (wa, g_a, wb, g_b) in sub:
                if rng.random() < 0.5:
                    wa, g_a, wb, g_b = wb, g_b, wa, g_a
                sa += wa; ga += g_a; sb += wb; gb += g_b
            vals.append((sb / gb) / (sa / ga))
        obs = s.ratio_of_sums(sub)["ratio"]
        null[str(n_sub)] = {"null_median": s.percentile(vals, 0.5),
                            "null_ci95": [s.percentile(vals, 0.025), s.percentile(vals, 0.975)],
                            "observed": obs,
                            "null_fraction_le_observed": sum(x <= obs for x in vals) / len(vals)}
    res["label_swap_null_dgbaa_dgbf"] = null
    # phase sums and memory (descriptive; measured records)
    ph = {}
    for arm, d_ in arms.items():
        acc = {"solve_wall_ns": 0}
        rss = []
        for r in d_.values():
            acc["solve_wall_ns"] += r["time"]["solve_wall_ns"]
            for p in r["phases"]:
                acc[p["name"]] = acc.get(p["name"], 0) + p["wall_ns"]
            for name, ns in r["online"]["phases_ns"].items():
                acc["online." + name] = acc.get("online." + name, 0) + ns
            rss.append(r["time"]["max_rss_kib"])
        acc["max_rss_kib_max"] = max(rss)
        acc["max_rss_kib_median"] = s.percentile(rss, 0.5)
        acc["records"] = len(d_)
        ph[arm] = acc
    res["phase_sums_ns"] = ph
    v.dump(res, sys.argv[2])
    D = res["decision"]
    print(f"P{b} pairs {rd['n']} below {rd['pairs_below_level']}")
    print(" ns/GAE dgb %.6f dgbf %.6f" % (rd["ns_per_gae_a"], rd["ns_per_gae_b"]))
    print(" ratio %.9f log%s lin%s" % (rd["ratio"], fmt(rd["ci95_log"]), fmt(rd["ci95_linear"])))
    print(" aa    %.9f log%s lin%s" % (ra["ratio"], fmt(ra["ci95_log"]), fmt(ra["ci95_linear"])))
    print(" corr  %.15f dof %.2f log%s lin%s" % (c["ratio"], c["dof"], fmt(c["ci95_log"]), fmt(c["ci95_linear"])))
    print(" cond9 %.15f reldiff %.3e" % (r9["ratio"], res["cond9"]["rel_diff_vs_corrected"]))
    for k in ("dgb->dgbf-edge", "dgbf-edge->dgbf", "dgbf-edge->dgb-aa"):
        print(" %s %.6f %s" % (k, res[k]["ratio"], fmt(res[k]["ci95_log"])))


def fmt(x):
    return "[" + ", ".join("%.6f" % y for y in x) + "]"


if __name__ == "__main__":
    main()


def inpod_crosscheck(b):
    """Recompute the in-pod native A/A admission comparison (control-comparison.json,
    --a dgbf-edge --b dgb-aa, aa arms dgbf-edge/dgbf) with this file's method and
    return (mine, file) at full precision."""
    import json as _json
    import os as _os
    arms = v.by_arm(v.load_wall(b))
    p, bl, _ = pairs_for(arms, "dgbf-edge", "dgb-aa")
    pa, ba, _ = pairs_for(arms, "dgbf-edge", "dgbf")
    r, a = summarize(p, bl), summarize(pa, ba)
    c = s.corrected(r, a)
    f = _json.load(open(_os.path.join(v.session_dir(b, "wall"), "control-comparison.json")))["wall"]
    mine = {"ratio_b_over_a": r["ratio"], "ci95_lo": r["ci95_log"][0], "ci95_hi": r["ci95_log"][1],
            "aa_ratio": a["ratio"], "aa_ci95_lo": a["ci95_log"][0], "aa_ci95_hi": a["ci95_log"][1],
            "corrected_ratio": c["ratio"], "corrected_ci95_lo": c["ci95_log"][0],
            "corrected_ci95_hi": c["ci95_log"][1], "corrected_dof": c["dof"],
            "ns_per_gae_a": r["ns_per_gae_a"], "ns_per_gae_b": r["ns_per_gae_b"], "dof": r["dof"]}
    filev = {"ratio_b_over_a": f["ratio_b_over_a"], "ci95_lo": f["ci95"][0], "ci95_hi": f["ci95"][1],
             "aa_ratio": f["aa_ratio"], "aa_ci95_lo": f["aa_ci95"][0], "aa_ci95_hi": f["aa_ci95"][1],
             "corrected_ratio": f["corrected_ratio"], "corrected_ci95_lo": f["corrected_ci95"][0],
             "corrected_ci95_hi": f["corrected_ci95"][1], "corrected_dof": f["corrected_dof"],
             "ns_per_gae_a": f["ns_per_gae_a"], "ns_per_gae_b": f["ns_per_gae_b"], "dof": f["dof"]}
    rel = {k: abs(mine[k] - filev[k]) / abs(filev[k]) for k in mine}
    return {"mine": mine, "file": filev, "rel_diff": rel, "max_rel_diff": max(rel.values())}

