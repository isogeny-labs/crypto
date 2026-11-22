#!/usr/bin/env python3
"""Draws graph.svg from RESULTS.json (pure Python SVG). usage: python3 plot.py"""
import json, math, os

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "RESULTS.json")))
ps = [int(p) for p in R["ps"]]
W, H = 1180, 430
PANEL = 360
cols = {"rho": "#1b1b1b", "none": "#1f77b4", "z2b": "#2ca02c", "z3": "#d62728"}
names = {"rho": "measured negation rho on J_H", "none": "S without order phase (ell public)", "z2b": "S, order by N1,N2 + BSGS (Z2B)", "z3": "S, order by zeta counts over F_p, F_p2, F_p3 (Z3)"}
out = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d" font-family="sans-serif" font-size="12">' % (W, H, W, H),
       '<rect width="100%" height="100%" fill="white"/>',
       '<text x="20" y="22" font-size="15" font-weight="bold">Smith route to a plane quartic: S and measured rho on J_H (GAE_H, single target) [pcc:unverified]</text>']


def panel(ix, w):
    x0 = 55 + ix * (PANEL + 25)
    y0, y1 = 50, 330
    series = {}
    for k in ("rho", "none", "z2b", "z3"):
        if k == "rho":
            series[k] = [R["per_p"][str(p)]["S"]["w%g" % w]["rho_mean"] for p in ps]
        else:
            series[k] = [R["per_p"][str(p)]["S"]["w%g" % w][k]["S_mean"] for p in ps]
    allv = [v for s in series.values() for v in s]
    lo, hi = math.floor(math.log10(min(allv))), math.ceil(math.log10(max(allv)))
    xs = [math.log10(p) for p in ps]
    xlo, xhi = math.log10(80), math.log10(1300)

    def X(p):
        return x0 + (math.log10(p) - xlo) / (xhi - xlo) * PANEL

    def Y(v):
        return y1 - (math.log10(v) - lo) / (hi - lo) * (y1 - y0)
    out.append('<text x="%d" y="%d" font-weight="bold">w = %g per native operation</text>' % (x0, y0 - 12, w))
    out.append('<rect x="%d" y="%d" width="%d" height="%d" fill="none" stroke="#888"/>' % (x0, y0, PANEL, y1 - y0))
    for e in range(lo, hi + 1):
        out.append('<line x1="%d" y1="%.1f" x2="%d" y2="%.1f" stroke="#ddd"/><text x="%d" y="%.1f" text-anchor="end">1e%d</text>' % (x0, Y(10.0 ** e), x0 + PANEL, Y(10.0 ** e), x0 - 4, Y(10.0 ** e) + 4, e))
    for p in ps:
        out.append('<line x1="%.1f" y1="%d" x2="%.1f" y2="%d" stroke="#eee"/><text x="%.1f" y="%d" text-anchor="middle">%d</text>' % (X(p), y0, X(p), y1, X(p), y1 + 15, p))
    out.append('<text x="%d" y="%d" text-anchor="middle">p</text>' % (x0 + PANEL // 2, y1 + 32))
    for k, vals in series.items():
        pts = " ".join("%.1f,%.1f" % (X(p), Y(v)) for p, v in zip(ps, vals))
        out.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (cols[k], pts))
        for p, v in zip(ps, vals):
            out.append('<circle cx="%.1f" cy="%.1f" r="3.5" fill="%s"/>' % (X(p), Y(v), cols[k]))


for i, w in enumerate((0.0, 0.1, 1.0)):
    panel(i, w)
y = 385
for j, k in enumerate(("rho", "none", "z2b", "z3")):
    x = 60 + (j % 2) * 540
    yy = y + (j // 2) * 18
    out.append('<line x1="%d" y1="%d" x2="%d" y2="%d" stroke="%s" stroke-width="3"/><text x="%d" y="%d">%s</text>' % (x, yy, x + 24, yy, cols[k], x + 30, yy + 4, names[k]))
out.append("</svg>")
open(os.path.join(HERE, "graph.svg"), "w").write("\n".join(out))
