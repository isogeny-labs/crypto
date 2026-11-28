#!/usr/bin/env python3
"""Independent binomial expansion and supporting-line verifier for R111."""
import hashlib
import importlib.util
import json
import math
from fractions import Fraction
from pathlib import Path
root=Path(__file__).resolve().parent
rounds=root.parent
r=json.loads((root/'result.json').read_text())
donor=rounds/'51/runs/base23-struct51-03/marked23-triangular51.sing'
parser=rounds/'105/jets105.py'
for name,p in [('donor51',donor),('parser105',parser)]:
 assert hashlib.sha256(p.read_bytes()).hexdigest()==r['source_sha256'][name]
spec=importlib.util.spec_from_file_location('jets105',parser)
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
f,g=m.parse_source(donor.read_text())
assert len(f)==8 and len(g)==29
for key,shift in [('y0',0),('y1',1)]:
 support={}
 for ye,exps in f:
  for xe in exps:
   for ze in range(ye+1):
    if shift==0 and ze!=ye:continue
    if shift==1 and not(math.comb(ye,ze)&1):continue
    pair=(ze,xe);support[pair]=support.get(pair,0)^1
 pts=sorted(p for p,v in support.items() if v)
 assert [list(p) for p in pts]==r[key]['f_support']
 vertices=[tuple(p) for p in r[key]['lower_hull']]
 assert vertices[0][0]==0 and vertices[-1][0]==7
 for (za,xa),(zb,xb) in zip(vertices,vertices[1:]):
  assert za<zb
  # Every monomial lies on or above each reported lower supporting line.
  assert all((x-xa)*(zb-za)>=(xb-xa)*(z-za) for z,x in pts)
 edges=r[key]['positive_edges']
 for e in edges:
  (za,xa),(zb,xb)=e['endpoints']
  slope=Fraction(xa-xb,zb-za)
  assert str(slope)==e['z_valuation'] and slope>0
  on=sorted((z,x) for z,x in pts if za<=z<=zb and (x-xa)*(zb-za)==(xb-xa)*(z-za))
  assert [list(p) for p in on]==e['edge_support']
  assert [z-za for z,x in on]==e['residual_exponents']
  bounds={}
  vy=slope if shift==0 else Fraction(0)
  for le,ye,den,exps in g:
   w=Fraction(min(exps)-den)+ye*vy
   bounds[le]=min(bounds.get(le,w),w)
  got=r[key]['g_coefficient_lower_bounds'][edges.index(e)]['bound_by_l_exponent']
  assert got=={str(j):str(bounds[j]) for j in sorted(bounds)}
 assert len(edges)==(2 if shift==0 else 1)
 if shift==0:
  assert [e['residual_exponents'] for e in edges]==[[0,1],[0,3]]
 else:
  assert edges[0]['residual_exponents']==[0,1,2,3]
print('NEWTON111_INDEPENDENT_SUPPORT_PASS')
