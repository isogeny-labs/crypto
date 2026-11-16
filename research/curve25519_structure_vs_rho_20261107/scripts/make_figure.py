#!/usr/bin/env python3
"""Lever factor on expected cost per solve, divided by rho-neg, at w = 0, 0.1, 1 (log scale). SVG, no deps.

Values: C5 Monte Carlo (data/levers.json) for the torsion keys at w = 0; everything else derived in
README section 3 (the endomorphism row uses the optimistic evaluation cost C_phi stated there).
"""
import json, math
from pathlib import Path

here = Path(__file__).resolve().parent.parent
d = json.loads((here / "data" / "levers.json").read_text())["checks"]["C5"]
xi = d["xi"]["mean_over_sqrt_r"] / d["neg"]["mean_over_sqrt_r"]
W = (0.0, 0.1, 1.0)


def endo(m, C):
    return [(1 + w * (m - 1) * C) / math.sqrt(m) for w in W]


rows = [
    ("T0 key x -> 1/x on <G>+<T0> (MC)", [xi, xi, xi]),
    ("E[8] keys by 7 translations", [8 * d["e8"]["mean_over_sqrt_r"] / d["neg"]["mean_over_sqrt_r"]] * 3),
    ("endomorphism fold, m = 3 (C_phi approx 140)", endo(3, 140)),
    ("endomorphism fold, m = 9 (C_phi approx 270)", endo(9, 270)),
    ("twist, E', x-only, special prime, DP, batching", [1.0, 1.0, 1.0]),
]
colors = ["#2a6fdb", "#d9822b", "#c23b5a", "#8a4fbf", "#4d8f3a"]
Wd, H, L, R, T, B = 760, 420, 70, 330, 30, 50
lo, hi = math.log10(0.2), math.log10(2000)


def y(v):
    return T + (hi - math.log10(v)) / (hi - lo) * (H - T - B)


def x(i):
    return L + i * (Wd - L - R) / 2


s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{Wd}" height="{H}" font-family="sans-serif" font-size="12">',
     f'<rect width="{Wd}" height="{H}" fill="white"/>']
for v in (0.2, 0.5, 1, 2, 5, 10, 100, 1000):
    s.append(f'<line x1="{L}" x2="{Wd-R}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="#ddd"/>')
    s.append(f'<text x="{L-6}" y="{y(v)+4:.1f}" text-anchor="end">{v:g}</text>')
s.append(f'<line x1="{L}" x2="{Wd-R}" y1="{y(1):.1f}" y2="{y(1):.1f}" stroke="#000" stroke-dasharray="5,3"/>')
s.append(f'<text x="{Wd-R-4}" y="{y(1)-5:.1f}" text-anchor="end">rho-neg (1.0)</text>')
for i, w in enumerate(W):
    s.append(f'<text x="{x(i):.1f}" y="{H-B+20}" text-anchor="middle">w = {w:g}</text>')
s.append(f'<text x="18" y="{(H)/2}" transform="rotate(-90 18 {H/2})" text-anchor="middle">expected cost per solve / rho-neg (log)</text>')
for k, (name, v) in enumerate(rows):
    c = colors[k]
    pts = " ".join(f"{x(i):.1f},{y(max(vv, 0.21)):.1f}" for i, vv in enumerate(v))
    s.append(f'<polyline points="{pts}" fill="none" stroke="{c}" stroke-width="2"/>')
    for i, vv in enumerate(v):
        s.append(f'<circle cx="{x(i):.1f}" cy="{y(vv):.1f}" r="4" fill="{c}"/>')
    s.append(f'<rect x="{Wd-R+20}" y="{T+10+k*22}" width="12" height="12" fill="{c}"/>')
    s.append(f'<text x="{Wd-R+38}" y="{T+20+k*22}">{name.replace("&","&amp;").replace("<","&lt;").replace(">","&gt;")}</text>')
s.append(f'<text x="{Wd-R+20}" y="{T+10+len(rows)*22+20}" font-size="11">Below 1 only at w = 0 (endomorphism</text>')
s.append(f'<text x="{Wd-R+20}" y="{T+10+len(rows)*22+36}" font-size="11">rows): unpriced native work; §5a</text>')
s.append(f'<text x="{Wd-R+20}" y="{T+10+len(rows)*22+52}" font-size="11">band order flips, so no rank.</text>')
s.append("</svg>")
(here / "figures").mkdir(exist_ok=True)
(here / "figures" / "lever_factors.svg").write_text("\n".join(s))
json.dump({n: v for n, v in rows}, open(here / "data" / "lever_factors.json", "w"), indent=1)
print({n: [round(t, 3) for t in v] for n, v in rows})
