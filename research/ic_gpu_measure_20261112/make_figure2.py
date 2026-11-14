#!/usr/bin/env python3
"""fig_round2.svg from results2.json (analyze2.py output), round 2: main c5678a98ae -> this branch. Speedup = old median / new median,
both measured on the same A100 pod; the arms named in each label. Log x axis, one series.
    make_figure.py results.json fig_speedups.svg
"""
import json, math, sys

r = json.load(open(sys.argv[1]))

def sp(row, arm):
    return 1.0 / row[arm]["ratio"]

rows = [
    ("FES, n=32: Gray walk → block walk (K=6)", sp(r["fes"]["fes32"], "B6")),
    ("FES, n=30: Gray walk → block walk (K=6)", sp(r["fes"]["fes30"], "B6")),
    ("FES, n=26: Gray walk → block walk (K=6)", sp(r["fes"]["fes26"], "B6")),
    ("FES, n=24 m=12: Gray walk → block walk", sp(r["fes"]["fes24m12"], "B6")),
    ("macaulay, batch 8192: cursor → 2D update", sp(r["mac"]["batch 8192"], "D1024")),
    ("rootfind best knob vs default (hash/C64)", sp(r["rf-tp"]["hash/C64"], "R64_0")),
]
W, left, right, top, bar, gap = 760, 300, 70, 56, 22, 14
H = top + len(rows) * (bar + gap) + 44
xmax = 8.0
def x(v):
    return left + (W - left - right) * math.log10(v) / math.log10(xmax)

out = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" '
       'font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" '
       'aria-labelledby="t d">',
       '<title id="t">IC GPU kernel speedups, round 2 (A100)</title>',
       '<desc id="d">Speedup (old median kernel time over new), log scale; each row names the two arms. '
       'Medians of 5 interleaved runs; A/A spread at most 0.7 percent.</desc>',
       '<style>.s{fill:#fcfcfb}.b{fill:#2a78d6}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
       '@media (prefers-color-scheme: dark){.s{fill:#1a1a19}.b{fill:#3987e5}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
       f'<rect class="s" width="{W}" height="{H}"/>',
       f'<text class="p" x="16" y="24" font-size="15" font-weight="600">Round 2: speedups over main c5678a98ae (A100 80GB PCIe)</text>',
       f'<text class="q" x="16" y="42" font-size="12">old median ÷ new median kernel time, log scale; identical outputs in every arm</text>']
for t in (1, 2, 4, 8):
    gx = x(t)
    out.append(f'<line class="g" x1="{gx:.1f}" x2="{gx:.1f}" y1="{top - 4}" y2="{H - 34}" stroke-width="1"/>')
    out.append(f'<text class="q" x="{gx:.1f}" y="{H - 18}" font-size="11" text-anchor="middle">{t}×</text>')
for i, (label, v) in enumerate(rows):
    y = top + i * (bar + gap)
    w = max(x(v) - left, 2)
    out.append(f'<text class="p" x="{left - 10}" y="{y + bar * 0.7:.1f}" font-size="12" text-anchor="end">{label}</text>')
    out.append(f'<g><title>{label}: {v:.2f}× faster</title>'
               f'<path class="b" d="M{left},{y} h{w - 4:.1f} a4,4 0 0 1 4,4 v{bar - 8} a4,4 0 0 1 -4,4 h{-(w - 4):.1f} z"/></g>')
    out.append(f'<text class="p" x="{left + w + 6:.1f}" y="{y + bar * 0.7:.1f}" font-size="12" font-weight="600">{v:.2f}×</text>')
out.append('</svg>')
open(sys.argv[2], "w").write("\n".join(out) + "\n")
print("\n".join(f"{v:7.2f}x  {l}" for l, v in rows))
