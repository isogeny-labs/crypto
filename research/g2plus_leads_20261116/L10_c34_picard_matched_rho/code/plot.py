#!/usr/bin/env python3
"""plot.svg: mean fpm_total per arm vs p (log-log), from raw/measure.jsonl. Stdlib only."""
import json, math, sys
raw = sys.argv[1]; out = sys.argv[2]
rows = [json.loads(l) for l in open(raw) if l.startswith("{")]
arms = ["Q_full", "C_full", "P_full", "P_orbit"]
col = {"Q_full": "#1f77b4", "C_full": "#2ca02c", "P_full": "#d62728", "P_orbit": "#9467bd"}
data = {a: {} for a in arms}
for d in rows:
    data[d["arm"]].setdefault(d["p"], []).append(d["fpm_total"])
ps = sorted({d["p"] for d in rows})
pts = {a: [(p, sum(v) / len(v)) for p, v in sorted(data[a].items())] for a in arms}
allv = [y for a in arms for _, y in pts[a]]
x0, x1 = math.log10(min(ps)), math.log10(max(ps))
y0, y1 = math.floor(math.log10(min(allv))), math.ceil(math.log10(max(allv)))
W, H, L, R, T, B = 720, 460, 80, 20, 40, 60
X = lambda p: L + (math.log10(p) - x0) / (x1 - x0) * (W - L - R)
Y = lambda v: H - B - (math.log10(v) - y0) / (y1 - y0) * (H - T - B)
s = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d" font-family="sans-serif" font-size="12">' % (W, H, W, H)]
s.append('<rect width="100%" height="100%" fill="white"/>')
s.append('<text x="%d" y="22" font-size="14">L10 [pcc:unverified]: mean fpm_total (FPM, counts) per arm vs p</text>' % L)
for e in range(y0, y1 + 1):
    y = Y(10 ** e)
    s.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="#ddd"/><text x="%d" y="%.1f" text-anchor="end">1e%d</text>' % (L, W - R, y, y, L - 6, y + 4, e))
for p in ps:
    s.append('<line x1="%.1f" x2="%.1f" y1="%d" y2="%d" stroke="#eee"/><text x="%.1f" y="%d" text-anchor="middle">%d</text>' % (X(p), X(p), T, H - B, X(p), H - B + 16, p))
s.append('<text x="%d" y="%d" text-anchor="middle">p (log scale)</text>' % ((L + W - R) // 2, H - 14))
for i, a in enumerate(arms):
    path = " ".join("%s%.1f,%.1f" % ("M" if j == 0 else "L", X(p), Y(v)) for j, (p, v) in enumerate(pts[a]))
    s.append('<path d="%s" fill="none" stroke="%s" stroke-width="2"/>' % (path, col[a]))
    for p, v in pts[a]:
        s.append('<circle cx="%.1f" cy="%.1f" r="3" fill="%s"/>' % (X(p), Y(v), col[a]))
    s.append('<rect x="%d" y="%d" width="12" height="12" fill="%s"/><text x="%d" y="%d">%s</text>' % (W - 160, T + 8 + 18 * i, col[a], W - 142, T + 19 + 18 * i, a))
s.append("</svg>")
open(out, "w").write("\n".join(s))
