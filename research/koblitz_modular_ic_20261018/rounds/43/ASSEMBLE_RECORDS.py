#!/usr/bin/env python3
"""Copy terminal native counters and proof receipts; no mathematical calculation."""
from pathlib import Path
import json
h=Path(__file__).resolve().parent
runs=[]
for attempt,tag in [('encode43-01','ENC43'),('jets43-02','JET43'),('multiply43-01','MULT43')]:
 p=h/'runs'/attempt;log=(p/'native.log').read_text();assert(p/'exit.txt').read_text().strip()=='0';assert tag+'_ALL_CHECKS_PASS' in log and 'panicked at' not in log
 if tag!='ENC43':assert(p/'build.exit').read_text().strip()=='0'
 runs.append({'attempt':attempt,'kind':'native','status':'accepted','native_exit':0,'source_revision':(p/'source-revision.txt').read_text().strip(),'observations':[s for s in log.splitlines() if s.startswith(tag+'_RESULT') or s.startswith(tag+'_COMPLETE')]})
p=h/'runs/jets43-01';assert(p/'build.exit').read_text().strip()=='1';assert 'error[E0308]' in (p/'build.log').read_text()
runs.append({'attempt':p.name,'kind':'native','status':'rejected','build_exit':1,'native_executed':False,'reason':'series-method edit accidentally targeted Field; repaired in committed source before fresh jets43-02'})
p=h/'runs/lean43-01';assert(p/'compile.exit').read_text().strip()=='1';assert 'unsolved goals' in(p/'compile.log').read_text()
runs.append({'attempt':p.name,'kind':'Lean','status':'rejected','compile_exit':1,'reason':'norm_num did not discharge characteristic-two cast; repaired with explicit CharP.cast_eq_zero'})
p=h/'runs/lean43-02';assert(p/'compile.exit').read_text().strip()=='0' and(p/'audit.exit').read_text().strip()=='0';audit=(p/'audit.log').read_text();assert 'audited 6 theorems' in audit and '0 failing, 0 trusted' in audit
runs.append({'attempt':p.name,'kind':'Lean','status':'accepted','compile_exit':0,'audit_exit':0,'audited_declarations':6,'source_revision':(p/'source-revision.txt').read_text().strip()})
assert {x['attempt'] for x in runs}=={p.name for p in(h/'runs').iterdir()if p.is_dir()}
def kv(p):return dict(s.split('=',1)for s in p.read_text().splitlines())
stages=[]
for label in ['L4D0','L6D0','L12D0','L24D0']:
 p=h/'runs/multiply43-01/output';v=kv(p/(label+'-result.txt'));stages.append({'label':label,'input_rank':int(v['input_rank']),'output_rank':int(v['output_rank']),'attempted_products':int(v['attempted_products']),'independent_products':int(v['independent_products']),'stop':v['stop'],'all_recombinations':v['all_recombinations']})
r={'schema':'koblitz.round-results/v1','round':43,'runs':runs,'source_precision':615,'reference_precision':154,'reference_rows':610,'coordinates':6,'registered_units':8,'unit_inverses':8,'coefficient_conjugates':5,'evaluation_dimension':3075,'fixed_section_ranks':[68,196,324],'product_stages':stages,'requested_larger_space_ranks':[452,1476,3012],'completeness_interpretation':'native independent rank plus retained proper-curve, pole-divisor and RR-dimension bridge','complete_larger_spaces_constructed_under_retained_bridge':True,'formal':{'audited_declarations':6,'generic_AS_identity':True,'ten_term_AS_precision615':True,'coefficient_lift_precision615':True,'coefficient_lift_proof_reused_from_R38':True,'native_arrays_formalized':False,'proper_geometry_formalized':False,'independent_semantic_review':False},'scope':{'ordinary460_or367_complete_divisors_evaluated':False,'chosen_Koblitz_model_isogeny_evaluated':False,'larger_n_geometric_transfer_evaluated':False,'full_modular_Hom_lattice':False,'full_quotient_saturation':False,'proper_polarized_embedding':False,'node187_coupled_divisor_closed':False},'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False,'global_metrics_integration':'exporter extended after T-901 reservation cleared; committed-ref validation follows'}
(h/'results.json').write_text(json.dumps(r,indent=2)+'\n')
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':43,'machine_index_created_after_data':True,'normative_prospective_protocols':['PROTOCOL.md','MULTIPLICATION-PROTOCOL.md','LEAN-PROTOCOL.md'],'claim_class':'exact finite source expansions/product ranks plus conditional geometric interpretation','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'version':'4.34.1','jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n')
print(json.dumps({'accepted_native':3,'accepted_Lean':1,'retained_rejected_attempts':2,'audited_declarations':6,'stages':stages}))
