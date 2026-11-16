#!/usr/bin/env sage
"""Replay the saved composite cover vectors without the source cover or isogeny formulas."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
SOURCE=HERE/'compose_cover.sage'
REC=HERE/'compose_cover.json'
rec=json.loads(REC.read_text())
assert rec['status']=='PASS' and rec['degree']==14
assert rec['source_sha256']==hashlib.sha256(SOURCE.read_bytes()).hexdigest()
K=GF(3^5,'a',modulus=x^5+2*x+1)
a=K.gen()
R=PolynomialRing(K,'x');x=R.gen()
F=FractionField(R)
def from_vector(v):
    return R([sum((K(c)*a^i for i,c in enumerate(cs)),K(0)) for cs in v])
xn=from_vector(rec['x_numerator'])
xd=from_vector(rec['x_denominator'])
yn=from_vector(rec['y_coefficient_numerator'])
yd=from_vector(rec['y_coefficient_denominator'])
assert xn.gcd(xd)==1 and yn.gcd(yd)==1
assert max(xn.degree(),xd.degree())==14
U=F(xn)/F(xd)
V=F(yn)/F(yd)
f=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
assert 2*F(f)*V^2==U^3+U^2+(a^4+2*a+1)
out={'schema':'ghs.odd5.mask167-degree14-cover-audit/v1',
     'status':'PASS','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'cover_sha256':hashlib.sha256(REC.read_bytes()).hexdigest(),
     'degree':14,'full_identity_verified':True}
(HERE/'audit_cover.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps(out,sort_keys=True,default=int))
