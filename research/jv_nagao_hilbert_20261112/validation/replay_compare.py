#!/usr/bin/env python3
"""Field-by-field equality of pod replay records against committed records.
Usage: replay_compare.py <committed_dir> <replay_dir> <out.json> <label>...
Timing fields (any key ending in '_ms') are excluded; every other field, nested, must be equal
(exact JSON value equality, ints and floats as parsed)."""
import json, sys, hashlib, os
cd, rd, out = sys.argv[1:4]; labels = sys.argv[4:]
def flat(x, pre=''):
    if isinstance(x, dict):
        for k, v in x.items(): yield from flat(v, f'{pre}.{k}' if pre else k)
    elif isinstance(x, list):
        for i, v in enumerate(x): yield from flat(v, f'{pre}[{i}]')
    else: yield pre, x
sha = lambda p: hashlib.sha256(open(p, 'rb').read()).hexdigest()
res = []
for L in labels:
    a, b = os.path.join(cd, L + '.json'), os.path.join(rd, L + '.json')
    if not os.path.exists(b): res.append(dict(label=L, present=False, identical=False)); continue
    fa, fb = dict(flat(json.load(open(a)))), dict(flat(json.load(open(b))))
    timing = sorted(k for k in set(fa) | set(fb) if k.split('.')[-1].endswith('_ms'))
    keys = sorted((set(fa) | set(fb)) - set(timing))
    diff = [dict(field=k, committed=fa.get(k, '<missing>'), replay=fb.get(k, '<missing>')) for k in keys if fa.get(k, '<missing>') != fb.get(k, '<missing>')]
    r0 = json.load(open(a))[0]
    res.append(dict(label=L, present=True, committed=os.path.relpath(a), committed_sha256=sha(a), replay_sha256=sha(b),
                    fields_compared=len(keys), timing_fields_excluded=timing, mismatches=diff, identical=not diff,
                    solved=r0.get('solved'), correct=r0.get('correct'), total_muls=r0.get('total_muls'), s=r0.get('s'),
                    relation_digest=r0.get('relation_digest')))
json.dump(res, open(out, 'w'), indent=1)
for r in res: print(r['label'], 'identical' if r['identical'] else f"MISMATCH {r.get('mismatches', 'missing')[:3]}", r.get('fields_compared'))
print('ALL IDENTICAL' if all(r['identical'] for r in res) else 'NOT ALL IDENTICAL')
