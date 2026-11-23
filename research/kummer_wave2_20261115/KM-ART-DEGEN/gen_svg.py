#!/usr/bin/env python3
import json, sys, collections
src, out = sys.argv[1], sys.argv[2]
rows = [json.loads(l) for l in open(src) if l.strip()]
rows = [r for r in rows if r["kind"] == "main" and r["dF"] is not None]
vals = collections.defaultdict(set)
for r in rows: vals[(r["family"], r["eps"], r["t"])].add(r["dF"])
W, H = 760, 420
col = {1: "#1f5fa8", 0: "#c4571a", 5: "#2e8b57"}
off = {1: -7, 0: 0, 5: 7}
s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12">',
     f'<rect width="{W}" height="{H}" fill="#ffffff"/>',
     '<text x="380" y="20" text-anchor="middle" font-size="14">Last fall degree d_F by top constant eps (m = 2; every plotted dot is a cell value; [pcc:unverified])</text>']
for pi, (fam, label) in enumerate((("tower", "tower (S3)"), ("w1", "W = 1 box"))):
    x0 = 50 + pi * 370; y0 = 360; pw = 300; ph = 280
    s.append(f'<text x="{x0 + pw/2}" y="45" text-anchor="middle">{label}</text>')
    for d in range(2, 8):
        y = y0 - (d - 2) / 5 * ph
        s.append(f'<line x1="{x0}" y1="{y}" x2="{x0+pw}" y2="{y}" stroke="#dddddd"/><text x="{x0-8}" y="{y+4}" text-anchor="end">{d}</text>')
    for t in range(2, 9):
        x = x0 + (t - 2) / 6 * pw
        s.append(f'<text x="{x}" y="{y0+18}" text-anchor="middle">{t}</text>')
    s.append(f'<text x="{x0+pw/2}" y="{y0+38}" text-anchor="middle">t (K = 2^t)</text>')
    for eps in (1, 0, 5):
        pts = []
        for t in range(2, 9):
            for v in sorted(vals.get((fam, eps, t), ())):
                x = x0 + (t - 2) / 6 * pw + off[eps]; y = y0 - (v - 2) / 5 * ph
                pts.append((x, y))
                s.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="3.5" fill="{col[eps]}" fill-opacity="0.75"/>')
for i, (eps, lab) in enumerate(((1, "eps = 1 (real tower)"), (0, "eps = 0 (Artinian)"), (5, "eps = 5 (generic control)"))):
    s.append(f'<circle cx="{60 + i*200}" cy="{H-12}" r="4" fill="{col[eps]}"/><text x="{70 + i*200}" y="{H-8}">{lab}</text>')
s.append('</svg>')
open(out, "w").write("\n".join(s))
import xml.dom.minidom as m; m.parse(out); print("ok", out)
