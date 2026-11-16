#!/usr/bin/env python3
"""fig_stages.svg from results.json: median stage times of the device pair-table build (daemon
RF_SERVE_TIMING) at |F_x| = 2048 and 4096, stacked.   make_figure.py results.json fig_stages.svg"""
import json, sys
st = json.load(open(sys.argv[1]))["stages"]
ST = [("keys", "c1", "key kernel"), ("dedupe", "c2", "dedupe"), ("compact", "c3", "compact"), ("probe", "c4", "probe insert"), ("copy", "c5", "copy of the 67 MB map to the host")]
W, left, right, top, bar, gap = 900, 140, 40, 92, 26, 18
rows = [n for n in ("2048", "4096") if n in st]
H = top + len(rows) * (bar + gap) + 60
xmax = 0.075
X = lambda v: left + (W - left - right) * v / xmax
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">Stages of the device pair-table build on an A100</title>',
     '<desc id="d">Median stage times: at 4096 the copy of the build-index map to the host is 47 ms of about 64 ms; the key kernel 2 ms.</desc>',
     '<style>.bg{fill:#fcfcfb}.c1{fill:#2a78d6}.c2{fill:#86b6ee}.c3{fill:#c3c2b7}.c4{fill:#8f8e89}.c5{fill:#d95f3b}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.c1{fill:#3987e5}.c2{fill:#1f5ea8}.c3{fill:#6b6a64}.c4{fill:#8f8e89}.c5{fill:#e0714f}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="20" y="30" font-size="17" font-weight="600">Where the device pair-table build spends its time (round 6)</text>',
     '<text class="q" x="20" y="52" font-size="12.5">A100, daemon stage timings, median per session creation [pcc:unverified]</text>']
for v in (0, 0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07):
    o.append(f'<line class="g" x1="{X(v):.1f}" x2="{X(v):.1f}" y1="{top - 8}" y2="{H - 50}"/><text class="q" x="{X(v):.1f}" y="{H - 34}" font-size="11" text-anchor="middle">{v * 1000:.0f} ms</text>')
y = top
for n in rows:
    o.append(f'<text class="p" x="{left - 10}" y="{y + bar / 2 + 4}" font-size="12" text-anchor="end">|F_x| = {n}</text>')
    x0 = left
    for k, c, _ in ST:
        w = X(st[n][k]) - left
        o.append(f'<rect class="{c}" x="{x0:.1f}" y="{y}" width="{w:.1f}" height="{bar}"/>'); x0 += w
    o.append(f'<text class="q" x="{x0 + 6:.1f}" y="{y + bar / 2 + 4}" font-size="11">{sum(st[n].values()) * 1000:.1f} ms</text>')
    y += bar + gap
lx = 20
for _, c, lab in ST:
    o.append(f'<rect class="{c}" x="{lx}" y="{H - 18}" width="11" height="11" rx="2"/><text class="q" x="{lx + 16}" y="{H - 8.5}" font-size="11">{lab}</text>'); lx += 16 + 6.3 * len(lab) + 18
o.append("</svg>"); open(sys.argv[2], "w").write("\n".join(o))
