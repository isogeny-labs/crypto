#!/usr/bin/env python3
"""Structural audit for three degree-eight trace-minus14 denominator shapes."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
CELLS=((9735,11),(17371,11),(9735,202),(17371,202),
       (9735,38),(17371,38),(9735,80),(17371,80))
ARMS={
 'allcell':('search_all.sage','9c7a8d9a63fac4e4a54ec91f4f8aeabff3c5b97fb64d37e008386631942a1b11','ghs.trace-minus14.degree8-quadratic-square-orbits/v1'),
 'l3cell':('search_L3.sage','4e4105547f3396c3c762b81c577d37b9751a4f47b23b35ecc25205e82971c0cc','ghs.trace-minus14.degree8-L3-quadratic-square-orbits/v1'),
 'l5cell':('search_L5linear.sage','ee529a87e23a7cb1a39bf8d51563ab27a237eecc3bb6f2f5f568497a8a25fbfc','ghs.trace-minus14.degree8-L5-linear-square-orbits/v1')}

def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_text())

for _,(source,h,_) in ARMS.items():assert digest(HERE/source)==h
prev=read(ROOT/'research/ghs_trace_minus14_degree7_orbits_20261116/AUDIT.json')
assert prev['exact_equation_pairs_covered']==240 and prev['degree_window_all_recorded_pairs']==[8,33]
rows=[]
for f,b in CELLS:
 for name,(_,h,schema) in ARMS.items():
  p=HERE/f'{name}_{f}_{b}.json'
  out=HERE/f'{name}_{f}_{b}.stdout'
  err=HERE/f'{name}_{f}_{b}.stderr'
  d=read(p)
  assert err.exists() and err.read_text()==''
  progress=[json.loads(line) for line in out.read_text().splitlines()]
  assert progress[-1]['phase']=='complete' and progress[-1]['counts']==d['counts']
  assert d['schema']==schema and d['source_sha256']==h
  assert d['fmask']==f and d['bmask']==b
  assert d['field']=='F3[a]/(a^5+2a+1)' and d['target_order']==258 and d['trace']==-14
  assert d['hits']==[] and d['counts']['exact_hits']==0
  assert d['counts']['branch_assignments' if name=='l5cell' else 'systems']==243
  if name=='l5cell':assert d['counts']['parameter_pairs']==58806
  else:assert d['counts']['u_specializations']==59049
  assert d['counts']['point_survivors']==d['counts']['f_divides']
  rows.append({'carrier_mask':f,'source_mask':b,'arm':name,'source_sha256':h,
               'counts':d['counts'],'cpu_seconds':d['cpu_seconds']})
assert len(rows)==24
out={'schema':'ghs.trace-minus14.degree8-three-shape-audit/v1',
     'audit_source_sha256':digest(Path(__file__)),
     'representative_cells':8,'exact_equation_pairs_represented':240,
     'shapes':list(ARMS),'receipts':len(rows),'exact_maps':0,
     'claim_boundary':'three named degree-eight denominator shapes only; one Sage implementation per shape',
     'rows':rows}
print(json.dumps(out,sort_keys=True,indent=2))
