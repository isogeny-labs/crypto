#!/usr/bin/env python3
"""Draw graph.svg from RESULTS.json (pure python)."""
import json, os
here = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(here, 'RESULTS.json')))
W, H = 1200, 330
o = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d" font-family="sans-serif" font-size="11">' % (W, H, W, H),
     '<rect width="100%" height="100%" fill="white"/>']
def T(x, y, s, a='start', w='normal'):
    o.append('<text x="%g" y="%g" text-anchor="%s" font-weight="%s">%s</text>' % (x, y, a, w, s))
# Panel A: one-step rate vs p
x0, y0, pw, ph = 50, 40, 250, 220
T(x0, 22, 'A  one-step Smith full_success (n=3000/seed)', w='bold')
ymap = lambda v: y0 + ph - (v - 0.10) / (0.28 - 0.10) * ph
o.append('<rect x="%d" y="%g" width="%d" height="%g" fill="#d8ecd8"/>' % (x0, ymap(0.24), pw, ymap(0.14) - ymap(0.24)))
T(x0 + 4, ymap(0.24) + 12, 'preregistered band [0.14, 0.24]')
o.append('<line x1="%d" y1="%g" x2="%d" y2="%g" stroke="#888" stroke-dasharray="4"/>' % (x0, ymap(0.1857), x0 + pw, ymap(0.1857)))
T(x0 + pw - 2, ymap(0.1857) - 3, 'Smith 0.1857', 'end')
for v in (0.10, 0.15, 0.20, 0.25):
    T(x0 - 4, ymap(v) + 4, '%.2f' % v, 'end')
for i, (p, v) in enumerate(R['onestep_pooled_by_p'].items()):
    cx = x0 + 50 + i * 75
    lo, hi = v['wilson95']
    o.append('<line x1="%g" y1="%g" x2="%g" y2="%g" stroke="#1f4e79" stroke-width="3"/>' % (cx, ymap(lo), cx, ymap(hi)))
    o.append('<circle cx="%g" cy="%g" r="5" fill="#1f4e79"/>' % (cx, ymap(v['rate'])))
    for j, s in enumerate(v['per_seed']):
        o.append('<circle cx="%g" cy="%g" r="2.5" fill="#c55a11"/>' % (cx - 10 + j * 10, ymap(s['rate'])))
    T(cx, y0 + ph + 16, 'p=%s' % p, 'middle')
T(x0, y0 + ph + 34, 'blue: pooled with Wilson 95%; orange: seeds')
# Panel B: ceilings
bx = 350; T(bx, 22, 'B  existence ceilings (exhaustive, % of curves)', w='bold')
names = [('direct_pair', 'pair kernel,\\none step'), ('direct_any', 'any Lagrangian,\\none step'), ('class_pair', 'pair kernel\\nin class'), ('class_any', 'any Lagrangian\\nin class')]
for ci, c in enumerate(R['coverage']):
    for k, (key, lab) in enumerate(names):
        v = c['frac'][key]
        bxx = bx + 10 + k * 62 + ci * 15
        hgt = v * ph
        o.append('<rect x="%g" y="%g" width="14" height="%g" fill="%s"/>' % (bxx, y0 + ph - hgt, hgt, ['#1f4e79', '#7aa6d1'][ci]))
        T(bxx + 7, y0 + ph - hgt - 3, '%.0f' % (100 * v), 'middle')
        if ci == 0:
            for li, part in enumerate(lab.split('\\n')):
                T(bxx + 14, y0 + ph + 14 + 11 * li, part, 'middle')
T(bx + 10, y0 - 6, 'dark: p=%d, light: p=%d' % (R['coverage'][0]['p'], R['coverage'][-1]['p']) if len(R['coverage']) > 1 else 'dark: p=%d' % R['coverage'][0]['p'])
# Panel C: kernel dims: old projection vs canonical map
cx0 = 640; T(cx0, 22, 'C  plane curves through X points (p=101)', w='bold')
old = R['modeltest']['extended_exploratory_p101']['kernel_dim_d2_to_d12_example'][:5]
newd = [0, 0, 1, 3, 6]
for i in range(5):
    d = i + 2
    for k, (v, col) in enumerate(((old[i], '#999999'), (newd[i], '#c55a11'))):
        hh = v * 18
        o.append('<rect x="%g" y="%g" width="12" height="%g" fill="%s"/>' % (cx0 + i * 30 + k * 13, y0 + ph - hh, max(hh, 1), col))
        T(cx0 + i * 30 + k * 13 + 6, y0 + ph - hh - 3, str(v), 'middle')
    T(cx0 + i * 30 + 12, y0 + ph + 14, str(d), 'middle')
T(cx0, y0 + ph + 34, 'x: degree d; grey: (b0:b1:b2) projection; orange: canonical map')
cn = R['canon_S2b']['by_p']
T(cx0, y0 + 14, 'canonical: unique quartic in %d of %d instances' % (sum(v['unique_quartic'] for v in cn.values()), sum(v['instances'] for v in cn.values())))
# Panel D: priced cost per target vs p, log scale
dx0 = 900; T(dx0, 22, 'D  priced F_p ops per DLP (log10)', w='bold')
import math
dd = R['dlp_S3b']['by_p']
lo, hi = 5.0, 7.6
ym = lambda v: y0 + ph - (math.log10(v) - lo) / (hi - lo) * ph
for t in (5, 6, 7):
    T(dx0 - 4, ym(10 ** t) + 4, '1e%d' % t, 'end')
    o.append('<line x1="%d" y1="%g" x2="%d" y2="%g" stroke="#ddd"/>' % (dx0, ym(10 ** t), dx0 + 250, ym(10 ** t)))
for i, (p, v) in enumerate(dd.items()):
    bx2 = dx0 + 20 + i * 80
    for k, (key, col) in enumerate((('ic_priced_fp_ops_per_target_median', '#1f4e79'), ('rho_priced_median', '#7aa6d1'))):
        val = v[key]
        o.append('<rect x="%g" y="%g" width="28" height="%g" fill="%s"/>' % (bx2 + k * 30, ym(val), y0 + ph - ym(val), col))
    T(bx2 + 30, y0 + ph + 14, 'p=%s' % p, 'middle')
    T(bx2 + 30, ym(v['ic_priced_fp_ops_per_target_median']) - 4, 'x%.1f' % v['S_over_rho']['0']['median'], 'middle')
T(dx0, y0 + ph + 34, 'dark: BSGS on quartic (measured); light: rho (analytic, negation)')
o.append('</svg>')
open(os.path.join(here, 'graph.svg'), 'w').write('\n'.join(o))
