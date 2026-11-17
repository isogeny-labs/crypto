#!/usr/bin/env python3
"""L12 engine analysis (PROTOCOL.md, section 4, frozen before data).

Reads data/engine/<cell>_tr{0,1}.json (f4cost JSON lines), decides P1-P6 by
the preregistered numeric rules, writes data/engine/summary.json and
data/engine/GATE ("open" when rho_win <= 0.5, else "closed").
Usage: analyze_engine.py <lead dir>
"""
import json, os, sys
lead = sys.argv[1]
eng = os.path.join(lead, 'data', 'engine')
# cell -> (plain mpt from T10 data/cells, columns, covered, G)
T10 = {'t5_g4_s1307': (52816.0, 22, 210, 4), 't5_g3_s1307': (177112.0, 22, 210, 3),
       't6_g4_s1308': (710095.08, 36, 498, 4), 't6_g4_s0': (710094.0, 25, 172, 4)}
R = 131203
IDENT = ['columns', 'covered_points', 'hits', 'hits_verified', 'hits_exact', 'undecided',
         'capped', 'targets_done', 'planted_done', 'planted_hits', 'planted_exact']
def load(c, tr):
    p = os.path.join(eng, f'{c}_tr{tr}.json')
    if not os.path.exists(p): return None
    txt = open(p).read().strip().splitlines()
    try: return json.loads(txt[-1])
    except Exception: return None
out = {'cells': {}, 'predictions': {}}
p1 = p3 = p4 = p5 = p6 = True; rhos = {}
for c, (mpt0, cols, cov, G) in T10.items():
    a, b = load(c, 0), load(c, 1)
    cell = {'plain_present': a is not None, 'trace_present': b is not None}
    if a is None or b is None:
        out['cells'][c] = cell; p1 = p3 = p4 = p5 = p6 = False; continue
    diffs = {k: (a.get(k), b.get(k)) for k in IDENT if a.get(k) != b.get(k)}
    ra, rb = a['solver_extra'].get('refutations'), b['solver_extra'].get('refutations')
    if ra != rb: diffs['refutations'] = (ra, rb)
    cell['verdict_diffs'] = diffs
    cell['planted_complete'] = (b.get('planted_done') == b.get('planted_hits') == a.get('planted_hits')) and b.get('planted_done', 0) > 0
    if diffs or not cell['planted_complete']: p1 = False
    rho = b['muladds_per_target'] / a['muladds_per_target']; rhos[c] = rho
    nat = b.get('native', {}); setup = b.get('setup_native', {})
    replays = nat.get('pkm_trace_replays', 0); fb = nat.get('pkm_trace_fallbacks', 0)
    cell.update(dict(mpt_plain=a['muladds_per_target'], mpt_trace=b['muladds_per_target'], rho=rho,
                     learn_muladds=b.get('trace_learn_muladds'), replays=replays, refuted=nat.get('pkm_trace_refuted', 0),
                     fallbacks=fb, untraced=nat.get('pkm_trace_untraced_branch_runs', 0),
                     branches=setup.get('pkm_trace_branches'), branches_learned=setup.get('pkm_trace_branches_learned'),
                     learn_runs=setup.get('pkm_trace_learn_runs'), selfcheck=setup.get('pkm_trace_selfcheck_refuted'),
                     fallback_rate=(fb / replays if replays else None),
                     E0_plain=cols * R * a['muladds_per_target'] / cov, E0_trace=cols * R * b['muladds_per_target'] / cov,
                     learn_over_E0=(b.get('trace_learn_muladds', 0) / (cols * R * a['muladds_per_target'] / cov)),
                     t10_mpt=mpt0, t10_rel_dev=abs(a['muladds_per_target'] - mpt0) / mpt0,
                     census_match=(a['columns'] == cols and a['covered_points'] == cov)))
    if rho > 0.5: p3 = False
    if replays == 0 or cell['fallback_rate'] > 0.01 or cell['untraced'] != 0 or cell['branches_learned'] != 2 ** G: p4 = False
    if not (b.get('trace_learn_muladds', 0) <= 8 * a['muladds_per_target'] and cell['learn_over_E0'] <= 0.01): p5 = False
    if cell['t10_rel_dev'] > 1e-6 or not cell['census_match']: p6 = False
    out['cells'][c] = cell
rw = rhos.get('t5_g4_s1307')
seed_dev = abs(rhos['t6_g4_s1308'] - rhos['t6_g4_s0']) if ('t6_g4_s1308' in rhos and 't6_g4_s0' in rhos) else None
out['predictions'] = {
    'P1_verdict_identity': p1,
    'P2_rho_win_le_0.5': (rw is not None and rw <= 0.5), 'rho_win': rw,
    'P3_all_cells_le_0.5_and_seed_indep': (p3 and seed_dev is not None and seed_dev <= 0.02), 'rho_seed_dev_t6': seed_dev,
    'P4_fallbacks_le_1pct_all_learned': p4, 'P5_learning_bounded': p5, 'P6_trace0_replicates_T10': p6,
    'rhos': rhos}
gate = 'open' if out['predictions']['P2_rho_win_le_0.5'] else 'closed'
out['gate'] = gate
json.dump(out, open(os.path.join(eng, 'summary.json'), 'w'), indent=1)
open(os.path.join(eng, 'GATE'), 'w').write(gate + '\n')
print(json.dumps(out['predictions'], indent=1)); print('GATE', gate)
