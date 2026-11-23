from pathlib import Path
import json,hashlib,importlib.util,copy,collections
from unittest import mock
out=Path(__file__).resolve().parent;base=out.parent;fixture=Path('/var/tmp/ml-dataset-cli46-032469-v3-20261101')
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
load=lambda p:json.loads(Path(p).read_text())
state=load(fixture/'prepared.json');protocol=load(fixture/'protocol.json');driver=fixture/'retained-analysis.py'
assert sha(driver)==state['driver_sha256']=='11c870294a2c597dc902f5379b02e40884a8e1437808862fdfb75ccf28b5bf25'
original_names=['experiment.export.json','functional-rows.jsonl','analysis.json','policy-analysis.json','execution-failed.json','failure-export-status.json','protocol.json','prepared.json','retained-analysis.py']
before={n:sha(fixture/n)for n in original_names}
s=importlib.util.spec_from_file_location('retained_original_driver',driver);d=importlib.util.module_from_spec(s);s.loader.exec_module(d)
with mock.patch.object(d,'save',side_effect=AssertionError('original writes forbidden')),mock.patch.object(d.subprocess,'Popen',side_effect=AssertionError('processes forbidden')),mock.patch.object(d.subprocess,'run',side_effect=AssertionError('processes forbidden')),mock.patch.object(d.subprocess,'call',side_effect=AssertionError('processes forbidden')):
 native=load(fixture/'analysis.json');policy=d.policy_analyze(fixture,state)
 assert native==load(fixture/'native-analysis-checkpoint.json')['analysis'] and policy==load(fixture/'policy-analysis.json')==load(fixture/'policy-analysis-checkpoint.json')['analysis']
 cases=d.failure_schedule(fixture);assert len(cases)==46
 for c in cases:
  manifest=c['root']/'run/cells'/f"{c['index']:012}"/'manifest.json'
  assert d.diagnostic_checkpoint(fixture,state,c,sha(manifest))
 for name,argv in d.expected_cli_argv(state,fixture).items():
  paths=d.exact_command_directories(fixture,name);assert len(paths)==1;d.checked_receipt(paths[0],argv,state)
 indexes=[json.loads(x)for x in (fixture/'export/restricted/index.jsonl').read_text().splitlines()if x]
 d.native_causal_check(fixture,indexes)
 evidence={c['id']:d.phase_replay_evidence(fixture,state,c) for c in cases}
commands=[]
for p in sorted((fixture/'commands').iterdir()):
 a=load(p/'argv.json');r=load(p/'receipt.json');commands.append({'name':p.name,'argv':a,'exit_code':r['exit_code']})
dataset=[c for c in commands if len(c['argv'])>1 and c['argv'][1]=='dataset'];assert dataset
raw=load(fixture/'experiment.export.json');value=copy.deepcopy(raw)
assert len(raw['runs'])==92
expected={(arm,c['id'])for arm in d.PHASE_ARMS for c in cases};assert {(r['arm'],r['instance_id'])for r in raw['runs']}==expected
assert all(r['status']=='succeeded' and r['verification']['status']=='verified' for r in raw['runs'])
# Retain all original measurements/statuses. Only attach authenticated metadata/custody.
byarm={a['id']:a for a in protocol['arms']}
for arm in value['arms']:
 arm['binary_sha256']=byarm[arm['id']]['instrument']['binary_sha256'];arm['commit']=byarm[arm['id']]['instrument']['commit']
value['analysis']['script']=d.diagnostic_file(fixture,driver,'analysis')
case_mapping={c['id']:c for c in cases}
for row in value['runs']:
 c=case_mapping[row['instance_id']]
 row['artifacts']=[d.diagnostic_file(fixture,c['root']/'run/cells'/f"{c['index']:012}"/'manifest.json','receipt')] if row['arm']=='producer-source' else evidence[c['id']]
software_cases=[{k:v for k,v in c.items()if k!='root'}for c in cases]
(out/'software-cases.json').write_text(json.dumps({'schema':'crypto.ml-dataset.software-cases/v1','cases':software_cases,'planned':46,'scientific_corpus_rows':0},indent=2)+'\n')
value['backend_record']={'software_cases':software_cases,'functional_validation_only':True,'planned_dataset_cells':46,'planned_validation_observations':92,'additional_solver_runs':0,'derivation':'Separate metadata/custody binding of retained converter output; original terminal failure preserved.'}
value['files'].append(d.diagnostic_file(fixture,fixture/'experiment.export.json','raw'))
# Make existing references point to original immutable evidence; no fixture copy/rewrite.
def absolute_refs(v):
 if isinstance(v,dict):
  for k,x in v.items():
   if k=='path' and isinstance(x,str) and not Path(x).is_absolute() and (fixture/x).is_file():v[k]=str(fixture/x)
   else:absolute_refs(x)
 elif isinstance(v,list):
  for x in v:absolute_refs(x)
absolute_refs(value)
value['files'].append({'path':str(out/'software-cases.json'),'sha256':sha(out/'software-cases.json'),'bytes':(out/'software-cases.json').stat().st_size,'kind':'receipt'})
value['notes'].append({'kind':'observation','by':'independent metadata reconciliation','text':'Original version3 execution ended with no_retry failure at final from-rows export metadata validation. This separately derived export changes no outcome, measurement, case, seed, or solver run. Original failure-export-status no_dataset_commands=true is contradicted by retained argv inventory and is not inherited.'})
for a,b in zip(raw['runs'],value['runs']):assert {k:v for k,v in a.items()if k!='artifacts'}=={k:v for k,v in b.items()if k!='artifacts'}
(out/'experiment.export.derived.json').write_text(json.dumps(value,indent=2)+'\n')
source=load(Path(state['checker']).parent.parent/'SOURCE.json')
assert source['commit']=='37e193bf8b3e673e0942709794f44d28e6ab66fb' and sha(state['checker'])==state['checker_sha256']
for rel,digest in source['sha256'].items():assert sha(Path(state['checker']).parent.parent/rel)==digest
s=importlib.util.spec_from_file_location('official_checker',state['checker']);checker=importlib.util.module_from_spec(s);s.loader.exec_module(checker)
findings=checker.schema_errors(value,'experiment.export/v1')+checker.schema_errors(protocol,'experiment.protocol/v1');findings+=checker.lint_export(value,protocol,out,True)
(out/'official-validation.json').write_text(json.dumps({'accepted':not findings,'findings':findings,'checker_commit':source['commit'],'checker_sha256':sha(state['checker']),'derived_export_sha256':sha(out/'experiment.export.derived.json'),'original_protocol_sha256':sha(fixture/'protocol.json'),'method':'schema_errors and lint_export verify_files=True; no execution'},indent=2)+'\n')
counts=collections.Counter();case_details=[]
for c in cases:
 m=load(c['root']/'run/cells'/f"{c['index']:012}"/'manifest.json');o=m['outcome'];counts[(c['panel'],o.get('status'),str(o.get('solver_completed')))]+=1
 case_details.append({'id':c['id'],'panel':c['panel'],'index':c['index'],'manifest_sha256':sha(c['root']/'run/cells'/f"{c['index']:012}"/'manifest.json'),'status':o.get('status'),'solver_completed':o.get('solver_completed'),'exhaustive_no_decomposition':o.get('exhaustive_no_decomposition'),'pdp_witness_verified':o.get('pdp_witness_verified')})
assert before=={n:sha(fixture/n)for n in original_names}
(out/'audit.json').write_text(json.dumps({'original_inputs_sha256':before,'original_terminal_status':'FAIL/no_retry at export metadata validation','policy_analysis_recomputed_equal':True,'native_prefailure_checkpoint_authenticated':True,'native_causal_checks_recomputed':True,'native_cli_receipts_checked':True,'original_standalone_analysis_not_reexecuted_after_failure':True,'software_cases':46,'phase_observations':92,'all_phase_contract_rows_succeeded':True,'dataset_command_count':len(dataset),'no_dataset_commands':False,'original_no_dataset_commands_flag_incorrect':load(fixture/'failure-export-status.json')['no_dataset_commands'] is True,'additional_solver_runs':0,'outcomes':[{'panel':k[0],'status':k[1],'solver_completed':k[2],'count':v}for k,v in counts.items()],'case_details':case_details,'official_derived_export_accepted':not findings,'original_files_unchanged':True},indent=2)+'\n')
print(json.dumps({'findings':findings,'dataset_commands':len(dataset),'outcomes':[list(k)+[v]for k,v in counts.items()]},indent=2))
