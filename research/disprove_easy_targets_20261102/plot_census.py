#!/usr/bin/env python3
"""Draw results/census.svg from results/p1.txt, p2.txt, p3.txt (standard library only).

Log-scale bars per population: pairs examined, reverse counterexamples with the field
equations outside (all degenerate), those with linearly independent reduced generators
(zero everywhere), and, for reference, inside-placement pairs with d_F < d'_F.
"""
import html
import math
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
PAT = re.compile(
    r"CENSUS (?P<label>P\d[^:]*): pairs=(?P<pairs>\d+) .*?d_F<d'_F \(both finite\) out=\d+ in=(?P<lt_in>\d+) "
    r"\| reverse counterexamples OUT: finite=(?P<rf>\d+) no-fake-fall=(?P<rn>\d+) "
    r"\(both generators multilinear: \d+; reduced generators independent: (?P<nd>\d+)\)"
)

rows = []
for name in ("p1", "p2", "p3"):
    text = (HERE / "results" / f"{name}.txt").read_text()
    m = PAT.search(text)
    assert m, f"no CENSUS line in {name}.txt"
    rows.append({
        "label": m["label"],
        "pairs": int(m["pairs"]),
        "out": int(m["rf"]) + int(m["rn"]),
        "nondeg": int(m["nd"]),
        "inside": int(m["lt_in"]),
    })

series = [
    ("pairs examined", "pairs", "#8a99a8"),
    ("reverse counterexamples, outside (all degenerate)", "out", "#c0504d"),
    ("…with independent reduced generators", "nondeg", "#257942"),
    ("inside placement, d_F < d'_F (reference)", "inside", "#d9a441"),
]

W, left, right, top_pad, bar_h, gap, group_gap = 880, 250, 140, 70, 16, 3, 22
xmax = math.log10(max(r["pairs"] for r in rows)) + 0.3
scale = (W - left - right) / xmax
group_h = len(series) * (bar_h + gap) + group_gap
H = top_pad + len(rows) * group_h + 84


def x_of(v):
    return left + (math.log10(v) * scale if v >= 1 else 0)


out = [
    f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" '
    'font-family="DejaVu Sans, sans-serif">',
    f'<rect width="{W}" height="{H}" fill="white"/>',
    f'<text x="{W/2}" y="28" text-anchor="middle" font-size="17">'
    "Reverse repair d'_F ≤ d_F (field equations outside): exact census over F_2</text>",
    f'<text x="{W/2}" y="48" text-anchor="middle" font-size="12" fill="#445566">'
    "unordered pairs of generators, degree cap 6; log scale; 0 drawn as a marker</text>",
]
for k in range(0, int(xmax) + 1):
    x = left + k * scale
    out.append(f'<line x1="{x:.1f}" y1="{top_pad-8}" x2="{x:.1f}" y2="{H-70}" stroke="#e3e8ee"/>')
    out.append(f'<text x="{x:.1f}" y="{H-56}" text-anchor="middle" font-size="11" fill="#445566">10^{k}</text>')
y = top_pad
for r in rows:
    out.append(f'<text x="{left-10}" y="{y+2*(bar_h+gap)+4}" text-anchor="end" font-size="12">{html.escape(r["label"])}</text>')
    for _, key, color in series:
        v = r[key]
        if v >= 1:
            out.append(f'<rect x="{left}" y="{y}" width="{x_of(v)-left:.1f}" height="{bar_h}" fill="{color}"/>')
            out.append(f'<text x="{x_of(v)+5:.1f}" y="{y+bar_h-4}" font-size="11">{v:,}</text>')
        else:
            out.append(f'<rect x="{left}" y="{y}" width="3" height="{bar_h}" fill="{color}"/>')
            out.append(f'<text x="{left+8}" y="{y+bar_h-4}" font-size="11">0</text>')
        y += bar_h + gap
    y += group_gap
for idx, (name, _, color) in enumerate(series):
    lx = 40 + (idx % 2) * 420
    ly = H - 34 + (idx // 2) * 18
    out.append(f'<rect x="{lx}" y="{ly}" width="12" height="12" fill="{color}"/>')
    out.append(f'<text x="{lx+16}" y="{ly+10}" font-size="11">{html.escape(name)}</text>')
out.append("</svg>")
(HERE / "results" / "census.svg").write_text("\n".join(out) + "\n")
print("wrote results/census.svg", [(r["label"], r["pairs"], r["out"], r["nondeg"], r["inside"]) for r in rows])
