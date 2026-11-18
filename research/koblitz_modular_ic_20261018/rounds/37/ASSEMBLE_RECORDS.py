#!/usr/bin/env python3
"""Index exact terminal native/Lean receipts; no mathematical replay."""
from pathlib import Path
import json,re
h=Path(__file__).resolve().parent;rows=[];quotients=[]
for p in sorted((h/'runs').iterdir()):
 if not p.is_dir():continue
 if 'lean' in p.name:
  assert (p/'compile.exit').exists(),p.name+' still live'
  code=int((p/'compile.exit').read_text());row={'attempt':p.name,'kind':'Lean','compile_exit':code}
  if code==0:
   assert (p/'audit.exit').exists(),p.name+' audit still live';audit=int((p/'audit.exit').read_text());row['audit_exit']=audit;row['status']='accepted' if audit==0 else 'rejected'
   if audit==0:
    log=(p/'audit.log').read_text();assert '0 failing, 0 trusted' in log;row['audited_declarations']=int(re.search(r'audited (\d+) theorems',log).group(1))
  else:row.update(status='rejected',audit_exit=None,reason=next((x for x in (p/'compile.log').read_text().splitlines() if 'error:' in x),'compile failed; see retained log'))
 else:
  assert (p/'exit.txt').exists(),p.name+' still live'
  code=int((p/'exit.txt').read_text());log=(p/'native.log').read_text();marker=p.name.split('-')[0].upper()+'_ALL_CHECKS_PASS';errors=bool(re.search(r'^\s*\?',log,re.M));accepted=code==0 and marker in log.splitlines() and not errors
  row={'attempt':p.name,'kind':'native','native_exit':code,'terminal_marker':marker in log.splitlines(),'Singular_errors':errors,'status':'accepted' if accepted else ('capped' if code==124 else 'rejected'),'observations':[s for s in log.splitlines() if re.match(r'^(ORDER37_|SIGNED37_(INPUT|EQUALITY|RESULT)|TORSION37_RESULT|COFACTOR37_(NORMS|RESULT)|ISOLATE37_RESULT|CLASSORDER37_RESULT)',s)]}
  for line in log.splitlines():
   if line.startswith('QUOT36_QUOTIENT '):quotients.append({'attempt':p.name,**dict(x.split('=',1) for x in line.split()[1:])})
 rows.append(row)
assert {'order37-01','signed37-01','torsion37-01','cofactor37-01','isolate37-01','classorder37-01','lean37-01','torsionlean37-01','torsionlean37-02','classorderlean37-01','classorderlean37-02'} <= {r['attempt'] for r in rows}
signed=(h/'runs/signed37-01/native.log').read_text();torsion=(h/'runs/torsion37-01/native.log').read_text();iso=(h/'runs/isolate37-01/native.log').read_text()
assert 'signed_class_principal=false' in signed and 'independent_expression_equal=true' in signed
assert 'eleven_kills=false five_kills=false' in torsion
fields=dict(x.split('=',1) for x in next(x for x in iso.splitlines() if x.startswith('ISOLATE37_RESULT ')).split()[1:]);fields={k:(v=='true' if v in ('true','false') else int(v) if v.isdigit() else v) for k,v in fields.items()}
orderlog=(h/'runs/classorder37-01/native.log').read_text();assert 'exact_source_order_under_retained_bridge=671 least_scalar_to_eleven_torsion=61' in orderlog
result={'schema':'koblitz.round-results/v1','round':37,'runs':rows,'construction_counts':{'actual_deck_labels_bound':10,'literal_old_section_bindings':680,'nonprincipal_signed_source_classes':1,'ordinary_eleven_source_classes':1,'exact_subspace_divisions':len(quotients),'coefficient_field_dimension':16,'complementary_Euler_degree':32},'signed_class':{'weights':[4,1,-1,1,-1,-4,-1,1,-1,1],'actual_deck_order':['r1-f2','r3-f2','r4-f3','r2-f3','r5-f2','r1-f3','r3-f3','r4-f2','r2-f2','r5-f3'],'all_Frobenius_phases':0,'principal':False,'independent_expressions_equal':True,'eleven_annihilates':False,'five_annihilates':False,'exact_order_under_retained_bridge':671,'least_scalar_to_eleven_torsion':61,'finite_cyclic_projector_scalar':122},'complementary_factor':{'label':'77.2.l.b','norm_at_plus_one':125238481,'norm_at_minus_one':3721,'norm_minus_mod11':3,'full_basis_inverse_checks':16,'independent_norms':2,'modular_proper_twist_binding':'retained written bridge, model-annihilator check below'},'cofactor_class_tests':fields,'source_order_test':{'six_seventy_one_principal':True,'exact_order':671,'least_scalar_to_eleven_torsion':61},'quotient_receipts':quotients,'formal':{'accepted_runs':sum(r['kind']=='Lean' and r['status']=='accepted' for r in rows),'rejected_runs':sum(r['kind']=='Lean' and r['status']=='rejected' for r in rows),'audited_declarations':sum(r.get('audited_declarations',0) for r in rows),'native_arrays_formalized':False,'proper_geometry_formalized':False,'exact_order_eleven_lemma_applied_to_original_E':False,'exact_order_eleven_lemma_applied_to_cofactor_class':True},'intended_sign_factor_image':None,'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False,'global_metrics_integration':'pending exclusive scope T-865; scoped patch retained'}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n');(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':37,'machine_index_created_after_data':True,'normative_prospective_protocols':sorted(p.name for p in h.glob('*PROTOCOL.md')),'claim_class':'exact_computation and scoped group identities','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'version':'4.34.1','jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n');print(json.dumps({'attempts':len(rows),'accepted':sum(x['status']=='accepted' for x in rows),'quotients':len(quotients),'cofactor_result':fields}))
