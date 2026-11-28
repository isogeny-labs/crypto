#!/usr/bin/env python3
"""Independent R112 composition check of the second blowup."""
import hashlib
import json
import math
from fractions import Fraction
from pathlib import Path
root=Path(__file__).resolve().parent;rounds=root.parent
r=json.loads((root/'result.json').read_text())
paths={'donor51':rounds/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'parser105':rounds/'105/jets105.py','result112':rounds/'112/result.json'}
for name,p in paths.items():assert hashlib.sha256(p.read_bytes()).hexdigest()==r['source_sha256'][name]
prior=json.loads(paths['result112'].read_text())
terms=set()
for we,xe in prior['support']:
 for ve in range(we+1):
  if math.comb(we,ve)&1:
   p=(ve,xe+we-3)
   if p in terms:terms.remove(p)
   else:terms.add(p)
assert all(x>=0 for v,x in terms)
pts=sorted(terms)
assert [list(p) for p in pts]==r['support']
assert len(pts)==34 and [v for v,x in pts if x==0]==[2,3]
vertices=[tuple(p) for p in r['lower_hull']]
assert vertices==[(0,3),(2,0),(3,0),(7,8)]
for (va,xa),(vb,xb) in zip(vertices,vertices[1:]):
 assert all((x-xa)*(vb-va)>=(xb-xa)*(v-va) for v,x in pts)
assert len(r['positive_edges'])==1
e=r['positive_edges'][0]
(va,xa),(vb,xb)=e['endpoints']
assert Fraction(xa-xb,vb-va)==Fraction(3,2)
on=sorted((v,x) for v,x in pts if va<=v<=vb and (x-xa)*(vb-va)==(xb-xa)*(v-va))
assert [list(p) for p in on]==e['edge_support']
assert e['residual_exponents']==[0,2]
print('BLOWUP113_INDEPENDENT_COMPOSITION_PASS')
