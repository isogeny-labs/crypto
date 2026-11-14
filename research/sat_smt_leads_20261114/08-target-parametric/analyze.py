"""Aggregate raw_l*.jsonl and cores_l*.jsonl into results.json and print tables."""
import json, glob, math, statistics as st

res = dict(tag='[pcc:unverified] pilot', cells={}, rows=[], cores=[], calibration_l6=dict(
    note='single smoke solve, not a measured cell', n=19, l=6, solver='cadical153', arm='FRESH-A', verdict='UNSAT',
    conflicts=112195, decisions=149261, propagations=516899576, secs=105.59))
ARMS = ['cadical153/INC', 'cadical153/FRESH-A', 'cadical153/FRESH-U', 'cadical153/SPEC', 'glucose4/INC', 'glucose4/FRESH-A']
for fn in sorted(glob.glob('raw_l*.jsonl')):
    rows = [json.loads(x) for x in open(fn)]
    meta = rows[0]; rows = rows[1:]
    res['rows'] += rows
    l, n = meta['l'], meta['n']
    cell = dict(meta=meta, targets_done=len(rows), errors=sum(len(r['errors']) for r in rows))
    cell['est_decomposable_abscissae'] = (meta['random_decomposable'] / meta['random_drawn']) * 2 ** (n - 1) if meta['random_drawn'] else None
    cell['heuristic_decomposable_abscissae'] = 2 ** (3 * l) / 12
    cell['reference_counts'] = dict(enumeration_tuples=2 ** (3 * l) / 6, mitm=2 ** (2 * l) / 2 + 2 ** l, linear21_leaf_solves=2 ** l)
    for cls, name in ((False, 'UNSAT'), (True, 'SAT')):
        sub = [r for r in rows if r['decomposable'] == cls]
        cell[name] = {a: dict(n=len(sub), median_conflicts=st.median([r['arms'][a]['conflicts'] for r in sub]),
                              sum_conflicts=sum(r['arms'][a]['conflicts'] for r in sub),
                              median_decisions=st.median([r['arms'][a]['decisions'] for r in sub]),
                              median_propagations=st.median([r['arms'][a]['propagations'] for r in sub]),
                              sum_secs=round(sum(r['arms'][a]['secs'] for r in sub), 2)) for a in ARMS if sub}
    half = len(rows) // 2
    rat = {}
    for sv in ('cadical153', 'glucose4'):
        for nm, sel in (('first_half', rows[:half]), ('second_half', rows[half:]), ('all', rows),
                        ('second_half_UNSAT', [r for r in rows[half:] if not r['decomposable']]),
                        ('second_half_SAT', [r for r in rows[half:] if r['decomposable']]),
                        ('first_half_UNSAT', [r for r in rows[:half] if not r['decomposable']])):
            a = sum(r['arms'][sv + '/INC']['conflicts'] for r in sel); b = sum(r['arms'][sv + '/FRESH-A']['conflicts'] for r in sel)
            rat[sv + '/' + nm] = round(a / b, 3) if b else None
        for nm, sel in (('second_half', rows[half:]),):
            a = sum(r['arms'][sv + '/INC']['propagations'] for r in sel); b = sum(r['arms'][sv + '/FRESH-A']['propagations'] for r in sel)
            rat[sv + '/second_half_propagations'] = round(a / b, 3) if b else None
    cell['inc_over_freshA'] = rat
    cell['reuse_curve_cadical'] = [round(r['arms']['cadical153/INC']['conflicts'] / max(1, r['arms']['cadical153/FRESH-A']['conflicts']), 3) for r in rows]
    cell['reuse_curve_glucose'] = [round(r['arms']['glucose4/INC']['conflicts'] / max(1, r['arms']['glucose4/FRESH-A']['conflicts']), 3) for r in rows]
    cell['reuse_curve_decomposable_flags'] = [r['decomposable'] for r in rows]
    cell['first_core_sizes_INC_cadical'] = [r['arms']['cadical153/INC'].get('core') for r in rows if not r['decomposable']]
    # (b) learnt clause target-literal histogram (glucose FRESH-A proofs)
    for cls, name in ((False, 'UNSAT'), (True, 'SAT')):
        hr = {}; hc = {}; nl = 0
        for r in rows:
            if r['decomposable'] != cls:
                continue
            p = r['arms']['glucose4/FRESH-A']['proof']
            nl += p['learnts']
            for k, v in p['hist_r'].items():
                hr[int(k)] = hr.get(int(k), 0) + v
            for k, v in p['hist_cone'].items():
                hc[int(k)] = hc.get(int(k), 0) + v
        if nl:
            def q(h, qq):
                acc = 0
                for k in sorted(h):
                    acc += h[k]
                    if acc >= qq * nl:
                        return k
            cell['learnt_' + name] = dict(learnts=nl, share_zero_r=round(hr.get(0, 0) / nl, 4), share_zero_cone=round(hc.get(0, 0) / nl, 4),
                                          share_le2_r=round(sum(v for k, v in hr.items() if k <= 2) / nl, 4),
                                          median_r=q(hr, .5), p90_r=q(hr, .9), max_r=max(hr), median_cone=q(hc, .5), p90_cone=q(hc, .9),
                                          expected_transfer_weight=round(sum(v * 2.0 ** -k for k, v in hr.items()) / nl, 4), hist_r=dict(sorted(hr.items())))
    res['cells'][str(l)] = cell
for fn in sorted(glob.glob('cores_l*.jsonl')):
    cr = [json.loads(x) for x in open(fn)]
    res['cores'] += cr
    if cr and str(cr[0]['l']) in res['cells']:
        c = res['cells'][str(cr[0]['l'])]
        mins = [min(o['min_core'] for o in r['orders'].values()) for r in cr]
        c['min_cores'] = dict(n=cr[0]['n'], targets=len(cr), first_core=[r['first_core'] for r in cr], minimal=mins,
                              smallest=min(mins), smallest_over_n=round(min(mins) / cr[0]['n'], 3))
# slopes
ls = sorted(int(k) for k in res['cells'])
def fit(xs, ys):
    mx = sum(xs) / len(xs); my = sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
res['slopes_bits_per_l_UNSAT_median_conflicts'] = {}
if len(ls) >= 2:
    for a in ARMS:
        res['slopes_bits_per_l_UNSAT_median_conflicts'][a] = round(fit(ls, [math.log2(res['cells'][str(l)]['UNSAT'][a]['median_conflicts']) for l in ls]), 3)
    res['slopes_bits_per_l_UNSAT_median_conflicts']['sizes_l'] = ls
import os
if os.path.exists('core_null.json'):
    res['core_null'] = json.load(open('core_null.json'))
json.dump(res, open('results.json', 'w'), indent=1)
for l in ls:
    c = res['cells'][str(l)]
    print('== l', l, 'n', c['meta']['n'], 'targets', c['targets_done'], 'errors', c['errors'], 'est dec abscissae', c['est_decomposable_abscissae'], 'refs', c['reference_counts'])
    for name in ('UNSAT', 'SAT'):
        for a in ARMS:
            print('  ', name, a, c[name][a])
    print('   ratios', c['inc_over_freshA'])
    print('   curve cad', c['reuse_curve_cadical']); print('   curve g4 ', c['reuse_curve_glucose']); print('   flags', ''.join('S' if x else 'U' for x in c['reuse_curve_decomposable_flags']))
    print('   first cores', c['first_core_sizes_INC_cadical'], 'min cores', c.get('min_cores'))
    for name in ('UNSAT', 'SAT'):
        print('   learnt', name, c.get('learnt_' + name))
print('slopes', res['slopes_bits_per_l_UNSAT_median_conflicts'])
