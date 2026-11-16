#!/usr/bin/env sage
"""Certify the two Frobenius orbits of trace-+26 normal-form masks."""
import hashlib
import json
from pathlib import Path

PTH=Path(__file__).resolve().parent
F3=GF(3);R=PolynomialRing(F3,'z');z=R.gen()
K=GF(3^5,'a',modulus=z^5+2*z+1);a=K.gen()
def elem(mask):return sum(K((mask//3^i)%3)*a^i for i in range(5))
def mask(v):return sum(int(c)*3^i for i,c in enumerate(v.polynomial().list()))
orbits=[[3,27,167,182,230],[30,194,103,139,233]]
for row in orbits:
 assert [mask(elem(row[0])^(3^j)) for j in range(5)]==row
 assert all(EllipticCurve(K,[0,2,0,0,elem(m)]).cardinality()==218 for m in row)
assert len(set(sum(orbits,[])))==10
out={'schema':'ghs.odd5.trace26-frobenius-orbits/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
 'field_modulus':[int(c) for c in K.modulus().list()],'orbits':orbits,
 'cover_positive_orbit':orbits[1],'partial_degree7_nonhit_orbit':orbits[0]}
target=PTH/'frobenius_orbits.json';assert not target.exists()
target.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps(out,sort_keys=True,default=int))
