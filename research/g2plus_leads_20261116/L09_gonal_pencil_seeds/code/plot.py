#!/usr/bin/env python3
"""plot.svg: median trial count T per arm (log scale) vs base fraction, one panel per prime. Stdlib only."""
import json, sys, math
S = json.load(open(sys.argv[1]))
out = sys.argv[2]
arms = ["A", "A2", "B", "PC1", "PC4"]
col = {"A": "#1f77b4", "A2": "#7f7f7f", "B": "#d62728", "PC1": "#2ca02c", "PC4": "#9467bd"}
betas = ["1/8", "3/16", "1/4", "3/8", "1/2", "3/4"]
PS = [251, 1021, 4093, 16381]
W, H, pw, ph = 1000, 760, 400, 270
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="11">',
       '<rect width="100%" height="100%" fill="white"/>',
       '<text x="10" y="18" font-size="14">L09: median line trials T per arm (open marker = at least 3 of 20 seeds censored). [pcc:unverified]</text>']
for pi, p in enumerate(PS):
    ox, oy = 60 + (pi % 2) * 480, 50 + (pi // 2) * 340
    vals = []
    for b in range(6):
        c = S["cells"].get(f"{p}_b{b}")
        if c:
            for a in arms:
                v = c["median_T"].get(a)
                if v and v > 0:
                    vals.append(v)
    if not vals:
        continue
    lo, hi = math.log10(min(vals)) - 0.1, math.log10(max(vals)) + 0.1
    def X(b): return ox + 20 + b * (pw - 40) / 5
    def Y(v): return oy + ph - (math.log10(max(v, 1e-9)) - lo) / (hi - lo) * ph
    svg.append(f'<rect x="{ox}" y="{oy}" width="{pw}" height="{ph}" fill="none" stroke="#444"/>')
    svg.append(f'<text x="{ox}" y="{oy-8}" font-size="12">p = {p}</text>')
    for b in range(6):
        svg.append(f'<text x="{X(b)-8}" y="{oy+ph+14}">{betas[b]}</text>')
    for e in range(math.ceil(lo), math.floor(hi) + 1):
        y = Y(10 ** e)
        svg.append(f'<line x1="{ox}" x2="{ox+pw}" y1="{y}" y2="{y}" stroke="#ddd"/><text x="{ox-34}" y="{y+4}">1e{e}</text>')
    for a in arms:
        pts = []
        for b in range(6):
            c = S["cells"].get(f"{p}_b{b}")
            if not c:
                continue
            v = c["median_T"].get(a)
            if v is None or v <= 0:
                continue
            cens = c["censored"][a] >= 3
            pts.append((X(b), Y(v), cens))
        if len(pts) > 1:
            svg.append('<polyline fill="none" stroke="%s" stroke-width="1.5" points="%s"/>' % (col[a], " ".join(f"{x:.1f},{y:.1f}" for x, y, _ in pts)))
        for x, y, cens in pts:
            svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="3.5" fill="{"white" if cens else col[a]}" stroke="{col[a]}"/>')
for i, a in enumerate(arms):
    svg.append(f'<rect x="{60+i*90}" y="{H-28}" width="10" height="10" fill="{col[a]}"/><text x="{75+i*90}" y="{H-19}">{a}</text>')
svg.append("</svg>")
open(out, "w").write("\n".join(svg))
