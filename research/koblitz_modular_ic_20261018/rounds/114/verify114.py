#!/usr/bin/env python3
"""Independent original-donor bivariate composition for R114."""
import hashlib
import importlib.util
import json
from fractions import Fraction
from pathlib import Path
root=Path(__file__).resolve().parent;rounds=root.parent
r=json.loads((root/'result.json').read_text())
paths={'donor51':rounds/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'parser105':rounds/'105/jets105.py','result113':rounds/'113/result.json'}
for name,p in paths.items():assert hashlib.sha256(p.read_bytes()).hexdigest()==r['source_sha256'][name]
spec=importlib.util.spec_from_file_location('jets105',paths['parser105'])
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
f,g=m.parse_source(paths['donor51'].read_text())

def mul(a,b):
 out=set()
 for ra,ha in a:
  for rb,hb in b:
   p=(ra+rb,ha+hb)
   if p in out:out.remove(p)
   else:out.add(p)
 return out

def power(a,n):
 out={(0,0)}
 for _ in range(n):out=mul(out,a)
 return out

y={(0,0),(2,0),(4,0),(7,0),(7,1)}
terms=set()
for ye,exps in f:
 yp=power(y,ye)
 for xe in exps:
  for re,he in yp:
   p=(re+2*xe,he)
   if p in terms:terms.remove(p)
   else:terms.add(p)
assert all(re>=18 for re,he in terms)
pts=sorted((he,re-18) for re,he in terms)
assert [list(p) for p in pts]==r['support']
assert len(pts)==74 and [he for he,re in pts if re==0]==[2]
vertices=[tuple(p) for p in r['lower_hull']]
assert vertices==[(0,3),(2,0),(3,3),(7,31)]
for (ha,ra),(hb,rb) in zip(vertices,vertices[1:]):
 assert all((re-ra)*(hb-ha)>=(rb-ra)*(he-ha) for he,re in pts)
assert len(r['positive_edges'])==1
edge=r['positive_edges'][0]
assert edge['h_valuation']=='3/2' and edge['residual_exponents']==[0,2]
assert [re for he,re in pts if he==1][0]==3
print('WILD114_INDEPENDENT_DONOR_PASS')
