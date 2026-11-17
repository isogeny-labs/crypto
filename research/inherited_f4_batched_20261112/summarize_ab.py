#!/usr/bin/env python3
"""Summarize the paired ic descent cells of run_ab.sh.

    summarize_ab.py results/ RESULTS_AB.md results_ab.json results_ab.svg

Per cell and engine: the median over targets of the per-target median wall
(ms), the per-target ops (equal across repeats), decided targets, budget hits
and agreement with the reference.  Paired columns: the median over targets of
the per-target ratio inherited-f4-next / inherited-f4 (with min and max),
the median per-target ops ratio batched / inherited-f4 (the counted work), and fes-f2 /
inherited-f4.  Standard library only."""
import glob, json, math, os, statistics, sys
src, md_out, json_out, svg_out = sys.argv[1:5]
OLD, NEW, REF = 'inherited-f4', 'inherited-f4-batched', 'fes-f2'
cells = []
for f in sorted(glob.glob(os.path.join(src, 'cell-*.json'))):
    for c in json.load(open(f)).get('cells', []):
        cells.append(c)
cells.sort(key=lambda c: (c['n_vars'], c['family']))
rows = []
for c in cells:
    med = {}; ops = {}; ratios = []; fes = []; ops_equal = True; ops_ratios = []; ops_not_more = True
    for t in c['per_target']:
        runs = t['runs']
        m = {e: statistics.median(r['wall_ns'] for r in rs) / 1e6 for e, rs in runs.items() if rs}
        for e, v in m.items():
            med.setdefault(e, []).append(v)
        for e, rs in runs.items():
            ops.setdefault(e, []).append(rs[0].get('ops'))
        if OLD in m and NEW in m:
            ratios.append(m[NEW] / m[OLD])
            o_old, o_new = runs[OLD][0].get('ops'), runs[NEW][0].get('ops')
            if o_old and o_new is not None:
                ops_ratios.append(o_new / o_old)
                if o_new > o_old:
                    ops_not_more = False
            if [r.get('ops') for r in runs[OLD]] != [r.get('ops') for r in runs[NEW]] or runs[OLD][0].get('ops') != runs[NEW][0].get('ops'):
                ops_equal = False
        if OLD in m and REF in m:
            fes.append(m[REF] / m[OLD])
    summ = {e['engine']: e for e in c['engines']}
    rows.append({'family': c['family'], 'n': c['n'], 'n_prime': c['n_prime'], 'm': c['summands'], 'n_vars': c['n_vars'],
                 'targets': c['targets'], 'repeats': c['repeats'],
                 'ms': {e: statistics.median(v) for e, v in med.items()},
                 'ops_median': {e: statistics.median([o for o in v if o is not None]) if any(o is not None for o in v) else None for e, v in ops.items()},
                 'decided': {e: s['decided'] for e, s in summ.items()}, 'over_budget': {e: s['over_budget'] for e, s in summ.items()},
                 'agrees': {e: s['agrees_with_reference'] for e, s in summ.items()},
                 'next_over_old_median': statistics.median(ratios) if ratios else None,
                 'next_over_old_min': min(ratios) if ratios else None, 'next_over_old_max': max(ratios) if ratios else None,
                 'next_below_old_targets': sum(1 for r in ratios if r < 1), 'paired_targets': len(ratios),
                 'ops_equal_every_target': ops_equal,
                 'ops_ratio_median': statistics.median(ops_ratios) if ops_ratios else None,
                 'ops_ratio_min': min(ops_ratios) if ops_ratios else None, 'ops_ratio_max': max(ops_ratios) if ops_ratios else None,
                 'ops_never_more': ops_not_more,
                 'fes_over_old_median': statistics.median(fes) if fes else None})
json.dump(rows, open(json_out, 'w'), indent=1)
def f(v): return 'n/a' if v is None else (f'{v:,.0f}' if v >= 100 else f'{v:.2f}')
with open(md_out, 'w') as o:
    o.write('Median over targets of the per-target median wall (ms per call, one thread). Paired: median (min–max) over targets of the per-target ratio. `ops` = median per-target word-XOR count (the engines\' counted unit). `dec` = targets decided / targets; every decided answer is checked against the reference `fes-f2`.\n\n')
    o.write('| family | n | vars | `inherited-f4` ms | `-batched` ms | wall batched/old paired | targets batched < old | ops old | ops batched | ops batched/old (min–max) | ops never more | `fes-f2` ms | dec old/batched/fes | agree |\n')
    o.write('|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|--:|:--|:--|\n')
    for r in rows:
        ms = r['ms']; d = r['decided']; t = r['targets']
        ag = all(v is not False for v in r['agrees'].values())
        nr = f"{r['next_over_old_median']:.3f} ({r['next_over_old_min']:.2f}–{r['next_over_old_max']:.2f})" if r['next_over_old_median'] else 'n/a'
        orr = f"{r['ops_ratio_median']:.3f} ({r['ops_ratio_min']:.3f}–{r['ops_ratio_max']:.3f})" if r['ops_ratio_median'] else 'n/a'
        o.write(f"| {r['family']} | {r['n']} | {r['n_vars']} | {f(ms.get(OLD))} | {f(ms.get(NEW))} | {nr} | {r['next_below_old_targets']}/{r['paired_targets']} | "
                f"{f(r['ops_median'].get(OLD))} | {f(r['ops_median'].get(NEW))} | {orr} | {'yes' if r['ops_never_more'] else 'NO'} | {f(ms.get(REF))} | "
                f"{d.get(OLD)}/{d.get(NEW)}/{d.get(REF)} of {t} | {'y' if ag else 'NO'} |\n")
# SVG: ms per call per engine against unknowns, log scale
colors = {OLD: '#ff7f0e', NEW: '#d62728', REF: '#1f77b4'}
W, H, L, B, T = 900, 420, 70, 60, 44
pts = [(r['n_vars'], e, r['ms'][e], r['family']) for r in rows for e in (OLD, NEW, REF) if r['ms'].get(e)]
if pts:
    xs = sorted({p[0] for p in pts}); lo = min(p[2] for p in pts) / 1.5; hi = max(p[2] for p in pts) * 1.5
    def X(v): return L + (W - L - 30) * (v - xs[0]) / max(xs[-1] - xs[0], 1)
    def Y(v): return T + (H - T - B) * (1 - (math.log10(v) - math.log10(lo)) / (math.log10(hi) - math.log10(lo)))
    s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica, Arial, sans-serif" font-size="11">', f'<rect width="{W}" height="{H}" fill="white"/>',
         f'<text x="{L}" y="20" font-size="14" font-weight="bold">One decomposition call: inherited-f4 vs inherited-f4-batched vs fes-f2 (median ms, log scale, one thread)</text>',
         f'<text x="{L}" y="36" fill="#555">circles Koblitz, squares random; Weil descent at m = 2 (unknowns = 2·n\')</text>']
    e10 = math.floor(math.log10(lo))
    while 10 ** e10 <= hi:
        if 10 ** e10 >= lo:
            s.append(f'<line x1="{L}" x2="{W-30}" y1="{Y(10**e10):.1f}" y2="{Y(10**e10):.1f}" stroke="#e5e5e5"/><text x="{L-6}" y="{Y(10**e10)+4:.1f}" text-anchor="end">{10**e10:g} ms</text>')
        e10 += 1
    for x in xs:
        s.append(f'<text x="{X(x):.1f}" y="{H-B+16}" text-anchor="middle">{x}</text>')
    s.append(f'<text x="{(L+W)/2:.0f}" y="{H-B+34}" text-anchor="middle">unknowns</text>')
    for e in (OLD, NEW, REF):
        for fam in ('K', 'R'):
            ser = sorted((p[0], p[2]) for p in pts if p[1] == e and p[3] == fam)
            if len(ser) > 1:
                s.append('<polyline fill="none" stroke="%s" stroke-width="1.4" %s points="%s"/>' % (colors[e], 'stroke-dasharray="4 3"' if fam == 'R' else '', ' '.join(f'{X(a):.1f},{Y(b):.1f}' for a, b in ser)))
            for a, b in ser:
                s.append(f'<circle cx="{X(a):.1f}" cy="{Y(b):.1f}" r="3" fill="{colors[e]}"/>' if fam == 'K' else f'<rect x="{X(a)-3:.1f}" y="{Y(b)-3:.1f}" width="6" height="6" fill="{colors[e]}"/>')
    for i, e in enumerate((OLD, NEW, REF)):
        s.append(f'<rect x="{W-230}" y="{T+6+i*16}" width="10" height="10" fill="{colors[e]}"/><text x="{W-215}" y="{T+15+i*16}">{e}</text>')
    s.append('</svg>')
    open(svg_out, 'w').write('\n'.join(s))
print(open(md_out).read())
