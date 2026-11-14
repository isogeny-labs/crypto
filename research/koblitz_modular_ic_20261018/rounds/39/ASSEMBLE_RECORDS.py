#!/usr/bin/env python3
"""Index exact terminal receipts; no field or curve arithmetic."""
from pathlib import Path
import json,re
h=Path(__file__).resolve().parent
native=[('encode39-01','ENC39'),('diamond39-01','DIAMOND39'),('collect39-01','COLLECT39'),('joint39-01','JOINT39'),('local39-01','LOCAL39')]
runs=[]
for name,tag in native:
 p=h/'runs'/name;log=(p/'native.log').read_text()
 assert (p/'exit.txt').read_text().strip()=='0'
 assert tag+'_ALL_CHECKS_PASS' in log and not re.search(r'^\s*\?',log,re.M)
 runs.append({'attempt':name,'kind':'native','status':'accepted','native_exit':0,
 'source_revision':(p/'source-revision.txt').read_text().strip(),
 'observations':[s for s in log.splitlines() if s.startswith(tag+'_RESULT') or s.startswith(tag+'_GLOBAL') or s.startswith(tag+'_FIXED')]})
local=(h/'runs/local39-01/native.log').read_text().splitlines()
assert len([s for s in local if 'presentation=standard_etale_over_u' in s])==60
assert not any('presentation=untested' in s or 'presentation=not_certified' in s for s in local)
for p in sorted((h/'runs').glob('lean39-*')):
 c=int((p/'compile.exit').read_text())
 a=int((p/'audit.exit').read_text()) if (p/'audit.exit').exists() else None
 ok=c==0 and a==0
 row={'attempt':p.name,'kind':'Lean','status':'accepted' if ok else 'rejected','compile_exit':c,'audit_exit':a}
 if ok:
  audit=(p/'audit.log').read_text()
  assert '0 failing, 0 trusted' in audit
  m=re.search(r'audited ([0-9]+) theorems',audit);assert m
  row['audited_declarations']=int(m[1])
 else:row['errors']=(p/'compile.log').read_text()
 runs.append(row)
assert any(r['kind']=='Lean' and r['status']=='accepted' for r in runs)
assert {r['attempt'] for r in runs}=={p.name for p in (h/'runs').iterdir() if p.is_dir()}
r={'schema':'koblitz.round-results/v1','round':39,'runs':runs,
 'measured_diamond_character':7,'prospective_character_six_matched':False,
 'twisted_shared_rational_joint_systems':0,'twisted_shared_total_joint_systems':2,
 'crossed_integral_operator_actions_mod11':{'character7':[9,0],'character8':[0,9]},
 'rational_recovery_scalar':5,'operator_pair_separates_under_both_diamond_conventions':True,
 'collector':{'field':128,'rational_chart_points':420,'degree7_closed_points':60,'diamond_orbits':6,'local_presentations_certified':60,'source_equations':5,'registered_units':8},
 'geometry':{'local_standard_etale_certificates':True,'proper_model_binding':'retained written function-field bridge','degree7_jets_constructed':False,'complete_degree460_divisor_evaluated':False,'new_collector_classes_evaluated':False,'chosen_Koblitz_model_map_evaluated':False,'full_quotient_saturation':False,'proper_polarized_embedding':False},
 'formal':{'audited_declarations':sum(x.get('audited_declarations',0) for x in runs),'native_arrays_formalized':False,'proper_geometry_formalized':False},
 'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False,
 'global_metrics_integration':'pending exclusive T-865; scoped R37 patch retained'}
(h/'results.json').write_text(json.dumps(r,indent=2)+'\n')
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':39,'machine_index_created_after_data':True,'normative_prospective_protocols':['PROTOCOL.md','DIAMOND-PROTOCOL.md','COLLECTOR-PROTOCOL.md','JOINT-PROTOCOL.md','LOCAL-PROTOCOL.md','LEAN-PROTOCOL.md'],'claim_class':'exact_computation and universal algebra/group identities','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'version':'4.34.1','jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n')
print(json.dumps({'native_accepted':len(native),'Lean_runs':len(runs)-len(native),'audited_declarations':r['formal']['audited_declarations']}))
