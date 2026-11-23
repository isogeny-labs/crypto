#!/usr/bin/env python3
"""L13 per-pair identity checks on the pooled chunks (PROTOCOL.md P2, P4): for every (workload, round)
the three same-base arms (F4 winner, grid, mitm) share the algorithm seed, so they should walk the
same trial sequence: identical `trials` and `relations`; total_gae(grid) == total_gae(F4) exactly;
total_gae(mitm) - total_gae(grid) within 506 +- 2*relations. Also sums the oracle counters and
checks the grid arm's unpriced bookkeeping (solver.ops == 0, pkm_grid_fp_mults == 1248 * trials).
Usage: identity_check.py dir1 [dir2 ...] -> JSON on stdout.
"""
import json, os, sys
F4, GRID, MITM = 'pkm-iso-t5-g4-s1307', 'grid-iso-t5-s1307', 'mitm-iso-t5-s1307'
TABLE_ADDS, GRID_PER_TARGET = 506, 1248
recs = []
for d in sys.argv[1:]:
    assert json.load(open(os.path.join(d, 'session.json'))).get('status') == 'complete', d
    recs += [json.loads(l) for l in open(os.path.join(d, 'records.jsonl'))]
def ph(x, n): return next(p for p in x['phases'] if p['name'] == n)
by = {}
for x in recs: by.setdefault((x['workload']['workload_id'], x['round']), {})[x['arm']] = x
out = {'pairs': 0, 'seeds_paired': True, 'trials_identical': 0, 'relations_identical': 0, 'gae_grid_eq_f4': 0,
       'mitm_minus_grid_in_band': 0, 'grid_solver_ops_zero': 0, 'grid_mults_eq_1248_trials': 0, 'mitm_minus_grid_values': [],
       'details': []}
for k, arms in sorted(by.items()):
    if not all(a in arms for a in (F4, GRID, MITM)): continue
    out['pairs'] += 1
    f, g, m = arms[F4], arms[GRID], arms[MITM]
    if len({f['algorithm_seed'], g['algorithm_seed'], m['algorithm_seed']}) != 1: out['seeds_paired'] = False
    tr = [ph(x, 'relations')['native']['trials'] for x in (f, g, m)]
    rl = [ph(x, 'relations')['native']['relations'] for x in (f, g, m)]
    ga = [x['cost']['total_gae'] for x in (f, g, m)]
    out['trials_identical'] += len(set(tr)) == 1
    out['relations_identical'] += len(set(rl)) == 1
    out['gae_grid_eq_f4'] += ga[1] == ga[0]
    dmg = ga[2] - ga[1]; out['mitm_minus_grid_values'].append(dmg)
    out['mitm_minus_grid_in_band'] += abs(dmg - TABLE_ADDS) <= 2 * rl[1]
    gs = g.get('solver') or {}
    out['grid_solver_ops_zero'] += (gs.get('ops', 0) == 0)
    out['grid_mults_eq_1248_trials'] += ph(g, 'relations')['native'].get('pkm_grid_fp_mults_uncharged', -1) == GRID_PER_TARGET * tr[1]
    out['details'].append({'workload': k[0], 'round': k[1], 'trials': tr, 'relations': rl, 'total_gae': ga,
                           'mitm_oracle_setup_gae': ph(m, 'oracle_setup')['gae'], 'grid_unpriced_names': g['cost']['unpriced']})
out['all_identity_rules_hold'] = (out['pairs'] > 0 and out['seeds_paired'] and
    all(out[k] == out['pairs'] for k in ('trials_identical', 'relations_identical', 'gae_grid_eq_f4', 'mitm_minus_grid_in_band')))
print(json.dumps(out, indent=1))
