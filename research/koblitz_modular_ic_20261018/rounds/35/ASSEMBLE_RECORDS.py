#!/usr/bin/env python3
"""Parse terminal receipts into an observation index; no mathematical replay."""
from pathlib import Path
import json,re
h=Path(__file__).resolve().parent
markers={'algebra35-01':'ALG35_ALL_CHECKS_PASS','jets35-01':'JET35_ALL_CHECKS_PASS','multiply35-01':'MULT35_ALL_CHECKS_PASS','inflate35-01':'INFLATE35_ALL_CHECKS_PASS'}
rows=[]
for p in sorted((h/'runs').iterdir()):
 if not p.is_dir():continue
 if p.name.startswith('lean'):
  assert (p/'audit.exit').exists(), 'Lean invocation is still live; do not classify pending as rejected'
  compile_exit=int((p/'compile.exit').read_text());audit_exit=int((p/'audit.exit').read_text()) if (p/'audit.exit').exists() else None
  row={'attempt':p.name,'compile_exit':compile_exit,'audit_exit':audit_exit,'status':'accepted' if compile_exit==audit_exit==0 else 'rejected'}
  if row['status']=='accepted':
   log=(p/'audit.log').read_text();assert '0 failing, 0 trusted' in log;row['audited_declarations']=int(re.search(r'audited (\d+) theorems',log).group(1))
 else:
  code=int((p/'exit.txt').read_text());log=(p/'native.log').read_text();errors=bool(re.search(r'^\s*\?',log,re.M));marker=markers[p.name]
  accepted=code==0 and marker in log.splitlines() and not errors
  row={'attempt':p.name,'status':'accepted' if accepted else ('capped' if code==124 else 'rejected'),'native_exit':code,'terminal_marker':marker in log.splitlines(),'native_errors':errors,'observations':[x for x in log.splitlines() if re.match(r'^(ALG35_RESULT|JET35_(STAGE|SOURCE|REGRESSION|RESULT)|MULT35_(INPUT|STAGE|RESULT)|INFLATE35_(DIVISOR|RESULT))',x)]}
 rows.append(row)
for name in markers:assert any(r['attempt']==name and r['status']=='accepted' for r in rows)
assert sum(x.startswith('INFLATE35_DIVISOR ') for x in (h/'runs/inflate35-01/native.log').read_text().splitlines())==10
accepted_lean=[r for r in rows if r['attempt'].startswith('lean') and r['status']=='accepted']
result={'schema':'koblitz.round-results/v1','round':35,'runs':rows,'construction_counts':{'closed_point_algebra_dimension':770,'source_jet_precision':154,'complete_base_sections':68,'complete_quadratic_sections':196,'large_model_sections':324,'multiplication_space_dimension':708,'complete_multiplication_products':52650,'independently_checked_multiplication_products':52650,'large_model_vanishing_spaces':10,'large_model_vanishing_dimension':196,'inflation_products':46240,'independently_checked_inflation_products':46240},'controls':{'native_field_products':1024,'basis_inverse_recombinations':770,'all_source_equations':5,'registered_units':8,'coefficient_conjugates':5,'all_old_precision9_values_reproduced':True,'native_corrupt_controls_rejected':True},'formal':{'accepted_runs':len(accepted_lean),'audited_declarations':sum(r.get('audited_declarations',0) for r in accepted_lean),'native_arrays_imported':False,'proper_geometry_formalized':False},'geometry':{'complete_RR_interpretation':'conditional on retained proper/local/pole bridge in R33/R34','signed_Picard_class_evaluated':False,'desired_modular_factor_image':None,'full_quotient_saturation_proved':False,'proper_polarized_factor_embedding':False},'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False,'root_library_suite_passed':False}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':35,'machine_index_created_after_data':True,'normative_prospective_protocols':sorted(p.name for p in h.glob('*PROTOCOL.md')),'claim_class':'exact_computation and scoped algebraic theorem','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'version':'4.34.1','mathlib':'d13f23b723b8a846827a245b89c10fc7d3f11612','jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n')
print(json.dumps({'attempts':len(rows),'accepted':sum(r['status']=='accepted' for r in rows),'Lean_declarations':result['formal']['audited_declarations']}))
