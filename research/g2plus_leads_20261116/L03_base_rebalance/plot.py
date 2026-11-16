#!/usr/bin/env python3
"""plot.svg: mean FPM_mid (lambda=1) over all 12 runs per prime, vs cc, one curve per d. Standard library only."""
import json, glob, math
GRID = [2.0 ** (k / 2.0) for k in range(-6, 5)]
BANDI, KAPPA = 10, 9
runs = {}
for fn in sorted(glob.glob("raw/runs_*.jsonl")):
    for l in open(fn):
        if '"kind":"run"' in l:
            x = json.loads(l)
            runs.setdefault((x["p"], round(math.log2(x["cc"]) * 2)), []).append(x)
def fpm(x, d):
    I = BANDI
    return (x["cm_setup"] + I * x["ci_setup"] + x["cm_verify"] + I * x["ci_verify"] + x["rel_fixed_mul"]
            + (x["cm_loop"] + I * x["ci_loop"] + x["rel_trial_mul"]) / d + KAPPA * x["la_mul"])
P = [8209, 16411, 32771]; Ds = [1, 3, 10, 30]
cols = ["#1b6ca8", "#2a9d8f", "#e07a1f", "#b5179e"]
W, H, ml, mr, mt, mb = 360, 300, 60, 10, 30, 45
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{3*W}" height="{H+30}" font-family="sans-serif" font-size="11">',
       '<rect width="100%" height="100%" fill="white"/>']
for pi, p in enumerate(P):
    ox = pi * W
    data = {d: [sum(fpm(x, d) for x in runs[(p, k)]) / len(runs[(p, k)]) for k in range(-6, 5)] for d in Ds}
    ys = [math.log10(v) for d in Ds for v in data[d]]
    y0, y1 = min(ys) - 0.05, max(ys) + 0.05
    X = lambda k: ox + ml + (k + 6) / 10 * (W - ml - mr)
    Y = lambda v: mt + (1 - (math.log10(v) - y0) / (y1 - y0)) * (H - mt - mb)
    svg.append(f'<text x="{ox+ml}" y="18" font-weight="bold">p = {p}: mean FPM (band 10,9; lambda=1) vs cc [pcc:unverified]</text>')
    svg.append(f'<rect x="{ox+ml}" y="{mt}" width="{W-ml-mr}" height="{H-mt-mb}" fill="none" stroke="#888"/>')
    for k in range(-6, 5, 2):
        svg.append(f'<text x="{X(k)}" y="{H-mb+14}" text-anchor="middle">{2**(k/2):.3g}</text>')
    svg.append(f'<text x="{ox+ml+(W-ml-mr)/2}" y="{H-8}" text-anchor="middle">cc (base constant)</text>')
    t = math.ceil(y0)
    while t <= y1:
        svg.append(f'<text x="{ox+ml-4}" y="{Y(10**t)+4}" text-anchor="end">1e{t}</text>')
        svg.append(f'<line x1="{ox+ml}" x2="{ox+W-mr}" y1="{Y(10**t)}" y2="{Y(10**t)}" stroke="#ddd"/>')
        t += 1
    for ci, d in enumerate(Ds):
        pts = " ".join(f"{X(k):.1f},{Y(v):.1f}" for k, v in zip(range(-6, 5), data[d]))
        svg.append(f'<polyline points="{pts}" fill="none" stroke="{cols[ci]}" stroke-width="2"/>')
        svg.append(f'<text x="{ox+ml+8+ci*70}" y="{mt+14}" fill="{cols[ci]}">d={d}{" (synthetic)" if d>1 else ""}</text>')
svg.append('</svg>')
open("plot.svg", "w").write("\n".join(svg))
