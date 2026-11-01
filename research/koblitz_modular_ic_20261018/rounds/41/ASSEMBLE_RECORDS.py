#!/usr/bin/env python3
"""Index exact terminal native/Lean receipts; no field or curve computation."""
from pathlib import Path
import json,re
h=Path(__file__).resolve().parent
def kv(path):return dict(s.split('=',1) for s in path.read_text().splitlines())
runs=[]
spec=[(f'signed41-shift{s}-01','SIGNED41') for s in range(3)]+[(f'cofactor41-shift{s}-01','COFACTOR41') for s in range(3)]+[('split41-01','SPLIT41'),('images41-01','IMAGES41')]
for name,tag in spec:
 p=h/'runs'/name;log=(p/'native.log').read_text()
 assert (p/'exit.txt').read_text().strip()=='0' and tag+'_ALL_CHECKS_PASS' in log
 assert 'panicked at' not in log
 rows=[s for s in log.splitlines() if s.startswith('QUOT36_QUOTIENT ')]
 assert rows and all('independent_products=all' in s for s in rows)
 runs.append({'attempt':name,'kind':'native','status':'accepted','native_exit':0,'source_revision':(p/'source-revision.txt').read_text().strip(),'completed_full_generator_quotients':len(rows),'observations':[s for s in log.splitlines() if s.startswith(tag+'_RESULT') or s.startswith(tag+'_ORDER') or s.startswith(tag+'_POLYNOMIAL')]})
for name,count in [('lean41-01',6),('independence41-01',3)]:
 p=h/'runs'/name;assert (p/'compile.exit').read_text().strip()=='0' and (p/'audit.exit').read_text().strip()=='0'
 audit=(p/'audit.log').read_text();assert f'audited {count} theorems' in audit and '0 failing, 0 trusted' in audit
 runs.append({'attempt':name,'kind':'Lean','status':'accepted','compile_exit':0,'audit_exit':0,'audited_declarations':count})
assert {r['attempt'] for r in runs}=={p.name for p in (h/'runs').iterdir() if p.is_dir()}
split=kv(h/'runs/split41-01/split-result.txt');image=kv(h/'runs/images41-01/image-result.txt')
assert split['gate']=='true' and split['component7_principal']=='false' and split['component8_principal']=='false'
assert image['operator7_principal']=='false' and image['operator8_principal']=='false'
scalar=lambda s:None if s=='none' else int(s)
result={'schema':'koblitz.round-results/v1','round':41,'runs':runs,'independent_collector_orbit':2,'signed_shifts':[0,1,2],'signed_expressions_per_shift':2,'signed_classes_principal':[False,False,False],'cofactor':3721,'independent_cofactor_expression':'61*(61*E_second)','cofactored_classes_nonzero':[True,True,True],'cofactored_classes_killed_by_11':[True,True,True],'quadratic_gate':'T²F+F=4TF','quadratic_gate_passed':True,'components':{'P7':'8F-TF','P8':'TF-7F','P7_nonzero':True,'P8_nonzero':True,'both_killed_by_11':True,'diamond_characters_checked':[7,8],'exact_recovery':True,'old_R_scalar_P7':scalar(image['component7_old_scalar']),'old_R_scalar_P8':scalar(image['component8_old_scalar']),'old_scalar_candidates_each':11},'source_operator_images':{'Pi7':'6F+2TF','Pi8':'3F+9TF','independent_forms':['9P7','9P8'],'both_nonzero':True,'recovery_scalar':5,'exact_recovery':True,'Frobenius_identity_interpretation':'F2-rational source divisor classes'},'native_full_generator_quotients':sum(r.get('completed_full_generator_quotients',0) for r in runs),'formal':{'audited_declarations':9,'native_arrays_formalized':False,'proper_geometry_formalized':False,'chosen_Koblitz_model_isogeny_formalized':False},'scope':{'one_independent_collector_orbit_evaluated':True,'chosen_Koblitz_model_isogeny_evaluated':False,'full_quotient_saturation':False,'proper_polarized_embedding':False,'larger_n_geometric_transfer_evaluated':False,'precision615_source_jets_constructed':False,'complete_degree460_or367_divisors_evaluated':False},'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False,'global_metrics_integration':'pending exclusive T-853'}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':41,'machine_index_created_after_data':True,'normative_prospective_protocols':['PROTOCOL.md','COFACTOR-PROTOCOL.md','SPLIT-PROTOCOL.md','IMAGES-PROTOCOL.md','LEAN-PROTOCOL.md','INDEPENDENCE-LEAN-PROTOCOL.md'],'claim_class':'exact computations and conditional algebra/group implications','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'version':'4.34.1','jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n')
print(json.dumps({'accepted_native':8,'accepted_Lean':2,'audited_declarations':9,'full_generator_quotients':result['native_full_generator_quotients'],'old_R_scalar_P7':result['components']['old_R_scalar_P7'],'old_R_scalar_P8':result['components']['old_R_scalar_P8']}))
