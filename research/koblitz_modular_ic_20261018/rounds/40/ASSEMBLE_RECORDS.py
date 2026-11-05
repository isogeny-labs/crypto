#!/usr/bin/env python3
"""Index exact terminal receipts only; no field or curve computation."""
from pathlib import Path
import json,re
h=Path(__file__).resolve().parent;runs=[]
for name,tag in [('encode40-01','ENC40'),('jets40-01','JET40'),('couple40-01','COUPLE40'),('pair40-01','PAIR40')]:
 p=h/'runs'/name;log=(p/'native.log').read_text()
 assert (p/'exit.txt').read_text().strip()=='0'
 assert tag+'_ALL_CHECKS_PASS' in log and not re.search(r'^\s*\?',log,re.M)
 runs.append({'attempt':name,'kind':'native','status':'accepted','native_exit':0,'source_revision':(p/'source-revision.txt').read_text().strip(),'observations':[s for s in log.splitlines() if s.startswith(tag+'_RESULT') or s.startswith(tag+'_CONTROL')]})
p=h/'runs/lean40-01';assert (p/'compile.exit').read_text().strip()=='0' and (p/'audit.exit').read_text().strip()=='0'
audit=(p/'audit.log').read_text();assert 'audited 5 theorems' in audit and '0 failing, 0 trusted' in audit
runs.append({'attempt':'lean40-01','kind':'Lean','status':'accepted','compile_exit':0,'audit_exit':0,'audited_declarations':5})
assert {r['attempt'] for r in runs}=={p.name for p in (h/'runs').iterdir() if p.is_dir()}
pair=dict(line.split('=') for line in (h/'runs/pair40-01/pair-result.txt').read_text().splitlines())
r={'schema':'koblitz.round-results/v1','round':40,'runs':runs,'source_jets':{'closed_points':60,'precision':17,'residue_degree':7,'evaluation_divisor_degree':7140,'source_equations':5,'units_per_point':8,'coefficient_conjugates':7,'corrupt_top_nu_rejected':True,'F32_regression_rows':588,'F32_regression_precision':17,'section_ranks':[68,196,324],'original_column_pivot_recombinations':True},'coupled_divisors':{'complete_divisors':59,'degree':128,'padding':'20Q+3R0','Q_degree':5,'R0_degree':7,'vanishing_condition_rank':128,'small_rank':68,'large_rank':196,'all_kernel_recombinations':True,'proper_RR_interpretation':'retained written bridge'},'actual_collector_difference':{'support_pair':['o1-d1','o2-d0'],'forward_image_construction':False,'pair_equal':pair['pair_equal']=='true','difference_principal':pair['difference_principal']=='true','self_control':True,'inverse_cancellation':True,'modular_factor_images_evaluated':False},'formal':{'audited_declarations':5,'native_arrays_formalized':False,'proper_geometry_formalized':False,'Newton_convergence_formalized':False},'larger_geometry':{'precision615_source_jets_constructed':False,'larger_RR_spaces_constructed':False,'complete_degree460_divisor_evaluated':False,'full_quotient_saturation':False,'proper_polarized_embedding':False},'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False,'global_metrics_integration':'pending exclusive T-865'}
(h/'results.json').write_text(json.dumps(r,indent=2)+'\n')
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':40,'machine_index_created_after_data':True,'normative_prospective_protocols':['PROTOCOL.md','JET-PROTOCOL.md','COUPLE-PROTOCOL.md','PAIR-PROTOCOL.md','LEAN-PROTOCOL.md'],'claim_class':'exact computations and universal linear algebra identities','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'version':'4.34.1','jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n')
print('Indexed four native runs, five Lean theorems, and actual collector class outcome.')
