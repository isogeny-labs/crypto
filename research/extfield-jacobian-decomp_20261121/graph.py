#!/usr/bin/env python3
"""Draws graph.svg from RESULTS.json (pure Python SVG)."""
import json, math, os
HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "RESULTS.json")))
QS = D["qs"]; ARMS = D["arms"]
COL = {"control_a1": "#1f77b4", "a0.75_c1": "#d62728", "a0.75_c2": "#ff7f0e", "a0.6_c1": "#2ca02c", "a0.6_c2": "#9467bd", "rho": "#000000"}
W, H = 980, 420
def panel(x0, y0, w, h, title, series, ylabel, ref=None):
    pts = [v for s in series.values() for v in s if v]
    lo, hi = math.log10(min(pts)) - 0.2, math.log10(max(pts)) + 0.2
    lx0, lx1 = math.log10(QS[0]) - 0.05, math.log10(QS[-1]) + 0.05
    X = lambda q: x0 + (math.log10(q) - lx0) / (lx1 - lx0) * w
    Y = lambda v: y0 + h - (math.log10(v) - lo) / (hi - lo) * h
    o = [f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" fill="none" stroke="#888"/>',
         f'<text x="{x0}" y="{y0-10}" font-size="13" font-weight="bold">{title}</text>',
         f'<text x="{x0-45}" y="{y0+h/2}" font-size="11" transform="rotate(-90 {x0-45} {y0+h/2})" text-anchor="middle">{ylabel}</text>']
    for q in QS:
        o.append(f'<line x1="{X(q)}" y1="{y0+h}" x2="{X(q)}" y2="{y0+h+4}" stroke="#444"/><text x="{X(q)}" y="{y0+h+16}" font-size="10" text-anchor="middle">{q}</text>')
    for e in range(math.ceil(lo), math.floor(hi) + 1):
        o.append(f'<line x1="{x0}" y1="{Y(10**e)}" x2="{x0+w}" y2="{Y(10**e)}" stroke="#ddd"/><text x="{x0-4}" y="{Y(10**e)+3}" font-size="10" text-anchor="end">1e{e}</text>')
    if ref is not None:
        o.append(f'<line x1="{x0}" y1="{Y(ref)}" x2="{x0+w}" y2="{Y(ref)}" stroke="#000" stroke-dasharray="5,4"/><text x="{x0+w-4}" y="{Y(ref)-4}" font-size="10" text-anchor="end">rho (ratio 1)</text>')
    for name, vals in series.items():
        pp = [(X(q), Y(v)) for q, v in zip(QS, vals) if v]
        if not pp:
            continue
        o.append(f'<polyline fill="none" stroke="{COL[name]}" stroke-width="2" points="{" ".join(f"{a:.1f},{b:.1f}" for a, b in pp)}"/>')
        for a, b in pp:
            o.append(f'<circle cx="{a:.1f}" cy="{b:.1f}" r="3" fill="{COL[name]}"/>')
    return o
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="sans-serif">',
       f'<rect width="{W}" height="{H}" fill="white"/>']
s1 = {a: [D["cells"][a].get(str(q), {}).get("S_w0.1", {}).get("mean") for q in QS] for a in ARMS}
s1["rho"] = [D["rho"].get(str(q), {}).get("S_w0.1", {}).get("mean") for q in QS]
svg += panel(70, 40, 400, 290, "Total S at w = 0.1 GAE/op (mean of 3 seeds) [pcc:unverified]", s1, "S (GAE)")
s2 = {a: D["ratios"][a]["w0.1"]["values"] for a in ARMS}
svg += panel(550, 40, 400, 290, "S(arm)/S(rho) at w = 0.1 [pcc:unverified]", s2, "ratio", ref=1.0)
lx = 70
for i, name in enumerate(ARMS + ["rho"]):
    svg.append(f'<rect x="{lx+i*130}" y="375" width="12" height="12" fill="{COL[name]}"/><text x="{lx+i*130+16}" y="386" font-size="11">{name}</text>')
svg.append(f'<text x="70" y="410" font-size="10">q on the x axes (log scale); same instances and seeds for every arm; local development check, not a lab benchmark.</text>')
svg.append("</svg>")
open(os.path.join(HERE, "graph.svg"), "w").write("\n".join(svg))
print("wrote graph.svg")
