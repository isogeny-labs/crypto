#!/usr/bin/env python3
"""fig_overhead.svg from results.json: fes-f2-gpu solver time per call, old (text serve, slow
from_poly) vs new (binary serve, direct from_poly), by instance and repeat; one A100, ABBA.
    make_figure.py results.json fig_overhead.svg
"""
import json, sys
r = json.load(open(sys.argv[1]))
rows = []
for key, label in (("koblitz-n23", "Koblitz n=23 · 24 unk."), ("char2-n25", "char-2 n=25 · 26 unk."), ("char2-n29", "char-2 n=29 · 30 unk.")):
    g = r[key]["fes-f2-gpu"]
    for i in sorted(g["solver_ms_per_call_old"], key=int):
        rows.append((f"{label}" + (f" · target {int(i) + 1}" if len(g["solver_ms_per_call_old"]) > 1 else ""),
                     g["solver_ms_per_call_old"][i], g["solver_ms_per_call_new"][i]))
W, left, right, top, bar, gap, ggap = 760, 230, 90, 76, 16, 3, 16
H = top + len(rows) * (2 * bar + gap + ggap) + 36
xmax = 3.0
x = lambda v: left + (W - left - right) * v / xmax
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">fes-f2-gpu solver time per call, before and after</title>',
     '<desc id="d">Milliseconds per solver call, old text-serve engine against new binary-serve engine, per instance and target, A100; counted ops identical.</desc>',
     '<style>.bg{fill:#fcfcfb}.s1{fill:#eb6834}.s2{fill:#2a78d6}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.s1{fill:#d95926}.s2{fill:#3987e5}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">fes-f2-gpu: solver time per call, before and after</text>',
     '<text class="q" x="16" y="42" font-size="12">ms per call (round trip to the CUDA worker plus Rust-side work), mean of two runs each, A100; counted ops identical</text>',
     '<rect class="s1" x="16" y="52" width="12" height="12" rx="2"/><text class="p" x="34" y="62" font-size="12">before (text protocol, AnfRow route)</text>',
     '<rect class="s2" x="290" y="52" width="12" height="12" rx="2"/><text class="p" x="308" y="62" font-size="12">after (binary protocol, direct from_poly)</text>']
for t in (0, 1, 2, 3):
    o.append(f'<line class="g" x1="{x(t):.1f}" x2="{x(t):.1f}" y1="{top - 4}" y2="{H - 30}" stroke-width="1"/>')
    o.append(f'<text class="q" x="{x(t):.1f}" y="{H - 14}" font-size="11" text-anchor="middle">{t} ms</text>')
y = top
for label, old, new in rows:
    o.append(f'<text class="p" x="{left - 10}" y="{y + bar + 2:.1f}" font-size="12" text-anchor="end">{label}</text>')
    for k, (v, cls) in enumerate(((old, "s1"), (new, "s2"))):
        yy = y + k * (bar + gap); w = max(x(v) - left, 3)
        o.append(f'<g><title>{label}: {v:.3f} ms per call ({"before" if k == 0 else "after"})</title><path class="{cls}" d="M{left},{yy} h{w - 4:.1f} a4,4 0 0 1 4,4 v{bar - 8} a4,4 0 0 1 -4,4 h{-(w - 4):.1f} z"/></g>')
        o.append(f'<text class="p" x="{left + w + 6:.1f}" y="{yy + bar * 0.75:.1f}" font-size="11">{v:.2f} ms' + (f'  ({old / new:.1f}× less)' if k == 1 else '') + '</text>')
    y += 2 * bar + gap + ggap
o.append('</svg>')
open(sys.argv[2], "w").write("\n".join(o) + "\n")
