#!/usr/bin/env python3
"""Draw figure.svg from RESULTS.json (pure Python SVG). [pcc:unverified] numbers only."""
import json, math, os, statistics as st

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "RESULTS.json")))
LADDER = [251, 509, 1021, 2039]
COL = {"0.500": "#1b9e77", "0.667": "#7570b3", "0.750": "#d95f02", "rho": "#000000"}


def panel(x0, y0, w, h, title, series, ylabel):
    """series: list of (label, color, {p: value}); log-log axes."""
    pts = [v for _, _, s in series for v in s.values() if v]
    if not pts:
        return ""
    lo, hi = math.log10(min(pts)) - 0.2, math.log10(max(pts)) + 0.2
    xs = lambda p: x0 + 40 + (math.log(p) - math.log(200)) / (math.log(2500) - math.log(200)) * (w - 60)
    ys = lambda v: y0 + h - 30 - (math.log10(v) - lo) / (hi - lo) * (h - 60)
    o = [f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" fill="none" stroke="#888"/>',
         f'<text x="{x0+w/2}" y="{y0+16}" text-anchor="middle" font-size="12">{title}</text>',
         f'<text x="{x0+8}" y="{y0+h/2}" font-size="10" transform="rotate(-90 {x0+8} {y0+h/2})" text-anchor="middle">{ylabel}</text>']
    for p in LADDER:
        o.append(f'<text x="{xs(p)}" y="{y0+h-12}" text-anchor="middle" font-size="10">{p}</text>')
    for e in range(int(math.floor(lo)), int(math.ceil(hi)) + 1):
        if lo <= e <= hi:
            o.append(f'<line x1="{x0+40}" x2="{x0+w-20}" y1="{ys(10**e)}" y2="{ys(10**e)}" stroke="#ddd"/>'
                     f'<text x="{x0+36}" y="{ys(10**e)+3}" text-anchor="end" font-size="9">1e{e}</text>')
    for i, (lab, col, s) in enumerate(series):
        pp = [(xs(p), ys(v)) for p, v in sorted(s.items()) if v]
        if pp:
            o.append(f'<polyline fill="none" stroke="{col}" stroke-width="1.5" points="{" ".join("%.1f,%.1f" % q for q in pp)}"/>')
            for q in pp:
                o.append(f'<circle cx="{q[0]:.1f}" cy="{q[1]:.1f}" r="3" fill="{col}"/>')
        o.append(f'<text x="{x0+50}" y="{y0+30+12*i}" font-size="10" fill="{col}">{lab}</text>')
    return "\n".join(o)


def med(rs, k):
    v = [r[k] for r in rs if r["solved"] and r.get(k)]
    return st.median(v) if v else None


series = {"rel": [], "S0": [], "S01": [], "S1": [], "ratio": []}
for key, per in R["arms"].items():
    col = COL.get(key, "#999")
    d = lambda k: {int(p): med(rs, k) for p, rs in per.items() if int(p) in LADDER}
    series["rel"].append(("alpha=" + key, col, d("rel_nat_all")))
    series["S0"].append(("alpha=" + key, col, d("S_all_w0")))
    series["S01"].append(("alpha=" + key, col, d("S_all_w0.1")))
    series["S1"].append(("alpha=" + key, col, d("S_all_w1")))
    series["ratio"].append(("alpha=" + key, col, d("ratio_w0.1")))
rho = {int(p): v["adds_mean"] for p, v in R["rho"].items()}
rho01 = {int(p): v["adds_mean"] + 0.1 * v["native_mean"] for p, v in R["rho"].items()}
rho1 = {int(p): v["adds_mean"] + 1.0 * v["native_mean"] for p, v in R["rho"].items()}
series["S0"].append(("rho (measured)", "#000", rho))
series["S01"].append(("rho (measured)", "#000", rho01))
series["S1"].append(("rho (measured)", "#000", rho1))
W, H = 900, 640
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="sans-serif">',
       f'<rect width="{W}" height="{H}" fill="white"/>',
       '<text x="450" y="18" text-anchor="middle" font-size="14">Genus-4 (3,3): root-finding relation phase + large primes vs plain rho [pcc:unverified]</text>']
svg.append(panel(10, 30, 430, 290, "relation-phase native ops (all rungs), median", series["rel"], "F_p mulmods"))
svg.append(panel(460, 30, 430, 290, "total S, w = 0 GAE/op (group ops only)", series["S0"], "GAE"))
svg.append(panel(10, 330, 290, 290, "total S, w = 0.1", series["S01"], "GAE"))
svg.append(panel(310, 330, 290, 290, "total S, w = 1", series["S1"], "GAE"))
svg.append(panel(610, 330, 280, 290, "S_IC / S_rho at w = 0.1", series["ratio"], "ratio"))
svg.append("</svg>")
open(os.path.join(HERE, "figure.svg"), "w").write("\n".join(svg))
print("wrote figure.svg")
