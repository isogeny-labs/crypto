#!/usr/bin/env sage -python
"""Independently recount one equation for each case and Weil polynomial."""
from sage.all import GF
from pathlib import Path
import hashlib
import json

here=Path(__file__).parent
input_path=here/'SCREEN.json'
rows=json.loads(input_path.read_text())['records']
chosen={}
for row in rows:
    key=(row['case'],row['s1'],row['e2'])
    chosen.setdefault(key,row)
assert len(chosen)==16

def factor_value(name,x):
    if name=='x':return x
    if name=='x-1':return x-1
    if name=='x+1':return x+1
    if name=='x^2+1':return x*x+1
    if name=='x^2+x+2':return x*x+x+2
    if name=='x^3-x-1':return x**3-x-1
    raise AssertionError(name)

results=[]
for row in chosen.values():
    counts=[]
    for r in (5,10):
        K=GF(3**r,name='w')
        total=0
        for x in K:
            values=[factor_value(name,x) for name in row['factors']]
            if any(v==0 for v in values):
                total+=1
                continue
            v=K(1)
            for factor,e in zip(values,row['exponents']):v*=factor**e
            total+=11 if v**((3**r-1)//11)==1 else 0
        degree={'x':1,'x-1':1,'x+1':1,'x^2+1':2,
                'x^2+x+2':2,'x^3-x-1':3}
        order=sum(e*degree[f] for e,f in zip(row['exponents'],row['factors']))%11
        total+=1 if order else 11
        counts.append(total)
    assert counts==[row['N5'],row['N10']]
    results.append({'case':row['case'],'exponents':row['exponents'],
                    's1':row['s1'],'e2':row['e2'],'counts':counts})

print(json.dumps({'schema':'ghs.cyclic11-direct-replay/v1',
                  'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'input_sha256':hashlib.sha256(input_path.read_bytes()).hexdigest(),
                  'checked':len(results),'records':results},sort_keys=True))
