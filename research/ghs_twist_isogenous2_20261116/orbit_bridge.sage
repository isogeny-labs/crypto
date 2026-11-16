#!/usr/bin/env sage
"""Verify the complete two-orbit degree-two isogeny bridge over F243."""
import hashlib
import json
import time
from pathlib import Path

t0=time.process_time()
HERE=Path(__file__).resolve().parent
K=GF(3^5,'a',modulus=x^5+2*x+1)
a=K.gen()
def decode(mask):
    return sum((K((mask//3^i)%3)*a^i for i in range(5)),K(0))
def encode(v):
    return sum(int(c)*3^i for i,c in enumerate(v.polynomial().list()))
pairs=[(30,167),(194,182),(103,230),(139,3),(233,27)]
assert all(encode(decode(u)^3)==pairs[(i+1)%5][0] for i,(u,v) in enumerate(pairs))
assert all(encode(decode(v)^3)==pairs[(i+1)%5][1] for i,(u,v) in enumerate(pairs))
rows=[]
for left,right in pairs:
    for twist,a2,expected_order in [('positive',2,218),('negative',1,270)]:
        bleft=decode(left) if twist=='positive' else 2*decode(left)
        bright=decode(right) if twist=='positive' else 2*decode(right)
        E=EllipticCurve(K,[0,a2,0,0,bleft])
        F=EllipticCurve(K,[0,a2,0,0,bright])
        assert E.order()==F.order()==expected_order
        isos=E.isogenies_prime_degree(2)
        assert len(isos)==1
        phi=isos[0]
        assert phi.degree()==2
        assert phi.codomain().is_isomorphic(F)
        assert E.j_invariant()!=F.j_invariant()
        rows.append({'left_mask':left,'right_mask':right,'twist':twist,
                     'source_order':expected_order,'degree':2,
                     'left_j':str(E.j_invariant()),
                     'right_j':str(F.j_invariant()),
                     'kernel_polynomial':str(phi.kernel_polynomial()),
                     'codomain_isomorphic':True})
out={'schema':'ghs.odd5.two-orbit-degree2/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'field':'F3[a]/(a^5+2a+1)','pairs':pairs,'rows':rows,
     'cpu_seconds':time.process_time()-t0,'status':'PASS'}
(HERE/'orbit_bridge.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'status':out['status'],'pairs':pairs,'rows_checked':len(rows),'cpu_seconds':out['cpu_seconds']},sort_keys=True,default=int))
