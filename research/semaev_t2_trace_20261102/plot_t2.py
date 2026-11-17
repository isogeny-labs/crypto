#!/usr/bin/env python3
"""Draw results/t2_degrees.svg from results/t2_trace.txt (standard library only).

Per cell and field polynomial: first fall degree d_ff (2 on every instance), the share of
instances whose Macaulay degree of 1 is 2 or 3, and Semaev's reported d_F4 = 4 (Table 2).
"""
import ast
import html
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
text = (HERE / "results" / "t2_trace.txt").read_text()
rows = []
for m in re.finditer(r"CELL n=(\d+) m=(\d+) t=2 k=(\d+) field (\S+) .*?d_ff=2 in (\d+).*?Macaulay degree of 1 \(unsolvable\) (\{[^}]*\})", text):
    rows.append((int(m[1]), int(m[2]), m[4], int(m[5]), ast.literal_eval(m[6])))

W, H = 960, 140 + 34 * len(rows)
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="DejaVu Sans, sans-serif">',
     f'<rect width="{W}" height="{H}" fill="white"/>',
     f'<text x="{W/2}" y="26" text-anchor="middle" font-size="16">Semaev 2015, t = 2: first fall degree 2; degree needed to prove no solution 2 or 3 (reported d_F4 = 4)</text>']
lx, top, bar_w = 360, 70, 520
o.append(f'<text x="{lx}" y="{top - 12}" font-size="12" fill="#445566">share of 20 instances by Macaulay degree of 1 (exact, each with a recomputed certificate)</text>')
for i, (n, m, field, ff2, D) in enumerate(rows):
    y = top + i * 34
    total = sum(D.values())
    label = f"n={n} m={m} ({'sparse' if not field.startswith('random') else 'random'} f): d_ff = 2 in {ff2}/20"
    o.append(f'<text x="{lx - 10}" y="{y + 15}" text-anchor="end" font-size="11">{html.escape(label)}</text>')
    x = lx
    for deg, color in ((2, "#257942"), (3, "#d9a441"), (4, "#c0504d")):
        c = D.get(deg, 0)
        w = bar_w * c / total if total else 0
        if w:
            o.append(f'<rect x="{x:.1f}" y="{y}" width="{w:.1f}" height="22" fill="{color}"/>')
            if w > 24:
                o.append(f'<text x="{x + w/2:.1f}" y="{y + 15}" text-anchor="middle" font-size="11" fill="white">{c}</text>')
        x += w
legend_y = top + len(rows) * 34 + 12
for j, (name, color) in enumerate((("needs degree 2 (= d_ff)", "#257942"), ("needs degree 3 (> d_ff = 2)", "#d9a441"), ("needs degree 4 (Semaev's reported d_F4): none", "#c0504d"))):
    lxj, lyj = 40 + (j % 2) * 440, legend_y + (j // 2) * 18
    o.append(f'<rect x="{lxj}" y="{lyj}" width="12" height="12" fill="{color}"/>')
    o.append(f'<text x="{lxj + 16}" y="{lyj + 10}" font-size="11">{html.escape(name)}</text>')
o.append("</svg>")
(HERE / "results" / "t2_degrees.svg").write_text("\n".join(o) + "\n")
print("wrote results/t2_degrees.svg", [(r[0], r[1], r[2][:6], r[3], r[4]) for r in rows])
