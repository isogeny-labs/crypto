#!/usr/bin/env python3
"""Apply PROTOCOL_v2.md's frozen rules to results_v2/*.json. Writes results_v2/verdict.json and RESULTS_V2.md."""
import glob, json, math, os
import numpy as np

D = "results_v2"


def ci(v, seed=7, n=10000):
    v = np.asarray(v, float)
    m = v[np.random.default_rng(seed).integers(0, len(v), (n, len(v)))].mean(1)
    return [float(v.mean()), float(np.percentile(m, 2.5)), float(np.percentile(m, 97.5))]


def name(o):
    r = o["rule"]
    key = r if isinstance(r, str) else next(iter(r))
    base = {"All": "all", "CompleteOnly": "complete_only", "FirstCome": "first_come",
            "SinglesOnly": "singles_only", "SeenLp": "seen_lp", "MaxLp": "max_lp"}[key]
    if key == "MaxLp":
        base += f"_{r['MaxLp']}"
    return base


def scores(run):
    """{(rule, cap): (primary draws, secondary draws, censored)}"""
    out, budget = {}, run["draws_available"]
    t90 = math.ceil(0.9 * run["target"])
    for o in run["outcomes"]:
        prim = o["draws_to_target"]
        sec = o["draws_to_90pct"] if "draws_to_90pct" in o else next((d for d, r in o["trace"] if r >= t90), None)
        out[(name(o), o["cap"])] = (prim or budget, sec or budget, prim is None)
    return out


fmt = lambda c: f"{c[0]:.3f} [{c[1]:.3f}, {c[2]:.3f}]"
V, L = {"fp": {}, "real": {}}, []
fp = {os.path.basename(f)[:-5]: json.load(open(f)) for f in sorted(glob.glob(f"{D}/fp*.json"))}
real = {os.path.basename(f)[:-5]: json.load(open(f)) for f in sorted(glob.glob(f"{D}/real*.json"))}

caps = sorted({o["cap"] for d in fp.values() for o in d["runs"][0]["outcomes"] if o["cap"]})
rules_capped = sorted({name(o) for d in fp.values() for o in d["runs"][0]["outcomes"] if o["cap"]})
censored = 0
for s, d in fp.items():
    S = [scores(r) for r in d["runs"]]
    censored += sum(v[2] for sc in S for v in sc.values())
    fb = d["runs"][0]["fb_columns"]
    row = {"fb": fb, "all_mean_draws": float(np.mean([sc[("all", None)][0] for sc in S])),
           "complete_only_mean_draws": float(np.mean([sc[("complete_only", None)][0] for sc in S]))}
    for k, idx in (("primary", 0), ("secondary", 1)):
        row[f"B1_{k}"] = ci([sc[("complete_only", None)][idx] / sc[("all", None)][idx] for sc in S])
        for cap in caps:
            for r in rules_capped:
                row[f"{k}:{r}@{cap}/all"] = ci([sc[(r, cap)][idx] / sc[("all", None)][idx] for sc in S])
                row[f"{k}:{r}@{cap}/first_come"] = ci([sc[(r, cap)][idx] / sc[("first_come", cap)][idx] for sc in S])
                row[f"{k}:{r}@{cap}/complete_only"] = ci([sc[(r, cap)][idx] / sc[("complete_only", None)][idx] for sc in S])
    V["fp"][s] = row
V["censored_cells"] = int(censored)

FP = list(fp)
B1 = {k: all(V["fp"][s][f"B1_{k}"][1] > 1 for s in FP) for k in ("primary", "secondary")}
V["B1"] = B1
B2 = {}
for k in ("primary", "secondary"):
    for cap in caps:
        for r in rules_capped:
            if r == "first_come":
                continue
            rows = [V["fp"][s][f"{k}:{r}@{cap}/first_come"] for s in FP]
            B2[f"{k}:{r}@{cap}"] = ("beats first_come" if all(x[2] < 1 for x in rows)
                                    else "worse than first_come" if all(x[1] > 1 for x in rows) else "not separated")
V["B2"] = B2
# B2 prediction parts
pred_small = all(abs(V["fp"][s][f"primary:{r}@{caps[0]}/complete_only"][0] - 1) <= 0.05 for s in FP for r in rules_capped)
V["B2_prediction_cap_smallest_within_5pct_of_complete_only"] = pred_small

L.append(f"# v2 results (frozen protocol PROTOCOL_v2.md)\n\nCensored cells (target not reached within the draw budget): {censored}\n")
L.append(f"**B1** partials matter (complete_only / all, lower bound > 1 on every prime): primary {B1['primary']}, secondary {B1['secondary']}\n")
for s in FP:
    r = V["fp"][s]
    L.append(f"## {s} (|FB| {r['fb']}; mean draws to full rank: all {r['all_mean_draws']:.0f}, complete_only {r['complete_only_mean_draws']:.0f})\n")
    L.append(f"complete_only / all: primary {fmt(r['B1_primary'])}, secondary (90% rank) {fmt(r['B1_secondary'])}\n")
    for k in ("primary", "secondary"):
        L.append(f"{k}: draws(rule) / draws(all), mean of 10 seeds [95% CI]\n")
        L.append("| rule | " + " | ".join(f"cap {c}" for c in caps) + " |")
        L.append("|--|" + "--:|" * len(caps))
        for rr in rules_capped:
            L.append(f"| {rr} | " + " | ".join(fmt(r[f'{k}:{rr}@{c}/all']) for c in caps) + " |")
        L.append("")
L.append("## B2: versus first_come at the same cap (all three primes)\n")
for k, v in B2.items():
    L.append(f"- {k}: {v}")
L.append(f"\nB2 prediction 'at the smallest cap every rule is within 5% of complete_only': {pred_small}\n")

# REAL
for s, d in real.items():
    run = d["runs"][0]
    sc = scores(run)
    a = sc[("all", None)]
    tab = {f"{r}@{c}": [v[0] / a[0], v[1] / a[1], v[2]] for (r, c), v in sc.items()}
    V["real"][s] = {"rows": run["rows"], "fb": run["fb_columns"], "target": run["target"],
                    "all_draws": a[0], "all_draws_90": a[1], "ratios_to_all": tab}
    L.append(f"## {s}: {run['rows']} rows, |FB| {run['fb_columns']}, target rank {run['target']}, "
             f"all reaches it at draw {a[0]} (90%: {a[1]})\n")
    L.append("| rule@cap | primary / all | secondary / all | censored |\n|--|--:|--:|--|")
    for k, v in sorted(tab.items()):
        L.append(f"| {k} | {v[0]:.3f} | {v[1]:.3f} | {v[2]} |")
    L.append("")
if real:
    within = all(v[0] <= 1.10 for s in V["real"] for v in V["real"][s]["ratios_to_all"].values())
    V["B3_prediction_within_10pct"] = within
    L.append(f"B3 prediction 'every rule within 10% of all on REAL': {within}")
json.dump(V, open(f"{D}/verdict.json", "w"), indent=1)
open("RESULTS_V2.md", "w").write("\n".join(L) + "\n")
print("\n".join(L))
