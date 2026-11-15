#!/usr/bin/env python3
"""Figure from results.json (no computation): each arm's paired ratio to rho-cubic (the matched rho
on j = 0 curves) at w = 0, 0.1, 1 per field size, with 95% intervals."""
import json
D = 'research/curve25519_autolab_20261104/siblings/stage4-aut3'
R = json.load(open(f'{D}/results.json'))
arms = [('sib8-p512', '#7fb0ea'), ('sib8-p512-aut3', '#1b4f9c'), ('tw2cj-lazy-aut3', '#8a8a8a')]
PW, PH, L, T, GAP = 330, 300, 60, 64, 50
W = L + 3 * PW + 2 * GAP + 20; H = T + PH + 80
y = lambda v: T + PH - v / 3.5 * PH
s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif" font-size="12">', f'<rect width="{W}" height="{H}" fill="#fff"/>',
     '<text x="20" y="24" font-size="15" font-weight="bold">Stage 4 (local L0): ratio to rho-cubic (matched rho, A = 6) on j = 0, b = 7 prime-order curves</text>',
     '<text x="20" y="42" fill="#555">paired GAE ratios over 180 pairs per cell, 95% intervals; aut=3 = orbit-canonical keys under the order-3 automorphism</text>']
for i, b in enumerate((34, 37, 40)):
    x0 = L + i * (PW + GAP)
    s.append(f'<text x="{x0 + PW/2}" y="{T - 8}" text-anchor="middle" font-weight="bold">p {b} bits</text>')
    for v in (0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5):
        s.append(f'<line x1="{x0}" x2="{x0+PW}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="{"#c0392b" if v == 1.0 else "#e3e3e3"}" stroke-width="{1.5 if v == 1 else 1}"/><text x="{x0-5}" y="{y(v)+4:.1f}" text-anchor="end">{v:.1f}</text>')
    for j, w in enumerate(('w=0', 'w=0.1', 'w=1')):
        gx = x0 + 14 + j * 108
        s.append(f'<text x="{gx+40}" y="{T+PH+18}" text-anchor="middle">{w.replace("w=", "w = ")}</text>')
        for a_i, (a, col) in enumerate(arms):
            c = R[f'j0b7-{b}: {a} / rho-cubic'][w]; bx = gx + a_i * 27
            s.append(f'<rect x="{bx}" y="{y(c["ratio"]):.1f}" width="22" height="{y(0)-y(c["ratio"]):.1f}" fill="{col}"/>')
            s.append(f'<line x1="{bx+11}" x2="{bx+11}" y1="{y(c["ci95"][0]):.1f}" y2="{y(c["ci95"][1]):.1f}" stroke="#000" stroke-width="1.5"/>')
for a_i, (a, col) in enumerate(arms):
    s.append(f'<rect x="{L + a_i*250}" y="{H-30}" width="12" height="12" fill="{col}"/><text x="{L + a_i*250 + 18}" y="{H-20}">{a} / rho-cubic</text>')
s.append(f'<line x1="{L + 760}" x2="{L+790}" y1="{H-24}" y2="{H-24}" stroke="#c0392b" stroke-width="1.5"/><text x="{L+796}" y="{H-20}">rho-cubic = 1</text>')
s.append('</svg>')
open(f'{D}/figures/stage4_aut3.svg', 'w').write('\n'.join(s))
