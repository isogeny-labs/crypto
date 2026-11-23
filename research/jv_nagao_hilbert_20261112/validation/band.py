#!/usr/bin/env python3
"""Absolute section-5a band values (ic, rho) per unit, prime and w, recomputed from raw records, for VALIDATION.json.
Producer-side cross-check of the negrho_rescore point estimates (not a validator report).
Usage: band.py <cover dir with hilbert_p*_s*.json> <rho dir(s) with rhostrong_p*_s*_run*.json ...> -- <fit.json> <out.json>
ic(w)  = geometric mean over the 3 seeds of (total_muls/(c_add_e sqrt l) + w f4_native_ops/sqrt l)
rho(w) = mean over the pooled reference runs of (s_harness [U1] or s_tuned [U2]) + w * mean native/sqrt l,
native = canon_cmp+hash+dp_check+scalar_ops+table_ops+cycle_cmp (as negrho_rescore)."""
import json, sys, glob, math, os
i = sys.argv.index('--'); cdir = sys.argv[1]; rdirs = sys.argv[2:i]; fitp, outp = sys.argv[i + 1:i + 3]
P = (1301, 1451, 1601); W = (0.0, 0.1, 1.0)
cov = {p: [] for p in P}
for p in P:
    for s in (1, 2, 3):
        r = json.load(open(os.path.join(cdir, f'hilbert_p{p}_s{s}.json')))[0]
        assert r['solved'] and r['correct'] and r['mismatches'] == 0
        sl = math.sqrt(r['l']); cov[p].append((r['total_muls'] / (r['c_add_e'] * sl), r['f4_native_ops'] / sl))
runs = []
for d in rdirs:
    for f in glob.glob(os.path.join(d, 'rhostrong_p*_s*_run*.json')):
        for r in json.load(open(f)):
            assert r['correct'] and not r['capped'] and r['negation'] and r['batch'] == 16 and r['r'] == 256
            sl = math.sqrt(r['l']); nat = sum(r['ops'][k] for k in ('canon_cmp', 'hash', 'dp_check', 'scalar_ops', 'table_ops', 'cycle_cmp'))
            runs.append((r['s_harness'], r['s_tuned'], nat / sl))
n = len(runs); m = lambda k: sum(x[k] for x in runs) / n
ref = {'U1 group additions': m(0), 'U2 textbook F_p mults': m(1)}; rnat = m(2)
fit = json.load(open(fitp)); tool = {(b['unit'], b['p']): b['ratio'] for b in fit['band_5a']}
out = dict(reference_runs=n, rho_native_per_sqrt_l=rnat, reference_mean=ref, cells=[])
maxdev = 0.0
for unit, rm in ref.items():
    for p in P:
        cell = dict(unit=unit, p=p, band={})
        for wi, w in enumerate(W):
            ic = math.exp(sum(math.log(s + w * nt) for s, nt in cov[p]) / 3); rho = rm + w * rnat
            cell['band'][str(w).rstrip('0').rstrip('.') if w != 0.1 else '0.1'] = dict(ic=ic, rho=rho, ratio=ic / rho, tool_ratio=tool[(unit, p)][wi])
            maxdev = max(maxdev, abs(ic / rho / tool[(unit, p)][wi] - 1))
        vals = [cell['band'][k]['ratio'] for k in cell['band']]
        cell['point_order_same_across_band'] = all(v < 1 for v in vals) or all(v > 1 for v in vals)
        out['cells'].append(cell)
out['max_rel_dev_from_tool'] = maxdev
json.dump(out, open(outp, 'w'), indent=1)
print('reference runs', n, 'max rel dev from negrho_rescore point ratios', f'{maxdev:.2e}')
for c in out['cells']: print(c['unit'], c['p'], {k: round(v['ratio'], 4) for k, v in c['band'].items()})
