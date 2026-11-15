#!/usr/bin/env python3
"""V3: do J2's c = 2.00 records reproduce the committed N4v2 DL1 / D / DL records (same instances, same source lineage)?
usage: v3_compare.py <J2 dir> <N4v2 raw units dir> <N4v2 results dir> <out.json>"""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import compare
J2, N4, N4RES, OUT = sys.argv[1:5]
inst = {}
for l in open(os.path.join(N4RES, 'instances.jsonl')):
    r = json.loads(l); inst[(r['q'], r['seed'])] = r
res = []
for q in (65479, 262111, 1048507, 4194271):
    for sd in (1, 2, 3):
        a = [r for r in compare.recs(os.path.join(N4, f'ic_{q}_{sd}_SD.jsonl')) if r.get('rec') == 'ratios' or r.get('arm') in ('D', 'DL1', 'DL')]
        b = compare.recs(os.path.join(J2, 'units', f'ic_{q}_{sd}_c2.00.jsonl'))
        na, nb, nf, d = compare.diff(a, b, extra=('tag',))   # `tag` labels which arms the producing process ran (S,D vs D), not a counter
        ii = inst[(q, sd)]; rat = [r for r in b if r['rec'] == 'ratios'][0]
        res.append(dict(q=q, seed=sd, n4v2_records=na, j2_records=nb, fields_compared=nf, identical=(not d), diffs=d[:5],
                        ell_matches_instances_jsonl=(rat['ell'] == ii['ell'] and rat['order'] == ii['order'])))
        print(q, sd, 'IDENTICAL' if not d else 'DIFFERS', na, nb, nf, 'instances.jsonl ell/order match:', res[-1]['ell_matches_instances_jsonl'])
json.dump(res, open(OUT, 'w'), indent=1)
print('ALL IDENTICAL' if all(r['identical'] and r['ell_matches_instances_jsonl'] for r in res) else 'SOME DIFFER')
