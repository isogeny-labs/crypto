#!/usr/bin/env python3
"""fig_pairs.svg from results.json: per session (rf3, rf2d), curve and base size, the median solve wall of
the CPU arm, the GPU arm with a worker per run (spawn) and the GPU arm on the daemon, log scale, with
the shared counted S against rho.
    make_figure.py results.json fig_pairs.svg
"""
import json, math, sys
R = json.load(open(sys.argv[1]))
rows = [("rf3pairs", x) for x in R.get("rf3pairs", {}).get("rows", [])] + [("rf2mt2", x) for x in R.get("rf2mt2", {}).get("rows", [])]
ARMS = [("cpu", "c1", "CPU, 1 thread"), ("mt", "c4", "CPU, 14 threads (quota 13.6)"), ("gpud", "c2", "GPU, CPU-built pair table"), ("gpup", "c3", "GPU, pair table on device")]
W, left, right, top, bar, gap, ggap = 880, 230, 160, 92, 10, 2, 14
nb = lambda x: sum(a in x["arms"] for a, _, _ in ARMS)
H = top + sum(nb(x) * (bar + gap) + ggap for _, x in rows) + 40
vals = [x["arms"][a]["solve_s"] for _, x in rows for a, _, _ in ARMS if a in x["arms"]]
lo, hi = 10 ** math.floor(math.log10(min(vals))), 10 ** math.ceil(math.log10(max(vals)))
X = lambda v: left + (W - left - right) * (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo))
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">rootfind-subtract on the GPU, m = 2 and m = 3: solve wall time per arm</title>',
     '<desc id="d">Median complete-solve wall time of the CPU arm, the GPU arm with a worker per run, and the GPU arm on a daemon worker, per curve and base size, log scale; all arms report the same counted S.</desc>',
     '<style>.bg{fill:#fcfcfb}.c1{fill:#8f8e89}.c2{fill:#86b6ee}.c3{fill:#2a78d6}.c4{fill:#c3c2b7}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.c1{fill:#8f8e89}.c2{fill:#1f5ea8}.c3{fill:#3987e5}.c4{fill:#52514e}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">rootfind-subtract in complete IC solves: the m = 3 pair table on the device</text>',
     '<text class="q" x="16" y="42" font-size="12">median solve wall (L0, descriptive), log scale; A100 pod; every arm reports the same counted S, logarithm and relation counts</text>']
for i, (_, c, l) in enumerate(ARMS):
    o.append(f'<rect class="{c}" x="{16 + 190 * i}" y="54" width="12" height="12" rx="2"/><text class="p" x="{34 + 190 * i}" y="64" font-size="12">{l}</text>')
t = lo
while t <= hi * 1.0001:
    o.append(f'<line class="g" x1="{X(t):.1f}" x2="{X(t):.1f}" y1="{top - 6}" y2="{H - 30}" stroke-width="1"/>')
    o.append(f'<text class="q" x="{X(t):.1f}" y="{H - 14}" font-size="11" text-anchor="middle">{t:g} s</text>')
    t *= 10
y = top
for sess, x in rows:
    o.append(f'<text class="p" x="{left - 10}" y="{y + 1.5 * bar:.1f}" font-size="12" text-anchor="end">m = {x["m"]}, log2 r = {x["log2_r"]:.1f}, |F_x| = {x["fb_abscissae"]}</text>')
    o.append(f'<text class="q" x="{left - 10}" y="{y + 1.5 * bar + 15:.1f}" font-size="11" text-anchor="end">S = {x["mean_S"]:.1f} ({x["S_over_rho"]:.0f}× rho)</text>')
    k = -1
    for a, cls, _ in ARMS:
        if a not in x["arms"]:
            continue
        k += 1
        v = x["arms"][a]["solve_s"]; yy = y + k * (bar + gap)
        if x["m"] == 2 and a == "gpud":
            cls = "c3"  # m = 2 has no pair table: its GPU arm is the GPU daemon
        o.append(f'<g><title>{a}: {v:.3f} s</title><rect class="{cls}" x="{left}" y="{yy}" width="{max(X(v) - left, 2):.1f}" height="{bar}" rx="2"/></g>')
        lab = f'{v:.3g} s' + ("" if a == "cpu" else f' ({x["arms"][a]["solve_speedup"]:.1f}× CPU)')
        o.append(f'<text class="{"q" if a == "cpu" else "p"}" x="{X(v) + 6:.1f}" y="{yy + bar * 0.85:.1f}" font-size="11">{lab}</text>')
    y += nb(x) * (bar + gap) + ggap
o.append('</svg>')
open(sys.argv[2], "w").write("\n".join(o) + "\n")
