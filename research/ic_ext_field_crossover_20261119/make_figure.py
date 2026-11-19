#!/usr/bin/env python3
"""figure.svg: log2(IC/rho) against group bits, n=3, from results.json."""
import json, os, numpy as np
from analyze import ratio_log2, HERE
res = json.load(open(os.path.join(HERE, "results.json")))
W, H, L, Rr, T, B = 760, 440, 70, 20, 30, 60
X0, X1, Y0, Y1 = 20, 330, -12, 40
sx = lambda x: L + (x - X0) / (X1 - X0) * (W - L - Rr)
sy = lambda y: T + (Y1 - y) / (Y1 - Y0) * (H - T - B)
lp = np.arange(X0 / 3, X1 / 3, 0.1)
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="11">',
     f'<rect width="{W}" height="{H}" fill="#fff"/>']
for y in range(-10, 41, 10):
    o.append(f'<line x1="{L}" x2="{W-Rr}" y1="{sy(y)}" y2="{sy(y)}" stroke="#ddd"/><text x="{L-6}" y="{sy(y)+4}" text-anchor="end">{y}</text>')
for x in range(40, 321, 40):
    o.append(f'<line y1="{T}" y2="{H-B}" x1="{sx(x)}" x2="{sx(x)}" stroke="#eee"/><text x="{sx(x)}" y="{H-B+16}" text-anchor="middle">{x}</text>')
o.append(f'<line x1="{L}" x2="{W-Rr}" y1="{sy(0)}" y2="{sy(0)}" stroke="#000" stroke-width="1.5"/>')
o.append(f'<text x="{(L+W-Rr)/2}" y="{H-24}" text-anchor="middle">group size, bits (log2 #E = 3 log2 p)</text>')
o.append(f'<text transform="translate(16,{(T+H-B)/2}) rotate(-90)" text-anchor="middle">log2 (IC cost / matched rho cost)   [0 = crossover]</text>')
def curve(t, kappa, col, dash="", w=2):
    y = np.clip(ratio_log2(t, lp, kappa), Y0, Y1)
    pts = " ".join(f"{sx(3*a):.1f},{sy(b):.1f}" for a, b in zip(lp, y))
    o.append(f'<polyline points="{pts}" fill="none" stroke="{col}" stroke-width="{w}" stroke-dasharray="{dash}"/>')
F = res["fits"]
lab = []
for key, col, dash, name in (("scen_forced_theory", "#1f77b4", "", "n=3 LP, theory exponents (R p^4/3, LA p^4/3)"),
                             ("scen_R_fit_LA_theory", "#2ca02c", "6 3", "n=3 LP, fitted R exponent, LA p^4/3"),
                             ("scen_free", "#d62728", "2 3", "n=3 LP, all exponents fitted (LA p^1.54)")):
    curve({k: tuple(v) for k, v in F["LP"][key].items()}, 0.886, col, dash); lab.append((col, dash, name))
curve({k: tuple(v) for k, v in F["NL"]["scen_forced_theory"].items()}, 0.886, "#7f7f7f", "", 2); lab.append(("#7f7f7f", "", "n=3 no large primes (LA p^2)"))
# measured points (rho plain 0.886)
for arm, col in (("LP", "#1f77b4"), ("NL", "#7f7f7f")):
    for p, v in res["per_p"][arm]["per_p"].items():
        y = np.log2(v["S"] / 0.886)
        o.append(f'<circle cx="{sx(3*np.log2(int(p))):.1f}" cy="{sy(y):.1f}" r="4" fill="{col}"/>')
# desk crossovers n=4, n=5
for x, name in ((103.3, "n=4 desk: 2^103 (gaudry_quartic_d4_cost)"), (291, "n=5 desk: 2^291 (joux_vitse_n1)")):
    o.append(f'<line x1="{sx(x)}" x2="{sx(x)}" y1="{T}" y2="{H-B}" stroke="#ff7f0e" stroke-dasharray="4 4"/>')
    o.append(f'<text x="{sx(x)-4}" y="{T+12+ (0 if x<200 else 14)}" text-anchor="end" fill="#ff7f0e">{name}</text>')
for i, (col, dash, name) in enumerate(lab):
    y = T + 6 + i * 14 + 150
    o.append(f'<line x1="{L+10}" x2="{L+34}" y1="{y}" y2="{y}" stroke="{col}" stroke-width="2" stroke-dasharray="{dash}"/><text x="{L+40}" y="{y+4}">{name}</text>')
o.append(f'<text x="{L+10}" y="{T+12+150+5*14+8}" fill="#444">dots: measured mean S_IC / 0.886 (rho theory), single thread; rho plain (kappa 0.886); [pcc:unverified]</text>')
o.append("</svg>")
open(os.path.join(HERE, "figure.svg"), "w").write("\n".join(o))
print("ok")
