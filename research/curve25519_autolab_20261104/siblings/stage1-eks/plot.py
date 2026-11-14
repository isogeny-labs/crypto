#!/usr/bin/env python3
"""Figure from results.json (no computation): ratio to tw2cj-lazy at w = 0, 0.1, 1, with 95% intervals."""
import json
D = 'research/curve25519_autolab_20261104/siblings/stage1-eks'
R = json.load(open(f'{D}/results.json'))
arms = [('tw2cj-lazy-sib8-p64', '#3b7dd8'), ('tw2cj-lazy-sib8-p256', '#1b4f9c'), ('tw2cj-cv0', '#8a8a8a')]
W, H, L, T, PW, PH = 900, 430, 70, 50, 360, 300
def y(v): return T + PH - (v - 0.0) / 1.4 * PH
s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
     f'<rect width="{W}" height="{H}" fill="#fff"/>',
     '<text x="20" y="22" font-size="15" font-weight="bold">Stage 1 (local L0): paired ratio to tw2cj-lazy in GAE, five curves, 240 pairs per cell</text>']
for i, k in enumerate((28, 32)):
    x0 = L + i * (PW + 80)
    s.append(f'<text x="{x0 + PW/2}" y="{T - 8}" text-anchor="middle" font-weight="bold">k = {k}</text>')
    for v in (0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4):
        s.append(f'<line x1="{x0}" x2="{x0+PW}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="{"#000" if v == 1.0 else "#e3e3e3"}" stroke-width="{1.2 if v == 1.0 else 1}"/>')
        s.append(f'<text x="{x0-6}" y="{y(v)+4:.1f}" text-anchor="end">{v:.1f}</text>')
    rho = R[f'k{k}: tw2cj-lazy / rho-neg']['w=0']['ratio']
    s.append(f'<line x1="{x0}" x2="{x0+PW}" y1="{y(1/rho):.1f}" y2="{y(1/rho):.1f}" stroke="#c0392b" stroke-dasharray="5,4"/>')
    lab_y = y(1.3)
    s.append(f'<line x1="{x0+8}" x2="{x0+36}" y1="{lab_y:.1f}" y2="{lab_y:.1f}" stroke="#c0392b" stroke-dasharray="5,4"/>')
    s.append(f'<text x="{x0+42}" y="{lab_y+4:.1f}" fill="#c0392b">rho-neg / tw2cj-lazy (w = 0: {1/rho:.3f})</text>')
    for j, w in enumerate(('w=0', 'w=0.1', 'w=1')):
        gx = x0 + 20 + j * 115
        s.append(f'<text x="{gx+45}" y="{T+PH+18}" text-anchor="middle">{w.replace("w=", "w = ")}</text>')
        for a_i, (a, col) in enumerate(arms):
            c = R[f'k{k}: {a} / tw2cj-lazy'][w]; bx = gx + a_i * 30
            s.append(f'<rect x="{bx}" y="{y(c["ratio"]):.1f}" width="24" height="{y(0)-y(c["ratio"]):.1f}" fill="{col}"/>')
            s.append(f'<line x1="{bx+12}" x2="{bx+12}" y1="{y(c["ci95"][0]):.1f}" y2="{y(c["ci95"][1]):.1f}" stroke="#000" stroke-width="1.5"/>')
for a_i, (a, col) in enumerate(arms):
    s.append(f'<rect x="{L + a_i*230}" y="{H-32}" width="12" height="12" fill="{col}"/><text x="{L + a_i*230 + 18}" y="{H-22}">{a} / tw2cj-lazy</text>')
s.append('</svg>')
open(f'{D}/figures/stage1_ratios.svg', 'w').write('\n'.join(s))
