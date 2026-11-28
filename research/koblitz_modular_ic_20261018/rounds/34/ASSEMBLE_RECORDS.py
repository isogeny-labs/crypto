#!/usr/bin/env python3
"""Read native/Lean receipts and index exact outcomes; no mathematical replay."""
from pathlib import Path
import re,json
h=Path(__file__).resolve().parent
markers={'prefix34-verify01':'PREFIX34_DENOMINATORS_ALL_CHECKS_PASS','prefix34-function01':'PREFIX34_FUNCTION_ALL_CHECKS_PASS','prefix34-check5-01':'PREFIX34_CHECK5_ALL_CHECKS_PASS','prefix34-check83-01':'PREFIX34_CHECK83_ALL_CHECKS_PASS','prefix34-check83-02':'PREFIX34_CHECK83_ALL_CHECKS_PASS','primitive34-01':'PRIMITIVE34_ALL_CHECKS_PASS','qpoly34-01':'QPOLY34_ALL_CHECKS_PASS','qcolon34-01':'QCOLON34_ALL_CHECKS_PASS','qcolonpoint34-01':'QCOLONPOINT34_ALL_CHECKS_PASS','jets34-01':'JETS34_ALL_CHECKS_PASS','couple34-01':'COUPLE34_ALL_CHECKS_PASS'}
rows=[]
for p in sorted((h/'runs').iterdir()):
 if not p.is_dir():continue
 if p.name.startswith('lean'):
  row={'attempt':p.name,'compile_exit':int((p/'compile.exit').read_text()),'audit_exit':int((p/'audit.exit').read_text())};assert row['compile_exit']==row['audit_exit']==0
  assert 'audited 2 theorems' in (p/'audit.log').read_text() and '0 failing, 0 trusted' in (p/'audit.log').read_text();row.update(status='accepted',audited_declarations=2,trusted=0)
 else:
  assert p.name in markers;pcode=int((p/'exit.txt').read_text());log=(p/'native.log').read_text();errors=bool(re.search(r'^\s*\?',log,re.M));terminal=markers[p.name] in log.splitlines()
  status='accepted' if pcode==0 and terminal and not errors else ('capped' if pcode==124 else 'rejected')
  row={'attempt':p.name,'status':status,'native_exit':pcode,'Singular_errors':errors,'terminal_marker':terminal,'observations':[s for s in log.splitlines() if re.match(r'^(PREFIX34_(DENOMINATORS|POINT)|PRIMITIVE34_RESULT|QPOLY34_(BASE_MINIMAL|POINT)|QCOLON34_RESULT|QCOLONPOINT34_POINT|JETS34_CM_BASE|JETS33_RESULT|COUPLE34_RESULT)',s)]}
 rows.append(row)
assert len(rows)==12
assert [r['attempt'] for r in rows if r['status']!='accepted']==['prefix34-check83-01']
p83=(h/'runs/prefix34-check83-02/native.log').read_text();q=(h/'runs/qcolonpoint34-01/native.log').read_text();coupled=(h/'runs/couple34-01/native.log').read_text()
assert len(re.findall(r'^PREFIX34_POINT .*presentation=standard_etale_over_u$',p83,re.M))==10
assert len(re.findall(r'^QCOLONPOINT34_POINT q=[12] products=148 presentation=1 all_syzygy_values_zero=0$',q,re.M))==2
assert len(re.findall(r'^COUPLE34_RESULT .* rank=128 kernel=68 original_recombination=all corrupt_control=rejected$',coupled,re.M))==10
result={'schema':'koblitz.round-results/v1','round':34,'runs':rows,'accepted_native':10,'rejected_native':1,'capped_native':0,'accepted_Lean':1,'construction_counts':{'local_source_presentations':12,'degree83_source_presentations':10,'degree5_padding_presentations':2,'new_integral_syzygies':20,'total_integral_syzygies':37,'complete_coupled_divisors':10,'coupled_kernel_dimension':68},'coefficient_domain':{'distinct_denominator_expressions':682,'padding_zero_entries_each':566,'padding_generic_basis_admissible':False,'all_degree83_domains_admissible':True},'q_colon':{'parameter':'u^5+u^3+1','steps':1,'old_columns':17,'new_columns':20,'all_old_columns_zero_at_both_Q':True,'all_new_source_identities_checked_before_specialization':True,'full_q_infinity_saturation_proved':False,'full_quotient_variety_saturation_proved':False},'jets':{'precision':9,'closed_point_degree':5,'constraints':45,'rank':45,'kernel':151,'all_source_residuals_units_conjugates_checked':True,'j_invariant_at_Q':1},'coupled_divisors':{'point_degree':83,'padding':'9Q','padding_degree':45,'effective_divisor_degree':128,'ambient_RR_dimension':196,'evaluation_rank':128,'kernel_dimension':68,'all_ten_complete':True,'individual_nonprincipality':'written consequence using prior L(D0-P)=0 and the proper/RR bridge','projected_E_class_nonzero':None,'desired_modular_factor_image':None},'geometry':{'local_dimension_one_at_selected_supports':'written standard-etale consequence of native identities','proper_point_binding':'uses retained proper function-field degree32 bridge; see LOCAL_GEOMETRY.md','new_global_normalization_algorithm':False,'proper_polarized_factor_embedding':False},'formal':{'new_named_theorems':2,'audited_declarations':2,'failing':0,'trusted':0,'native_arrays_imported':False,'geometry_formalized':False},'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False,'root_library_suite_passed':False}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':34,'machine_index_created_after_data':True,'normative_prospective_protocols':sorted(p.name for p in h.glob('*PROTOCOL.md')),'claim_class':'exact_computation and scoped written theorem','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'version':'4.34.1','mathlib':'d13f23b723b8a846827a245b89c10fc7d3f11612','jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n')
print(json.dumps({'attempts':len(rows),'native_accepted':10,'native_rejected':1,'Lean_accepted':1,'coupled_divisors':10}))
