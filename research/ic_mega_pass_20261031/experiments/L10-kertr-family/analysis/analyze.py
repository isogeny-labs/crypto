"""raw/*.jsonl -> RESULTS.json"""
import json, os, glob, collections
from scipy.stats import beta
H = os.path.dirname(os.path.abspath(__file__)); RAW = os.path.join(H, '..', 'raw'); EXP = os.path.join(H, '..', '..')
def cp(x, n):
    lo = 0.0 if x == 0 else beta.ppf(0.025, x, n - x + 1); hi = 1.0 if x == n else beta.ppf(0.975, x + 1, n - x)
    return [round(lo, 4), round(hi, 4)]
def load(fn): return [json.loads(l) for l in open(fn)]
res = {}
for fn in sorted(glob.glob(os.path.join(RAW, '[TRA]_n*.jsonl'))):
    tag = os.path.basename(fn)[:-6]; R = load(fn)
    U = [r for r in R if r['nsol'] == 0 and 'one' in r]
    fail = [r for r in U if r['one'] == 0]
    res[tag] = dict(draws=len(R), unsat=len(U), sat_frac=round(1 - sum(r['nsol'] == 0 for r in R) / len(R), 3),
                    nsol_hist=dict(sorted(collections.Counter(r['nsol'] for r in R).items())),
                    rejects_total=sum(r.get('rejects', 0) for r in R),
                    span_rank=dict(collections.Counter(r['span_rank'] for r in R)), span_low=dict(collections.Counter(r['span_low'] for r in R)),
                    fail_d4=len(fail), fail_rate=round(len(fail) / len(U), 4) if U else None, fail_CP95=cp(len(fail), len(U)) if U else None,
                    low4=dict(collections.Counter(r['low4'] for r in U)), dim4=dict(collections.Counter(r['dim4'] for r in U)),
                    unsat_seeds=[r['s'] for r in U], fail_seeds=[r['s'] for r in fail],
                    cpu_closure=round(sum(r['cpu'] for r in U), 1))
# A baselines imported from N1 raw and L8 RESULTS
imp = {}
for n in (29, 31):
    k = (n + 1) // 2; fn = os.path.join(EXP, 'N1-gf2-d4-profile', 'raw', 'n%d_k%d_d4.jsonl' % (n, k))
    U = [r for r in load(fn) if r['arm'] == 'A' and r['nsol'] == 0 and 'one' in r]
    f = sum(r['one'] == 0 for r in U); imp['A_n%d' % n] = dict(source=os.path.relpath(fn, EXP), unsat=len(U), fail_d4=f, fail_CP95=cp(f, len(U)),
                                                                 low4=dict(collections.Counter(r['low4'] for r in U)))
L8 = json.load(open(os.path.join(EXP, 'L8-n33-control', 'RESULTS.json')))['n33']['A']
imp['A_n33'] = dict(source='L8-n33-control/RESULTS.json', unsat=L8['unsat'], fail_d4=L8['unsat'] - L8['one_in_W4'],
                    fail_CP95=cp(L8['unsat'] - L8['one_in_W4'], L8['unsat']), low4=L8['low4'])
named = load(os.path.join(RAW, 'named.jsonl'))
d5 = [load(f)[-1] for f in sorted(glob.glob(os.path.join(RAW, 'd5_*.jsonl')))]
json.dump(dict(families=res, imported_A=imp, named_validation=named, d5=d5), open(os.path.join(H, '..', 'RESULTS.json'), 'w'), indent=1)
for k, v in res.items(): print(k, {x: v[x] for x in ('draws', 'unsat', 'fail_d4', 'fail_CP95', 'low4', 'span_rank', 'span_low', 'cpu_closure')})
for k, v in imp.items(): print(k, v)
print('named', [(r['name'], r['d'], r['match']) for r in named]); print('d5', d5)
