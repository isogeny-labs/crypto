#!/usr/bin/env python3
"""Derived statistic: weighted LS of ln(sib8-p256 / rho-neg, GAE raw ~ w=0) on log2 r.
Points: Table B round 3 and ablation re-run (per-curve rows of the compare tables), and the 28/32-bit curves here.
SE on the log scale = (ln hi - ln lo) / (2 * 1.96)  (interval treated as symmetric on the log scale).  Not a preregistered scaling claim."""
import re, json, math, sys
T = "research/ic_tableb_sib_20261119"; H = "research/ic_scale_l4_20261119"
row = re.compile(r"^\| (icv1-\S+) \| ([\d.]+) \| (\d+) \| ([\d.]+) \| \[([\d.]+), ([\d.]+)\]")
def rows(path, group):
    out = []
    for l in open(path):
        m = row.match(l)
        if m: out.append(dict(group=group, curve=m[1], x=float(m[2]), r=float(m[4]), lo=float(m[5]), hi=float(m[6])))
    return out
pts = []
for s in (20, 22, 24):
    pts += rows(f"{T}/compare-P{s}-rho-neg-vs-sib8-p256.txt", "tableB-round3")
    pts += rows(f"{T}/ablation/compare-P{s}-rho-neg-vs-sib8-p256.txt", "tableB-ablation")
for s in (28, 32):
    pts += rows(f"{H}/compare-P{s}-rho-neg-vs-sib8-p256.txt", f"L4-{s}")
for p in pts: p["y"] = math.log(p["r"]); p["se"] = (math.log(p["hi"]) - math.log(p["lo"])) / (2 * 1.96)
def wls(P):
    w = [1 / p["se"] ** 2 for p in P]; sw = sum(w)
    xb = sum(wi * p["x"] for wi, p in zip(w, P)) / sw; yb = sum(wi * p["y"] for wi, p in zip(w, P)) / sw
    sxx = sum(wi * (p["x"] - xb) ** 2 for wi, p in zip(w, P)); sxy = sum(wi * (p["x"] - xb) * (p["y"] - yb) for wi, p in zip(w, P))
    b = sxy / sxx; a = yb - b * xb; se = math.sqrt(1 / sxx)
    chi2 = sum(wi * (p["y"] - a - b * p["x"]) ** 2 for wi, p in zip(w, P)); df = len(P) - 2
    return dict(n=len(P), slope=b, slope_se=se, slope_ci=[b - 1.96 * se, b + 1.96 * se], intercept=a, chi2=chi2, df=df,
                slope_se_scaled=se * math.sqrt(max(chi2 / df, 1)), xb=xb, yb=yb,
                fit=lambda x: math.exp(a + b * x))
out = {}
sets = {"tableB_only(20-24)": [p for p in pts if p["group"].startswith("tableB")],
        "all(20-32)": pts, "L4_only(28,32)": [p for p in pts if p["group"].startswith("L4")]}
for k, P in sets.items():
    if len(P) >= 3 or k == "L4_only(28,32)" and len(P) >= 3:
        f = wls(P); f.pop("fit"); out[k] = f
out["points"] = pts
json.dump(out, open(f"{H}/trend.json", "w"), indent=1)
for k in sets:
    if k in out:
        f = out[k]; print(k, f"n={f['n']} slope/bit={f['slope']:.4f} +-{f['slope_se']:.4f} CI[{f['slope_ci'][0]:.4f},{f['slope_ci'][1]:.4f}] chi2/df={f['chi2']:.1f}/{f['df']}",
              f"scaled-SE CI half-width={1.96*f['slope_se_scaled']:.4f}", "ratio@20,28,32 =", *[f"{math.exp(f['intercept']+f['slope']*x):.3f}" for x in (20, 28, 32)])
