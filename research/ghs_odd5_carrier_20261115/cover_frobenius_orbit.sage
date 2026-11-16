#!/usr/bin/env sage
"""Check whether five saved cover equations are one Frobenius orbit."""
import hashlib
import json
from pathlib import Path

PTH=Path(__file__).resolve().parent
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,'a',modulus=z^5+2*z+1);a=K.gen()
R=PolynomialRing(K,'x')
def poly(v):return R([K(c) for c in v])
def record(mask):return json.loads((PTH/('degree7_subgroup_map_%d.json'%mask)).read_text())['maps'][0]
def frob(p,j):return R([c^(3^j) for c in p.list()])
source=record(30)
rows=[]
for j,mask in enumerate([30,194,103,139,233]):
 hit=record(mask)
 assert hit['degree']==7 and source['degree']==7
 signs=[]
 for name in ('A','B','C'):
  lhs=poly(hit[name]);rhs=frob(poly(source[name]),j)
  if name=='C':assert lhs==rhs or lhs==-rhs;signs.append(1 if lhs==rhs else -1)
  else:assert lhs==rhs
 rows.append({'mask':mask,'frobenius_power':j,'A_equal':True,'B_equal':True,'C_sign':signs[0]})
out={'schema':'ghs.odd5.degree7-cover-frobenius-orbit/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
 'base_mask':30,'rows':rows}
target=PTH/'cover_frobenius_orbit.json';assert not target.exists()
target.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps(out,sort_keys=True,default=int))
