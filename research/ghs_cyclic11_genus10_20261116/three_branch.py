#!/usr/bin/env sage -python
"""Exact genus-five Kummer census for three geometric branch points over F3."""
from sage.all import GF
from collections import Counter
from pathlib import Path
import hashlib
import json

K=GF(243,name='u')
g=K.multiplicative_generator()
logs={};z=K(1)
for i in range(242):logs[z]=i%11;z*=g
assert len(logs)==242
h=Counter()
for x in K:
    if x in (0,1):continue
    h[(logs[x],logs[x-1])]+=1
rows=[]
for a in range(1,11):
    for b in range(1,11):
        if (a+b)%11==0:continue
        m=sum(n for (i,j),n in h.items() if (a*i+b*j)%11==0)
        n5=3+11*m
        assert (244-n5)%5==0
        rows.append({'case':'three_rational','exponents':[a,b],
                     'N5':n5,'trace_t':(244-n5)//5})
assert len(rows)==90

powers={v**11 for v in K if v}
n5=1+11*sum(x*x+1 in powers for x in K)
assert (244-n5)%5==0
rows.append({'case':'one_rational_quadratic','exponents':[1],
             'N5':n5,'trace_t':(244-n5)//5})
assert set(r['trace_t'] for r in rows)=={-24,-9,24}
print(json.dumps({'schema':'ghs.cyclic11-genus5-screen/v1',
                  'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'equations':len(rows),'trace_histogram':dict(sorted(Counter(
                      r['trace_t'] for r in rows).items())),
                  'records':rows},sort_keys=True))
