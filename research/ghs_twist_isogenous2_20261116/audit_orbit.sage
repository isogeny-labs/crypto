#!/usr/bin/env sage
"""Independent modular-polynomial and Frobenius replay of the ten bridge cells."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
SOURCE=HERE/'orbit_bridge.sage'
REC=HERE/'orbit_bridge.json'
rec=json.loads(REC.read_text())
assert rec['schema']=='ghs.odd5.two-orbit-degree2/v1'
assert rec['source_sha256']==hashlib.sha256(SOURCE.read_bytes()).hexdigest()
assert rec['status']=='PASS' and len(rec['rows'])==10
K=GF(3^5,'a',modulus=x^5+2*x+1)
a=K.gen()
def b(mask):return sum((K((mask//3^i)%3)*a^i for i in range(5)),K(0))
def mask(v):return sum(int(c)*3^i for i,c in enumerate(v.polynomial().list()))
pairs=[(30,167),(194,182),(103,230),(139,3),(233,27)]
assert [list(p) for p in pairs]==rec['pairs']
Phi2=classical_modular_polynomial(2)
checks=[]
for i,(left,right) in enumerate(pairs):
    assert mask(b(left)^3)==pairs[(i+1)%5][0]
    assert mask(b(right)^3)==pairs[(i+1)%5][1]
    for twist,a2,order in [('positive',2,218),('negative',1,270)]:
        c=b(left) if twist=='positive' else 2*b(left)
        d=b(right) if twist=='positive' else 2*b(right)
        E=EllipticCurve(K,[0,a2,0,0,c])
        F=EllipticCurve(K,[0,a2,0,0,d])
        assert E.order()==F.order()==order
        assert E.j_invariant()!=F.j_invariant()
        assert Phi2(E.j_invariant(),F.j_invariant())==0
        row=next(z for z in rec['rows'] if z['left_mask']==left and z['right_mask']==right and z['twist']==twist)
        assert row['degree']==2 and row['source_order']==order
        assert row['left_j']==str(E.j_invariant()) and row['right_j']==str(F.j_invariant())
        checks.append([left,right,twist])
assert len(checks)==10
out={'schema':'ghs.odd5.two-orbit-audit/v1','status':'PASS',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'bridge_sha256':hashlib.sha256(REC.read_bytes()).hexdigest(),
     'checks':checks,'modular_polynomial':'Phi_2','group_orders':[218,270]}
(HERE/'audit_orbit.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'status':'PASS','rows':len(checks),'modular_polynomial':'Phi_2'},sort_keys=True))
