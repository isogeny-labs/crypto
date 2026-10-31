"""Lay out recorded E12b r2 d_F values against ell as SVG (no computation beyond medians of
recorded values). Context points: the committed E10 diagonal cell 3:21:7 (N = 42).
Usage: plot_e12b.py STUDY_DIR OUT.svg"""
import glob, json, os, sys, collections
D, OUT = sys.argv[1], sys.argv[2]
def rows(pat):
    out = []
    for f in sorted(glob.glob(os.path.join(D, pat))):
        out += [json.loads(l) for l in open(f) if l.strip().startswith('{')]
    return out
new = rows('e12/run-b2/out/E_*.jsonl')
ctx = rows('e10/run-r3-B/out/E_3-21-7.jsonl') + rows('e10/run-r3-C/out/E_3-21-7.jsonl')
def val(r):
    return (r['d_f'], False) if r.get('d_f') is not None else (r['d_f_at_least'], True)
W, H, x0, top, pw, ph = 720, 440, 70, 60, 600, 300
sx = lambda x: x0 + (x - 3) / (8 - 3) * pw
sy = lambda y: top + ph - (y - 2.5) / (6 - 2.5) * ph
s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif">',
     '<rect width="100%" height="100%" fill="white"/>',
     f'<text x="{x0}" y="30" font-size="14" font-weight="bold">E12b (m = 3, N = 42–45): d_F against ℓ, job r2</text>',
     f'<rect x="{x0}" y="{top}" width="{pw}" height="{ph}" fill="none" stroke="#999"/>']
for y in (3, 4, 5):
    s.append(f'<line x1="{x0}" x2="{x0+pw}" y1="{sy(y)}" y2="{sy(y)}" stroke="#eee"/>')
    s.append(f'<text x="{x0-10}" y="{sy(y)+4}" font-size="11" text-anchor="end">{y}</text>')
for x in range(3, 9):
    s.append(f'<text x="{sx(x)}" y="{top+ph+16}" font-size="11" text-anchor="middle">{x}</text>')
s.append(f'<line x1="{x0}" x2="{x0+pw}" y1="{sy(4.5)}" y2="{sy(4.5)}" stroke="#555" stroke-dasharray="4,3"/>')
s.append(f'<text x="{x0+pw-4}" y="{sy(4.5)-4}" font-size="10" text-anchor="end">cmax = 4: above the line = d_F ≥ 5 (censored)</text>')
s.append(f'<text x="{x0+pw/2}" y="{top+ph+36}" font-size="12" text-anchor="middle">ℓ (factor-base dimension per summand)</text>')
s.append(f'<text x="22" y="{top+ph/2}" font-size="12" transform="rotate(-90 22 {top+ph/2})" text-anchor="middle">d_F (HKY last fall degree)</text>')
cnt = collections.Counter()
for src, rs, dx in (('r2', new, -0.15), ('e10', ctx, 0.15)):
    for r in rs:
        v, c = val(r)
        cnt[(src, r['ell'], r['n'], v, c, dx)] += 1
for (src, ell, n, v, c, dx), k in sorted(cnt.items()):
    col = '#c53030' if src == 'r2' else '#718096'
    cx, cy, rad = sx(ell + dx), sy(v + (0.25 if c else 0)), 3 + 2 * k
    fill = 'none' if (c or src != 'r2') else col
    dash = ' stroke-dasharray="2,2"' if src != 'r2' else ''
    s.append(f'<circle cx="{cx:.1f}" cy="{cy:.1f}" r="{rad}" fill="{fill}" stroke="{col}" stroke-width="2"{dash}/>')
    ly = cy - rad - 4 if src == 'r2' else cy + rad + 12
    s.append(f'<text x="{cx:.1f}" y="{ly:.1f}" font-size="10" text-anchor="middle" fill="{col}">{"≥5" if c else v} ×{k} (n={n})</text>')
s.append(f'<text x="{x0}" y="{H-12}" font-size="10">Red: E12b r2 (3:24:7 not completed: 1 of 4 draws). Grey dashed: E10 3:21:7 (N = 42, diagonal), for context. Marker area ∝ draws.</text>')
s.append('</svg>')
open(OUT, 'w').write('\n'.join(s))
