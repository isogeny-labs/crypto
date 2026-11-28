#!/usr/bin/env python3
"""Hand-written SVG figure (matplotlib absent): calibration ladder and per-instance ratios."""
import json, math, statistics as st, csv
from pathlib import Path

R = Path(__file__).resolve().parents[1]
cal = json.loads((R / 'data/calibration.json').read_text())
summ = json.loads((R / 'data/summary.json').read_text())
inst = list(csv.DictReader(open(R / 'data/per_instance.csv')))

W, H = 980, 830
C = dict(ideal='#c2410c', visit='#1d4ed8', B='#15803d', grid='#d4d4d8', ink='#18181b', mute='#52525b')
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
       f'<rect width="{W}" height="{H}" fill="#ffffff"/>']


def panel(x0, y0, w, h, xs, ys, title, xlabel, ylabel, xlog=True):
    lx = (lambda v: math.log10(v)) if xlog else (lambda v: v)
    X0, X1 = lx(xs[0]), lx(xs[1])
    Y0, Y1 = math.log10(ys[0]), math.log10(ys[1])
    px = lambda v: x0 + (lx(v) - X0) / (X1 - X0) * w
    py = lambda v: y0 + h - (math.log10(v) - Y0) / (Y1 - Y0) * h
    out.append(f'<text x="{x0}" y="{y0-14}" font-size="14" font-weight="bold" fill="{C["ink"]}">{title}</text>')
    for e in range(int(math.floor(Y0)), int(math.ceil(Y1)) + 1):
        v = 10 ** e
        if ys[0] <= v <= ys[1]:
            out.append(f'<line x1="{x0}" x2="{x0+w}" y1="{py(v):.1f}" y2="{py(v):.1f}" stroke="{C["grid"]}"/>')
            lab = f'{v:g}' if 1e-3 <= v <= 1e4 else f'1e{e}'
            out.append(f'<text x="{x0-6}" y="{py(v)+4:.1f}" text-anchor="end" fill="{C["mute"]}">{lab}</text>')
    out.append(f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" fill="none" stroke="{C["ink"]}"/>')
    out.append(f'<text x="{x0+w/2}" y="{y0+h+34}" text-anchor="middle" fill="{C["ink"]}">{xlabel}</text>')
    out.append(f'<text transform="translate({x0-48},{y0+h/2}) rotate(-90)" text-anchor="middle" fill="{C["ink"]}">{ylabel}</text>')
    return px, py


# Panel 1: calibration ladder
Ms = sorted({r['M'] for r in cal})
px, py = panel(80, 50, 360, 290, (4, 128), (1e-3, 20), 'Calibration: one solve per process (median of 3 charts)',
               'M (domain [0, M] per variable)', 'seconds per instance')
for M in Ms:
    out.append(f'<text x="{px(M):.1f}" y="{50+290+16}" text-anchor="middle" fill="{C["mute"]}">{M}</text>')
for arm, col in (('F4-ideal', C['ideal']), ('F4-visit', C['visit'])):
    pts = []
    for M in Ms:
        v = [r['wall'] for r in cal if r['arm'] == arm and r['M'] == M and r['complete']]
        if len(v) == 3:
            pts.append((px(M), py(st.median(v))))
    out.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (col, ' '.join(f'{a:.1f},{b:.1f}' for a, b in pts)))
    for a, b in pts:
        out.append(f'<circle cx="{a:.1f}" cy="{b:.1f}" r="3" fill="{col}"/>')
bmed = summ['median_seconds']['B']
out.append(f'<rect x="{px(128)-5:.1f}" y="{py(bmed)-5:.1f}" width="10" height="10" fill="{C["B"]}"/>')
out.append(f'<line x1="80" x2="440" y1="{py(1):.1f}" y2="{py(1):.1f}" stroke="{C["ink"]}" stroke-dasharray="5 4"/>')
out.append(f'<text x="86" y="{py(1)-5:.1f}" fill="{C["ink"]}">gate: fastest baseline ≥ 1 s</text>')
ly = 60
for lab, col in (('F4-ideal (domain polys; only Gröbner arm)', C['ideal']), ('F4-visit ((M+1)^2 enumeration, tiny F4 leaves)', C['visit']), ('program B at M=128 (panel median)', C['B'])):
    out.append(f'<rect x="92" y="{ly}" width="10" height="10" fill="{col}"/><text x="108" y="{ly+9}" fill="{C["ink"]}">{lab}</text>')
    ly += 16

# Panel 2: per-instance ratios
n = len(inst)
lo = min(min(float(r['ratio']) for r in inst), 0.5)
hi = max(max(float(r['ratio_vs_ideal']) for r in inst if r['ratio_vs_ideal'] != 'nan'), 10) * 1.6
px2, py2 = panel(560, 50, 390, 290, (0, n + 1), (lo, hi), f'Panel M=128, 20 charts: F4 time / program-B time',
                 'chart (1–10 planted, 11–20 unplanted)', 'ratio (>1: program B faster)', xlog=False)
for k, col, lab in ((1, C['mute'], '1×'), (2, C['ink'], '2× release'), (8, C['ink'], '8× target')):
    out.append(f'<line x1="560" x2="950" y1="{py2(k):.1f}" y2="{py2(k):.1f}" stroke="{col}" stroke-dasharray="{"2 3" if k == 1 else "6 4"}"/>')
    out.append(f'<text x="946" y="{py2(k)-4:.1f}" text-anchor="end" fill="{col}">{lab}</text>')
for j, r in enumerate(inst, 1):
    x = px2(j)
    out.append(f'<circle cx="{x:.1f}" cy="{py2(float(r["ratio"])):.1f}" r="4" fill="{C["visit"]}"/>')
    if r['ratio_vs_ideal'] != 'nan':
        out.append(f'<rect x="{x-3.5:.1f}" y="{py2(float(r["ratio_vs_ideal"]))-3.5:.1f}" width="7" height="7" fill="{C["ideal"]}"/>')
    if j % 5 == 0 or j == 1:
        out.append(f'<text x="{x:.1f}" y="{50+290+16}" text-anchor="middle" fill="{C["mute"]}">{j}</text>')
out.append(f'<circle cx="572" cy="{392}" r="4" fill="{C["visit"]}"/><text x="582" y="{396}" fill="{C["ink"]}">vs F4-visit (2D enumeration tree): geometric mean {summ["geomean_ratio_vs_fastest_F4"]:.1f}×</text>')
gi = summ['geomean_ratio_vs_F4_ideal']
out.append(f'<rect x="568" y="{405}" width="7" height="7" fill="{C["ideal"]}"/><text x="582" y="{412}" fill="{C["ink"]}">vs F4-ideal (Gröbner; no A/A duplicate): geometric mean {gi:.0f}×</text>')
out.append(f'<text x="80" y="{434}" font-size="11" fill="{C["mute"]}">p = 4294967291; process wall / 5 copies; median of 3 interleaved reps; L0 shared host [pcc:unverified].</text>')
out.append(f'<text x="80" y="{449}" font-size="11" fill="{C["mute"]}">Gate: FAIL by the frozen rule (baseline &lt; 1 s, B A/A 15 %), but those conditions were unreachable by design: inconclusive. M=128 &lt; p^(1/3): easy regime.</text>')
# Panel 3: POST-HOC exploratory M ladder (not gate-scored)
ex = list(csv.DictReader(open(R / 'data/explore_posthoc.csv')))
px3, py3 = panel(80, 490, 870, 260, (120, 1750), (2e-3, 20), 'POST-HOC exploratory (not gate-scored): charts s1, s2, s11 per M; B node cap 100000; all M &lt; p^(1/3) (easy regime)',
                 'M', 'seconds per instance')
for M in sorted({int(r['M']) for r in ex}):
    out.append(f'<text x="{px3(M):.1f}" y="{490+260+16}" text-anchor="middle" fill="{C["mute"]}">{M}</text>')
out.append(f'<line x1="80" x2="950" y1="{py3(1):.1f}" y2="{py3(1):.1f}" stroke="{C["ink"]}" stroke-dasharray="5 4"/>')
out.append(f'<text x="86" y="{py3(1)-5:.1f}" fill="{C["ink"]}">1 s</text>')
for Mv, lab in ((4294967291 ** 0.25, 'p^(1/4) = 256 (model scale)'), (4294967291 ** (1/3), 'p^(1/3) ≈ 1625: below it an O(M) integer-cubic scan solves exactly')):
    out.append(f'<line x1="{px3(Mv):.1f}" x2="{px3(Mv):.1f}" y1="490" y2="750" stroke="{C["mute"]}" stroke-dasharray="3 3"/>')
    out.append(f'<text x="{px3(Mv)-4:.1f}" y="{490+250}" text-anchor="end" font-size="11" fill="{C["mute"]}">{lab}</text>')
for key, col in (('F4visit_s_per_instance', C['visit']), ('B_s_per_instance', C['B'])):
    meds = []
    for M in sorted({int(r['M']) for r in ex}):
        vs = [r for r in ex if int(r['M']) == M]
        for r in vs:
            x, y = px3(M), py3(float(r[key]))
            if key.startswith('B') and r['B_complete'] == 'False':
                out.append(f'<path d="M{x-4:.1f},{y-4:.1f}L{x+4:.1f},{y+4:.1f}M{x-4:.1f},{y+4:.1f}L{x+4:.1f},{y-4:.1f}" stroke="#b91c1c" stroke-width="2"/>')
            else:
                out.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="3" fill="{col}" fill-opacity="0.8"/>')
        meds.append((px3(M), py3(st.median(float(r[key]) for r in vs))))
    out.append('<polyline fill="none" stroke="%s" stroke-width="1.5" points="%s"/>' % (col, ' '.join(f'{a:.1f},{b:.1f}' for a, b in meds)))
out.append(f'<circle cx="100" cy="{490+20}" r="4" fill="{C["visit"]}"/><text x="110" y="{490+24}" fill="{C["ink"]}">F4-visit (complete at every M)</text>')
out.append(f'<circle cx="100" cy="{490+38}" r="4" fill="{C["B"]}"/><text x="110" y="{490+42}" fill="{C["ink"]}">program B, complete</text>')
out.append(f'<path d="M96,{490+52}L104,{490+60}M96,{490+60}L104,{490+52}" stroke="#b91c1c" stroke-width="2"/><text x="110" y="{490+60}" fill="{C["ink"]}">program B INCOMPLETE (node cap hit; returns no roots, flagged)</text>')
out.append('</svg>')
(R / 'figure.svg').write_text('\n'.join(out))
print('wrote figure.svg')
