#!/usr/bin/env python3
"""L13: decide P1..P8 strictly by PROTOCOL.md section 4 from the frozen outputs under data/ecbench/.
Reads band_pooled.json, pooled_compare.jsonl, identity_check.json, verify/rc.txt. Writes analysis/verdicts.json.
Nothing here is retuned; thresholds are copied from PROTOCOL.md."""
import json, os, sys, statistics
L = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EB = os.path.join(L, 'data', 'ecbench')
F4, GRID, MITM, PA, RHO = 'pkm-iso-t5-g4-s1307', 'grid-iso-t5-s1307', 'mitm-iso-t5-s1307', 'proj-a', 'rho-neg'
band = json.load(open(os.path.join(EB, 'band_pooled.json')))
rows = {r['arm']: r for r in band['rows']}
ranks = {(r['a'], r['b']): r['verdict'] for r in band['ranks']}
def verdict(a, b): return ranks.get((a, b)) or ranks.get((b, a))
pc = {}
for l in open(os.path.join(EB, 'pooled_compare.jsonl')):
    j = json.loads(l); pc[(j['a'], j['b'], j['weight'])] = j
idc = json.load(open(os.path.join(EB, 'identity_check.json')))
rc = open(os.path.join(EB, 'verify', 'rc.txt')).read().split()
rc_ok = all(x == 'rc=0' for x in rc if x.startswith('rc=')) and len([x for x in rc if x.startswith('rc=')]) == 16
V = {}
def R(a, b, w):
    j = pc.get((a, b, float(w)))
    return None if j is None or 'error' in j else j
def fmt(j): return 'n/a' if j is None else f"{j['ratio']:.6g} [{j['ci95'][0]:.6g}, {j['ci95'][1]:.6g}] wins_b={j['wins_b']}/{j['workloads']}"
# P1
p1 = all(rows[a]['verified'] == rows[a]['runs'] == 32 for a in (F4, GRID, MITM, PA, RHO)) and rc_ok
V['P1'] = dict(held=p1, evidence={a: f"{rows[a]['verified']}/{rows[a]['runs']}" for a in rows} | {'verify_rc': rc})
# P2
g1, g2 = R(F4, GRID, 0.1), R(F4, GRID, 1)
p2 = all(j is not None and j['ratio'] <= 0.1 and j['ci95'][1] < 1 for j in (g1, g2))
V['P2'] = dict(held=p2, evidence={'w0.1': fmt(g1), 'w1': fmt(g2), 'exploratory_interval_0.020_0.030': [bool(j and 0.020 <= j['ratio'] <= 0.030) for j in (g1, g2)]})
# P3
g0 = R(F4, GRID, 0)
if idc['seeds_paired']:
    p3 = all(idc[k] == idc['pairs'] == 32 for k in ('trials_identical', 'relations_identical', 'gae_grid_eq_f4'))
    V['P3'] = dict(held=p3, form='as stated (seeds paired)', evidence={k: idc[k] for k in ('pairs', 'trials_identical', 'relations_identical', 'gae_grid_eq_f4')} | {'grid/F4 w0': fmt(g0)})
else:
    p3 = g0 is not None and 0.9 <= g0['ratio'] <= 1.1 and g0['ci95'][0] <= 1 <= g0['ci95'][1]
    V['P3'] = dict(held=p3, form='replacement rule (seeds not paired)', evidence={'grid/F4 w0': fmt(g0)})
# P4
m0, m1, m2 = R(GRID, MITM, 0), R(GRID, MITM, 0.1), R(GRID, MITM, 1)
p4 = (m1 is not None and m1['ratio'] <= 0.05 and m1['ci95'][1] < 1 and m2 is not None and m2['ratio'] <= 0.01 and m2['ci95'][1] < 1
      and m0 is not None and 1.02 <= m0['ratio'] <= 1.10 and idc['mitm_minus_grid_in_band'] == idc['pairs'] == 32
      and verdict(GRID, MITM) == 'unresolved inside the band')
V['P4'] = dict(held=p4, evidence={'w0': fmt(m0), 'w0.1': fmt(m1), 'w1': fmt(m2), 'mitm_minus_grid_in_band': f"{idc['mitm_minus_grid_in_band']}/{idc['pairs']}", 'band_verdict': verdict(GRID, MITM),
               'mitm_minus_grid_values_min_max': [min(idc['mitm_minus_grid_values']), max(idc['mitm_minus_grid_values'])]})
# P5
a0, a1, a2 = R(PA, GRID, 0), R(PA, GRID, 0.1), R(PA, GRID, 1)
p5 = (a0 is not None and a0['ratio'] >= 1.5 and a0['ci95'][0] > 1 and a1 is not None and a1['ratio'] >= 50 and a2 is not None and a2['ratio'] >= 300
      and verdict(PA, GRID) == f'{PA} < {GRID}')
V['P5'] = dict(held=p5, evidence={'w0': fmt(a0), 'w0.1': fmt(a1), 'w1': fmt(a2), 'band_verdict': verdict(PA, GRID)})
# P6
b = [R(PA, MITM, w) for w in (0, 0.1, 1)]
p6 = all(j is not None and j['ratio'] >= 1.5 and j['ci95'][0] > 1 for j in b) and verdict(PA, MITM) == f'{PA} < {MITM}'
V['P6'] = dict(held=p6, evidence={f'w{w}': fmt(j) for w, j in zip((0, 0.1, 1), b)} | {'band_verdict': verdict(PA, MITM)})
# P7
ic = (F4, GRID, MITM, PA)
p7 = all(verdict(RHO, a) == f'{RHO} < {a}' for a in ic)
V['P7'] = dict(held=p7, evidence={a: verdict(RHO, a) for a in ic} | {f'{a}/rho w{w}': fmt(R(RHO, a, w)) for a in (GRID, MITM, F4) for w in (0, 0.1, 1)})
# P8
p8 = idc['grid_solver_ops_zero'] == idc['grid_mults_eq_1248_trials'] == idc['pairs'] == 32
V['P8'] = dict(held=p8, evidence={k: idc[k] for k in ('pairs', 'grid_solver_ops_zero', 'grid_mults_eq_1248_trials')})
# report extras
V['_rows'] = {a: dict(runs=rows[a]['runs'], verified=rows[a]['verified'], mean_s=rows[a]['mean_s'], charged_sum=rows[a]['charged_sum'], native_sum=rows[a]['native_sum']) for a in rows}
V['_pairs'] = {f'{k[1]}/{k[0]} w{k[2]}': fmt(v if 'error' not in v else None) for k, v in pc.items()}
V['_ranks'] = band['ranks']
json.dump(V, open(os.path.join(L, 'analysis', 'verdicts.json'), 'w'), indent=1)
for k in ('P1', 'P2', 'P3', 'P4', 'P5', 'P6', 'P7', 'P8'):
    print(k, 'HELD' if V[k]['held'] else 'FAILED', json.dumps(V[k]['evidence']))
