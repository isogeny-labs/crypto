#!/usr/bin/env python3
"""Source-bound structural audit of the trace-minus14 degree-seven orbit receipts."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OLD=ROOT/'research/ghs_trace_minus14_20261116'
PREV=ROOT/'research/ghs_trace_minus14_degree6_orbits_20261116'
CELLS=((11,17371),(202,9735),(202,17371),(38,9735),(38,17371),(80,9735),(80,17371))
SOURCES={
 'cell':('screen.sage','d573256afd8e22dafc5cb0c9717480630d07020821b927c9685f18672d30ed87'),
 'sparse':('sparse.sage','50050f0425d85206cc86d4a0a1e46ab6a2c7446f4c209f3ea011325445fa9754'),
 'linear_square':('linear_square.sage','33b74175dd81e7aa285a53d48e3773c18acdd2ad83473c07c283b34fe6030add'),
 'cubic_square':('cubic_square.sage','2baeaaf16a6e38ba665fea287e2084d32f87a12ecada7f977254d82835ba942e'),
 'two_poles_linear':('two_poles_linear.sage','1e97198f2d8cdf908eabf30390f2f3c09614514acc99d87840c475b6045b3cbe'),
 'three_poles':('three_poles.sage','f87aec9025c16ac94ab97ca00ac1e0721c6bfab82657a10debdec27fd12efb10')}
SCHEMAS={
 'cell':'ghs.odd5.trace-minus14-degree7-quadratic-square/v1',
 'sparse':'ghs.degree7.trace-minus14-orbit-sparse/v1',
 'linear_square':'ghs.degree7.trace-minus14-orbit-linear-square/v1',
 'cubic_square':'ghs.degree7.orbit-cubic-square/v1',
 'two_poles_linear':'ghs.degree7.two-poles-linear/v1',
 'three_poles':'ghs.degree7.three-poles/v1'}

def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_text())

for name,(file,h) in SOURCES.items():assert digest(HERE/file)==h
prev=read(PREV/'AUDIT.json')
assert prev['exact_equation_pairs']==240 and prev['representative_cells']==8
old=read(OLD/'AUDIT.json')
for name in ('sparse','linear_square','quadratic_square','cubic_square','two_poles_linear','three_poles'):
 assert old['checks'][name]['hits']==0
assert old['checks']['cubic_square']['counts']['censored_systems']==0
rows=[]
for b,f in CELLS:
 for name,(file,h) in SOURCES.items():
  p=HERE/f'{name}_{b}_{f}.json'; err=HERE/f'{name}_{b}_{f}.stderr'
  d=read(p)
  assert err.exists() and 'Traceback' not in err.read_text()
  assert d['source_sha256']==h and d['schema']==SCHEMAS[name]
  assert d['bmask']==b and d['carrier_mask']==f
  assert d['target_order']==258 and d['field']=='F3[a]/(a^5+2a+1)'
  assert d['hits']==[]
  if name=='cell':
   assert d['trace']==-14 and d['allowed_H']==58559
   assert d['counts']['Hquadratic']['systems']==243
   assert d['counts']['QiHquadratic']['systems']==405
   assert d['counts']['Hquadratic']['logical_assignments']==14229837
   assert d['counts']['QiHquadratic']['logical_assignments']==23716395
  elif name=='sparse':
   assert [d['counts'][s]['branch_assignments'] for s in ('one','Qi','Qi3','QiQj')]==[243,405,405,270]
  elif name=='linear_square':
   assert [d['counts'][s]['logical_assignments'] for s in ('H2','H4','H6','QiH2','QiH4')]==[58806,58806,58806,98010,98010]
  elif name=='cubic_square':
   assert d['counts']['systems']==243 and d['counts']['censored_systems']==0 and d['censored']==[]
   assert d['counts']['zero_dimensional']+d['counts']['positive_dimensional']+d['counts']['constant_branch_assignments']==243
  elif name=='two_poles_linear':
   assert d['counts']['pole_pairs']==10 and d['counts']['branch_assignments']==270
   assert d['counts']['logical_candidates']==7906140
  elif name=='three_poles':
   assert d['counts']['denominators']==10 and d['counts']['branch_assignments']==90
   assert d['counts']['logical_candidates']==643043610 and d['counts']['bitset_controls']==1440
  if name in ('cell','sparse','linear_square'):
   assert all(v['full_equation_hits']==0 for v in d['counts'].values())
  else:assert d['counts']['full_equation_hits']==0
  rows.append({'source_mask':b,'carrier_mask':f,'arm':name,'source_sha256':h,'exact_maps':0,'cpu_seconds':d.get('cpu_seconds')})
assert len(rows)==42
out={'schema':'ghs.trace-minus14.degree7-orbit-audit/v1',
     'audit_source_sha256':digest(Path(__file__)),
     'cells_new':7,'cells_total_with_prior':8,'new_receipts':len(rows),
     'exact_equation_pairs_covered':240,'exact_maps':0,
     'degree_window_all_recorded_pairs':[8,33],
     'claim_boundary':'computer-assisted degree-seven K-morphism exclusion under prior geometric denominator partition; one Sage elimination implementation per arm',
     'rows':rows}
print(json.dumps(out,sort_keys=True,indent=2))
