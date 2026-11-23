"""Dependency-free SVG: per-trial rates vs bit size (log scale).  [pcc:unverified]
Reads out/summary.json; writes out/rates.svg.  Zero counts are drawn as open triangles at
their 95% Clopper-Pearson upper bound."""
import os, json, math

OUT = os.path.join(os.path.dirname(__file__), '..', 'out')
S = json.load(open(os.path.join(OUT, 'summary.json')))

W, Hh = 860, 520
L, R, T, B = 80, 230, 30, 60
xmin, xmax = 7.5, 16.5
ymin, ymax = -6.0, 0.0


def X(b):
    return L + (b - xmin) / (xmax - xmin) * (W - L - R)


def Y(v):
    lv = math.log10(max(v, 10 ** ymin))
    return T + (ymax - lv) / (ymax - ymin) * (Hh - T - B)


series = []
colors = {3: '#1f77b4', 4: '#ff7f0e', 6: '#2ca02c', 2: '#9467bd', 5: '#8c564b'}
for r in (3, 4, 6):
    pts = []
    for s in sorted([s for s in S if s['arm'] == 'X' and s['r'] == r], key=lambda s: s['bits']):
        k, n = s['successes_nontrivial'], s['n']
        pts.append((s['bits'], k / n if k else s['succ_nt_ub95'], k == 0))
    series.append((f'honest xedni, nontrivial successes / trial, r={r}', colors[r], 'circle', pts))
for r in (3, 4, 6):
    pts = []
    for s in sorted([s for s in S if s['arm'] == 'PC' and s['r'] == r], key=lambda s: s['bits']):
        k, n = s['planted_realized'], s['planted_n']
        pts.append((s['bits'], k / n if k else s['planted_ci'][1], k == 0))
    series.append((f'planted collinear, realised / trial, r={r}', colors[r], 'square', pts))

refs = []
qs = {}
for s in S:
    if s['arm'] == 'X':
        qs.setdefault(s['bits'], s['q_mean'])
for r in (3, 6):
    refs.append((f'box-1 union bound ((3^r-1)/2)/q, r={r}', colors[r], [(b, min(1.0, (3 ** r - 1) / 2 / q)) for b, q in sorted(qs.items())]))
refs.append(('1/q', '#555555', [(b, 1 / q) for b, q in sorted(qs.items())]))

o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{Hh}" font-family="DejaVu Sans, sans-serif" font-size="12">',
     '<rect width="100%" height="100%" fill="white"/>']
for e in range(int(ymin), int(ymax) + 1):
    y = Y(10 ** e)
    o.append(f'<line x1="{L}" y1="{y:.1f}" x2="{W-R}" y2="{y:.1f}" stroke="#e5e5e5"/>')
    o.append(f'<text x="{L-8}" y="{y+4:.1f}" text-anchor="end">1e{e}</text>')
for b in range(8, 17, 2):
    x = X(b)
    o.append(f'<line x1="{x:.1f}" y1="{T}" x2="{x:.1f}" y2="{Hh-B}" stroke="#f0f0f0"/>')
    o.append(f'<text x="{x:.1f}" y="{Hh-B+18}" text-anchor="middle">{b}</text>')
o.append(f'<text x="{(L+W-R)/2:.0f}" y="{Hh-15}" text-anchor="middle">bits of p (q ~ p)</text>')
o.append(f'<text x="18" y="{(T+Hh-B)/2:.0f}" transform="rotate(-90 18 {(T+Hh-B)/2:.0f})" text-anchor="middle">probability per trial</text>')
o.append(f'<rect x="{L}" y="{T}" width="{W-L-R}" height="{Hh-T-B}" fill="none" stroke="#333"/>')
for name, col, pts in refs:
    d = ' '.join(f'{X(b):.1f},{Y(v):.1f}' for b, v in pts)
    o.append(f'<polyline points="{d}" fill="none" stroke="{col}" stroke-dasharray="5,4" stroke-width="1.2"/>')
for name, col, shape, pts in series:
    d = ' '.join(f'{X(b):.1f},{Y(v):.1f}' for b, v, z in pts if not z)
    if d:
        o.append(f'<polyline points="{d}" fill="none" stroke="{col}" stroke-width="1.5" opacity="0.8"/>')
    for b, v, z in pts:
        x, y = X(b), Y(v)
        if z:
            o.append(f'<path d="M{x-5:.1f},{y-4:.1f} L{x+5:.1f},{y-4:.1f} L{x:.1f},{y+5:.1f} Z" fill="none" stroke="{col}"/>')
        elif shape == 'circle':
            o.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="4" fill="{col}"/>')
        else:
            o.append(f'<rect x="{x-4:.1f}" y="{y-4:.1f}" width="8" height="8" fill="{col}"/>')
ly = T + 10
lx = W - R + 12
for name, col, shape, pts in series:
    o.append(f'<circle cx="{lx}" cy="{ly}" r="4" fill="{col}"/>' if shape == 'circle' else f'<rect x="{lx-4}" y="{ly-4}" width="8" height="8" fill="{col}"/>')
    o.append(f'<text x="{lx+10}" y="{ly+4}" font-size="10">{name}</text>')
    ly += 18
for name, col, pts in refs:
    o.append(f'<line x1="{lx-6}" y1="{ly}" x2="{lx+6}" y2="{ly}" stroke="{col}" stroke-dasharray="5,4"/>')
    o.append(f'<text x="{lx+10}" y="{ly+4}" font-size="10">{name}</text>')
    ly += 18
o.append(f'<path d="M{lx-5},{ly-4} L{lx+5},{ly-4} L{lx},{ly+5} Z" fill="none" stroke="#333"/><text x="{lx+10}" y="{ly+4}" font-size="10">zero count: 95% upper bound</text>')
o.append('</svg>')
open(os.path.join(OUT, 'rates.svg'), 'w').write('\n'.join(o))
print('wrote rates.svg')
