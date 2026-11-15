#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md.  python3 analyze.py tune     -> frozen_choice.json, results/tune_table.json
                                     python3 analyze.py holdout  -> summary.json
Time of a key in a configuration = median over its repeats of ms_arm (the whole solve)."""
import json, sys, os, math, statistics as st, collections
HERE = os.path.dirname(os.path.abspath(__file__))
def load(phase):
    rows = [json.loads(l) for l in open(os.path.join(HERE, "results", phase, "runs.jsonl"))]
    bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
    t = collections.defaultdict(list)
    for r in rows:
        if r not in bad: t[(r["T"], r["idx"], r["key"])].append(r["ms_arm"])
    return rows, bad, {k: st.median(v) for k, v in t.items()}
def gmean(xs): return math.exp(sum(math.log(x) for x in xs) / len(xs))
if sys.argv[1] == "tune":
    rows, bad, tm = load("tune")
    assert not bad, f"{len(bad)} failed or unverified executions: the reading stops"
    cfgs = sorted({(T, i) for T, i, _ in tm}); keys = sorted({k for _, _, k in tm})
    score = {k: gmean([tm[(T, i, k)] / tm[(T, i, "ic:4:1.0")] for T, i in cfgs]) for k in keys}
    best = min(keys, key=lambda k: (score[k], k))
    _, k, mu = best.split(":"); k, mu = int(k), float(mu)
    wins = sum(tm[(T, i, best)] <= tm[(T, i, "ic:4:1.0")] for T, i in cfgs)
    json.dump(dict(k=k, mu=mu, key=best, score=score[best], configs_at_or_below_default=wins, n_configs=len(cfgs),
                   mu_on_grid_edge=mu in (0.4, 1.2), rule="argmin over the 21 grid keys of the geometric mean over the "
                   "12 tune configurations of ms_arm(key) / ms_arm(k=4, mu=1.0); ties by key name"),
              open(os.path.join(HERE, "frozen_choice.json"), "w"), indent=1)
    json.dump({k: dict(score=score[k], per_cell={str(T): st.median(tm[(T, i, k)] / tm[(T, i, "ic:4:1.0")]
               for i in sorted({i for TT, i in cfgs if TT == T})) for T in sorted({T for T, _ in cfgs})}) for k in keys},
              open(os.path.join(HERE, "results", "tune_table.json"), "w"), indent=1)
    print("chosen", best, round(score[best], 4), f"{wins}/{len(cfgs)} configs at or below default")
elif sys.argv[1] == "holdout":
    rows, bad, tm = load("holdout")
    ch = json.load(open(os.path.join(HERE, "frozen_choice.json")))
    cfgs = sorted({(T, i) for T, i, _ in tm})
    rho = sorted({k for _, _, k in tm if k.startswith("rho")})
    out = dict(correctness=dict(executions=len(rows), failed_or_unverified=len(bad)), choice=ch, cells={})
    for T in sorted({T for T, _ in cfgs}):
        ii = [i for TT, i in cfgs if TT == T]
        def per(num, den): return [num(i) / den(i) for i in ii]
        best_rho = lambda i: min(tm[(T, i, k)] for k in rho)
        c_d = per(lambda i: tm[(T, i, "IC-chosen")], lambda i: tm[(T, i, "IC-default")])
        c_r = per(lambda i: tm[(T, i, "IC-chosen")], best_rho)
        d_r = per(lambda i: tm[(T, i, "IC-default")], best_rho)
        out["cells"][str(T)] = dict(
            instances=ii, chosen_over_default=dict(median=st.median(c_d), per_instance=c_d),
            chosen_over_best_rho=dict(median=st.median(c_r), per_instance=c_r),
            default_over_best_rho=dict(median=st.median(d_r), per_instance=d_r),
            best_rho_key={str(i): min(rho, key=lambda k: tm[(T, i, k)]) for i in ii},
            lower_4_of_4=all(x < 1 for x in c_d), not_lower_4_of_4=all(x >= 1 for x in c_d),
            chosen_ahead_of_rho_4_of_4=all(x <= 1 for x in c_r), chosen_behind_rho_4_of_4=all(x >= 1 for x in c_r))
    cells = out["cells"].values()
    out["gates"] = dict(
        correctness="pass" if not bad else "FAIL",
        primary="lower" if all(c["lower_4_of_4"] for c in cells) else
                ("not lower" if all(c["not_lower_4_of_4"] for c in cells) else "mixed"),
        distance_to_rho={T: c["chosen_over_best_rho"]["median"] for T, c in out["cells"].items()},
        ic_ahead_of_rho_cells=sum(c["chosen_ahead_of_rho_4_of_4"] for c in cells))
    json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
    print(json.dumps(out["gates"], indent=1))
