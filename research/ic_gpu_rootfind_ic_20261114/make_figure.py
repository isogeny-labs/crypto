#!/usr/bin/env python3
"""fig_ic.svg from results.json: per curve and base size, median solve and relation-phase wall of the CPU
and GPU rootfind-subtract arms (log scale), with the shared counted S against rho.
    make_figure.py results.json fig_ic.svg
"""
import json, math, sys
r = json.load(open(sys.argv[1]))["ic"]["rows"]
W, left, right, top, bar, gap, ggap = 820, 210, 120, 92, 11, 2, 16
H = top + len(r) * (4 * bar + 3 * gap + ggap) + 40
lo = 10 ** math.floor(math.log10(min(min(x["gpu_relations_s_median"], x["gpu_solve_s_median"]) for x in r)))
hi = 10 ** math.ceil(math.log10(max(x["cpu_solve_s_median"] for x in r)))
X = lambda v: left + (W - left - right) * (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo))
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">rootfind-subtract in a complete index-calculus solve: CPU against GPU wall time</title>',
     '<desc id="d">Median solve and relation-phase wall time of the CPU and GPU arms per curve and base size, log scale; both arms report the same counted S.</desc>',
     '<style>.bg{fill:#fcfcfb}.c1{fill:#8f8e89}.c2{fill:#2a78d6}.c3{fill:#c3c2b7}.c4{fill:#86b6ee}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.c1{fill:#8f8e89}.c2{fill:#3987e5}.c3{fill:#52514e}.c4{fill:#1f5ea8}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">rootfind-subtract (m = 2) in a complete IC solve: CPU arm against GPU arm</text>',
     '<text class="q" x="16" y="42" font-size="12">median wall time, log scale; A100 + one EPYC 9454 core; the two arms report identical relations, logarithm and counted S</text>']
leg = [("c1", "CPU solve"), ("c2", "GPU solve"), ("c3", "CPU relation phase"), ("c4", "GPU relation phase")]
for i, (c, l) in enumerate(leg):
    o.append(f'<rect class="{c}" x="{16 + 170 * i}" y="54" width="12" height="12" rx="2"/><text class="p" x="{34 + 170 * i}" y="64" font-size="12">{l}</text>')
t = lo
while t <= hi * 1.0001:
    o.append(f'<line class="g" x1="{X(t):.1f}" x2="{X(t):.1f}" y1="{top - 6}" y2="{H - 30}" stroke-width="1"/>')
    o.append(f'<text class="q" x="{X(t):.1f}" y="{H - 14}" font-size="11" text-anchor="middle">{t:g} s</text>')
    t *= 10
y = top
for x in r:
    o.append(f'<text class="p" x="{left - 10}" y="{y + 2 * bar:.1f}" font-size="12" text-anchor="end">log2 r = {x["log2_r"]:.1f}, |F_x| = {x["fb_abscissae"]}</text>')
    o.append(f'<text class="q" x="{left - 10}" y="{y + 2 * bar + 15:.1f}" font-size="11" text-anchor="end">S = {x["mean_S"]:.0f} ({x["S_over_rho"]:.0f}× rho)</text>')
    for k, (cls, key) in enumerate([("c1", "cpu_solve_s_median"), ("c2", "gpu_solve_s_median"),
                                    ("c3", "cpu_relations_s_median"), ("c4", "gpu_relations_s_median")]):
        v = x[key]; yy = y + k * (bar + gap)
        o.append(f'<g><title>{key}: {v:.3f} s</title><rect class="{cls}" x="{left}" y="{yy}" width="{max(X(v) - left, 2):.1f}" height="{bar}" rx="2"/></g>')
        if k in (1, 3):
            sp = x["solve_speedup"] if k == 1 else x["relations_speedup"]
            o.append(f'<text class="p" x="{X(v) + 6:.1f}" y="{yy + bar * 0.85:.1f}" font-size="11">{v:.3g} s ({sp:.1f}× faster)</text>')
        else:
            o.append(f'<text class="q" x="{X(v) + 6:.1f}" y="{yy + bar * 0.85:.1f}" font-size="11">{v:.3g} s</text>')
    y += 4 * bar + 3 * gap + ggap
o.append('</svg>')
open(sys.argv[2], "w").write("\n".join(o) + "\n")
