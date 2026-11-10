#!/usr/bin/env python3
"""Summarize `ic descent --solver` JSON (schema_version 1, operation descent-engines)
into RESULTS.md, results.json and results.svg.  Standard library only.

    summarize.py results/ladder.json RESULTS.md results.json results.svg

Per cell and engine: the median over targets of the per-target median wall
(milliseconds), the ratio of that to f4-f2 and to the cell's reference engine,
decided targets, budget hits, agreement with the reference, max degree reached.
A cell where an engine declined or hit the budget on every target shows that
instead of a number."""
import json, math, statistics, sys
src, md_out, json_out, svg_out = sys.argv[1:5]
d = json.load(open(src))
rows = []
for c in d['cells']:
    per_engine = {}
    for t in c['per_target']:
        for eng, runs in t['runs'].items():
            dec = [r for r in runs if r['verdict'] in ('solved', 'unsatisfiable')]
            if len(dec) == len(runs) and runs:
                per_engine.setdefault(eng, []).append(statistics.median(r['wall_ns'] for r in runs) / 1e6)
    summ = {e['engine']: e for e in c['engines']}
    cell = {'family': c['family'], 'curve': c['curve'], 'n': c['n'], 'n_prime': c['n_prime'], 'm': c['summands'],
            'n_vars': c['n_vars'], 'equations': c['equations'], 'targets': c['targets'], 'repeats': c['repeats'],
            'reference': c['reference_engine'], 'engines': {}}
    for eng, s in summ.items():
        ms = per_engine.get(eng, [])
        cell['engines'][eng] = {
            'median_ms': statistics.median(ms) if ms else None, 'min_ms': min(ms) if ms else None, 'max_ms': max(ms) if ms else None,
            'decided': s['decided'], 'over_budget': s['over_budget'], 'declined': s['declined'],
            'agrees': s['agrees_with_reference'], 'd_reach_max': s.get('d_reach_max'), 'ms_mean': s['ms_mean'],
            'wall_over_reference': s.get('wall_over_reference'), 'n_targets_decided_all_reps': len(ms)}
    rows.append(cell)
json.dump(rows, open(json_out, 'w'), indent=1)

engines = []
for cell in rows:
    for e in cell['engines']:
        if e not in engines: engines.append(e)
def fmt(v): return 'n/a' if v is None else (f'{v:.1f}' if v >= 10 else f'{v:.2f}')
with open(md_out, 'w') as f:
    f.write('Median over targets of the per-target median wall, ms, one call. Ratio columns: engine / `f4-f2` and engine / reference on the same cell. `dec` = targets decided in every repetition / targets; `ob` = targets with a budget verdict; `agree` = every decided answer equals the reference.\n\n')
    f.write('| family | n | n\' | m | vars | ref | ' + ' | '.join(f'`{e}` ms (dec, ob, agree, Dmax)' for e in engines) + ' | m4gb / f4-f2 | m4gb / ref |\n')
    f.write('|:--|--:|--:|--:|--:|:--|' + '--:|' * len(engines) + '--:|--:|\n')
    for cell in rows:
        cols = []
        for e in engines:
            s = cell['engines'].get(e)
            if s is None: cols.append('—'); continue
            if s['declined']: cols.append('declined'); continue
            cols.append(f"{fmt(s['median_ms'])} ({s['decided']}/{cell['targets']}, {s['over_budget']}, {'y' if s['agrees'] else 'NO'}, {s['d_reach_max'] if s['d_reach_max'] is not None else '-'})")
        m4 = cell['engines'].get('m4gb-f2', {}).get('median_ms'); f4 = cell['engines'].get('f4-f2', {}).get('median_ms'); ref = cell['engines'].get(cell['reference'], {}).get('median_ms')
        r1 = f'{m4/f4:.2f}×' if m4 and f4 else 'n/a'; r2 = f'{m4/ref:.2f}×' if m4 and ref else 'n/a'
        f.write(f"| {cell['family']} | {cell['n']} | {cell['n_prime']} | {cell['m']} | {cell['n_vars']} | `{cell['reference']}` | " + ' | '.join(cols) + f' | {r1} | {r2} |\n')
# SVG: median ms per cell, one marker series per engine, log scale
colors = {'fes-f2': '#8c8c8c', 'f4-f2': '#2ca02c', 'inherited-f4': '#ff7f0e', 'm4gb-f2': '#1f77b4'}
pts = [(i, e, c['engines'][e]['median_ms']) for i, c in enumerate(rows) for e in engines if c['engines'].get(e, {}).get('median_ms')]
W, H, L, Bm, T = 1100, 460, 70, 150, 40
if pts:
    vals = [v for _, _, v in pts]; lo, hi = min(vals) / 1.5, max(vals) * 1.5
    def y(v): return T + (H - T - Bm) * (1 - (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo)))
    gw = (W - L - 20) / max(len(rows), 1)
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica, Arial, sans-serif" font-size="11">', f'<rect width="{W}" height="{H}" fill="white"/>',
           f'<text x="{L}" y="22" font-size="14" font-weight="bold">One decomposition call: median wall per cell (Weil descent of the Semaev polynomial, m·n\' unknowns), log scale, one thread</text>']
    e10 = math.floor(math.log10(lo))
    while 10 ** e10 <= hi:
        if 10 ** e10 >= lo:
            out.append(f'<line x1="{L}" x2="{W-20}" y1="{y(10**e10):.1f}" y2="{y(10**e10):.1f}" stroke="#ddd"/>'); out.append(f'<text x="{L-6}" y="{y(10**e10)+4:.1f}" text-anchor="end">{10**e10:g} ms</text>')
        e10 += 1
    for ei, e in enumerate(engines):
        series = [(L + i * gw + gw / 2, y(v)) for i, ee, v in pts if ee == e]
        if len(series) > 1:
            out.append('<polyline fill="none" stroke="%s" stroke-width="1.5" points="%s"/>' % (colors.get(e, '#000'), ' '.join(f'{x:.1f},{yy:.1f}' for x, yy in series)))
        for x, yy in series:
            out.append(f'<circle cx="{x:.1f}" cy="{yy:.1f}" r="3.5" fill="{colors.get(e, "#000")}"/>')
        out.append(f'<rect x="{W-230}" y="{30+ei*16}" width="12" height="12" fill="{colors.get(e, "#000")}"/><text x="{W-214}" y="{40+ei*16}">{e}</text>')
    for i, c in enumerate(rows):
        out.append(f'<text transform="translate({L+i*gw+gw/2:.1f},{H-Bm+14}) rotate(45)" font-size="9">{c["family"]} n={c["n"]} m={c["m"]} ({c["n_vars"]} vars)</text>')
    out.append('</svg>'); open(svg_out, 'w').write('\n'.join(out))
print(open(md_out).read())
