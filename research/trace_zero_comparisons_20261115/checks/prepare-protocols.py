"""Administrative protocol/export serialization; no curve or solver computations."""
import copy, hashlib, json
from pathlib import Path
base=Path('/workspace/crypto-trace')
study=base/'research/trace_zero_comparisons_20261115'
template=json.loads((base/'research/trace_zero_experiments_20261109/protocol.json').read_text())
binary=study/'native/target/release/trace-zero-comparisons'
binary_hash=hashlib.sha256(binary.read_bytes()).hexdigest()
analysis_hash=hashlib.sha256((study/'native/src/lib.rs').read_bytes()).hexdigest()
for panel, names, descriptions in [
 ('encoding',['certified','constants','implicit','full-point','constants-aa'],['Certified finite-domain S3','Same domain with constant folding','Basis-only compact inverse membership plus S3','Complete finite-base full-point chain','Exact repeated constants arm']),
 ('tau',['direct','tau-image','tau-preimage','direct-aa'],['Direct constant-specialized S3','tau-minus-one image of actual base and target','Inverse tau-minus-one image of actual base and target','Exact repeated direct arm'])]:
 p=copy.deepcopy(template)
 p['protocol_id']='TZC-20261115-'+panel;p['created']='2026-11-15';p['study_dir']=str((study/panel).relative_to(base))
 p['question']='Exact completion and counted-work differences on identical abstract instances for '+panel+' presentations, preserving every capped outcome and A/A discrepancy.'
 p['arms']=[]
 for i,(name,description) in enumerate(zip(names,descriptions)):
  a=copy.deepcopy(template['arms'][0]);a['id']=name;a['description']=description
  a['role']='baseline' if i==0 else ('control_aa' if name.endswith('-aa') else 'candidate')
  a['instrument']['commit']=None;a['instrument']['binary_sha256']=binary_hash
  a['instrument']['build']=[['cargo','build','--locked','--release','--manifest-path',str((study/'native/Cargo.toml').relative_to(base))]]
  a['parameters']['membership_encoding']=name
  if i:a['same_instrument_as']=names[0]
  p['arms'].append(a)
 comps=[{'id':'compare-'+n,'arms':[names[0],n],'differs_in':['presentation'],'design':'one_factor'} for n in names[1:-1]]
 comps.append({'id':'aa','arms':[names[1] if panel=='encoding' else names[0],names[-1]],'differs_in':[],'design':'aa'})
 ids=[c['id'] for c in comps if c['design']!='aa']
 p['variable_ledger']['independent']=[{'name':'presentation','levels':names[:-1]}]
 p['variable_ledger']['comparisons']=comps
 for f in p['variable_ledger']['factors']:
  if f['name']=='instrument':f['value']=binary_hash
  if f['name']=='order':f['how']='Rotate all '+str(len(names))+' arms by case index+replicate, preserving every row.'
 p['claims'][1]['decided_by']=ids
 p['claims'][1]['text']='Exact descriptive valid completion counts may differ on this fixed panel beyond repeated A/A outcome discrepancies; no population or clock claim.'
 p['controls']['positive']['not_applicable_reason']='Positive pilot embedded through every arm at n5; all must Found with original replay before primary freeze. Each n7/9/83/131 pilot is independently validated; any capped wide result remains censored, not a detector success. Primary planted/cancellation cases retained.'
 p['sample_size']['power']['tests_in_family']=len(ids)
 p['sample_size']['power']['note']='Exact complete fixed-panel simultaneous tabulation only, no population p-values or rejection; all contrasts shown, repeated targets not independent.'
 p['sample_size']['preflight_runtime']={k:None for k in ['fixed_seconds','per_sample_seconds','estimated_total_seconds']}
 p['sample_size']['preflight_runtime'].update(threads=1,measured_on='Pending authorized engineering pilot before final primary freeze.')
 p['metrics']['secondary'].append({'id':'group_ops','name':'Native group additions and doublings','unit':'group operations','kind':'counted','definition':'Thread-local actual additions+doublings across preprocessing, solve, admission, lifts and final replay. Field/ANF construction, non-group inversions, Frobenius squaring and SAT work uncharged; no full-cost claim.','source':'rows.jsonl group_ops','aggregation':'distribution','resolution':'1 group addition or doubling','uncertainty':'range'})
 p['measurement']['record'].extend(['group_ops','preprocessing_group_ops','through_solver_group_ops','all_group_ops'])
 p['stopping_rules'][0]['rule']=f'Fixed33 cases by{len(names)} arms by2 repetitions={33*len(names)*2} planned primary rows; separate{5*len(names)} pilot calls.'
 p['decision_rule']['comparison_ids']=ids
 p['decision_rule']['text']='Exact descriptive outcome/valid completion and counted-work tables, per field and complete panel; all capped outcomes retained. Read any completion difference against A/A outcome discrepancy; no timing, population or ECDLP claim.'
 p['analysis_plan']['script']={'path':'../native/src/lib.rs','sha256':analysis_hash}
 p['analysis_plan']['inputs']=['rows.jsonl'];p['analysis_plan']['outputs']=['summary.json','export.json']
 p['freeze']={'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':None}
 p['extensions']={'workload_path':'../workload.json','workload_sha256':hashlib.sha256((study/'workload.json').read_bytes()).hexdigest(),'planned_rows':33*len(names)*2,'authorization':'User2026-11-15 explicitly accepted steps one two three and described local EKS/S3 exception; one CPU,2GiB,600seconds per comparison,zero spend,no retries.','preflight':'One call per arm at n5/7/9/83/131,target3base[0]; every row valid,n5 all Found with mapped/original replay. Outer600seconds includes pilot,freeze and primary.','scope':'Finite actual domains; implicit basis only for original subspace; tau domains transported bijectively, never asserted linear.'}
 p['inadmissible']=['No timing, scaling, population, full-DLP or challenge-recovery claim.','UNKNOWN is not UNSAT; no row deletion, adaptive seeds or comparison of unpaired favorable subsets.','Native group counters exclude field/ANF, non-group inversions and SAT work; they are not a complete cost unit.']
 (study/panel/'protocol.json').write_text(json.dumps(p,indent=2)+'\n')
print(binary_hash)
