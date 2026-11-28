#!/usr/bin/env python3
"""Validator: check the disclosed smoke/data overlaps directly from the files."""
import json, os, glob, re
import numpy as np
BASE = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L5_selector_255'
out = {}
# frames k20
fr = {}
for f in sorted(glob.glob(os.path.join(BASE, 'smoke', 'frames', '*.npz'))):
    nm = os.path.basename(f); s = np.load(f); d = np.load(os.path.join(BASE, 'data', 'frames', nm))
    ns = len(s['rows'])
    eq_rows = bool(np.array_equal(s['rows'], d['rows'][:ns]))
    # first data row beyond the smoke block equal to anything in smoke? (no further overlap expected)
    sur_eq = int(np.sum(np.all(s['surrogate_counts'] == d['surrogate_counts'][:ns], axis=1)))
    fr[nm] = dict(smoke_rows=ns, rows_equal_first=eq_rows, surrogate_rows_equal=sur_eq,
                  smoke_rho=float(s['rho']), data_rho=float(d['rho']))
out['frames'] = fr
# iso
iso = {}
for f in sorted(glob.glob(os.path.join(BASE, 'smoke', 'iso', 'iso_k*.jsonl'))):
    nm = os.path.basename(f)
    S = [json.loads(l) for l in open(f)]; D = {(x['seed']): x for x in map(json.loads, open(os.path.join(BASE, 'data', 'iso', nm)))}
    iso[nm] = dict(smoke_rows=len(S), seeds=[min(x['seed'] for x in S), max(x['seed'] for x in S)], equal=sum(D[x['seed']] == x for x in S))
out['iso'] = iso
# PC3 pools
sp = json.load(open(os.path.join(BASE, 'smoke', 'pc3_pools.json'))); dp = json.load(open(os.path.join(BASE, 'data', 'pc3_pools.json')))
pc3 = {}
for cs, cd in zip(sp['cells'], dp['cells']):
    k = len(cs['realised'])
    pc3[cs['t']] = dict(smoke_pools=k, realised_equal_first=cs['realised'] == cd['realised'][:k],
                        ustat_equal_first=cs['ustat_J'] == cd['ustat_J'][:len(cs['ustat_J'])])
out['pc3'] = pc3
# real
s = np.load(os.path.join(BASE, 'smoke', 'real25519.npz')); d = np.load(os.path.join(BASE, 'data', 'real25519.npz'))
rl = {}
for k in s.files:
    if k.startswith('r1'):
        a = s[k]; b = d[k]
        rl[k] = dict(smoke_n=len(a), equal_first=bool(np.array_equal(a, b[:len(a)])))
rl['r2_smoke_shape'] = list(s['r2_counts'].shape); rl['r2_data_shape'] = list(d['r2_counts'].shape)
out['real'] = rl
sv = json.load(open(os.path.join(BASE, 'smoke', 'verify_curves_all.json'))); dv = json.load(open(os.path.join(BASE, 'data', 'verify_curves.json')))
out['verify_curves_smoke_all_equals_data'] = sv == dv
# PC1/PC2 smoke vs data
a = json.load(open(os.path.join(BASE, 'smoke', 'pc', 'pc_replay.json'))); b = json.load(open(os.path.join(BASE, 'data', 'pc', 'pc_replay.json')))
out['pc_replay_equal_except_seconds'] = {k: a[k] == b[k] for k in a if k != 'seconds_orig'}
print(json.dumps(out, indent=1))
