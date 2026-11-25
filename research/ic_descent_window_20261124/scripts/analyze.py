#!/usr/bin/env python3
"""Summarise the descent-window sweep: summary.json, summary.md, descent_window.svg.

usage: analyze.py DIR   (DIR holds data/rows.jsonl and data/isolation.jsonl.gz)
"""
import json, math, statistics as st, sys, collections, pathlib
D = pathlib.Path(sys.argv[1])
rows = [json.loads(l) for l in open(D / 'data/rows.jsonl')]
import gzip
iso = [json.loads(l) for l in gzip.open(D / 'data/isolation.jsonl.gz', 'rt')]
W = {'w16': 16, 'w32': 32, 'w64': 64, 'w128': 128, 'w207': 207, 'unset': 207, 'w414': 414,
     'w828': 828, 'd3': 2187, 'full': 6560}
ORDER = ['w16', 'w32', 'w64', 'w128', 'w207', 'unset', 'w414', 'w828', 'd3', 'full']
by = collections.defaultdict(list)
for r in rows:
    by[r['arm']].append(r)
rounds = sorted({r['round'] for r in rows})
def sec(arm, k):
    return next(r for r in by[arm] if r['round'] == k)['descent_seconds_total']
digests = {r['recovered_digest'] for r in rows}
arms = []
for a in ORDER:
    rs = by[a]
    ms = [r['descent_seconds_total'] * 1e3 for r in rs]
    arms.append({'arm': a, 'window': W[a], 'runs': len(rs),
                 'verified': [r['verified'] for r in rs], 'targets': rs[0]['targets'],
                 'descent_trials': sorted({r['descent_trials'] for r in rs}),
                 'summands_scanned': rs[0]['descent_trials'] * W[a],
                 'descent_ms_median': st.median(ms), 'descent_ms_min': min(ms), 'descent_ms_max': max(ms)})
def paired(x, y):
    v = sorted(sec(x, k) / sec(y, k) for k in rounds)
    return {'num': x, 'den': y, 'per_round': v, 'median': st.median(v)}
pairs = [paired('full', 'unset'), paired('unset', 'w207'), paired('w828', 'unset'),
         paired('d3', 'unset'), paired('w414', 'unset'), paired('w128', 'unset')]
summary = {'schema': 'ic-descent-window-sweep/v1', 'runs': len(rows), 'rounds': len(rounds),
           'contended_runs': sum(1 for i in iso if i.get('contended')),
           'recovered_digests': sorted(digests), 'arms': arms, 'paired_ratios': pairs}
json.dump(summary, open(D / 'summary.json', 'w'), indent=1)
with open(D / 'summary.md', 'w') as f:
    f.write('| arm | window | descent trials | summands scanned | descent ms, median [min, max] | verified |\n')
    f.write('|:--|--:|--:|--:|:--|:--|\n')
    for a in arms:
        f.write(f"| `{a['arm']}` | {a['window']:,} | {a['descent_trials'][0]:,} | {a['summands_scanned']:,} | "
                f"{a['descent_ms_median']:.2f} [{a['descent_ms_min']:.2f}, {a['descent_ms_max']:.2f}] | "
                f"{min(a['verified'])}/{a['targets']} in all {a['runs']} runs |\n")
    f.write('\n| ratio (per round, same round paired) | rounds, sorted | median |\n|:--|:--|--:|\n')
    for p in pairs:
        f.write(f"| {p['num']} / {p['den']} | {', '.join(f'{v:.3f}' for v in p['per_round'])} | {p['median']:.3f} |\n")
# ---- SVG: one series, median descent time against window (log x), min-max whiskers
pts = [a for a in arms if a['arm'] != 'unset']
Wd, H, L, Rm, T, B = 720, 400, 72, 24, 48, 64
xs = [math.log10(a['window']) for a in pts]
x0, x1 = math.log10(10), math.log10(10000)
y1 = 60.0
def X(w): return L + (math.log10(w) - x0) / (x1 - x0) * (Wd - L - Rm)
def Y(v): return T + (1 - v / y1) * (H - T - B)
ink1, ink2, grid, blue, surf = '#0b0b0b', '#52514e', '#e6e5e1', '#2a78d6', '#fcfcfb'
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {Wd} {H}" width="{Wd}" height="{H}" font-family="system-ui, sans-serif" font-size="12">',
     f'<rect width="{Wd}" height="{H}" fill="{surf}"/>',
     f'<text x="{L}" y="22" font-size="14" font-weight="600" fill="{ink1}">m = 3 descent time against window, k0n41-subgroup-wide (32 targets, one thread)</text>',
     f'<text x="{L}" y="38" fill="{ink2}">median of 5 interleaved isolated runs; whiskers min-max; every run 32/32 verified</text>']
for v in range(0, 61, 10):
    o.append(f'<line x1="{L}" x2="{Wd - Rm}" y1="{Y(v):.1f}" y2="{Y(v):.1f}" stroke="{grid}" stroke-width="1"/>')
    o.append(f'<text x="{L - 8}" y="{Y(v) + 4:.1f}" text-anchor="end" fill="{ink2}">{v}</text>')
for w in (10, 100, 1000, 10000):
    o.append(f'<text x="{X(w):.1f}" y="{H - B + 18}" text-anchor="middle" fill="{ink2}">{w:,}</text>')
o.append(f'<text x="{(L + Wd - Rm) / 2}" y="{H - 18}" text-anchor="middle" fill="{ink2}">descent window w (summands scanned a trial, log scale)</text>')
o.append(f'<text x="18" y="{(T + H - B) / 2}" text-anchor="middle" fill="{ink2}" transform="rotate(-90 18 {(T + H - B) / 2})">descent ms, 32 targets</text>')
for a in pts:
    x = X(a['window'])
    o.append(f'<line x1="{x:.1f}" x2="{x:.1f}" y1="{Y(a["descent_ms_min"]):.1f}" y2="{Y(a["descent_ms_max"]):.1f}" stroke="{blue}" stroke-width="2" stroke-linecap="round"/>')
path = ' '.join(f'{"M" if i == 0 else "L"}{X(a["window"]):.1f},{Y(a["descent_ms_median"]):.1f}' for i, a in enumerate(pts))
o.append(f'<path d="{path}" fill="none" stroke="{blue}" stroke-width="2"/>')
for a in pts:
    o.append(f'<circle cx="{X(a["window"]):.1f}" cy="{Y(a["descent_ms_median"]):.1f}" r="4.5" fill="{blue}" stroke="{surf}" stroke-width="2"><title>w = {a["window"]}: {a["descent_ms_median"]:.2f} ms median, {a["descent_trials"][0]} trials</title></circle>')
lab = {207: 'main (collection window 207)', 6560: 'v6: full scan', 2187: '⌈|F|/3⌉'}
for a in pts:
    if a['window'] in lab:
        x, y = X(a['window']), Y(a['descent_ms_median'])
        anchor = 'end' if a['window'] == 6560 else 'start'
        dx = -8 if anchor == 'end' else 8
        dy = -12 if a['window'] != 2187 else 18
        o.append(f'<text x="{x + dx:.1f}" y="{y + dy:.1f}" text-anchor="{anchor}" fill="{ink1}">{lab[a["window"]]}: {a["descent_ms_median"]:.1f} ms</text>')
o.append('</svg>')
open(D / 'descent_window.svg', 'w').write('\n'.join(o) + '\n')
print(open(D / 'summary.md').read())
