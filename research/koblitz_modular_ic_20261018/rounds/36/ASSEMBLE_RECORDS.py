#!/usr/bin/env python3
"""Index retained exact-native observations, without replaying mathematics."""
from pathlib import Path
import json,re
h=Path(__file__).resolve().parent;rows=[]
for name,marker in [('quotient36-01','QUOT36_ALL_CHECKS_PASS'),('group36-01','GROUP36_ALL_CHECKS_PASS')]:
 p=h/'runs'/name;log=(p/'native.log').read_text();assert (p/'build.exit').read_text().strip()=='0' and (p/'exit.txt').read_text().strip()=='0' and marker in log.splitlines()
 rows.append({'attempt':name,'status':'accepted','build_exit':0,'native_exit':0,'observations':[s for s in log.splitlines() if s.startswith(('QUOT36_','GROUP36_'))]})
assert {r['attempt'] for r in rows}=={p.name for p in(h/'runs').iterdir() if p.is_dir()}
result={'schema':'koblitz.round-results/v1','round':36,'runs':rows,'construction_counts':{'closed_point_algebra_dimension':770,'large_model_dimension':324,'complete_divisor_dimension':196,'flip_dimension':68,'same_section_double_flip_dimension':196,'exact_subspace_divisions':12,'actual_source_classes_tested':1},'class_tests':{'self_equality_dimension':1,'source_versus_zero_dimension':0,'negative_versus_zero_dimension':0,'add_to_inverse_versus_zero_dimension':1,'same_section_double_flip_equals_original':True,'signed_E_sum_evaluated':False,'desired_order11_image':None},'controls':{'all_original_generator_memberships_checked':True,'all_construction_and_final_products_independent_arithmetic':True,'all_kernel_recombinations_and_ranknullity':True,'nonunit_section_used':True,'nonmembership_control_rejected':True},'proof_scope':{'geometric_class_interpretation':'uses retained proper/local and complete RR/pole bridge','native_algorithm_formalized_in_Lean':False,'new_Lean_theorems':0,'R35_coefficient_lift_proofs_separate':True},'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False}
# Derive only receipt counters, not new mathematical data.
quotients=[]
for row in rows:
 for s in row['observations']:
  if s.startswith('QUOT36_QUOTIENT '):
   fields=dict(x.split('=',1) for x in s.split()[1:]);quotients.append(fields)
assert len(quotients)==12
result['quotient_receipts']=quotients
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':36,'machine_index_created_after_data':True,'normative_prospective_protocols':['PROTOCOL.md','GROUP-PROTOCOL.md'],'claim_class':'exact_computation','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n')
print(json.dumps({'runs':len(rows),'exact_subspace_divisions':len(quotients),'all_accepted':True}))
