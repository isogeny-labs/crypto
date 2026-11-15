#!/usr/bin/env python3
"""V2: compare every replayed IC unit with the producer's record, field by field (own comparator, tools/compare.py).
usage: replay_compare.py <J2 dir> <replay dir> <out.json>"""
import json, sys, os, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import compare
J2, RP, OUT = sys.argv[1:4]
res = []
for kind in ('units', 'heldout'):
    for f in sorted(glob.glob(os.path.join(J2, kind, 'ic_*_c*.jsonl'))):
        name = os.path.basename(f)
        g = os.path.join(RP, name)
        if not os.path.exists(g):
            res.append(dict(unit=name, kind=kind, status='not_replayed')); continue
        na, nb, nf, d = compare.diff(compare.recs(f), compare.recs(g))
        res.append(dict(unit=name, kind=kind, status='identical' if (not d and na > 0) else 'differs', producer_records=na, replay_records=nb, fields_compared=nf, diffs=d[:5]))
# A/A repeat that the producer kept: its own repeat vs its first run, and vs the validator replay of the same unit
aa = os.path.join(J2, 'units', 'aa_65479_1_c1.75_repeat.jsonl'); first = os.path.join(J2, 'units', 'ic_65479_1_c1.75.jsonl')
na, nb, nf, d = compare.diff(compare.recs(aa), compare.recs(first))
aa_res = dict(producer_repeat_vs_first=dict(identical=(not d), records=na, fields=nf, diffs=d[:3]))
g = os.path.join(RP, 'ic_65479_1_c1.75.jsonl')
if os.path.exists(g):
    na, nb, nf, d = compare.diff(compare.recs(aa), compare.recs(g)); aa_res['producer_repeat_vs_validator_replay'] = dict(identical=(not d), records=na, fields=nf, diffs=d[:3])
tot = len(res); ident = sum(1 for r in res if r['status'] == 'identical'); diff = [r for r in res if r['status'] == 'differs']
summary = dict(units_total=tot, identical=ident, differs=len(diff), not_replayed=sum(1 for r in res if r['status'] == 'not_replayed'),
               fields_compared_total=sum(r.get('fields_compared', 0) for r in res), aa=aa_res)
json.dump(dict(summary=summary, units=res), open(OUT, 'w'), indent=1)
print(json.dumps(summary))
for r in diff: print('DIFF', r['unit'], r['diffs'])
