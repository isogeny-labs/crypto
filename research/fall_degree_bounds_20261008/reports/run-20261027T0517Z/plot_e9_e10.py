"""Lay out recorded E9/E10 d_F values as SVG (no computation beyond medians of recorded values).
Usage: plot_e9_e10.py STUDY_DIR OUT.svg"""
import glob, json, os, sys
D, OUT = sys.argv[1], sys.argv[2]
def rows(pat):
    out = []
    for f in sorted(glob.glob(os.path.join(D, pat))):
        out += [json.loads(l) for l in open(f) if l.strip().startswith('{')]
    return out
new = rows('e9/run-r3/out/E_*.jsonl') + rows('e10/run-r3-B/out/E_*.jsonl') + rows('e10/run-r3-C/out/E_*.jsonl')
old = []  # committed diagonal points read with E10 (not rerun): E6 B cells
for r in rows('e6/B_3-18-6.jsonl') + rows('e6/B_5-10-2.rerun.jsonl'):
    old.append(r)
for r in rows('e8/E8_4-8-2.jsonl'):
    old.append(r)
def val(r):  # recorded d_F, or (cap+1, censored)
    if r.get('d_f') is not None:
        return r['d_f'], False
    return r['d_f_at_least'], True
COL = {3: '#c53030', 4: '#2b6cb0', 5: '#2f855a', 6: '#805ad5'}
W, H = 1100, 430
def panel(x0, xs_key, xlabel, title, xmin, xmax):
    pw, ph, top = 470, 300, 60
    sx = lambda x: x0 + 50 + (x - xmin) / (xmax - xmin) * (pw - 60)
    sy = lambda y: top + ph - (y - 1.5) / (6.5 - 1.5) * ph
    s = [f'<text x="{x0+50}" y="{top-25}" font-size="14" font-weight="bold">{title}</text>']
    s.append(f'<rect x="{x0+50}" y="{top}" width="{pw-60}" height="{ph}" fill="none" stroke="#999"/>')
    for y in (2, 3, 4, 5, 6):
        s.append(f'<line x1="{x0+50}" x2="{x0+pw-10}" y1="{sy(y)}" y2="{sy(y)}" stroke="#eee"/>')
        s.append(f'<text x="{x0+40}" y="{sy(y)+4}" font-size="11" text-anchor="end">{y}</text>')
    step = 1 if xmax - xmin <= 12 else 6
    x = xmin
    while x <= xmax:
        s.append(f'<text x="{sx(x)}" y="{top+ph+16}" font-size="11" text-anchor="middle">{x}</text>')
        x += step
    s.append(f'<line x1="{x0+50}" x2="{x0+pw-10}" y1="{sy(4.5)}" y2="{sy(4.5)}" stroke="#555" stroke-dasharray="4,3"/>')
    s.append(f'<text x="{x0+pw-12}" y="{sy(4.5)-4}" font-size="10" text-anchor="end">cmax = 4: above the line = not refuted at degree 4 (d_F ≥ 5)</text>')
    s.append(f'<text x="{x0+50+(pw-60)/2}" y="{top+ph+36}" font-size="12" text-anchor="middle">{xlabel}</text>')
    s.append(f'<text x="{x0+12}" y="{top+ph/2}" font-size="12" transform="rotate(-90 {x0+12} {top+ph/2})" text-anchor="middle">d_F (HKY last fall degree)</text>')
    import collections
    cnt = collections.Counter()
    for src, rs in (('new', new), ('old', old)):
        for r in rs:
            v, cens = val(r)
            key = (src, r['m'], xs_key(r), v, cens)
            cnt[key] += 1
    jit = {3: -0.18, 4: -0.06, 5: 0.06, 6: 0.18}
    for (src, m, xv, v, cens), c in sorted(cnt.items()):
        cx, cy = sx(xv + jit[m] * (1 if step == 1 else 3)), sy(v + (0.25 if cens else 0))
        r_ = 3 + 2 * c
        fill = 'none' if (cens or src == 'old') else COL[m]
        dash = ' stroke-dasharray="2,2"' if src == 'old' else ''
        s.append(f'<circle cx="{cx:.1f}" cy="{cy:.1f}" r="{r_}" fill="{fill}" stroke="{COL[m]}" stroke-width="2"{dash}/>')
        if cens:
            s.append(f'<text x="{cx:.1f}" y="{cy-r_-3:.1f}" font-size="10" text-anchor="middle" fill="{COL[m]}">≥5</text>')
    return '\n'.join(s)
body = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif">',
        '<rect width="100%" height="100%" fill="white"/>',
        panel(0, lambda r: r['ell'], 'ℓ (factor-base dimension per summand)', 'd_F against ℓ', 2, 9),
        panel(550, lambda r: r['n_vars'], 'N = (m − 2)·n + m·ℓ (unknowns)', 'd_F against N', 24, 60)]
ly = H - 22
lx = 60
for m in (3, 4, 5, 6):
    body.append(f'<circle cx="{lx}" cy="{ly}" r="6" fill="{COL[m]}" stroke="{COL[m]}"/><text x="{lx+10}" y="{ly+4}" font-size="11">m = {m}</text>')
    lx += 80
body.append(f'<text x="{lx}" y="{ly+4}" font-size="11">filled: E9/E10 draw · hollow + "≥5": censored at degree 4 · dashed: committed E6/E8 diagonal points · area ∝ draws</text>')
body.append('</svg>')
open(OUT, 'w').write('\n'.join(body))
print('wrote', OUT, len(new), 'new draws', len(old), 'committed')
