#!/usr/bin/env python3
"""Paired comparison, LEAD-C-jv-nagao-hilbert-at-scale stage 1 (counts only).
Arm A: frozen line-search records (research/jv_cover_fbline_20261102/results/line_p*_s*.json).
Arm B: --stop hilbert:16 on the same instances (results/hilbert_p*_s*.json).
Prints per-instance ratios and the falsification checks F1 (digest/relations/logs), F3 (fallbacks), F4 (native ops)."""
import json, math, os, sys
here = os.path.dirname(os.path.abspath(__file__)); A = os.path.join(here, '..', 'jv_cover_fbline_20261102', 'results')
rows = []
for p in (1301, 1451, 1601):
    for s in (1, 2, 3):
        fb = os.path.join(here, 'results', f'hilbert_p{p}_s{s}.json')
        if not os.path.exists(fb): continue
        a = json.load(open(os.path.join(A, f'line_p{p}_s{s}.json')))[0]; b = json.load(open(fb))[0]
        same = all(a[k] == b[k] for k in ('relation_digest', 'relations', 'residuals', 'decompositions', 'solved', 'correct'))
        rows.append(dict(p=p, seed=s, same=same, S_ratio=b['s'] / a['s'], total_ratio=b['total_muls'] / a['total_muls'],
                         f4_ratio=b['f4_muls'] / a['f4_muls'], native_ratio=b['f4_native_ops'] / a['f4_native_ops'],
                         applied=b['hilbert_applied'], fallbacks=b['hilbert_fallbacks'], hcheck=b['hilbert_check_muls'],
                         sA=a['s'], sB=b['s'], wallA=a['wall_ms'] / 1e3, wallB=b['wall_ms'] / 1e3))
print(f"{'p':>5} {'s':>2} {'same':>5} {'S_B/S_A':>8} {'tot':>7} {'f4':>7} {'native':>7} {'applied':>8} {'fallbk':>7} {'S_A':>7} {'S_B':>7}")
for r in rows:
    print(f"{r['p']:>5} {r['seed']:>2} {str(r['same']):>5} {r['S_ratio']:8.4f} {r['total_ratio']:7.4f} {r['f4_ratio']:7.4f} "
          f"{r['native_ratio']:7.4f} {r['applied']:>8} {r['fallbacks']:>7} {r['sA']:7.4f} {r['sB']:7.4f}")
for p in (1301, 1451, 1601):
    rs = [r['S_ratio'] for r in rows if r['p'] == p]
    if rs: print(p, 'geo-mean S_B/S_A', round(math.exp(sum(map(math.log, rs)) / len(rs)), 4), f'(n={len(rs)})')
json.dump(rows, open(os.path.join(here, 'results', 'paired.json'), 'w'), indent=1)
