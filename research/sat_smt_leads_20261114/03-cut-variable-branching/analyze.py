import json, glob, sys, statistics as st, math
S = sys.argv[1]; rows = []
for f in sorted(glob.glob(S + '/*.log')):
    for line in open(f):
        try: rows.append(json.loads(line))
        except Exception: pass
calib = [
 dict(n=7, l=2, xR=5, kind='nondecomposable', arm='D', result='UNSAT', conflicts=1226, decisions=1536, propagations=72460, cap_conflicts=10),
 dict(n=7, l=2, xR=5, kind='nondecomposable', arm='C', result='UNSAT', conflicts=1364, decisions=1409, propagations=80225, cut_decisions=137),
 dict(n=13, l=3, seed=999, kind='nondecomposable', arm='D', result='CAP', conflicts=150000, decisions=288737, propagations=22151698),
 dict(n=13, l=3, seed=999, kind='nondecomposable', arm='C', result='CAP', conflicts=150000, decisions=161939, propagations=31059498, cut_decisions=12935),
 dict(n=13, l=4, seed=999, kind='nondecomposable', arm='D', result='CAP', conflicts=150000, decisions=302488, propagations=26044172),
 dict(n=13, l=4, seed=999, kind='nondecomposable', arm='C', result='CAP', conflicts=150001, decisions=182877, propagations=33744134, cut_decisions=12214),
 dict(n=17, l=3, seed=999, kind='nondecomposable', arm='D', result='UNSAT', conflicts=73502, decisions=122368, propagations=14490298),
 dict(n=17, l=3, seed=999, kind='nondecomposable', arm='C', result='CAP', conflicts=150001, decisions=160783, propagations=24199819, cut_decisions=49932),
 dict(note='n=19 l=3 planted validation instance: arm D did not finish in >100 s wall; interrupt() had no effect; process killed'),
]
cells = {}
for r in rows: cells.setdefault((r['n'], r['kind'], r['l'], r['arm']), []).append(r)
summ = []; 
for k in sorted(cells):
    rs = cells[k]; c = [r['conflicts'] for r in rs]; cen = sum(r['result'] == 'CAP' for r in rs)
    summ.append(dict(n=k[0], kind=k[1], l=k[2], arm=k[3], runs=len(rs), censored=cen, median_conflicts=st.median(c), min_conflicts=min(c), max_conflicts=max(c),
                     log2_median=round(math.log2(st.median(c)), 2), median_decisions=st.median(r['decisions'] for r in rs),
                     median_propagations=st.median(r['propagations'] for r in rs), median_cut_decisions=st.median(r['cut_decisions'] for r in rs),
                     median_wall_s=st.median(r['wall_s'] for r in rs), U_size=rs[0]['U_size'], oracle_quad_solves=rs[0]['oracle_quad_solves']))
def fit(pts):
    if len(pts) < 2: return None
    xs, ys = zip(*pts); mx = sum(xs) / len(xs); my = sum(ys) / len(ys)
    return round(sum((x - mx) * (y - my) for x, y in pts) / sum((x - mx) ** 2 for x in xs), 2)
slopes = {}
for n in sorted({s['n'] for s in summ}):
    for kind in ('nondecomposable', 'decomposable'):
        for arm in ('D', 'Dp', 'C', 'C-lean', 'C-keep'):
            pts = [(s['l'], s['log2_median']) for s in summ if (s['n'], s['kind'], s['arm']) == (n, kind, arm) and s['censored'] * 2 <= s['runs']]
            if pts: slopes[f'n={n} {kind} {arm}'] = dict(bits_per_l=fit(pts), l_used=[p[0] for p in pts],
                         bits_per_l_last_step=(round(pts[-1][1] - pts[-2][1], 2) if len(pts) > 1 else None))
refs = {l: dict(enum_2_3l_over_24=2 ** (3 * l) / 24, mitm_2_2l=2 ** (2 * l), linear_leaf_2_2l=2 ** (2 * l)) for l in (1, 2, 3, 4)}
json.dump(dict(tag='[pcc:unverified] pilot', rows=rows, calibration=calib, cell_summary=summ, slopes=slopes, reference_counts=refs), open('results.json', 'w'), indent=1)
print('rows', len(rows))
for s in summ: print(s['n'], s['kind'][:6], 'l', s['l'], f"{s['arm']:7}", 'runs', s['runs'], 'cens', s['censored'], 'med', s['median_conflicts'], 'log2', s['log2_median'], 'min', s['min_conflicts'], 'max', s['max_conflicts'], 'cutdec', s['median_cut_decisions'], 'U', s['U_size'], 'wall', s['median_wall_s'])
for k, v in slopes.items(): print(k, v)
print('lift', {x: sum(r.get('lift') == x for r in rows) for x in ('lifted', 'nonrational', 'rational_no_sign')})
