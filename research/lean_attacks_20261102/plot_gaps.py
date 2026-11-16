#!/usr/bin/env python3
"""Draw results/gaps.svg from results/p3gap.txt and results/family.txt (standard library).

Left: histogram of d_F - d'_F over all 521,731 pairs of degree <= 2 in 3 variables, with the
field equations outside and inside. Right: the family W_k = {X^k Y, XY + X}: d'_F stays 2
while d_F (outside) = k + 1 grows; inside it stays 3.
"""
import ast
import html
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
gap_text = (HERE / "results" / "p3gap.txt").read_text()
hist_out = ast.literal_eval(re.search(r"outside: histogram of d_F - d'_F \(both finite\) (\{[^}]*\})", gap_text)[1])
hist_in = ast.literal_eval(re.search(r"inside:  histogram of d_F - d'_F \(both finite\) (\{[^}]*\})", gap_text)[1])
fam = [tuple(map(int, m)) for m in re.findall(
    r"FAMILY W_(\d+) .*?d_F\(outside\)=(\d+) d_F\(inside\)=(\d+) d'_F=(\d+)", (HERE / "results" / "family.txt").read_text())]

W, H = 940, 430
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="DejaVu Sans, sans-serif">',
       f'<rect width="{W}" height="{H}" fill="white"/>',
       f'<text x="{W/2}" y="26" text-anchor="middle" font-size="17">How far the true first fall degree d_F can sit above the fake one d\'_F (F_2)</text>']

# Left panel: grouped bars, log-free (counts shown), x = gap.
lx, ly, lw, lh = 60, 70, 400, 280
gaps = sorted(set(hist_out) | set(hist_in))
vmax = max(list(hist_out.values()) + list(hist_in.values()))
out.append(f'<text x="{lx + lw/2}" y="{ly - 18}" text-anchor="middle" font-size="13">All pairs, degree ≤ 2, 3 variables (521,731)</text>')
out.append(f'<line x1="{lx}" y1="{ly+lh}" x2="{lx+lw}" y2="{ly+lh}" stroke="#526575"/>')
slot = lw / len(gaps)
for i, g in enumerate(gaps):
    x0 = lx + i * slot
    for j, (hist, color) in enumerate(((hist_out, "#c0504d"), (hist_in, "#2468a2"))):
        v = hist.get(g, 0)
        h = lh * v / vmax
        bx = x0 + slot * 0.15 + j * slot * 0.35
        out.append(f'<rect x="{bx:.1f}" y="{ly+lh-h:.1f}" width="{slot*0.33:.1f}" height="{h:.1f}" fill="{color}"/>')
        out.append(f'<text x="{bx + slot*0.165:.1f}" y="{ly+lh-h-4:.1f}" text-anchor="middle" font-size="10">{v:,}</text>')
    out.append(f'<text x="{x0 + slot/2:.1f}" y="{ly+lh+16}" text-anchor="middle" font-size="12">{g}</text>')
out.append(f'<text x="{lx + lw/2}" y="{ly+lh+36}" text-anchor="middle" font-size="12">d_F − d\'_F</text>')
for j, (name, color) in enumerate((("field equations outside", "#c0504d"), ("field equations inside", "#2468a2"))):
    out.append(f'<rect x="{lx + 10 + j*200}" y="{ly+lh+48}" width="12" height="12" fill="{color}"/>')
    out.append(f'<text x="{lx + 26 + j*200}" y="{ly+lh+58}" font-size="11">{html.escape(name)}</text>')

# Right panel: family lines.
rx, ry, rw, rh = 560, 70, 340, 280
ks = [k for k, *_ in fam]
dmax = max(max(o, i, f) for _, o, i, f in fam) + 1
def px(k): return rx + (k - ks[0]) / (ks[-1] - ks[0]) * rw
def py(v): return ry + rh - v / dmax * rh
out.append(f'<text x="{rx + rw/2}" y="{ry - 18}" text-anchor="middle" font-size="13">Family W_k = {{X^k Y, XY + X}}</text>')
out.append(f'<line x1="{rx}" y1="{ry+rh}" x2="{rx+rw}" y2="{ry+rh}" stroke="#526575"/>')
out.append(f'<line x1="{rx}" y1="{ry}" x2="{rx}" y2="{ry+rh}" stroke="#526575"/>')
for v in range(0, dmax + 1, 2):
    out.append(f'<text x="{rx-8}" y="{py(v)+4:.1f}" text-anchor="end" font-size="11">{v}</text>')
for k in ks:
    out.append(f'<text x="{px(k):.1f}" y="{ry+rh+16}" text-anchor="middle" font-size="12">{k}</text>')
out.append(f'<text x="{rx + rw/2}" y="{ry+rh+36}" text-anchor="middle" font-size="12">k</text>')
for idx, (name, color, dash) in enumerate((("d_F outside = k + 1", "#c0504d", ""), ("d_F inside = 3", "#2468a2", ""), ("d'_F = 2", "#257942", "6,4"))):
    pts = " ".join(f"{px(k):.1f},{py(row[idx+1] if idx < 2 else row[3]):.1f}" for k, *rest in fam for row in [(k, *rest)])
    out.append(f'<polyline points="{pts}" fill="none" stroke="{color}" stroke-width="2.5" stroke-dasharray="{dash}"/>')
    for k, *rest in fam:
        v = (rest[0], rest[1], rest[2])[idx]
        out.append(f'<circle cx="{px(k):.1f}" cy="{py(v):.1f}" r="3.5" fill="{color}"/>')
    out.append(f'<rect x="{rx + 10 + (idx % 2)*170}" y="{ry+rh+48 + (idx//2)*16}" width="12" height="12" fill="{color}"/>')
    out.append(f'<text x="{rx + 26 + (idx % 2)*170}" y="{ry+rh+58 + (idx//2)*16}" font-size="11">{html.escape(name)}</text>')
out.append("</svg>")
(HERE / "results" / "gaps.svg").write_text("\n".join(out) + "\n")
print("wrote results/gaps.svg", hist_out, hist_in, fam)
