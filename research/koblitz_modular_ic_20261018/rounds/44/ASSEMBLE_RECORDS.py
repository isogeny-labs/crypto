#!/usr/bin/env python3
"""Index strict terminal native/proof receipts; no field calculation."""
from pathlib import Path
import json
h=Path(__file__).resolve().parent
spec=[('point12-02','POINT44'),('resume-twist436-01','RESUME44'),('control12-01','CONTROL44'),('matrix-control12-01','MCONTROL44'),('period-control12-01','PERIOD44'),('frobenius-control12-02','FROB44'),('trace12-01','TRACE44'),('trace436-01','TRACE44'),('filter83-01','FILTER44'),('filter284-01','FILTER44')]
runs=[];source=[];traces=[];filters=[]
def fields(line):return dict(x.split('=',1)for x in line.split()[1:] if '='in x)
for attempt,tag in spec:
 p=h/'runs'/attempt;log=(p/'native.log').read_text();assert(p/'exit.txt').read_text().strip()=='0';assert tag+'_ALL_CHECKS_PASS'in log;assert not any(s.lstrip().startswith('?')for s in log.splitlines())
 runs.append({'attempt':attempt,'kind':'native','status':'accepted','native_exit':0,'source_revision':(p/'source-revision.txt').read_text().strip(),'observations':[s for s in log.splitlines()if s.startswith((tag+'_RESULT',tag+'_SOURCE_RESULT'))]})
 for s in log.splitlines():
  if s.startswith(('POINT44_RESULT ','RESUME44_SOURCE_RESULT ')):source.append(fields(s))
  if s.startswith('TRACE44_RESULT '):traces.append(fields(s))
  if s.startswith('FILTER44_RESULT '):filters.append(fields(s))
for attempt,reason,exit_code in [('point12-01','reserved Singular kernel identifier; late marker rejected',0),('point436-01','1200s timeout during full source evaluation',124),('point436-02','1200s timeout after quotient-first source evaluation; full source not certified',124),('frobenius-control12-01','nested-list conversion error in control; late marker rejected',0)]:
 p=h/'runs'/attempt;assert(p/'exit.txt').read_text().strip()==str(exit_code)
 if exit_code==0:assert any(s.lstrip().startswith('?')for s in(p/'native.log').read_text().splitlines())
 runs.append({'attempt':attempt,'kind':'native','status':'rejected','native_exit':exit_code,'reason':reason,'mathematical_refutation':False})
p=h/'runs/point436-03';assert(p/'exit.txt').read_text().strip()=='124';log=(p/'native.log').read_text();assert not any(x.lstrip().startswith('?')for x in log.splitlines())
completed=[x for x in log.splitlines()if x.startswith('POINT44_RESULT ')];assert len(completed)==1 and fields(completed[0])['twist']=='0';source.append(fields(completed[0]))
assert 'twist=1 stage=full_source_checks_complete' in log and 'base_generation_complete period=2180' in log
runs.append({'attempt':p.name,'kind':'native','status':'partial','native_exit':124,'reason':'1200s timeout after complete geometry checks for both forms and full first-block Frobenius/period; missing twisted Frobenius completed by resume-twist436-01','completed_full_source_blocks':1,'completed_geometry_source_blocks':2,'mathematical_refutation':False})
for attempt,count,named in [('lean44-01',4,3),('trace-lean44-01',1,1),('quadratic-lean44-01',4,1),('odd-lean44-02',2,2)]:
 p=h/'runs'/attempt;assert(p/'compile.exit').read_text().strip()=='0'and(p/'audit.exit').read_text().strip()=='0';log=(p/'audit.log').read_text();assert f'audited {count} theorems'in log and'0 failing, 0 trusted'in log
 runs.append({'attempt':attempt,'kind':'Lean','status':'accepted','compile_exit':0,'audit_exit':0,'audited_declarations':count,'named_mathematical_theorems':named})
p=h/'runs/odd-lean44-01';assert(p/'compile.exit').read_text().strip()=='1';assert 'does not exist' in(p/'compile.log').read_text();runs.append({'attempt':p.name,'kind':'Lean','status':'rejected','compile_exit':1,'reason':'unavailable Mathlib.Tactic.Omega object; replaced by pinned ring tactic in fresh attempt'})
assert len(source)==6 and len(traces)==3 and len(filters)==20
assert {x['attempt']for x in runs}=={p.name for p in(h/'runs').iterdir()if p.is_dir()}
for p in source:assert p['AS_trace']=='1'and p['all_source_equations']=='5'and p['all_registered_units']=='8'
for c in traces:assert c['source_orientation_trace']=='1'and c['Tate_twist_trace']=='1'and c['seven_kernel_trace']=='0'
r={'schema':'koblitz.round-results/v1','round':44,'runs':runs,'source_blocks':source,'independent_trace_cases':traces,'retained_source_filter_controls':filters,'construction_counts':{'actual_source_blocks':len(source),'exact_orientation_trace_cases':len(traces),'orientation_filter_control_blocks':len(filters),'source_equations_each':5,'registered_units_each':8,'deck_iterates_each':10,'max_actual_source_tuple_degree':max(int(x['source_tuple_period'])for x in source)},'geometric_deduction':{'mixed_projector':'(1-sigma)*(5-N5)','ordinary12_and436_sign_equals_fifth_Frobenius_power':True,'their_F32_closed_divisor_mixed_contribution_zero':'under retained finite-source/proper/moduli binding; not a Picard solve','trace_zero_alone_implies_nonzero_class':False},'formal':{'audited_declarations':11,'named_mathematical_theorems':7,'generated_declarations':4,'native_arrays_formalized':False,'finite_field_generation_formalized':False,'proper_geometry_formalized':False,'independent_semantic_review':False},'scope':{'complete_collector_principal_divisors_evaluated':False,'arbitrary_ordinary_picard_images_evaluated':False,'chosen_Koblitz_model_isogeny_evaluated':False,'larger_n_geometric_transfer_evaluated':False,'full_quotient_saturation':False,'proper_polarized_embedding':False,'node187_coupled_divisor_closed':False},'performance_measured':False,'IC_relation_rank':None,'public_input_recoveries':0,'independent_scientific_replay':False}
(h/'results.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r['construction_counts']))
(h/'protocol.json').write_text(json.dumps({'schema':'koblitz.protocol-index/v1','round':44,'machine_index_created_after_data':True,'normative_prospective_protocols':['PROTOCOL.md','LEAN-PROTOCOL.md','MODULAR-PROTOCOL.md','MATRIX-PROTOCOL.md','PERIOD-PROTOCOL.md','TRACE-PROTOCOL.md','FILTER-CONTROL-PROTOCOL.md','FROBENIUS-PROTOCOL.md','ODD-DESCENT-PROTOCOL.md','RESUME-PROTOCOL.md'],'claim_class':'exact source algebras and conditional closed-divisor obstruction','native_limits':{'virtual_memory_MiB':4096,'wall_seconds':1200},'Lean_limits':{'version':'4.34.1','jobs':1,'heap_MiB':4096,'wall_seconds':1200},'statistical_or_comparative_claims':False},indent=2)+'\n')
