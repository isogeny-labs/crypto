#!/usr/bin/env python3
"""Index terminal receipts and copy native measurements; no mathematical solver."""
from pathlib import Path
import json,re
h=Path(__file__).resolve().parent
runs=[]
for attempt,tag in [('residual42-01','RESIDUAL42'),('global42-01','GLOBAL42'),('integer42-01','INTEGER42')]:
 p=h/'runs'/attempt;log=(p/'native.log').read_text()
 assert (p/'build.exit').read_text().strip()=='0' and (p/'exit.txt').read_text().strip()=='0'
 assert tag+'_ALL_CHECKS_PASS' in log and 'panicked at' not in log
 runs.append({'attempt':attempt,'kind':'native','status':'accepted','build_exit':0,'native_exit':0,'source_revision':(p/'source-revision.txt').read_text().strip(),'observations':[s for s in log.splitlines() if s.startswith(tag+'_RESULT') or s.startswith(tag+'_LATTICE')]})
for attempt,count in [('lean42-base-01',31),('lean42-global-02',15),('least-scalar42-02',1),('integer-witness42-01',7)]:
 p=h/'runs'/attempt
 assert (p/'compile.exit').read_text().strip()=='0' and (p/'audit.exit').read_text().strip()=='0'
 audit=(p/'audit.log').read_text();assert f'audited {count} theorems' in audit and '0 failing, 0 trusted' in audit
 runs.append({'attempt':attempt,'kind':'Lean','status':'accepted','compile_exit':0,'audit_exit':0,'audited_declarations':count,'source_revision':(p/'source-revision.txt').read_text().strip()})
p=h/'runs/least-scalar42-01';assert (p/'compile.exit').read_text().strip()=='1' and 'expected token' in (p/'compile.log').read_text()
runs.append({'attempt':p.name,'kind':'Lean','status':'rejected','compile_exit':1,'reason':'finite-sum notation import missing; repaired in fresh attempt least-scalar42-02'})
p=h/'runs/lean42-global-01';assert set(x.name for x in p.iterdir())=={'gate.log'}
runs.append({'attempt':p.name,'kind':'Lean','status':'rejected','compile_exit':None,'reason':'stopped before source copying/compilation because base audit.exit had not yet been written; no numerical or proof computation executed'})
assert {x['attempt'] for x in runs}=={p.name for p in (h/'runs').iterdir() if p.is_dir()}
s=json.loads((h/'runs/residual42-01/output/residual-systems.json').read_text());g=json.loads((h/'runs/global42-01/output/global-recovery.json').read_text());inte=json.loads((h/'runs/integer42-01/output/integer-recovery.json').read_text())
summary=[]
for c,t,v in zip(s['cases'],g['cases'],inte['cases']):
 assert c['id']==t['id']==v['id']
 summary.append({k:c[k] for k in ['id','n','target_modulus','target_modulus_bits','conductor_prime','genus_historical','CM_exponents','diamond_two_order','diamond_two_n_matches_delta','global_denominator_norm_mod_target','global_denominator_norm_unit']} | {'global_recovery_status':t['status'],'inverse_polynomial_degree':len(t['inverse_denominator'])-1 if t['status']=='certified' else None,'least_scalar':v['least_scalar'],'least_scalar_target_gcd':v['target_gcd'],'least_scalar_unit':v['invertible_at_target']})
result={'schema':'koblitz.round-results/v1','round':42,'runs':runs,'cases':summary,'integer_lattices':[{k:c[k] for k in ['n','dimension','determinant','least_scalar','difference_gcd','trace_sequence','lucas_sequence','exact_divisions','predicted_least_scalar','predicted_formula_matches']} for c in inte['lattices']],'formal':{'audited_declarations':54,'actual_integer_unit_and_primitive_witnesses':True,'actual_large_polynomial_congruences':True,'generic_primitive_divisibility_bridge':True,'all_integer_matrix_columns_formalized':False,'universal_Lucas_formula_formalized':False,'independent_semantic_review':False},'scope':{'full_modular_Hom_lattice':False,'old_four_cover_map_recovery_solved_here':False,'proper_level77_model_map':False,'full_quotient_saturation':False,'proper_polarized_embedding':False,'larger_n_geometric_transfer_evaluated':False,'node187_coupled_divisor_closed':False,'precision615_source_jets_constructed':False,'complete_degree460_or367_divisors_evaluated':False},'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False,'global_metrics_integration':'pending exclusive T-901'}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':42,'machine_index_created_after_data':True,'normative_prospective_protocols':['PROTOCOL.md','GLOBAL-PROTOCOL.md','INTEGER-PROTOCOL.md','LEAN-PROTOCOL.md','GLOBAL-LEAN-PROTOCOL.md','LEAST-SCALAR-LEAN-PROTOCOL.md','INTEGER-WITNESS-LEAN-PROTOCOL.md'],'claim_class':'exact finite algebra and conditional implications','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'version':'4.34.1','jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n')
print('Indexed three native successes, four accepted Lean workflows, two retained rejected/incomplete attempts, 54 audited declarations.')
