#!/usr/bin/env python3
"""Independent Horner expansion of f23(a+s,90+u)."""
import hashlib
import importlib.util
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
p=json.loads((HERE/'protocol.json').read_text())
r=json.loads((HERE/'result.json').read_text())
paths={'r51_source':ROUNDS/'51/source-data/marked23-field51.sing',
       'r127_result':ROUNDS/'127/result.json','r127_source':ROUNDS/'127/fiber127.py'}
for key,path in paths.items():assert hashlib.sha256(path.read_bytes()).hexdigest()==p['input_sha256'][key]
spec=importlib.util.spec_from_file_location('f127',paths['r127_source'])
f=importlib.util.module_from_spec(spec);spec.loader.exec_module(f)
spec=importlib.util.spec_from_file_location('v127',ROUNDS/'127/verify127.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)

def add(a,b):
    out=[0]*max(len(a),len(b))
    for i,x in enumerate(a):out[i]^=x
    for i,x in enumerate(b):out[i]^=x
    return v.trim(out)

def scale(a,c):return [v.times(x,c) for x in a]
def mul(a,b):return v.product(a,b)

f_terms=f.parse_source(paths['r51_source'].read_text())
x=[2,1]
xpowers=[[1]]
for _ in range(7):xpowers.append(mul(xpowers[-1],x))
coeff=[]
for j in range(8):
    z=[]
    for e in f_terms[j]:z=add(z,xpowers[e])
    coeff.append(z)
poly=[]
for j in range(7,-1,-1):
    next_poly=[[] for _ in range(len(poly)+1)]
    for k,series in enumerate(poly):
        next_poly[k]=add(next_poly[k],scale(series,90))
        next_poly[k+1]=add(next_poly[k+1],series)
    next_poly[0]=add(next_poly[0],coeff[j])
    poly=next_poly
support=[[i,j,a] for j,series in enumerate(poly) for i,a in enumerate(series) if a]
assert sorted(support)==r['support']
orders=[next((i for i,a in enumerate(series) if a),None) for series in poly]
assert orders==[row['s_order'] for row in r['by_u']]
assert r['hull']==[[0,1],[2,0],[7,0]]
assert r['edges'][0]['residual_terms']==[[0,84],[2,110]]
assert v.times(84,6)==110
assert r['s0_specialization']==[0,0,110,102,112,0,99,1]
print('NEWTON128_INDEPENDENT_HORNER_PASS')
