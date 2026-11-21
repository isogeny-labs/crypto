#!/usr/bin/env python3
"""fig_la.svg from results.json: per curve and base size, linear-algebra row_ops of incremental-gauss,
incremental-gauss-echelon and structured-gauss (log scale), with the ratio to incremental-gauss.
    make_figure.py results.json fig_la.svg
"""
import json, math, sys
R = json.load(open(sys.argv[1]))["rows"]
order = sorted({(r["log2_r"], r["fb"]) for r in R})
SOL = [("gauss", "c1", "incremental-gauss (the default)"), ("structured", "c2", "structured-gauss"), ("echelon", "c3", "incremental-gauss-echelon")]
W, left, right, top, bar, gap, ggap = 820, 200, 150, 92, 11, 2, 14
H = top + len(order) * (3 * bar + 2 * gap + ggap) + 40
vals = [r["la_row_ops_median"] for r in R]
lo, hi = 10 ** math.floor(math.log10(min(vals))), 10 ** math.ceil(math.log10(max(vals)))
X = lambda v: left + (W - left - right) * (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo))
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">Linear-algebra row operations of three solvers in m = 3 rootfind-subtract solves</title>',
     '<desc id="d">Median linear-algebra row_ops per run for incremental-gauss, structured-gauss and incremental-gauss-echelon, per curve and base size, log scale; all three give the same trials, relations, logarithm and S.</desc>',
     '<style>.bg{fill:#fcfcfb}.c1{fill:#8f8e89}.c2{fill:#86b6ee}.c3{fill:#2a78d6}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.c1{fill:#8f8e89}.c2{fill:#1f5ea8}.c3{fill:#3987e5}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">Linear algebra in m = 3 rootfind-subtract solves: row operations per solver</text>',
     '<text class="q" x="16" y="42" font-size="12">median row_ops per run (deterministic counts), log scale; the three solvers give identical trials, relation counts, logarithm and S</text>']
for i, (_, c, l) in enumerate(SOL):
    o.append(f'<rect class="{c}" x="{16 + 230 * i}" y="54" width="12" height="12" rx="2"/><text class="p" x="{34 + 230 * i}" y="64" font-size="12">{l}</text>')
t = lo
while t <= hi * 1.0001:
    o.append(f'<line class="g" x1="{X(t):.1f}" x2="{X(t):.1f}" y1="{top - 6}" y2="{H - 30}" stroke-width="1"/>')
    o.append(f'<text class="q" x="{X(t):.1f}" y="{H - 14}" font-size="11" text-anchor="middle">{t:.0e}</text>')
    t *= 10
y = top
for (lr, fb) in order:
    o.append(f'<text class="p" x="{left - 10}" y="{y + 1.5 * bar + 4:.1f}" font-size="12" text-anchor="end">log2 r = {lr:.1f}, |F_x| = {fb}</text>')
    for k, (s, cls, _) in enumerate(SOL):
        row = next(r for r in R if r["log2_r"] == lr and r["fb"] == fb and r["solver"] == s)
        v = row["la_row_ops_median"]; yy = y + k * (bar + gap)
        o.append(f'<g><title>{s}: {v:,.0f} row_ops</title><rect class="{cls}" x="{left}" y="{yy}" width="{max(X(v) - left, 2):.1f}" height="{bar}" rx="2"/></g>')
        lab = f'{v / 1e6:.1f}M' + ("" if s == "gauss" else f' ({row["ops_vs_gauss"]:.2f}×)')
        o.append(f'<text class="{"q" if s == "gauss" else "p"}" x="{X(v) + 6:.1f}" y="{yy + bar * 0.85:.1f}" font-size="11">{lab}</text>')
    y += 3 * bar + 2 * gap + ggap
o.append('</svg>')
open(sys.argv[2], "w").write("\n".join(o) + "\n")
