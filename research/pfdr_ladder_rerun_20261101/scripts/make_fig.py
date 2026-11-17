#!/usr/bin/env python3
"""results/analysis-main.json -> results/fig-kappa-ladder.svg"""
import json, math, os
os.chdir(os.path.join(os.path.dirname(__file__), ".."))
a = json.load(open("results/analysis-main.json")); L = a["lines"]
def get(kind, theta, x):
    return next(l for l in L if l["kind"] == kind and l["theta"] == theta and (l.get("d") == x or l.get("gamma") == x))
W, H = 980, 420
def p1(x0, w):
    ymin, ymax = 0.7, 500
    X = lambda v: x0 + 60 + (v - 15) / 18 * (w - 110)
    Y = lambda v: 28 + 280 * (1 - (math.log(v) - math.log(ymin)) / (math.log(ymax) - math.log(ymin)))
    o = [f'<text x="{x0+60}" y="16" font-size="13" font-weight="600">mean kappa per rung (log scale)</text>']
    for v in (1, 3, 10, 30, 100, 300):
        o.append(f'<line x1="{x0+60}" x2="{x0+w-50}" y1="{Y(v)}" y2="{Y(v)}" stroke="#ddd"/><text x="{x0+54}" y="{Y(v)+4}" font-size="10" text-anchor="end">{v}</text>')
    for v in (16, 20, 24, 28, 32): o.append(f'<text x="{X(v)}" y="326" font-size="10" text-anchor="middle">{v}</text>')
    o.append(f'<text x="{x0+w/2}" y="342" font-size="11" text-anchor="middle">log2 N (rung)</text>')
    def line(l, c, wd=2, dash=""):
        pts = [(k["Nbar"], k["kappa"]) for k in l["kappa_by_rung"]]
        o.append(f'<polyline fill="none" stroke="{c}" stroke-width="{wd}" {dash} points="' + " ".join(f"{X(u):.1f},{Y(v):.1f}" for u, v in pts) + '"/>')
    for th in (0.3, 0.35):
        for d in (1, 2, 3): line(get("struct", th, d), "#1f77b4", 1.6)
    for th in (0.3, 0.35): line(get("aa", th, 1), "#888", 1.2, 'stroke-dasharray="4 3"')
    for g, c in (("1/6", "#e69f00"), ("1/3", "#d55e00"), ("1/2", "#a02020")): line(get("plant", 0.35, g), c, 2)
    names = (("structured lines (6)", "#1f77b4"), ("A/A lines", "#888"), ("planted gamma 1/6", "#e69f00"), ("planted gamma 1/3", "#d55e00"), ("planted gamma 1/2", "#a02020"))
    for k, (n, c) in enumerate(names):
        xx, yy = x0 + 60 + (k % 3) * 135, 364 + (k // 3) * 16
        o.append(f'<rect x="{xx}" y="{yy-8}" width="10" height="4" fill="{c}"/><text x="{xx+14}" y="{yy}" font-size="10">{n}</text>')
    return "\n".join(o)
def p2(x0, w):
    X = lambda v: x0 + 190 + (v + 0.02) / 0.58 * (w - 240)
    o = [f'<text x="{x0+20}" y="16" font-size="13" font-weight="600">five-rung slope of log2 kappa (95% interval) vs break-even 1/3</text>']
    for v in (0, 0.1, 0.2, 0.3, 0.4, 0.5): o.append(f'<line x1="{X(v)}" x2="{X(v)}" y1="28" y2="326" stroke="#eee"/><text x="{X(v)}" y="340" font-size="10" text-anchor="middle">{v:g}</text>')
    o.append(f'<line x1="{X(1/3)}" x2="{X(1/3)}" y1="28" y2="326" stroke="#a02020" stroke-width="2" stroke-dasharray="5 3"/><text x="{X(1/3)+4}" y="38" font-size="10" fill="#a02020">break-even 1/3</text>')
    rows = [(f"structured theta={th:.2f} d={d}", get("struct", th, d), "#1f77b4") for th in (0.3, 0.35) for d in (1, 2, 3)]
    rows += [(f"planted gamma {g} (theta 0.35)", get("plant", 0.35, g), c) for g, c in (("1/6", "#e69f00"), ("1/3", "#d55e00"), ("1/2", "#a02020"))]
    for i, (n, l, c) in enumerate(rows):
        y = 56 + i * 30
        o.append(f'<text x="{x0+20}" y="{y+4}" font-size="11">{n}</text><line x1="{X(l["ci95"][0])}" x2="{X(l["ci95"][1])}" y1="{y}" y2="{y}" stroke="{c}" stroke-width="3"/><circle cx="{X(l["delta_hat"])}" cy="{y}" r="4" fill="{c}"/>')
    o.append(f'<text x="{x0+w/2+60}" y="358" font-size="11" text-anchor="middle">slope delta_hat</text>')
    return "\n".join(o)
open("results/fig-kappa-ladder.svg", "w").write(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif"><rect width="{W}" height="{H}" fill="white"/>{p1(0,470)}{p2(470,510)}</svg>')
