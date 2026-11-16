#!/usr/bin/env sage
"""Check the matched-rho Frobenius orbit on the saved 69-bit point."""
import hashlib
import json
from pathlib import Path

PTH=Path(__file__).resolve().parent
d=json.loads((PTH/'degree7_s11_transfer_30.json').read_text())
old=json.loads((PTH/'primitive_isogeny_s11.json').read_text())
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,'a',modulus=z^5+2*z+1)
L=GF(3^55,'w',modulus=Z(old['field_modulus']))
emb=K.embeddings(L)[0];a=emb(K.gen());w=L.gen()
assert [int(c) for c in a.polynomial().list()]==old['a_embedding']
def elt(v):return sum(L(c)*w^i for i,c in enumerate(v))
E=EllipticCurve(L,[0,2,0,0,a+a^3]);P=E(elt(d['point_P'][0]),elt(d['point_P'][1]))
r=ZZ(d['r']);assert is_prime(r) and r*P==E(0) and P!=E(0)
Fr=lambda Q:E(Q[0]^243,Q[1]^243)
R=PolynomialRing(GF(r),'t');t=R.gen()
lam=next(int(u) for u,m in (t^2-26*t+243).roots() if Fr(P)==int(u)*P)
assert lam!=1 and pow(lam,11,r)==1 and Fr(Fr(P))==pow(lam,2,r)*P
out={'schema':'ghs.odd5.s11-frobenius-eigenvalue/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
 'source_point_receipt_sha256':hashlib.sha256((PTH/'degree7_s11_transfer_30.json').read_bytes()).hexdigest(),
 'r':int(r),'lambda':lam,'order':11,'checked_on_saved_point':True}
target=PTH/'s11_frobenius_eigenvalue.json';assert not target.exists()
target.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps(out,sort_keys=True,default=int))
