#!/usr/bin/env python3
"""fig_prewarm.svg: wall time of the first fes-f2-gpu row per run, early worker spawn on against off (and off again)."""
import glob, json, os
D = os.path.dirname(os.path.abspath(__file__)) + "/"
pts = {"off": [], "off2": [], "on": []}
for f in sorted(glob.glob(D + "raw/*_*.json")):
    arm = os.path.basename(f).split("_")[0]
    if arm not in pts:
        continue
    for idx, r in enumerate(json.load(open(f))["rows"]):
        s = (r.get("decomposition") or {}).get("solver")
        if s and s.get("name") == "fes-f2-gpu":
            pts[arm].append(r["wall_seconds"]); break      # the first such row
W, H, x0, y0, y1 = 640, 330, 120, 270, 70
def Y(v): return y0 - v / 0.4 * (y0 - y1)
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">Early worker spawn: wall time of the first fes-f2-gpu row</title>',
     '<desc id="d">Wall time of the first fes-f2-gpu row of a host run, per run and as a mean, with the early worker spawn on and off, and off again as a noise floor; a worker that takes 150 ms to start stands in for a CUDA context.</desc>',
     '<style>.bg{fill:#fcfcfb}.on{fill:#1baf7a}.off{fill:#eb6834}.off2{fill:#c9a227}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.on{fill:#199e70}.off{fill:#d95926}.off2{fill:#b8921a}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">First fes-f2-gpu row: early worker spawn on vs off (host, 150 ms start-up stand-in)</text>',
     '<text class="q" x="16" y="42" font-size="12">per run (dots) and mean (bar), 4 interleaved runs per arm; counted fields identical in 24 of 24</text>']
for t in range(0, 5):
    v = t * 0.1
    o.append(f'<line class="g" x1="{x0 - 6}" y1="{Y(v):.1f}" x2="{W - 20}" y2="{Y(v):.1f}"/><text class="q" x="{x0 - 10}" y="{Y(v) + 4:.1f}" font-size="10" text-anchor="end">{v:.1f}</text>')
o.append(f'<text class="q" x="16" y="{(y0 + y1) / 2}" font-size="11" transform="rotate(-90 16 {(y0 + y1) / 2})" text-anchor="middle">row wall, s</text>')
for i, (arm, lab) in enumerate((("off", "off (spawn at first call)"), ("off2", "off again (noise)"), ("on", "on (spawn early)"))):
    cx = x0 + 70 + i * 160
    v = pts[arm]; m = sum(v) / len(v)
    o.append(f'<rect class="{arm}" x="{cx - 34}" y="{Y(m):.1f}" width="68" height="{y0 - Y(m):.1f}" opacity="0.55"><title>{lab}: mean {m:.3f} s</title></rect>')
    for k, x in enumerate(v):
        o.append(f'<circle class="{arm}" cx="{cx - 21 + k * 14}" cy="{Y(x):.1f}" r="3.5"/>')
    o.append(f'<text class="p" x="{cx}" y="{Y(m) - 8:.1f}" font-size="11" text-anchor="middle">{m:.3f}</text>')
    o.append(f'<text class="q" x="{cx}" y="{y0 + 18}" font-size="10" text-anchor="middle">{lab}</text>')
o.append('</svg>')
open(D + "fig_prewarm.svg", "w").write("\n".join(o))
