#!/usr/bin/env python3
"""Read retained native/Lean receipts; metadata only, never recompute mathematics."""
from pathlib import Path
import json,re
h=Path(__file__).resolve().parent
markers={'desc33-01':'DESC33_ALL_CHECKS_PASS','desc33-02':'DESC33_ALL_CHECKS_PASS','eval33-01':'EVAL33_ALL_CHECKS_PASS','pad33-01':'PAD33_ALL_CHECKS_PASS','local33-generate01':'LOCAL33_GENERATE_ALL_CHECKS_PASS','local33-check83-01':'LOCAL33_CHECK83_ALL_CHECKS_PASS','local33-compact-dump01':'LOCAL33_COMPACT_DUMP_ALL_CHECKS_PASS','local33-function01':'LOCAL33_FUNCTION_ALL_CHECKS_PASS','local33-compact-check5-01':'LOCAL33_COMPACT_CHECK5_ALL_CHECKS_PASS','padding-deck33-01':'PAD33_DECK_ALL_CHECKS_PASS','padding-deck33-02':'PAD33_DECK_ALL_CHECKS_PASS','local33-diagnostic01':'LOCAL33_DIAGNOSTIC_ALL_CHECKS_PASS','local33-diagnostic02':'LOCAL33_DIAGNOSTIC_ALL_CHECKS_PASS','local33-quotient01':'LOCAL33_QUOTIENT_ALL_CHECKS_PASS','quotient33-denominators01':'QUOTIENT33_DENOMINATORS_ALL_CHECKS_PASS','quotient33-function01':'QUOTIENT33_FUNCTION_ALL_CHECKS_PASS','quotient33-check5-01':'QUOTIENT33_CHECK5_ALL_CHECKS_PASS','quotient33-check83-01':'QUOTIENT33_CHECK83_ALL_CHECKS_PASS','quadrics33-01':'QUADRICS33_ALL_CHECKS_PASS'}
markers.update({'padding33-tangent01':'PADDING33_TANGENT_ALL_CHECKS_PASS','quadrics33-02':'QUADRICS33_ALL_CHECKS_PASS','jets33-01':'JETS33_ALL_CHECKS_PASS'})
rows=[]
for p in sorted((h/'runs').iterdir()):
 if not p.is_dir():continue
 row={'attempt':p.name}
 if p.name.startswith('lean'):
  for key in ['compile','audit']:
   f=p/(key+'.exit');row[key+'_exit']=int(f.read_text()) if f.exists() else None
  row['status']='accepted' if row['compile_exit']==0 and row['audit_exit']==0 else ('pending' if row['compile_exit'] is None or (row['compile_exit']==0 and row['audit_exit'] is None) else 'rejected')
  if (p/'audit.log').exists():
   m=re.search(r'audited (\d+) theorems.*; (\d+) failing, (\d+) trusted', (p/'audit.log').read_text())
   if m:row.update(audited_declarations=int(m[1]),failing=int(m[2]),trusted=int(m[3]))
 else:
  assert p.name in markers,p.name
  f=p/'exit.txt';row['native_exit']=int(f.read_text()) if f.exists() else None
  log=(p/'native.log').read_text() if (p/'native.log').exists() else ''
  row['singular_errors']=bool(re.search(r'^\s*\?',log,re.M));row['terminal_marker']=markers[p.name] in log.splitlines()
  row['status']='pending' if row['native_exit'] is None else ('accepted' if row['native_exit']==0 and not row['singular_errors'] and row['terminal_marker'] else ('capped' if row['native_exit']==124 else 'rejected'))
  row['recorded_results']=[s for s in log.splitlines() if re.match(r'^(DESC33_PRODUCT_RESULT|EVAL33_RESULT|PAD33_RESULT|PAD33_DECK_RESULT|LOCAL33_POINT_DIAGNOSTIC|LOCAL33_GENERIC_DIAGNOSTIC|LOCAL33_DIRECT_QUOTIENT|QUADRICS33_RESULT|QUOTIENT33_LOCAL_CHECK|PADDING33_TOTAL_TANGENT|JETS33_RESULT)',s)]
 rows.append(row)
def accepted(name):return next(r for r in rows if r['attempt']==name)['status']=='accepted'
assert all(accepted(n) for n in ['desc33-02','eval33-01','pad33-01','padding-deck33-02','local33-diagnostic02','lean33-02','lean-local33-01','quadrics33-02','padding33-tangent01'])
result={'schema':'koblitz.round-results/v1','round':33,'status':'construction progress; geometric and target-factor gates remain','construction_counts':{'descended_sections':68,'quadratic_space_dimension':196,'cubic_space_dimension':324,'evaluated_degree83_points':10,'padding_closed_points':2,'padding_point_degree':5,'twist_projective_quadrics':2150},'runs':rows,'evaluation':{'source_support_degree':83,'section_dimension':68,'section_evaluation_rank':68,'quadratic_dimension':196,'quadratic_evaluation_rank':83,'quadratic_kernel_dimension':113,'all_ten_points_checked':True,'IC_relation_rank':None,'wrapper_reconciliation':'eval33-wrapper-reconciliation.json'},'local_geometry':{'padding_maximal_minors_unit':True,'padding_total_tangent_rank':3,'padding_tangent_only_not_dimension_proof':True,'generic_fiber_dimension_over_F2_u':32,'compact_syzygy_presentation_accepted':False,'global_normalization_complete':False,'proper_polarized_factor_embedding':False},'formal':{'named_theorems':6,'audited_declarations':7,'native_arrays_imported':False,'geometric_bridge_formalized':False},'performance_measured':False,'public_input_recoveries':0,'independent_scientific_replay':False,'full_root_suite_passed':False}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
protocols=sorted(p.name for p in h.glob('*PROTOCOL.md'))
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':33,'machine_index_created_after_data':True,'normative_prospective_records':protocols,'claim_class':'exact_computation and explicitly conditional theorem','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_claims':False,'comparative_timings':False},indent=2)+'\n')
print(json.dumps({'attempts':len(rows),'pending':[r['attempt'] for r in rows if r['status']=='pending']}))
