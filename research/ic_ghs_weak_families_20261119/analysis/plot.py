#!/usr/bin/env python3
"""One SVG: IC/rho (M-eq, w = 0) against q for the GHS-weak plane-quartic family, committed v2 sizes plus the new fields of this study."""
import json, math, os, sys
LANE = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
rows = [json.loads(l) for l in open(f'{LANE}/../quartic_n4v2_20261020/results/summary.json') if l.strip()]
ic = {(r['arm'], r['q']): r['meq'][0] for r in rows if r['kind'] == 'ic_size'}
excl = {(r['arm'], r['q']) for r in rows if r['kind'] == 'rho_excluded'}
rho = {(r['arm'], r['q']): r['meq'][0] for r in rows if r['kind'] == 'rho_size' and (r['arm'], r['q']) not in excl}
new = json.load(open(f'{LANE}/measure_results.json'))['fields'] if os.path.exists(f'{LANE}/measure_results.json') else {}
W, H, L, R, T, B = 760, 440, 70, 20, 30, 60
xs = [math.log2(q) for q in (223, 4194271)]; ymin, ymax = -8, 5
X = lambda q: L + (math.log2(q) - xs[0])/(xs[1]-xs[0])*(W-L-R)
Y = lambda r: T + (ymax - math.log2(r))/(ymax-ymin)*(H-T-B)
col = {'B0': '#c1440e', 'DL1': '#1f6f8b'}
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="11">',
     f'<rect width="{W}" height="{H}" fill="white"/>',
     f'<text x="{L}" y="18" font-size="13">GHS-weak E0/F_q^3 (plane quartic): IC / rho, cold M-eq, w = 0, plain rho (hollow = this study)</text>']
for k in range(ymin, ymax+1, 2):
    o.append(f'<line x1="{L}" x2="{W-R}" y1="{Y(2**k):.1f}" y2="{Y(2**k):.1f}" stroke="#ddd"/><text x="{L-6}" y="{Y(2**k)+4:.1f}" text-anchor="end">2^{k}</text>')
o.append(f'<line x1="{L}" x2="{W-R}" y1="{Y(1):.1f}" y2="{Y(1):.1f}" stroke="black" stroke-dasharray="4 3"/><text x="{W-R-4}" y="{Y(1)-4:.1f}" text-anchor="end">parity (1.0)</text>')
for e in range(8, 23, 2):
    o.append(f'<text x="{X(2**e):.1f}" y="{H-B+16}" text-anchor="middle">2^{e}</text>')
o.append(f'<text x="{(L+W-R)/2}" y="{H-12}" text-anchor="middle">q (cubic extension F_q^3, group order about q^3)</text>')
for arm in ('B0', 'DL1'):
    pts = sorted((q, v/rho[('plain', q)]) for (a, q), v in ic.items() if a == arm and ('plain', q) in rho)
    o.append(f'<polyline fill="none" stroke="{col[arm]}" stroke-width="1.6" points="' + ' '.join(f'{X(q):.1f},{Y(r):.1f}' for q, r in pts) + '"/>')
    for q, r in pts: o.append(f'<circle cx="{X(q):.1f}" cy="{Y(r):.1f}" r="3" fill="{col[arm]}"/>')
    for q, F in new.items():
        v = F['arms'].get(f'{arm}/plain')
        if v: o.append(f'<circle cx="{X(int(q)):.1f}" cy="{Y(v[0]["ratio"]):.1f}" r="5" fill="white" stroke="{col[arm]}" stroke-width="2"/>')
o.append(f'<text x="{L+8}" y="{T+14}" fill="{col["B0"]}">B0 (single large prime)</text><text x="{L+8}" y="{T+28}" fill="{col["DL1"]}">DL1 (double large prime, headline arm)</text>')
o.append('</svg>')
open(f'{LANE}/ic_over_rho.svg', 'w').write('\n'.join(o))
