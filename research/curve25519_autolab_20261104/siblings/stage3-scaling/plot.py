#!/usr/bin/env python3
"""Figure from scaling.json (no computation): geometric-mean ratio by p bits per shape, GAE at w = 0."""
import json
D = 'research/curve25519_autolab_20261104/siblings/stage3-scaling'
S = json.load(open(f'{D}/scaling.json'))
shapes = [('a3', 'a = -3 (P-256 shape)'), ('gen', 'generic a, b'), ('j0b7', 'j = 0, b = 7 (secp256k1 shape)')]
series = [('sib8-p256 / tw2cj-lazy', '#3b7dd8', ''), ('sib8-p512 / tw2cj-lazy', '#1b4f9c', ''),
          ('sib8-p256 / rho-neg', '#c0392b', ''), ('sib8-p512 / rho-neg', '#7b241c', ''),
          ('tw2cj-lazy / rho-neg', '#888888', '5,4'), ('sib8-p512 / rho-cubic', '#27864a', '2,3')]
PW, PH, L, T, GAP = 330, 300, 60, 64, 50
W = L + 3 * PW + 2 * GAP + 20; H = T + PH + 110
y = lambda v: T + PH - (v - 0.3) / 1.8 * PH
xs = {34: 0.1, 37: 0.5, 40: 0.9}
s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif" font-size="12">', f'<rect width="{W}" height="{H}" fill="#fff"/>',
     '<text x="20" y="24" font-size="15" font-weight="bold">Stage 3 (local L0): cost ratio vs field size on prime-order prime-field curves, GAE at w = 0</text>',
     '<text x="20" y="42" fill="#555">geometric mean over 30 workloads per point; slope of ln(ratio) on ln r with 95% bootstrap interval in scaling.json</text>']
for i, (t, name) in enumerate(shapes):
    x0 = L + i * (PW + GAP)
    X = lambda b: x0 + xs[b] * PW
    s.append(f'<text x="{x0 + PW/2}" y="{T - 8}" text-anchor="middle" font-weight="bold">{name}</text>')
    for v in (0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8, 2.0):
        s.append(f'<line x1="{x0}" x2="{x0+PW}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="{"#000" if v == 1.0 else "#e3e3e3"}"/><text x="{x0-5}" y="{y(v)+4:.1f}" text-anchor="end">{v:.1f}</text>')
    for b in (34, 37, 40):
        s.append(f'<text x="{X(b):.1f}" y="{T+PH+18}" text-anchor="middle">p {b} bits</text>')
    for k, col, dash in series:
        key = f'{t}: {k} [gae]'
        if key not in S:
            continue
        g = S[key]['geomean_ratio_by_p_bits']
        pts = [(X(int(b)), y(v)) for b, v in g.items()]
        s.append(f'<polyline fill="none" stroke="{col}" stroke-width="2" {"stroke-dasharray=%s" % chr(34)+dash+chr(34) if dash else ""} points="{" ".join(f"{a:.1f},{c:.1f}" for a, c in pts)}"/>')
        for a, c in pts:
            s.append(f'<circle cx="{a:.1f}" cy="{c:.1f}" r="3" fill="{col}"/>')
for j, (k, col, dash) in enumerate(series):
    xx = L + (j % 3) * 330; yy = H - 50 + (j // 3) * 20
    s.append(f'<line x1="{xx}" x2="{xx+26}" y1="{yy}" y2="{yy}" stroke="{col}" stroke-width="2" {"stroke-dasharray=%s" % chr(34)+dash+chr(34) if dash else ""}/><text x="{xx+32}" y="{yy+4}">{k}{" (j0b7)" if "cubic" in k else ""}</text>')
s.append('</svg>')
open(f'{D}/figures/stage3_scaling.svg', 'w').write('\n'.join(s))
