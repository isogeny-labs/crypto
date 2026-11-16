#!/usr/bin/env sage
"""Evaluate the order-seven Frobenius eigenvalue on the saved s=7 target."""
import hashlib
import json
from pathlib import Path

here=Path(__file__).resolve().parent
inp=here/'s7_transfer.json';d=json.loads(inp.read_text())
F3=GF(3);Z=PolynomialRing(F3,'z')
L=GF(3^35,'w',modulus=Z(d['field_modulus']))
a=L(d['a_embedding'])
E=EllipticCurve(L,[0,2,0,0,a+a^3])
P=E(L(d['P'][0]),L(d['P'][1]))
Q=E(L(d['Q'][0]),L(d['Q'][1]))
r=ZZ(d['r']);lam=ZZ(1048494599)
Fr=lambda T:E(T[0]^243,T[1]^243)
assert (lam^2-26*lam+243)%r==0
assert pow(int(lam),7,int(r))==1 and lam%r!=1
assert Fr(P)==lam*P and Fr(Q)==lam*Q
out={'schema':'ghs.odd5.s7-frobenius/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'input_sha256':hashlib.sha256(inp.read_bytes()).hexdigest(),
     'r':int(r),'eigenvalue':int(lam),'order':int(7),
     'source_action_checked':True,'target_action_checked':True}
target=here/'s7_frobenius.json'
assert not target.exists(), 'refusing to overwrite receipt'
target.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
print(json.dumps(out,sort_keys=True))
