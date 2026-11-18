#!/usr/bin/env python3
"""fig_sweep.svg from results.json: S of each sweep fraction over S of the full sweep, per curve and
base size, measured (solid, markers) against the LEADS.md prediction (dashed).
    make_figure.py results.json fig_sweep.svg
"""
import json
import sys

C = json.load(open(sys.argv[1]))["cells"]
groups = sorted({(c["bits"], c["a"]) for c in C})
COL = ["c1", "c2", "c3", "c4"]
FS = [1, 2, 4, 8, 16]
W, H, L, R, T, B = 820, 470, 70, 230, 70, 56
xs = {f: L + (W - L - R) * i / (len(FS) - 1) for i, f in enumerate(FS)}
Y = lambda v: T + (H - T - B) * (1 - v)  # ratio 0..1
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" '
     'font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">Counted cost S of the rotating sweep window relative to the full sweep</title>',
     '<desc id="d">Mean S of m = 3 rootfind-subtract with sweep = 1/N over the mean S of sweep = 1, per curve '
     'and base size; solid lines measured, dashed lines the prediction frozen in PROTOCOL.md.</desc>',
     '<style>.bg{fill:#fcfcfb}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
     '.c1{stroke:#2a78d6;fill:#2a78d6}.c2{stroke:#86b6ee;fill:#86b6ee}.c3{stroke:#c2410c;fill:#c2410c}.c4{stroke:#f2a46b;fill:#f2a46b}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">Rotating sweep window: S relative to the full sweep</text>',
     '<text class="q" x="16" y="42" font-size="12">m = 3 rootfind-subtract, incremental-gauss-echelon; mean S over 3 targets x 2 rounds; '
     'solid = measured, dashed = predicted (LEADS.md)</text>']
for v in [0, 0.25, 0.5, 0.75, 1.0]:
    o.append(f'<line class="g" x1="{L}" x2="{W - R}" y1="{Y(v):.1f}" y2="{Y(v):.1f}"/>')
    o.append(f'<text class="q" x="{L - 8}" y="{Y(v) + 4:.1f}" font-size="11" text-anchor="end">{v:.2f}</text>')
for f in FS:
    o.append(f'<text class="q" x="{xs[f]:.1f}" y="{H - B + 18}" font-size="11" text-anchor="middle">{"1" if f == 1 else "1/" + str(f)}</text>')
o.append(f'<text class="q" x="{(L + W - R) / 2:.1f}" y="{H - 14}" font-size="12" text-anchor="middle">sweep fraction per trial</text>')
o.append(f'<text class="q" x="18" y="{(T + H - B) / 2:.1f}" font-size="12" text-anchor="middle" '
         f'transform="rotate(-90 18 {(T + H - B) / 2:.1f})">S / S(sweep = 1)</text>')
for gi, (bits, a) in enumerate(groups):
    cls = COL[gi % 4]
    cells = {c["f"]: c for c in C if (c["bits"], c["a"]) == (bits, a)}
    pts = [(xs[f], Y(cells[f]["ratio_to_f1"])) for f in FS if f in cells]
    o.append(f'<polyline class="{cls}" fill="none" stroke-width="2" points="{" ".join(f"{x:.1f},{y:.1f}" for x, y in pts)}"/>')
    for x, y in pts:
        o.append(f'<circle class="{cls}" cx="{x:.1f}" cy="{y:.1f}" r="3.5"/>')
    pr = [(xs[1], Y(1.0))] + [(xs[f], Y(cells[f]["predicted_ratio"])) for f in FS[1:] if f in cells and cells[f]["predicted_ratio"]]
    o.append(f'<polyline class="{cls}" fill="none" stroke-width="1.5" stroke-dasharray="5 4" '
             f'points="{" ".join(f"{x:.1f},{y:.1f}" for x, y in pr)}"/>')
    ly = T + 20 + 40 * gi
    o.append(f'<line class="{cls}" x1="{W - R + 20}" x2="{W - R + 44}" y1="{ly}" y2="{ly}" stroke-width="2"/>')
    last = cells[max(cells)]
    o.append(f'<text class="p" x="{W - R + 50}" y="{ly + 4}" font-size="12">log2 r = {bits}, |F_x| = {a}</text>')
    o.append(f'<text class="q" x="{W - R + 50}" y="{ly + 19}" font-size="11">1/16: {last["ratio_to_f1"]:.2f}, S/rho {last["S_over_rho"]:.0f}</text>')
o.append('</svg>')
open(sys.argv[2], "w").write("\n".join(o) + "\n")
