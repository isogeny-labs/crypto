#!/usr/bin/env python3
"""Draw results/inside_gap.svg from the results files (standard library only).

Left: largest inside gap d_F(inside) - d'_F per population against the proved bound 3
(n >= 2). Right: the one-variable family {X^k, X^k + 1}, whose gap k - 1 is unbounded.
"""
import ast
import html
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
R = HERE / "results"
pops = []
for name, label in (("series", None), ("q1", "Q1 n=4 multilinear deg≤2 (all)"), ("q2", "Q2 n=4 deg≤2 (2M sample)"), ("q3", "Q3 n=4 multilinear deg≤3 (2M sample)")):
    for m in re.finditer(r"INSIDE-GAP (.*?): systems=(\d+) .*?histogram d_F\(inside\)-d'_F (\{[^}]*\})", (R / f"{name}.txt").read_text()):
        hist = ast.literal_eval(m[3])
        lab = label or {"SERIES all deg<=2 n=2": "n=2 deg≤2 (all)", "SERIES all deg<=2 n=3": "n=3 deg≤2 (all)"}[m[1]]
        pops.append((lab, int(m[2]), max(hist), hist.get(max(hist), 0)))
n1 = [(int(k), int(d)) for k, d in re.findall(r"N1-FAMILY \{X\^(\d+), X\^\d+\+1\}: d_F\(inside\)=(\d+) d'_F=1", (R / "family_rust.txt").read_text())]

W, H = 960, 440
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="DejaVu Sans, sans-serif">',
     f'<rect width="{W}" height="{H}" fill="white"/>',
     f'<text x="{W/2}" y="26" text-anchor="middle" font-size="17">Inside gap d_F − d\'_F (field equations inside, F_2): bounded by 3 for n ≥ 2</text>']
lx, ly, lw, lh = 250, 70, 300, 260
o.append(f'<text x="{lx + lw/2}" y="{ly - 18}" text-anchor="middle" font-size="13">Largest gap found, per population</text>')
bh = lh / len(pops)
xmax = 4
for i, (lab, n, g, cnt) in enumerate(pops):
    y = ly + i * bh + 6
    w = lw * g / xmax
    o.append(f'<text x="{lx - 8}" y="{y + bh/2}" text-anchor="end" font-size="11">{html.escape(lab)}</text>')
    o.append(f'<rect x="{lx}" y="{y}" width="{w:.1f}" height="{bh - 14:.1f}" fill="#2468a2"/>')
    o.append(f'<text x="{lx + w + 6:.1f}" y="{y + bh/2}" font-size="11">{g}  ({cnt:,} of {n:,})</text>')
bx = lx + lw * 3 / xmax
o.append(f'<line x1="{bx:.1f}" y1="{ly - 4}" x2="{bx:.1f}" y2="{ly + lh}" stroke="#a33a3a" stroke-width="2" stroke-dasharray="6,4"/>')
o.append(f'<text x="{bx + 4:.1f}" y="{ly + lh + 14}" font-size="11" fill="#a33a3a">proved bound 3 (n ≥ 2)</text>')
for k in range(0, xmax + 1):
    o.append(f'<text x="{lx + lw*k/xmax:.1f}" y="{ly + lh + 30}" text-anchor="middle" font-size="11" fill="#445566">{k}</text>')
rx, ry, rw, rh = 640, 70, 280, 260
o.append(f'<text x="{rx + rw/2}" y="{ry - 18}" text-anchor="middle" font-size="13">n = 1: {{X^k, X^k + 1}}</text>')
kmax = max(k for k, _ in n1)
def px(k): return rx + (k - 2) / (kmax - 2) * rw
def py(v): return ry + rh - v / (kmax + 1) * rh
o.append(f'<line x1="{rx}" y1="{ry + rh}" x2="{rx + rw}" y2="{ry + rh}" stroke="#526575"/>')
o.append(f'<line x1="{rx}" y1="{ry}" x2="{rx}" y2="{ry + rh}" stroke="#526575"/>')
pts = " ".join(f"{px(k):.1f},{py(d - 1):.1f}" for k, d in n1)
o.append(f'<polyline points="{pts}" fill="none" stroke="#c0504d" stroke-width="2.5"/>')
for k, d in n1:
    o.append(f'<circle cx="{px(k):.1f}" cy="{py(d - 1):.1f}" r="3.5" fill="#c0504d"/>')
    o.append(f'<text x="{px(k):.1f}" y="{ry + rh + 16}" text-anchor="middle" font-size="11">{k}</text>')
for v in range(0, kmax + 1, 2):
    o.append(f'<text x="{rx - 8}" y="{py(v) + 4:.1f}" text-anchor="end" font-size="11">{v}</text>')
o.append(f'<text x="{rx + rw/2}" y="{ry + rh + 34}" text-anchor="middle" font-size="12">k   (gap = k − 1)</text>')
o.append(f'<text x="{W/2}" y="{H - 14}" text-anchor="middle" font-size="11" fill="#445566">For n ≥ 2: Y²(X²+X) + X²(Y²+Y) = XY² + X²Y is a fall at 4 for every system, so d_F(inside) ≤ 4 and gap ≤ 3.</text>')
o.append("</svg>")
(R / "inside_gap.svg").write_text("\n".join(o) + "\n")
print("wrote results/inside_gap.svg", pops, n1)
