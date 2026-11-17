#!/usr/bin/env python3
"""Summarize `ic bench --sweep` reports (results/*.json) into RESULTS.md,
results.json and results.svg.  Standard library only.

    summarize.py results/ RESULTS.md results.json results.svg

Per instance and configuration (label): mean and min S over the repeats, mean
s_over_rho (S over the matched counted rho's mean S), whether every repeat
recovered the planted logarithm, hit rate, targets tried, matrix rows/rank,
and for the algebraic oracle the solver's share of the total GAE, its
priced_by, calls, budget hits and wall per call."""
import glob, json, math, os, statistics, sys
src, md_out, json_out, svg_out = sys.argv[1:5]
files = sorted(glob.glob(os.path.join(src, '*.json'))) if os.path.isdir(src) else [src]
cells = []
for f in files:
    d = json.load(open(f))
    rho = d.get('rho_reference') or {}
    groups = {}
    for r in d.get('rows', []):
        groups.setdefault(r['label'], []).append(r)
    inst = {'file': os.path.basename(f), 'instance': d.get('instance'), 'regime': d.get('regime'),
            'rho_method': rho.get('method'), 'rho_mean_s': rho.get('mean_s'), 'rho_runs': rho.get('runs'),
            'rho_all_verified': rho.get('all_verified'), 'skipped': d.get('configurations_skipped', []),
            'elapsed_s': d.get('elapsed_seconds'), 'rows': []}
    r0 = next(iter(d.get('rows', [])), None)
    inst['r'] = r0['r'] if r0 else None
    for label, runs in groups.items():
        S = [x['s'] for x in runs]
        sor = [x.get('s_over_rho') for x in runs if x.get('s_over_rho') is not None]
        dec = [x['decomposition'] for x in runs]
        sol = [x['decomposition'].get('solver') for x in runs if x['decomposition'].get('solver')]
        row = {'label': label, 'repeats': len(runs), 's_mean': statistics.mean(S), 's_min': min(S), 's_max': max(S),
               's_over_rho_mean': statistics.mean(sor) if sor else None,
               'verified_all': all(x['verified'] for x in runs),
               'hit_rate': statistics.mean(x['hit_rate'] for x in dec),
               'targets_tried': statistics.mean(x['targets_tried'] for x in dec),
               'matrix_rows': statistics.mean(x['linear_algebra']['rows'] for x in runs),
               'rank': statistics.mean(x['linear_algebra']['rank'] for x in runs),
               'total_gae': statistics.mean(x['total_gae'] for x in runs),
               'oracle_gae': statistics.mean(x['cost']['gae'] if isinstance(x.get('cost'), dict) and 'gae' in x['cost'] else float('nan') for x in dec),
               'n_vars': (dec[0].get('system') or {}).get('n_vars'),
               'solver': None}
        if sol:
            tg = statistics.mean(x['total_gae'] for x in runs)
            row['solver'] = {'name': sol[0]['name'], 'priced_by': sol[0]['priced_by'],
                             'calls': statistics.mean(s['calls'] for s in sol),
                             'budget_exceeded': sum(s['budget_exceeded'] for s in sol),
                             'gae_share': statistics.mean(s['gae'] for s in sol) / tg if tg else None,
                             'wall_ms_per_call': statistics.mean(s['wall_ns'] / max(s['calls'], 1) for s in sol) / 1e6,
                             'solving_degree_max': max(s['solving_degree_max'] for s in sol)}
        inst['rows'].append(row)
    cells.append(inst)
json.dump(cells, open(json_out, 'w'), indent=1)

def f(v, nd=2): return 'n/a' if v is None or (isinstance(v, float) and math.isnan(v)) else (f'{v:.{nd}f}' if isinstance(v, float) else str(v))
with open(md_out, 'w') as out:
    out.write('Per instance: the matched counted rho (mean S over its runs) and every configuration run end to end. `S` = total group-addition equivalents / sqrt(r), mean over repeats (min–max); `S / rho` = mean s_over_rho; `ok` = every repeat recovered the planted logarithm; `hit` = decomposition hit rate; `tried` = targets tried; `solver` = engine, priced_by, mean calls, budget hits, share of the row\'s GAE, wall per call.\n\n')
    out.write('| instance (r) | configuration | S mean (min–max) | S / rho | ok | hit | tried | rows / rank | solver |\n|:--|:--|--:|--:|:--|--:|--:|--:|:--|\n')
    for c in cells:
        out.write(f"| `{c['instance']}` (r = {c['r']}) | **matched rho** `{c['rho_method']}`, {c['rho_runs']} runs | {f(c['rho_mean_s'])} | 1.00 | {'y' if c['rho_all_verified'] else 'NO'} | | | | |\n")
        for r in c['rows']:
            s = r['solver']
            scol = '' if not s else f"`{s['name']}` {s['priced_by']}, {s['calls']:.0f} calls, {s['budget_exceeded']} over budget, {100*s['gae_share']:.0f}% of GAE, {f(s['wall_ms_per_call'])} ms/call"
            out.write(f"| | `{r['label']}` | {f(r['s_mean'])} ({f(r['s_min'])}–{f(r['s_max'])}) | {f(r['s_over_rho_mean'])} | {'y' if r['verified_all'] else 'NO'} | {f(r['hit_rate'],4)} | {r['targets_tried']:.0f} | {r['matrix_rows']:.0f} / {r['rank']:.0f} | {scol} |\n")
        for sk in c['skipped']:
            out.write(f"| | `{sk.get('configuration')}` | skipped | | | | | | {sk.get('why','')[:120]} |\n")
# SVG: S per instance, one marker per configuration, rho as a grey bar
W, H, L, Bm, T = 1100, 480, 70, 170, 40
labels = []
for c in cells:
    for r in c['rows']:
        if r['label'] not in labels: labels.append(r['label'])
colors = ['#2ca02c', '#ff7f0e', '#1f77b4', '#d62728', '#9467bd', '#8c564b']
vals = [r['s_mean'] for c in cells for r in c['rows']] + [c['rho_mean_s'] for c in cells if c['rho_mean_s']]
if vals:
    lo, hi = min(vals) / 1.5, max(vals) * 1.6
    def y(v): return T + (H - T - Bm) * (1 - (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo)))
    gw = (W - L - 20) / max(len(cells), 1)
    o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica, Arial, sans-serif" font-size="11">', f'<rect width="{W}" height="{H}" fill="white"/>',
         f'<text x="{L}" y="22" font-size="14" font-weight="bold">End to end: S = group-addition equivalents / sqrt(r) per configuration (markers) and the matched counted rho (grey bar), log scale</text>']
    e = math.floor(math.log10(lo))
    while 10 ** e <= hi:
        if 10 ** e >= lo:
            o.append(f'<line x1="{L}" x2="{W-20}" y1="{y(10**e):.1f}" y2="{y(10**e):.1f}" stroke="#ddd"/>'); o.append(f'<text x="{L-6}" y="{y(10**e)+4:.1f}" text-anchor="end">{10**e:g}</text>')
        e += 1
    for i, c in enumerate(cells):
        x0 = L + i * gw
        if c['rho_mean_s']:
            o.append(f'<rect x="{x0+gw*0.1:.1f}" y="{y(c["rho_mean_s"]):.1f}" width="{gw*0.8:.1f}" height="{H-Bm-y(c["rho_mean_s"]):.1f}" fill="#e0e0e0"/>')
            o.append(f'<text x="{x0+gw/2:.1f}" y="{y(c["rho_mean_s"])-3:.1f}" text-anchor="middle" font-size="9" fill="#555">rho {c["rho_mean_s"]:.1f}</text>')
        for r in c['rows']:
            j = labels.index(r['label']); x = x0 + gw * (0.2 + 0.6 * j / max(len(labels) - 1, 1))
            o.append(f'<circle cx="{x:.1f}" cy="{y(r["s_mean"]):.1f}" r="4.5" fill="{colors[j % len(colors)]}"{"" if r["verified_all"] else " stroke=\"red\" stroke-width=\"2\""}/>')
        o.append(f'<text transform="translate({x0+gw/2:.1f},{H-Bm+14}) rotate(30)" font-size="9">{c["instance"]} (r={c["r"]})</text>')
    for j, lab in enumerate(labels):
        o.append(f'<circle cx="{W-300}" cy="{36+j*15}" r="4.5" fill="{colors[j % len(colors)]}"/><text x="{W-290}" y="{40+j*15}" font-size="10">{lab}</text>')
    o.append('</svg>'); open(svg_out, 'w').write('\n'.join(o))
print(open(md_out).read())
