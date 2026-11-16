#!/usr/bin/env python3
"""Figure from results.json (no computation): ratio to tw2cj-lazy per prime-field shape at w = 0, 0.1, 1, 95% intervals; matched-rho line."""
import json
D = 'research/curve25519_autolab_20261104/siblings/stage2-primefield'
R = json.load(open(f'{D}/results.json'))
arms = [('sib8-p64', '#7fb0ea'), ('sib8-p256', '#3b7dd8'), ('sib8-p512', '#1b4f9c')]
shapes = [('a3', 'a = -3 (P-256 shape)', 'rho-neg'), ('gen', 'generic a, b', 'rho-neg'), ('j0b7', 'j = 0, b = 7 (secp256k1 shape)', 'rho-cubic')]
PW, PH, L, T, GAP = 330, 300, 60, 60, 45
W = L + 3 * PW + 2 * GAP + 20; H = T + PH + 80
y = lambda v: T + PH - v / 1.4 * PH
s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif" font-size="12">', f'<rect width="{W}" height="{H}" fill="#fff"/>',
     '<text x="20" y="24" font-size="15" font-weight="bold">Stage 2 (local L0): sibling points vs tw2cj-lazy on prime-order prime-field curves, p ~ 2^31, 240 pairs per cell</text>']
for i, (t, name, rho) in enumerate(shapes):
    x0 = L + i * (PW + GAP)
    s.append(f'<text x="{x0 + PW/2}" y="{T - 10}" text-anchor="middle" font-weight="bold">{name}</text>')
    for v in (0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4):
        s.append(f'<line x1="{x0}" x2="{x0+PW}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="{"#000" if v == 1 else "#e3e3e3"}"/>')
        s.append(f'<text x="{x0-5}" y="{y(v)+4:.1f}" text-anchor="end">{v:.1f}</text>')
    lr = R[f'{t}: tw2cj-lazy / {rho}']['w=0']['ratio']
    s.append(f'<line x1="{x0}" x2="{x0+PW}" y1="{y(1/lr):.1f}" y2="{y(1/lr):.1f}" stroke="#c0392b" stroke-dasharray="5,4"/>')
    s.append(f'<line x1="{x0+8}" x2="{x0+34}" y1="{y(1.3):.1f}" y2="{y(1.3):.1f}" stroke="#c0392b" stroke-dasharray="5,4"/><text x="{x0+40}" y="{y(1.3)+4:.1f}" fill="#c0392b">{rho} (w = 0: {1/lr:.3f})</text>')
    for j, w in enumerate(('w=0', 'w=0.1', 'w=1')):
        gx = x0 + 12 + j * 108
        s.append(f'<text x="{gx+42}" y="{T+PH+18}" text-anchor="middle">{w.replace("w=", "w = ")}</text>')
        for a_i, (a, col) in enumerate(arms):
            c = R[f'{t}: {a} / tw2cj-lazy'][w]; bx = gx + a_i * 28
            s.append(f'<rect x="{bx}" y="{y(c["ratio"]):.1f}" width="22" height="{y(0)-y(c["ratio"]):.1f}" fill="{col}"/>')
            s.append(f'<line x1="{bx+11}" x2="{bx+11}" y1="{y(c["ci95"][0]):.1f}" y2="{y(c["ci95"][1]):.1f}" stroke="#000" stroke-width="1.5"/>')
for a_i, (a, col) in enumerate(arms):
    s.append(f'<rect x="{L + a_i*220}" y="{H-30}" width="12" height="12" fill="{col}"/><text x="{L + a_i*220 + 18}" y="{H-20}">{a} / tw2cj-lazy</text>')
s.append('</svg>')
open(f'{D}/figures/stage2_primefield.svg', 'w').write('\n'.join(s))
