#!/usr/bin/env python3
"""Emit verification-map.svg: the dependency/standing map of the Kaarelson v2
results, as stated by the paper (what is Lean, what is paper proof, what is
conditional on the unrefereed engine). Layout only; no computation.
See REPORT.md in this directory.
"""
import html

W, H = 920, 560
COL = {
    "lean":  ("#e3f3e8", "#2a9d55"),   # machine-checked Lean, no hypotheses
    "uncond":("#e3eefa", "#4b8fd1"),   # unconditional math (paper + Lean skeleton)
    "core":  ("#fbf1d8", "#caa43a"),   # Lean core + paper analytic inputs
    "cond":  ("#fae1e6", "#d1495b"),   # conditional on unrefereed engine (E),(N)
    "engine":("#ededed", "#8a8a8a"),   # unrefereed engine preprint
}

# box: (id, x, y, w, h, kind, title, subtitle, dashed)
boxes = [
    ("E",   30, 300, 220, 64, "engine", "Anthropic engine (E) [22]",
     "unrefereed preprint; quoted, not rebuilt", True),
    ("RR",  30, 196, 220, 64, "engine", "Rudich-Rousseau Lean engine [24]",
     "unrefereed preprint; imported in Lean", False),
    ("KW",  30, 92, 220, 64, "uncond", "Kummer / Weil lemmas",
     "N', F, 5.4, 5.5, Prop 5.6 (paper)", False),
    ("PI",  30, 420, 220, 76, "core", "Paper analytic inputs",
     "Schoof, factoring, sparse LA,\nLemma 3.8/3.9 (AI review + numerics)", False),

    ("T2ii",350, 196, 210, 64, "lean", "Thm 2(ii) subexponentialRate_all",
     "Lean, no hypotheses; 1 disclosed sorry", False),
    ("T2i", 350, 96, 210, 56, "lean", "Thm 2(i) countingRate_all",
     "Lean, no hypotheses", False),
    ("T3",  350, 300, 210, 64, "uncond", "Thm 3  coverage",
     "unconditional: no engine, no heuristic", False),

    ("T1",  660, 150, 230, 84, "core", "Thm 1  (main)",
     "every curve, p^{2/m+o(1)};\nm>=5 lower exponent than rho.\nLean core + paper inputs", False),
    ("T7",  660, 330, 230, 72, "cond", "Thm 7  (growing m)",
     "p^{o(1)}, conditional on (E) and (N);\nonly result using the engine", False),
]

# edges: (from, to)
edges = [
    ("RR", "T2ii"), ("T2ii", "T1"), ("KW", "T3"), ("T3", "T1"),
    ("PI", "T1"), ("E", "T7"), ("T2ii", "T7"),
]

bi = {b[0]: b for b in boxes}

def anchor(bid, side):
    _, x, y, w, h, *_ = bi[bid]
    if side == "r": return (x + w, y + h / 2)
    if side == "l": return (x, y + h / 2)
    return (x + w / 2, y + h / 2)

def esc(s): return html.escape(s)

parts = [
    f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" '
    f'viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif">',
    '<defs><marker id="arr" markerWidth="9" markerHeight="9" refX="7" refY="3" '
    'orient="auto"><path d="M0,0 L7,3 L0,6 Z" fill="#555"/></marker></defs>',
    f'<rect x="0" y="0" width="{W}" height="{H}" fill="white"/>',
    f'<text x="{W/2}" y="30" text-anchor="middle" font-size="16" font-weight="bold">'
    'Kaarelson v2: result dependencies and verification standing</text>',
    f'<text x="{W/2}" y="50" text-anchor="middle" font-size="11" fill="#444">'
    'as stated by the paper; left = foundations, right = discrete-log results</text>',
    f'<text x="145" y="78" text-anchor="middle" font-size="11" font-weight="bold" fill="#666">FOUNDATIONS</text>',
    f'<text x="455" y="78" text-anchor="middle" font-size="11" font-weight="bold" fill="#666">MACHINE-CHECKED SEARCH / COVERAGE</text>',
    f'<text x="775" y="128" text-anchor="middle" font-size="11" font-weight="bold" fill="#666">DISCRETE-LOG RESULTS</text>',
]

for f, t in edges:
    x1, y1 = anchor(f, "r")
    x2, y2 = anchor(t, "l")
    mx = (x1 + x2) / 2
    parts.append(
        f'<path d="M{x1:.0f},{y1:.0f} C{mx:.0f},{y1:.0f} {mx:.0f},{y2:.0f} '
        f'{x2-8:.0f},{y2:.0f}" fill="none" stroke="#555" stroke-width="1.5" '
        f'marker-end="url(#arr)"/>')

for bid, x, y, w, h, kind, title, sub, dashed in boxes:
    fill, stroke = COL[kind]
    dash = ' stroke-dasharray="5,3"' if dashed else ''
    parts.append(
        f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="7" fill="{fill}" '
        f'stroke="{stroke}" stroke-width="2"{dash}/>')
    parts.append(
        f'<text x="{x+10}" y="{y+19}" font-size="11.5" font-weight="bold" '
        f'fill="#222">{esc(title)}</text>')
    for i, line in enumerate(sub.split("\n")):
        parts.append(
            f'<text x="{x+10}" y="{y+35+i*13}" font-size="9.5" fill="#444">'
            f'{esc(line)}</text>')

# galactic banner
parts.append('<rect x="30" y="512" width="860" height="34" rx="6" '
             'fill="#fff6e8" stroke="#caa43a" stroke-width="1.5"/>')
parts.append('<text x="42" y="533" font-size="10.5" fill="#6b4e00">'
             'Galactic: the proved crossover sits at log2 log2 p from ~4.4e7 (lowest, under (R1)) '
             'to 6.756e16 (edge network, Lean). A real 256-bit key is at log2 L = 8.</text>')

# legend
lx, ly = 600, 30
legend = [("lean", "Lean, no hypotheses"), ("uncond", "unconditional math"),
          ("core", "Lean core + paper inputs"), ("cond", "conditional on engine (E),(N)"),
          ("engine", "unrefereed engine preprint")]
for i, (k, lab) in enumerate(legend):
    fill, stroke = COL[k]
    yy = ly + i * 16
    parts.append(f'<rect x="{lx}" y="{yy}" width="12" height="11" rx="2" fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>')
    parts.append(f'<text x="{lx+17}" y="{yy+10}" font-size="9" fill="#333">{esc(lab)}</text>')

parts.append('</svg>')
open("verification-map.svg", "w").write("\n".join(parts))
print("wrote verification-map.svg")
