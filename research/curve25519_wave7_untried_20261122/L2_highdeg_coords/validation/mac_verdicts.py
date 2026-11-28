"""Validator: recompute P5a-g, P6, C1, C2, consistency and the README proxy table from
data/macaulay_k*.json, data/engine_k*_T*.json (own code; thresholds from PROTOCOL section 5)."""
import json, statistics as st
from collections import defaultdict

LEAD = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L2_highdeg_coords'
DENSE = ('v', 'q', 'uv', 'jac6')
FS = ('x', 'ye', 'xe', 'v', 'q', 'xeye', 'jac6', 'uv')

recs = []
for k in (16, 18, 20):
    for r in json.load(open(f'{LEAD}/data/macaulay_k{k}.json')):
        recs.append(r)
out = {'n_records': len(recs), 'skipped': [(r['k'], r['f'], r['T'], r['target_log']) for r in recs if 'skip' in r]}
R = [r for r in recs if 'e' in r]


def epred(f, T):
    if f in DENSE:
        return 2 * T
    if f in ('xe', 'xeye'):
        return 8
    return 4


mism = [(r['k'], r['f'], r['T'], r['kind'], r['e'], epred(r['f'], r['T'])) for r in R if r['e'] != epred(r['f'], r['T'])]
out['P5a_pass'] = not mism
agg = defaultdict(set)
for (k, f, T, kind, e, pe) in mism:
    agg[(f, T, pe)].add((k, e))
out['P5a_mismatch_groups'] = {f'{f}/T{T}/pred{pe}': sorted(v) for (f, T, pe), v in agg.items()}
out['P5a_mismatch_count'] = len(mism)
out['e_table'] = {f: {T: sorted({r['e'] for r in R if r['f'] == f and r['T'] == T}) for T in (4, 5, 6)} for f in FS}
ref = [r for r in R if 'M2' in r and r['M2']['mode'] == 'ref']
root = [r for r in R if 'M2' in r and r['M2']['mode'] == 'root']
out['n_ref'] = len(ref); out['n_root'] = len(root)
out['no_M2_but_T_le_5'] = [(r['k'], r['f'], r['T'], r['kind'], r['roots']) for r in R if r['T'] <= 5 and 'M2' not in r]
out['P5b_pass'] = all(r['M2']['D_ref'] == r['n'] for r in ref if r['f'] in DENSE)
xs = {T: [r['M2']['D_ref'] for r in ref if r['f'] == 'x' and r['T'] == T] for T in (4, 5)}
out['P5c'] = {T: dict(n=len(v), le=sum(1 for d in v if d is not None and d <= (6 if T == 4 else 8)), values=sorted(set(v))) for T, v in xs.items()}
out['P5c_pass'] = all(x['le'] >= 0.9 * x['n'] and x['n'] > 0 for x in out['P5c'].values())
med = defaultdict(list)
for r in ref:
    med[(r['k'], r['f'], r['T'])].append(r['M2']['hist'][-1]['muladds'])
med = {kk: st.median(v) for kk, v in med.items()}
out['P5d_ratios'] = {f'{k}/{f}/T{T}': med[(k, f, T)] / med[(k, 'x', T)] for k in (16, 18, 20) for f in ('v', 'q') for T in (4, 5)}
out['P5d_pass'] = all(v >= 1.0 for v in out['P5d_ratios'].values())
dref = lambda f, k, T: st.median([r['M2']['D_ref'] for r in ref if r['f'] == f and r['k'] == k and r['T'] == T])
out['P5e'] = {k: (dref('ye', k, 5), dref('x', k, 5)) for k in (16, 18, 20)}
out['P5e_pass'] = all(abs(a - b) <= 1 for a, b in out['P5e'].values())
out['P5f'] = {f'{f}/{k}': (dref(f, k, 5), dref('x', k, 5)) for f in ('xe', 'xeye') for k in (16, 18, 20)}
out['P5f_pass'] = all(a >= b + 1 for a, b in out['P5f'].values())
m1 = [r for r in R if 'M1' in r]
out['P5g_pooled'] = f"{sum(r['M1']['d_mult'] == r['n'] for r in m1)}/{len(m1)}"
out['P5g_per_arm'] = {f: f"{sum(r['M1']['d_mult'] == r['n'] for r in m1 if r['f'] == f)}/{sum(1 for r in m1 if r['f'] == f)}" for f in FS}
out['P5g_values'] = {f: sorted({(r['T'], r['n'] - r['M1']['d_mult']) for r in m1 if r['f'] == f}) for f in FS}
out['P5g_pass_pooled'] = sum(r['M1']['d_mult'] == r['n'] for r in m1) >= 0.9 * len(m1)
out['P5g_pass_every_arm'] = all(sum(r['M1']['d_mult'] == r['n'] for r in m1 if r['f'] == f) >= 0.9 * sum(1 for r in m1 if r['f'] == f) for f in FS)
# P6 and full cost-per-relation table
bc = {}
for r in R:
    bc.setdefault((r['k'], r['f'], r['T']), set()).add(r['base_cov'])
out['base_cov_unique_per_cell'] = all(len(v) == 1 for v in bc.values())
bc = {kk: next(iter(v)) for kk, v in bc.items()}
out['base_cov_r_units'] = {f'{k}/{f}/T{T}': v for (k, f, T), v in sorted(bc.items()) if T in (4, 5)}
cpr = {kk: med[kk] / bc[kk] for kk in med if bc.get(kk)}
rat = {f'{k}/{f}/T{T}': cpr[(k, f, T)] / cpr[(k, 'x', T)] for (k, f, T) in cpr if f != 'x'}
out['cost_per_relation_ratio_vs_x'] = dict(sorted(rat.items()))
out['P6_ratios'] = {f'{k}/{f}': rat[f'{k}/{f}/T5'] for k in (16, 18, 20) for f in ('v', 'q')}
out['P6_pass'] = all(v >= 1.0 for v in out['P6_ratios'].values())
out['dense_T5_ratio_range'] = [min(rat[f'{k}/{f}/T5'] for k in (16, 18, 20) for f in DENSE), max(rat[f'{k}/{f}/T5'] for k in (16, 18, 20) for f in DENSE)]
out['dense_T5_below_1'] = {kk: v for kk, v in rat.items() if kk.endswith('T5') and kk.split('/')[1] in DENSE and v < 1}
out['dense_T4_ratio_range'] = [min(rat[f'{k}/{f}/T4'] for k in (16, 18, 20) for f in DENSE), max(rat[f'{k}/{f}/T4'] for k in (16, 18, 20) for f in DENSE)]
out['lead_positive_M2'] = sum(1 for k in (16, 18, 20) if any(rat[f'{k}/{f}/T5'] <= 0.67 for f in ('v', 'q', 'uv', 'jac6', 'xe', 'xeye'))) >= 2
out['min_T5_ratio_any_f'] = min((v, kk) for kk, v in rat.items() if kk.endswith('T5'))
out['C1_pass'] = all(r['terms'] == {4: 121, 5: 256, 6: 484}[r['T']] for r in R if r['f'] == 'x')
out['consistency_all'] = all(r['census_agrees'] for r in R)
out['rank_check'] = [r['M2']['rank_at_n_check'] for r in R if 'M2' in r and r['M2']['rank_at_n_check'] is not None]
out['rank_check_all_true'] = all(out['rank_check'])
# README proxy table
tab = {}
for f in FS:
    row = {}
    for T in (4, 5):
        rr = [r for r in ref if r['f'] == f and r['T'] == T]
        ro = [r for r in root if r['f'] == f and r['T'] == T]
        row[f'D_ref_T{T}'] = sorted({r['M2']['D_ref'] for r in rr})
        row[f'D_root_T{T}_rootmode'] = sorted({r['M2']['D_root'] for r in ro})
        row[f'D_root_T{T}_refmode'] = sorted({r['M2']['D_root'] for r in rr})
        row[f'med_muladds_T{T}'] = sorted({med[(k, f, T)] for k in (16, 18, 20)})
    row['M1_n_minus_dmult'] = sorted({(r['T'], r['n'] - r['M1']['d_mult']) for r in m1 if r['f'] == f})
    tab[f] = row
out['readme_proxy_table'] = tab
# C2 engine
eng = {}
for k in (16, 18, 20):
    for T in (4, 5):
        d = json.load(open(f'{LEAD}/data/engine_k{k}_T{T}.json'))
        eng[f'{k}/T{T}'] = dict(solving_degree_max=d['solving_degree_max'], poly_terms=d['poly_terms_per_specialisation'],
                                targets_done=d['targets_done'], f4_muladds_per_target=d['muladds_per_target'],
                                M2_median_Dref_x=dref('x', k, T), proxy_over_engine=med[(k, 'x', T)] / d['muladds_per_target'])
out['C2'] = eng
out['C2_pass'] = all(abs(v['solving_degree_max'] - v['M2_median_Dref_x']) <= 1 for v in eng.values())
json.dump(out, open('mac_verdicts.json', 'w'), indent=1, default=str)
for kk, v in out.items():
    if kk not in ('base_cov_r_units', 'rank_check'):
        print(kk, ':', json.dumps(v, default=str)[:1500])
