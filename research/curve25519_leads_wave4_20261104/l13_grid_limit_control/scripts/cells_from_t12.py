#!/usr/bin/env python3
"""L13 design cells, computed from EXISTING data only (T12's 16 complete chunks), before any L13 run.

Model (PROTOCOL.md section 2): on the same base and the same algorithm seed the F4, grid and
mitm arms walk the same trial sequence, so GAE(grid) = GAE(F4) and GAE(mitm) = GAE(F4) + 506
(the negation-folded pair table: sum over the 22 abscissa classes of 2*(22-i) additions).
Per target the grid costs n^2 + 7n F_p multiplications for n = |V| = 2^t = 32 (pkm_semaev.rs
`grid`, m = 2: 6 per row, 2 per unordered pair including the diagonal) = 1,248; the mitm
probe costs one lookup. S(w) = (GAE + w * unpriced) / sqrt(r).
Usage: cells_from_t12.py <T12 lead dir> <out json>
"""
import json, glob, os, statistics as st, sys
T12, out = sys.argv[1], sys.argv[2]
dirs = sorted(glob.glob(os.path.join(T12, 'ecbench/chunks/c[0-9][0-9]')))
recs = [json.loads(l) for d in dirs for l in open(os.path.join(d, 'records.jsonl'))]
sq = {round(x['cost']['total_gae'] / x['cost']['s'], 6) for x in recs if x['cost']['s']}
assert len(sq) == 1; sq = sq.pop()
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pooled_compare import unpriced  # the patched copy; same resolution as ecbench band.rs
def ph(x, n): return next(p for p in x['phases'] if p['name'] == n)
N_V, T_ABS, COLS = 32, 22, 22
GRID_PER_TARGET = N_V * N_V + 7 * N_V          # 1,248
TABLE_ADDS = sum(2 * (T_ABS - i) for i in range(T_ABS))  # 506
W = [0.0, 0.1, 1.0]
cells = {'inputs': {'t12_chunks': len(dirs), 'sqrt_r': sq, 'n_V': N_V, 'grid_fp_mults_per_target': GRID_PER_TARGET,
                    'mitm_table_additions': TABLE_ADDS, 'weights': W}, 'arms': {}}
F4 = [x for x in recs if x['arm'] == 'pkm-iso-t5-g4-s1307']
PA = [x for x in recs if x['arm'] == 'proj-a']
RN = [x for x in recs if x['arm'] == 'rho-neg']
def S(gae, nat): return [(gae + w * nat) / sq for w in W]
def summarise(name, R):
    gae = [x['cost']['total_gae'] for x in R]; nat = [unpriced(x) for x in R]
    d = {'runs': len(R), 'mean_gae': st.fmean(gae), 'mean_unpriced': st.fmean(nat),
         'mean_S': [st.fmean([(g + w * n) / sq for g, n in zip(gae, nat)]) for w in W]}
    if R[0]['arm'] != 'rho-neg':
        tr = [ph(x, 'relations')['native']['trials'] for x in R]; rel = [ph(x, 'relations')['native']['relations'] for x in R]
        d.update(mean_trials=st.fmean(tr), sd_trials=st.pstdev(tr), mean_relations=st.fmean(rel),
                 trials_per_relation=sum(tr) / sum(rel),
                 phase_gae={p: st.fmean([ph(x, p)['gae'] for x in R]) for p in ('factor_base', 'oracle_setup', 'relations', 'verify')})
    if R[0]['arm'].startswith('pkm'):
        d['mean_solver_ops'] = st.fmean([x['solver']['ops'] for x in R])
        d['solver_ops_per_trial'] = sum(x['solver']['ops'] for x in R) / sum(ph(x, 'relations')['native']['trials'] for x in R)
        d['mean_grid_mults_on_consistent'] = st.fmean([ph(x, 'relations')['native']['pkm_grid_fp_mults_uncharged'] for x in R])
        d['mean_consistent_systems'] = st.fmean([ph(x, 'relations')['native']['pkm_consistent_systems'] for x in R])
        d['grid_mults_per_consistent_system'] = sum(ph(x, 'relations')['native']['pkm_grid_fp_mults_uncharged'] for x in R) / sum(ph(x, 'relations')['native']['pkm_consistent_systems'] for x in R)
    cells['arms'][name] = d; return d
f4 = summarise('pkm-iso-t5-g4-s1307 (T12 measured)', F4)
pa = summarise('proj-a (T12 measured)', PA)
rn = summarise('rho-neg (T12 measured)', RN)
# Modelled same-base arms, per run, from the F4 run's own trials (paired by construction).
def common_native(x):  # everything unpriced in the F4 run except the oracle's own terms
    tot = 0
    for n in x['cost']['unpriced']:
        if n.startswith('solver_') or n.startswith('pkm_'): continue
        v = 0
        if n in x.get('counters', {}): v += x['counters'][n]
        for p in x['phases']:
            v += p.get('native', {}).get(n, 0) if n in p.get('native', {}) else p.get('native', {}).get(n[:-len('_uncharged')], 0)
        tot += v
    return tot
grid_S, mitm_S = [], []
for x in F4:
    tr = ph(x, 'relations')['native']['trials']; g = x['cost']['total_gae']; cn = common_native(x)
    grid_S.append(S(g, cn + tr * GRID_PER_TARGET)); mitm_S.append(S(g + TABLE_ADDS, cn + tr))
cells['arms']['grid-iso-t5-s1307 (modelled)'] = {'mean_S': [st.fmean(v[i] for v in grid_S) for i in range(3)],
    'model': 'GAE = GAE(F4 run); unpriced = common natives + trials * 1248'}
cells['arms']['mitm-iso-t5-s1307 (modelled)'] = {'mean_S': [st.fmean(v[i] for v in mitm_S) for i in range(3)],
    'model': 'GAE = GAE(F4 run) + 506; unpriced = common natives + trials lookups'}
def ratio(a, b): return [b[i] / a[i] for i in range(3)]
A = cells['arms']
cells['modelled_ratios_b_over_a'] = {
    'grid / F4': ratio(f4['mean_S'], A['grid-iso-t5-s1307 (modelled)']['mean_S']),
    'mitm / grid': ratio(A['grid-iso-t5-s1307 (modelled)']['mean_S'], A['mitm-iso-t5-s1307 (modelled)']['mean_S']),
    'grid / proj-a': ratio(pa['mean_S'], A['grid-iso-t5-s1307 (modelled)']['mean_S']),
    'mitm / proj-a': ratio(pa['mean_S'], A['mitm-iso-t5-s1307 (modelled)']['mean_S']),
    'grid / rho-neg': ratio(rn['mean_S'], A['grid-iso-t5-s1307 (modelled)']['mean_S']),
    'F4 / grid (how far above its grid limit)': ratio(A['grid-iso-t5-s1307 (modelled)']['mean_S'], f4['mean_S']),
    'F4 / rho-neg (T12)': ratio(rn['mean_S'], f4['mean_S']),
}
cells['per_trial_unpriced'] = {'F4 solver ops + system mults per trial': f4['solver_ops_per_trial'] + st.fmean([ph(x,'relations')['native']['pkm_system_fp_mults_uncharged'] for x in F4]) / f4['mean_trials'],
                               'grid mults per trial': GRID_PER_TARGET, 'mitm lookups per trial': 1}
# Positive control: the patched pooled_compare reproduces T12's frozen pooled_compare.jsonl.
from pooled_compare import pooled
ref = [json.loads(l) for l in open(os.path.join(T12, 'ecbench/pooled_compare.jsonl'))]
worst = 0.0
for r in ref:
    got = pooled(r['a'], r['b'], r['weight'], dirs)
    worst = max(worst, abs(got['ratio'] - r['ratio']) / r['ratio'], abs(got['ci95'][0] - r['ci95'][0]) / r['ci95'][0], abs(got['ci95'][1] - r['ci95'][1]) / r['ci95'][1])
cells['positive_control'] = {'patched_pooled_compare_vs_T12_jsonl_max_rel_diff': worst, 'pairs_checked': len(ref)}
json.dump(cells, open(out, 'w'), indent=1)
print(json.dumps(cells, indent=1))
