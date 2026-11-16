#!/usr/bin/env python3
"""Independent coefficientwise implicit lift from the direct R51 expansion."""
import hashlib
import importlib.util
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
p=json.loads((HERE/'protocol.json').read_text())
r=json.loads((HERE/'result.json').read_text())
for name,path in {'r51_source':ROUNDS/'51/source-data/marked23-field51.sing',
                  'r127_result':ROUNDS/'127/result.json','r128_result':ROUNDS/'128/result.json',
                  'r128_source':ROUNDS/'128/newton128.py'}.items():
    assert hashlib.sha256(path.read_bytes()).hexdigest()==p['input_sha256'][name]
spec=importlib.util.spec_from_file_location('v128',ROUNDS/'128/verify128.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
times=v.v.times
M=p['precision_u_exclusive']
terms=[(i,j,c) for j,row in enumerate(v.poly) for i,c in enumerate(row) if c]
assert len(terms)==52

def powers(s,k):
    result=[[1]+[0]*k,s[:k+1]]
    for _ in range(2,8):
        a=result[-1];z=[0]*(k+1)
        for i,x in enumerate(a):
            if x:
                for j,y in enumerate(s[:k-i+1]):
                    if y:z[i+j]^=times(x,y)
        result.append(z)
    return result

def at(s,k):
    po=powers(s,k)
    z=0
    for i,j,c in terms:
        if j<=k:z^=times(c,po[i][k-j])
    return z

inverse=next(x for x in range(1,128) if times(84,x)==1)
s=[0]*M
assert at(s,0)==0
for k in range(1,M):
    s[k]=times(at(s,k),inverse)
assert all(at(s,k)==0 for k in range(M))
support=[[k,x] for k,x in enumerate(s) if x]
assert support==r['s_support']
assert hashlib.sha256(json.dumps(support,separators=(',',':')).encode()).hexdigest()==r['s_support_sha256']
assert support[0]==[2,6] and support[1]==[3,21]
assert sum(x['residue_degree_over_F2']*x['ramification_index'] for x in r['base_places'])==49
print('LIFT129_INDEPENDENT_COEFFICIENTWISE_PASS')
