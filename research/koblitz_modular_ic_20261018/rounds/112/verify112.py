#!/usr/bin/env python3
"""Independent bivariate polynomial multiplication check of R112."""
import hashlib
import importlib.util
import json
from fractions import Fraction
from pathlib import Path
root=Path(__file__).resolve().parent;rounds=root.parent
r=json.loads((root/'result.json').read_text())
paths={'donor51':rounds/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'parser105':rounds/'105/jets105.py','result111':rounds/'111/result.json'}
for name,p in paths.items():assert hashlib.sha256(p.read_bytes()).hexdigest()==r['source_sha256'][name]
spec=importlib.util.spec_from_file_location('jets105',paths['parser105'])
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
f,g=m.parse_source(paths['donor51'].read_text())

def add(a,b):return a^b

def mul(a,b):
 out=set()
 for xa,wa in a:
  for xb,wb in b:
   p=(xa+xb,wa+wb)
   if p in out:out.remove(p)
   else:out.add(p)
 return out

def power(a,n):
 out={(0,0)}
 for _ in range(n):out=mul(out,a)
 return out

y={(0,0),(1,0),(1,1)}
all_terms=set()
for ye,exps in f:
 yp=power(y,ye)
 for xe in exps:
  for xp,wp in yp:
   t=(xe+xp,wp)
   if t in all_terms:all_terms.remove(t)
   else:all_terms.add(t)
assert all(x>=3 for x,w in all_terms)
support=sorted((w,x-3) for x,w in all_terms)
assert [list(p) for p in support]==r['support']
assert len(support)==24
assert [w for w,x in support if x==0]==[3]
vertices=[tuple(p) for p in r['lower_hull']]
assert vertices==[(0,7),(1,2),(3,0),(7,4)]
for (wa,xa),(wb,xb) in zip(vertices,vertices[1:]):
 assert all((x-xa)*(wb-wa)>=(xb-xa)*(w-wa) for w,x in support)
for e in r['positive_edges']:
 (wa,xa),(wb,xb)=e['endpoints']
 slope=Fraction(xa-xb,wb-wa)
 assert str(slope)==e['w_valuation']
 on=sorted((w,x) for w,x in support if wa<=w<=wb and (x-xa)*(wb-wa)==(xb-xa)*(w-wa))
 assert [list(p) for p in on]==e['edge_support']
 assert [w-wa for w,x in on]==e['residual_exponents']
assert [e['residual_exponents'] for e in r['positive_edges']]==[[0,1],[0,2]]
print('BLOWUP112_INDEPENDENT_BIVARIATE_PASS')
