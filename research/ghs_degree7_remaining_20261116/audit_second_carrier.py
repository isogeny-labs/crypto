#!/usr/bin/env python3
"""Independent F3 coordinate substitutions and structural receipt audit."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
VAR=HERE.parent/'ghs_trace22_carrier_variants_20261116'
ORBIT=json.loads((HERE/'CARRIER_ORBITS.json').read_text())
assert ORBIT['source_sha256']==hashlib.sha256((HERE/'carrier_orbits.sage').read_bytes()).hexdigest()
assert ORBIT['pgl2_matrices']==24
assert ORBIT['components']==[
 [32280,36380,106121,113379,168265,175771],
 [35349,51009,67186,112490,127268,175537]]

def mul(f,g):
 out=[0]*(len(f)+len(g)-1)
 for i,a in enumerate(f):
  for j,b in enumerate(g):out[i+j]=(out[i+j]+a*b)%3
 return out
def add(f,g):
 out=[0]*max(len(f),len(g))
 for i,a in enumerate(f):out[i]=(out[i]+a)%3
 for i,b in enumerate(g):out[i]=(out[i]+b)%3
 return out
def powers(f):
 out=[[1]]
 for _ in range(12):out.append(mul(out[-1],f))
 return out
def coeff(mask):
 return [(mask//3**i)%3 for i in range(11)]+[1,0]

maps_checked=0
for rep in (32280,35349):
 for target,matrix in ORBIT['representative_maps'][str(rep)].items():
  a,b,c,d=matrix
  assert (a*d-b*c)%3!=0
  A=powers([b,a]);B=powers([d,c])
  out=[0]*13
  for i,v in enumerate(coeff(rep)[:12]):
   if v:
    term=mul(A[i],B[12-i])
    out=add(out,[(v*x)%3 for x in term])
  assert out==coeff(int(target)),(rep,target,matrix)
  maps_checked+=1
assert maps_checked==12

arms={
 'sparse':(HERE/'sparse_35349.sage',lambda n:HERE/f'sparse_35349_{n}.json'),
 'linear':(HERE/'linear_square_35349.sage',lambda n:HERE/f'linear_square_35349_{n}.json'),
 'quadratic':(VAR/'screen_variant.sage',lambda n:VAR/f'f35349_e{n}.json'),
 'cubic':(HERE/'cubic_square_35349.sage',lambda n:HERE/f'cubic_square_35349_{n}.json'),
 'two_poles':(HERE/'two_poles_linear_35349.sage',lambda n:HERE/f'two_poles_linear_35349_{n}.json'),
 'three_poles':(HERE/'three_poles_35349.sage',lambda n:HERE/f'three_poles_35349_{n}.json'),
}
rows=[]
for arm,(source,receipt_path) in arms.items():
 sh=hashlib.sha256(source.read_bytes()).hexdigest()
 for mask in (68,84):
  p=receipt_path(mask);raw=p.read_bytes();d=json.loads(raw)
  assert d['source_sha256']==sh,(arm,mask,'source hash')
  assert d['bmask']==mask and d['target_order']==222
  assert d['hits']==[],(arm,mask,'unexpected hit')
  c=d['counts']
  if arm=='sparse':
   assert [c[k]['branch_assignments'] for k in ('one','Qi','Qi3','QiQj')]==[243,405,405,270]
   assert all(v['full_equation_hits']==0 for v in c.values())
  elif arm=='linear':
   assert [c[k]['logical_assignments'] for k in ('H2','H4','H6','QiH2','QiH4')]==[58806]*3+[98010]*2
   assert all(v['full_equation_hits']==0 for v in c.values())
  elif arm=='quadratic':
   assert d['fmask']==35349 and d['allowed_H']==58559
   assert c['Hquadratic']['systems']==243 and c['QiHquadratic']['systems']==405
   assert all(v['full_equation_hits']==0 for v in c.values())
  elif arm=='cubic':
   assert c['systems']==243 and c['zero_dimensional']+c['positive_dimensional']+c['constant_branch_assignments']==243
   assert c['censored_systems']==0 and d['censored']==[]
   assert c['full_equation_hits']==0
  elif arm=='two_poles':
   assert c['pole_pairs']==10 and c['branch_assignments']==270
   assert c['denominator_choices']==65340 and c['logical_candidates']==7906140
   assert c['full_equation_hits']==0
  elif arm=='three_poles':
   assert c['denominators']==10 and c['branch_assignments']==90
   assert c['logical_candidates']==643043610 and c['bitset_controls']==1440
   assert c['full_equation_hits']==0
  rows.append({'arm':arm,'mask':mask,'receipt_sha256':hashlib.sha256(raw).hexdigest(),
               'source_sha256':sh,'cpu_seconds':d.get('cpu_seconds')})
assert len(rows)==12
print(json.dumps({'schema':'ghs.trace22.second-carrier-audit/v1',
                  'carrier_mask':35349,'independent_F3_maps_checked':maps_checked,
                  'receipt_count':len(rows),'rows':rows,
                  'limit':'coordinate substitutions independently recomputed; cover elimination structurally audited only'},
                 sort_keys=True,indent=2))
