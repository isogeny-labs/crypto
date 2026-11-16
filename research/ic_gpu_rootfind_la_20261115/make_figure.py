#!/usr/bin/env python3
"""fig_la_gpu.svg from results.json: per workload, the GPU arm's (probe_build=pairs, daemon, cuda)
median solve split into oracle set-up, relation phase, linear algebra and the rest, with
incremental-gauss against incremental-gauss-echelon; the one-thread CPU arm's solve as a label.
    make_figure.py results.json fig_la_gpu.svg
"""
import json, sys
R = json.load(open(sys.argv[1]))["rows"]
get = lambda c, fb, a: next(r for r in R if r["curve"] == c and r["fb"] == fb and r["arm"] == a)
order = sorted({(r["log2_r"], r["fb"], r["curve"]) for r in R})
PH = [("setup_s", "c1", "oracle set-up"), ("rel_s", "c2", "relation phase"), ("la_s", "c3", "linear algebra"), ("rest", "c4", "rest")]
W, left, right, top, bar, gap, ggap = 1000, 250, 150, 92, 16, 4, 20
H = top + len(order) * (2 * bar + gap + ggap) + 50
xmax = 1.95
X = lambda v: left + (W - left - right) * v / xmax
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">GPU m = 3 rootfind solve by phase, incremental-gauss against incremental-gauss-echelon</title>',
     '<desc id="d">Median solve wall of the GPU arm split into set-up, relation phase, linear algebra and the rest; echelon cuts linear algebra about tenfold, the solve to 0.18-0.20 of gauss at |F_x| = 4096.</desc>',
     '<style>.bg{fill:#fcfcfb}.c1{fill:#c3c2b7}.c2{fill:#86b6ee}.c3{fill:#2a78d6}.c4{fill:#e4e3df}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.c1{fill:#6b6a64}.c2{fill:#1f5ea8}.c3{fill:#3987e5}.c4{fill:#383835}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="20" y="30" font-size="17" font-weight="600">m = 3 on the A100: the solve by phase, gauss against echelon</text>',
     '<text class="q" x="20" y="52" font-size="12.5">median of 6, L0 [pcc:unverified]; identical S, relations and logarithm in all 84 checks; right: one-thread CPU (gauss) solve</text>']
for v in (0, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5):
    o.append(f'<line class="g" x1="{X(v):.1f}" x2="{X(v):.1f}" y1="{top - 8}" y2="{H - 44}"/><text class="q" x="{X(v):.1f}" y="{H - 28}" font-size="11" text-anchor="middle">{v:g} s</text>')
y = top
for lr, fb, c in order:
    o.append(f'<text class="p" x="{left - 10}" y="{y + bar + 2}" font-size="12" text-anchor="end">log2 r = {lr:.1f}, |F_x| = {fb}</text>')
    cpu = get(c, fb, "cpu-gauss")["solve_s"]
    o.append(f'<text class="q" x="{W - right + 10}" y="{y + bar + 2}" font-size="11">CPU {cpu:.1f} s</text>')
    for a, lab in (("gpup-gauss", "gauss"), ("gpup-echelon", "echelon")):
        r = get(c, fb, a); x0 = left
        vals = {"setup_s": r["setup_s"], "rel_s": r["rel_s"], "la_s": r["la_s"]}
        vals["rest"] = max(r["solve_s"] - sum(vals.values()), 0)
        for k, cl, _ in PH:
            w = X(vals[k]) - left
            o.append(f'<rect class="{cl}" x="{x0:.1f}" y="{y}" width="{max(w, 0):.1f}" height="{bar}"/>'); x0 += w
        o.append(f'<text class="q" x="{x0 + 5:.1f}" y="{y + bar - 4}" font-size="10.5">{lab} {r["solve_s"]:.3f} s</text>')
        y += bar + gap
    y += ggap - gap
lx = 20
for _, cl, lab in PH:
    o.append(f'<rect class="{cl}" x="{lx}" y="{H - 16}" width="11" height="11" rx="2"/><text class="q" x="{lx + 16}" y="{H - 6.5}" font-size="11">{lab}</text>'); lx += 16 + 6.5 * len(lab) + 20
o.append("</svg>"); open(sys.argv[2], "w").write("\n".join(o))
