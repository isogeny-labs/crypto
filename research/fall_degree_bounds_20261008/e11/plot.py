#!/usr/bin/env python3
"""window_vs_dense.svg: degree-4 closure seconds against columns (log-log), the
w1x restricted closure (measured, 3 threads) against the dense engine on the same
cells (measured; committed records at 4 threads, fresh draws at 3), plus the
w1x column counts projected at N ~ 100 (vertical marks, no time measured).
Plain SVG, no plotting library."""
import glob
import json
import math
import statistics as st
from collections import defaultdict

E = "research/fall_degree_bounds_20261008/e11"
win = defaultdict(list)
den = defaultdict(list)
for p in glob.glob(f"{E}/validate/*.jsonl") + glob.glob(f"{E}/scaling/window_*.jsonl"):
    for ln in open(p):
        r = json.loads(ln)
        w = r["window"].get("w1x")
        if w and w.get("certified"):
            win[(r["m"], r["n"], r["ell"], w["cols"])].append(w["secs"])
dense_files = glob.glob("research/fall_degree_bounds_20261008/e7/E7_*.jsonl") + \
    glob.glob("research/fall_degree_bounds_20261008/e8/E8_*.jsonl") + \
    [f"{E}/../e6/B_4-13-4.par.jsonl"] + glob.glob(f"{E}/scaling/dense_*.jsonl")
for p in dense_files:
    for ln in open(p):
        r = json.loads(ln)
        s4 = r.get("per_degree", {}).get("4")
        if r.get("d_f") == 4 and s4 and s4.get("secs", 0) > 0:
            den[(r["m"], s4["of"])].append(s4["secs"])
proj = json.load(open(f"{E}/projection.json"))

W, H, L, B, T, R = 720, 460, 70, 50, 30, 20
x0, x1, y0, y1 = math.log10(1e4), math.log10(3e6), math.log10(0.3), math.log10(3000)
X = lambda c: L + (math.log10(c) - x0) / (x1 - x0) * (W - L - R)  # noqa: E731
Y = lambda s: H - B - (math.log10(s) - y0) / (y1 - y0) * (H - B - T)  # noqa: E731
col = {3: "#1b6ca8", 4: "#d1495b", 5: "#2a9d8f", 6: "#8d6a9f", 7: "#e9a03b"}
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="12">',
     f'<rect width="{W}" height="{H}" fill="white"/>']
for c in (1e4, 3e4, 1e5, 3e5, 1e6, 3e6):
    o.append(f'<line x1="{X(c):.1f}" y1="{T}" x2="{X(c):.1f}" y2="{H-B}" stroke="#ddd"/>')
    o.append(f'<text x="{X(c):.1f}" y="{H-B+16}" text-anchor="middle">{c:,.0f}</text>')
for s in (1, 10, 100, 1000):
    o.append(f'<line x1="{L}" y1="{Y(s):.1f}" x2="{W-R}" y2="{Y(s):.1f}" stroke="#ddd"/>')
    o.append(f'<text x="{L-6}" y="{Y(s)+4:.1f}" text-anchor="end">{s}</text>')
o.append(f'<text x="{(L+W-R)/2}" y="{H-12}" text-anchor="middle">columns (degree ≤ 4 monomials kept)</text>')
o.append(f'<text x="18" y="{(T+H-B)/2}" transform="rotate(-90 18 {(T+H-B)/2})" text-anchor="middle">degree-4 closure seconds (median)</text>')
for (m, n, ell, c), s in win.items():
    if c >= 1e4:
        o.append(f'<circle cx="{X(c):.1f}" cy="{Y(max(st.median(s), 0.3)):.1f}" r="4.5" fill="{col.get(m, "#555")}"/>')
for (m, c), s in den.items():
    if c >= 1e4:
        x, y = X(c), Y(st.median(s))
        o.append(f'<rect x="{x-4:.1f}" y="{y-4:.1f}" width="8" height="8" fill="none" stroke="{col.get(m, "#555")}" stroke-width="1.6"/>')
for p in proj:
    x = X(p["w1_cols"])
    o.append(f'<line x1="{x:.1f}" y1="{T}" x2="{x:.1f}" y2="{H-B}" stroke="#999" stroke-dasharray="4 3"/>')
    o.append(f'<text x="{x+3:.1f}" y="{T+12}" fill="#666">{p["cell"]} w1x (projected cols)</text>')
lx = L + 12
o.append(f'<text x="{lx}" y="{H-B-70}">filled dot: w1x (measured)   open square: dense hky_par (measured)</text>')
for i, m in enumerate(sorted(col)):
    o.append(f'<circle cx="{lx+6+i*70}" cy="{H-B-50}" r="4.5" fill="{col[m]}"/><text x="{lx+14+i*70}" y="{H-B-46}">m = {m}</text>')
o.append("</svg>")
open(f"{E}/window_vs_dense.svg", "w").write("\n".join(o))
print("wrote", f"{E}/window_vs_dense.svg")
