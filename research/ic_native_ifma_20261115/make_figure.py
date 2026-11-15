#!/usr/bin/env python3
"""fig_native.svg from results.json: per workload, relation-phase wall speed-up over rootfind-subtract
(one thread) of worker=native with 1 thread, the C++ emulation arm and worker=native on all 4 threads.
    make_figure.py results.json fig_native.svg
"""
import json, sys
R = json.load(open(sys.argv[1]))["rows"]
order = sorted({(r["session"], r["log2_r"]) for r in R})
ARMS = [("native1", "c1", "scalar native, 1 thread"), ("ifma1", "c2", "IFMA native, 1 thread"), ("native", "c4", "scalar native, 4 threads"),
        ("ifma", "c3", "IFMA native, 4 threads")]
W, left, right, top, bar, gap, ggap = 900, 210, 80, 92, 13, 2, 16
H = top + len(order) * (4 * bar + 3 * gap + ggap) + 46
xmax = 6.2
X = lambda v: left + (W - left - right) * v / xmax
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">Relation-phase speed-up of the scalar and IFMA native sweeps over one thread</title>',
     '<desc id="d">Median relation-phase wall of rootfind-subtract one thread divided by each arm\'s, on a 4-thread VM, L0; the IFMA sweep is 1.25-1.67 times the scalar native sweep.</desc>',
     '<style>.bg{fill:#fcfcfb}.c1{fill:#8f8e89}.c2{fill:#86b6ee}.c3{fill:#2a78d6}.c4{fill:#c3c2b7}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.c1{fill:#8f8e89}.c2{fill:#1f5ea8}.c3{fill:#3987e5}.c4{fill:#6b6a64}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="20" y="30" font-size="17" font-weight="600">Scalar and AVX-512 IFMA native sweeps: relation-phase speed-up over one thread</text>',
     '<text class="q" x="20" y="52" font-size="12.5">4-thread VM, L0, median of 4 [pcc:unverified]; identical S, relations, logarithm and charges in all 64 checks</text>']
for v in range(0, 7):
    o.append(f'<line class="g" x1="{X(v):.1f}" x2="{X(v):.1f}" y1="{top - 8}" y2="{H - 40}"/><text class="q" x="{X(v):.1f}" y="{H - 24}" font-size="11" text-anchor="middle">{v}×</text>')
y = top
for k in order:
    rs = {r["arm"].split("-")[1]: r for r in R if (r["session"], r["log2_r"]) == k}
    m = 2 if k[0] == "ifma2" else 3
    fb = 4096 if m == 2 else 2048
    o.append(f'<text class="p" x="{left - 10}" y="{y + 1.5 * bar + gap + 4}" font-size="12" text-anchor="end">m = {m}, log2 r = {k[1]:.1f}, |F_x| = {fb}</text>')
    for a, c, _ in ARMS:
        v = rs[a]["relation_speedup"]
        o.append(f'<rect class="{c}" x="{left}" y="{y}" width="{X(v) - left:.1f}" height="{bar}" rx="2"/>'
                 f'<text class="q" x="{X(v) + 5:.1f}" y="{y + bar - 2.5}" font-size="10.5">{v:.2f}×</text>')
        y += bar + gap
    y += ggap - gap
lx = 20
for a, c, label in ARMS:
    o.append(f'<rect class="{c}" x="{lx}" y="{H - 14}" width="11" height="11" rx="2"/><text class="q" x="{lx + 16}" y="{H - 4.5}" font-size="11">{label}</text>')
    lx += 16 + 6.3 * len(label) + 18
o.append("</svg>")
open(sys.argv[2], "w").write("\n".join(o))
