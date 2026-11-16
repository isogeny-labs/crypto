#!/usr/bin/env python3
"""fig_lookup.svg from results.json: per lookup arm, the best configuration's kernel time against
main's best, for both workloads (A100, medians of 5 rotated rounds).
    make_figure.py results.json fig_lookup.svg
"""
import json, sys
r = json.load(open(sys.argv[1]))
arms = [("E", "Eytzinger"), ("H", "packed hash"), ("P", "paired"), ("EP", "Eytzinger + paired"),
        ("HP", "packed + paired"), ("EHP", "all three (new default)")]
work = [("tp", "s1", "|F| = 65536"), ("tpsmall", "s2", "|F| = 1024")]
W, left, right, top, bar, gap, ggap = 760, 200, 80, 78, 15, 3, 14
H = top + len(arms) * (2 * bar + gap + ggap) + 36
lo, hi = 0.95, 1.15
x = lambda v: left + (W - left - right) * (v - lo) / (hi - lo)
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">rootfind lookup variants: speedup at the best configuration</title>',
     '<desc id="d">For each lookup variant, main best-configuration kernel time divided by the variant best-configuration time, for |F| = 65536 and |F| = 1024 on an A100.</desc>',
     '<style>.bg{fill:#fcfcfb}.s1{fill:#2a78d6}.s2{fill:#eb6834}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}.z{stroke:#8f8e89}'
     '@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.s1{fill:#3987e5}.s2{fill:#d95926}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}.z{stroke:#8f8e89}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">rootfind lookups: speedup over main at each arm\'s best configuration</text>',
     '<text class="q" x="16" y="42" font-size="12">main\'s best kernel time ÷ the arm\'s best, A100, medians of 5 rounds; A/A within 0.06%; every arm passed the device selftests</text>',
     '<rect class="s1" x="16" y="52" width="12" height="12" rx="2"/><text class="p" x="34" y="62" font-size="12">|F| = 65536, 65536 targets</text>',
     '<rect class="s2" x="230" y="52" width="12" height="12" rx="2"/><text class="p" x="248" y="62" font-size="12">|F| = 1024, 262144 targets</text>']
for t in (0.95, 1.0, 1.05, 1.10, 1.15):
    o.append(f'<line class="{"z" if t == 1.0 else "g"}" x1="{x(t):.1f}" x2="{x(t):.1f}" y1="{top - 4}" y2="{H - 30}" stroke-width="1"/>')
    o.append(f'<text class="q" x="{x(t):.1f}" y="{H - 14}" font-size="11" text-anchor="middle">{t:.2f}×</text>')
y = top
for a, label in arms:
    o.append(f'<text class="p" x="{left - 10}" y="{y + bar + 2:.1f}" font-size="12" text-anchor="end">{label}</text>')
    for k, (w, cls, wl) in enumerate(work):
        mb = r[w]["best"]["M"]["ms"]; b = r[w]["best"][a]
        v = mb / b["ms"]
        yy = y + k * (bar + gap)
        x0, x1 = sorted((x(1.0), x(v)))
        wdt = max(x1 - x0, 2)
        o.append(f'<g><title>{label}, {wl}: {v:.3f}× ({b["config"]} {b["ms"]:.2f} ms against main {mb:.2f} ms)</title>'
                 f'<rect class="{cls}" x="{x0:.1f}" y="{yy}" width="{wdt:.1f}" height="{bar}" rx="3"/></g>')
        o.append(f'<text class="p" x="{max(x1, x(1.0)) + 6:.1f}" y="{yy + bar * 0.78:.1f}" font-size="11">{v:.3f}× ({b["config"]})</text>')
    y += 2 * bar + gap + ggap
o.append('</svg>')
open(sys.argv[2], "w").write("\n".join(o) + "\n")
