#!/usr/bin/env python3
"""fig_e2e_wall.svg from results.json (analyze_e2e.py): whole-solve wall time (row wall, summed over
repeats) per instance and solver, log scale. Wall time is a practicality note (AGENTS.md section 6);
the counted S of each row is printed beside it.
    make_figure.py results.json fig_e2e_wall.svg
"""
import json, math, sys

r = json.load(open(sys.argv[1]))
inst = [("koblitz-n23", "Koblitz n=23 · 24 unknowns"), ("char2-n25", "char-2 n=25 · 26 unknowns"),
        ("char2-n29", "char-2 n=29 · 30 unknowns")]
series = [("inherited-f4", "s1", "inherited-f4 (CPU)"), ("fes-f2", "s2", "fes-f2 (CPU)"), ("fes-f2-gpu", "s3", "fes-f2-gpu (A100)")]
W, left, right, top, bar, gap, ggap = 820, 210, 210, 84, 18, 4, 22
H = top + len(inst) * (3 * bar + 2 * gap + ggap) + 40
lo, hi = 1.0, 2000.0
x = lambda v: left + (W - left - right) * (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo))
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">End-to-end IC solve wall time by solver</title>',
     '<desc id="d">Whole-solve wall seconds (log scale) per instance for inherited-f4, fes-f2 and fes-f2-gpu; counted S over matched rho beside each bar. fes-f2 was not run at n=29.</desc>',
     '<style>.bg{fill:#fcfcfb}.s1{fill:#2a78d6}.s2{fill:#eb6834}.s3{fill:#1baf7a}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.s1{fill:#3987e5}.s2{fill:#d95926}.s3{fill:#199e70}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">Index calculus end to end: whole-solve wall time by descent solver</text>',
     '<text class="q" x="16" y="42" font-size="12">seconds, log scale, summed over repeats; every row recovered the planted log. Right column: counted S / matched rho (the metric)</text>']
lx = 16
for key, cls, name in series:
    o.append(f'<rect class="{cls}" x="{lx}" y="54" width="12" height="12" rx="2"/><text class="p" x="{lx + 18}" y="64" font-size="12">{name}</text>')
    lx += 30 + 7 * len(name)
for t in (1, 10, 100, 1000):
    o.append(f'<line class="g" x1="{x(t):.1f}" x2="{x(t):.1f}" y1="{top - 6}" y2="{H - 34}" stroke-width="1"/>')
    o.append(f'<text class="q" x="{x(t):.1f}" y="{H - 18}" font-size="11" text-anchor="middle">{t} s</text>')
o.append(f'<text class="q" x="{W - right + 70}" y="{top - 10}" font-size="11">S / rho (counted)</text>')
y = top
for key, label in inst:
    rows = {row["solver"]: row for row in r[key]["rows"]}
    o.append(f'<text class="p" x="{left - 10}" y="{y + 1.5 * bar + gap + 4:.1f}" font-size="12" font-weight="600" text-anchor="end">{label}</text>')
    for k, (sk, cls, name) in enumerate(series):
        yy = y + k * (bar + gap)
        row = rows.get(sk)
        if not row:
            o.append(f'<text class="q" x="{left + 6}" y="{yy + bar * 0.72:.1f}" font-size="11">{name}: not run (≈0.7 s × 9 000 calls)</text>')
            continue
        v = sum(w or 0 for w in row["row_wall_s"])
        so = sum(row["s_over_rho"]) / len(row["s_over_rho"])
        w = max(x(max(v, lo)) - left, 3)
        o.append(f'<g><title>{label}, {sk}: {v:.1f} s wall, S/rho {so:,.0f}, {sum(row["solver_calls"])} solver calls</title>'
                 f'<path class="{cls}" d="M{left},{yy} h{w - 4:.1f} a4,4 0 0 1 4,4 v{bar - 8} a4,4 0 0 1 -4,4 h{-(w - 4):.1f} z"/></g>')
        o.append(f'<text class="p" x="{left + w + 6:.1f}" y="{yy + bar * 0.72:.1f}" font-size="11">{v:.1f} s</text>')
        o.append(f'<text class="q" x="{W - right + 70}" y="{yy + bar * 0.72:.1f}" font-size="11">{so:,.0f}</text>')
    y += 3 * bar + 2 * gap + ggap
o.append('</svg>')
open(sys.argv[2], "w").write("\n".join(o) + "\n")
