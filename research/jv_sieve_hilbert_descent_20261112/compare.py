#!/usr/bin/env python3
"""Paired comparison for LEAD-C-jv-sieve-hilbert-descent at p = 1511.
Arm A: committed records research/jv_sieve_negrho_20261102/results/inputs/ic_1511_sparse.json (plain V2).
Arm B: results/hilbert_s*.json (JV_SIEVE_HILBERT=16). Checks the lead's falsifiers."""
import json, math, os
here = os.path.dirname(os.path.abspath(__file__))
ref = {r['seed']: r for r in json.load(open(os.path.join(here, '..', 'jv_sieve_negrho_20261102', 'results', 'inputs', 'ic_1511_sparse.json')))}
rows = []
for s in range(17, 33):
    f = os.path.join(here, 'results', f'hilbert_s{s}.json')
    b = json.load(open(f))[0]; a = ref[s]
    same = all(a[k] == b[k] for k in ('descent_digest', 'relations', 'solved', 'correct', 'descent_successes', 'descent_residuals'))
    rows.append(dict(seed=s, same=same, total=b['total_muls'] / a['total_muls'], s_ratio=b['s'] / a['s'],
                     splus=b['s_plus'] / a['s_plus'], f4=b['descent_f4_muls'] / a['descent_f4_muls'],
                     applied=b['descent_hilbert_applied'], fb=b['descent_hilbert_fallbacks'], res=b['descent_residuals'],
                     native=b['descent_f4_native_ops']))
gm = lambda xs: math.exp(sum(map(math.log, xs)) / len(xs))
print(f"{'seed':>4} {'same':>5} {'total':>7} {'S':>7} {'S+':>7} {'descF4':>7} {'applied':>8} {'fallbk':>6}")
for r in rows:
    print(f"{r['seed']:>4} {str(r['same']):>5} {r['total']:7.4f} {r['s_ratio']:7.4f} {r['splus']:7.4f} {r['f4']:7.4f} {r['applied']:>8} {r['fb']:>6}")
print('geo-mean total %.4f  S %.4f  S+ %.4f  descent_f4 %.4f' % (gm([r['total'] for r in rows]), gm([r['s_ratio'] for r in rows]),
      gm([r['splus'] for r in rows]), gm([r['f4'] for r in rows])))
print('identical descent digest/relations/solution: %d/16; fallbacks %d of %d applied' % (sum(r['same'] for r in rows), sum(r['fb'] for r in rows), sum(r['applied'] for r in rows)))
json.dump(rows, open(os.path.join(here, 'results', 'paired.json'), 'w'), indent=1)
