#!/usr/bin/env python3
"""Second stage (unvalidated, pre-gate): compare predictors of the observed on/off trial ratio.

Reads results_h1_shortfall.json (written by h1_shortfall.py) and results/h1_defaults/rows.jsonl.
For each family: per-instance log residual  log(obs_pooled_i / pred_i)  for three predictors
  (1) 1/h                    the registered prediction
  (2) U                      uniform torsion classes + the diagonal (+-2F_i) term + all points distinct
  (3) exact                  the actual factor base's table: |T_off cap <G>| / |T_on|
and the sum of squared residuals.  obs_pooled_i = (sum of on trials) / (sum of off trials) over the
instance's two seeds (a ratio of two sums of 68 geometric draws: sd of log about 0.17).
Also per-rung Monte Carlo of the median paired ratio under the exact model (geometric trial counts,
fb + 2 = 34 relations per solve, P = |T cap <G>| / n), to say whether the observed medians are inside
the sampling spread of the model."""
import json, os, math, random, statistics, collections, sys

HERE = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(os.path.join(HERE, "results_h1_shortfall.json")))
inst = d["instances"]
M = d["config"]["M"]

def geometric(P, rng):
    if P >= 1:
        return 1
    return int(math.log(1.0 - rng.random()) / math.log(1.0 - P)) + 1

out = {}
for fam in ("E", "M"):
    rs = [r for r in inst if r["family"] == fam]
    preds = {"1/h": lambda r: 1.0 / r["h"], "U": lambda r: r["pred_uniform_U"], "exact": lambda r: r["pred_exact"],
             "exact_random_start": lambda r: r["pred_exact_random_start_mean"]}
    fo = {}
    for nm, f in preds.items():
        res = [math.log(r["obs_pooled_ratio"] / f(r)) for r in rs]
        rng = random.Random("boot-" + nm)
        bs = sorted(statistics.mean([rng.choice(res) for _ in res]) for _ in range(4000))
        fo[nm] = dict(mean_log_resid=statistics.mean(res), ci95=(bs[100], bs[3899]), rss=sum(x * x for x in res),
                      sd=statistics.pstdev(res))
    # per-rung and overall Monte Carlo of the median paired ratio under the exact model
    mc = {}
    for rung in sorted({r["rung"] for r in rs}) + ["all"]:
        sub = [r for r in rs if rung == "all" or r["rung"] == rung]
        mrng = random.Random(f"mc-{fam}-{rung}")
        meds = []
        for _ in range(600):
            pairs = []
            for r in sub:
                for _s in range(2):
                    t_off = sum(geometric(r["P_off"], mrng) for _ in range(M + 2))
                    t_on = sum(geometric(r["P_on"], mrng) for _ in range(M + 2))
                    pairs.append(t_on / t_off)
            meds.append(statistics.median(pairs))
        meds.sort()
        obs = statistics.median([x for r in sub for x in r["obs_pair_ratios"]])
        below = sum(1 for m in meds if m < obs) / len(meds)
        mc[str(rung)] = dict(n=len(sub), obs_median=obs, mc_p025=meds[15], mc_p50=meds[300], mc_p975=meds[584],
                             obs_quantile_in_mc=below, pred_exact_median=statistics.median([r["pred_exact"] for r in sub]))
    # decomposition of (obs median - 1/h)
    sm = d["summary"][fam]
    dec = dict(
        one_over_h=sm["pred_1_over_h"],
        plus_diagonal_term=sm["pred_uniform_U_median"] - sm["pred_1_over_h"],
        plus_actual_classes_and_collisions=sm["pred_exact_median"] - sm["pred_uniform_U_median"],
        plus_statistic_and_sampling=sm["obs_median_pair_ratio"] - sm["pred_exact_median"],
        observed_median=sm["obs_median_pair_ratio"],
        mc_median_of_pair_ratio_under_exact=mc["all"]["mc_p50"],
    )
    # spread of the exact prediction across instances of one rung (class-composition noise of 32 points)
    spread = {}
    for rung in sorted({r["rung"] for r in rs}):
        v = [r["pred_exact"] for r in rs if r["rung"] == rung]
        spread[str(rung)] = dict(mean=statistics.mean(v), sd=statistics.pstdev(v), min=min(v), max=max(v))
    out[fam] = dict(residuals=fo, monte_carlo=mc, decomposition=dec, exact_spread_by_rung=spread)
json.dump(out, open(os.path.join(HERE, "results_residuals.json"), "w"), indent=1, default=str)
for fam, o in out.items():
    print("==", fam)
    for nm, v in o["residuals"].items():
        print(f"  {nm:19} mean log resid {v['mean_log_resid']:+.3f}  CI [{v['ci95'][0]:+.3f},{v['ci95'][1]:+.3f}]  sd {v['sd']:.3f}  RSS {v['rss']:.3f}")
    for k, v in o["monte_carlo"].items():
        print(f"  MC rung {k:>3}: obs {v['obs_median']:.3f}  model p2.5/50/97.5 = {v['mc_p025']:.3f}/{v['mc_p50']:.3f}/{v['mc_p975']:.3f}  obs at model quantile {v['obs_quantile_in_mc']:.2f}  (exact median {v['pred_exact_median']:.3f})")
    print("  decomposition:", {k: round(v, 4) for k, v in o["decomposition"].items()})
    print("  exact spread by rung:", {k: (round(v['mean'], 3), round(v['sd'], 3)) for k, v in o["exact_spread_by_rung"].items()})
