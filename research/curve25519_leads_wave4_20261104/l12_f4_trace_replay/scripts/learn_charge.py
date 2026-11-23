#!/usr/bin/env python3
"""L12 ecbench hand-charged learning cost and w=0 pair identity (PROTOCOL.md 4.3).

The band prices solver.ops (replay + fallback mul-adds) but NOT
pkm_trace_learn_muladds (no `_uncharged` suffix). This adds it by hand:
S_charged(w) = (total_gae + w*(unpriced + learn)) / sqrt(r) per run.
Also checks, per (workload, round), total_gae and recovered scalar identity
between the trace arm and the plain arm (P9), and sums the trace counters.
Usage: learn_charge.py <chunk dir> [<chunk dir> ...] -> JSON on stdout
"""
import json, os, sys, statistics
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pooled_compare import unpriced, load
A, B = 'pkm-iso-t5-g4-s1307', 'pkm-iso-t5-g4-s1307-trace'
recs = load(sys.argv[1:])
sq = {round(x['cost']['total_gae'] / x['cost']['s'], 6) for x in recs if x['cost']['s']}.pop()
def learn(x):
    return sum(ph.get('native', {}).get('pkm_trace_learn_muladds', 0) for ph in x['phases'])
def trace_counters(x):
    out = {}
    for ph in x['phases']:
        for k, v in ph.get('native', {}).items():
            if k.startswith('pkm_trace_'): out[k] = out.get(k, 0) + v
    return out
res = {'sqrt_r': sq, 'arms': {}, 'pairs': []}
for arm in (A, B):
    xs = [x for x in recs if x['arm'] == arm]
    row = {'runs': len(xs), 'verified': sum(x['outcome']['status'] == 'verified' for x in xs)}
    for w in (0.0, 0.1, 1.0):
        row[f'S_band_w{w}'] = statistics.fmean((x['cost']['total_gae'] + w * unpriced(x)) / sq for x in xs)
        row[f'S_charged_w{w}'] = statistics.fmean((x['cost']['total_gae'] + w * (unpriced(x) + learn(x))) / sq for x in xs)
    row['learn_muladds_mean'] = statistics.fmean(learn(x) for x in xs)
    row['solver_ops_mean'] = statistics.fmean(x['solver']['ops'] for x in xs)
    tc = {}
    for x in xs:
        for k, v in trace_counters(x).items(): tc[k] = tc.get(k, 0) + v
    row['trace_counters_total'] = tc
    res['arms'][arm] = row
ia = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == A}
ib = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == B}
for k in sorted(set(ia) & set(ib)):
    a, b = ia[k], ib[k]
    res['pairs'].append({'workload': k[0], 'round': k[1], 'gae_plain': a['cost']['total_gae'], 'gae_trace': b['cost']['total_gae'],
                         'gae_equal': a['cost']['total_gae'] == b['cost']['total_gae'],
                         'scalar_equal': a['outcome']['recovered'] == b['outcome']['recovered'],
                         'seed_equal': a.get('algorithm_seed') == b.get('algorithm_seed'),
                         'ops_ratio': b['solver']['ops'] / a['solver']['ops'] if a['solver']['ops'] else None})
res['P9_all_pairs_gae_and_scalar_equal'] = all(p['gae_equal'] and p['scalar_equal'] for p in res['pairs']) and bool(res['pairs'])
res['seeds_paired'] = all(p['seed_equal'] for p in res['pairs']) and bool(res['pairs'])
for w in (0.1, 1.0):
    res[f'charged_ratio_w{w}'] = res['arms'][B][f'S_charged_w{w}'] / res['arms'][A][f'S_charged_w{w}']
print(json.dumps(res, indent=1))
