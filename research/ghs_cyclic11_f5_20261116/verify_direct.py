#!/usr/bin/env sage -python
"""Recount every F5 cyclic-11 row via an independent eleventh-power table."""
from pathlib import Path
from collections import Counter
import hashlib
import json
from sage.all import GF, PolynomialRing

here=Path(__file__).resolve().parent
source=here/'SCREEN.json'
d=json.loads(source.read_text())
F5=GF(5);R=PolynomialRing(F5,name='z')
F=GF(5**5,name='u',modulus=R(d['field_modulus']))
root_count=Counter(y**11 for y in F)
assert root_count[F.zero()]==1
assert sum(root_count.values())==3125
records=[]
for rec in d['records']:
    a,b=rec['a'],rec['b']
    n5=1+sum(root_count[x**a*(x-1)**b] for x in F)
    assert n5==rec['N5']
    records.append({'a':a,'b':b,'N5':n5,'trace_t':(3126-n5)//5})
assert len(records)==90
out={'schema':'ghs.cyclic11-f5-direct-recount/v1',
     'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     'screen_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
     'verified_rows':len(records),'trace_histogram':dict(sorted(
         Counter(r['trace_t'] for r in records).items())),
     'records':records}
target=here/'VERIFIED.json'
assert not target.exists(), 'refusing to overwrite recount receipt'
target.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
print(json.dumps({'schema':out['schema'],'verified_rows':out['verified_rows'],
                  'trace_histogram':out['trace_histogram']},sort_keys=True))
