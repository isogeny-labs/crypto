from pathlib import Path
import hashlib,importlib.util,json,datetime
from unittest.mock import patch
r=Path(__file__).parent
fixture=Path('/var/tmp/ml-dataset-cli46-e0a865-final008-20261101')
original=r.parent/'factor-base-cli-archive-adapter/driver.proposed.py'
spec=importlib.util.spec_from_file_location('corrected',r/'driver.corrected.py');d=importlib.util.module_from_spec(spec);spec.loader.exec_module(d)
exec(compile((r/'reconciliation-function.py').read_text(),str(r/'reconciliation-function.py'),'exec'),d.__dict__)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
inputs=[fixture/n for n in ['protocol.json','prepared.json','execution-failed.json','execution-failed-inventory.json','failure-flow.json','failure.experiment.export.json','native-analysis-checkpoint.json']]+[original,r/'driver.corrected.py',r/'matching.patch',r/'reconciliation-function.py',Path(__file__)]
before={str(p):sha(p) for p in inputs}
with patch.object(d,'save',side_effect=AssertionError('read-only reconciliation')),patch.object(d.subprocess,'Popen',side_effect=AssertionError('no commands')):
 result=d.reconcile_failure(fixture,original)
after={str(p):sha(p) for p in inputs};assert before==after
old=json.loads((fixture/'failure-flow.json').read_text());changes=[]
for a,b in zip(old['cases'],result['cases']):
 assert a['id']==b['id']
 if (a['state'],a.get('reason'),a.get('phase_states'))!=(b['state'],b.get('reason'),b.get('phase_states')):changes.append({'id':a['id'],'original_state':a['state'],'original_reason':a.get('reason'),'derived_state':b['state'],'derived_reason':b.get('reason'),'derived_phase_states':b.get('phase_states')})
result.update(schema='crypto.ml-dataset.derived-failure-reconciliation/v1',created_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),original_fixture=str(fixture),original_unchanged=True,original_bindings=before,changes=changes,not_a_replacement_experiment_export=True,no_dataset_commands=True,no_new_measurements=True)
with (r/'derived-reconciliation.json').open('x') as f:json.dump(result,f,indent=2);f.write('\n')
print(json.dumps({'changes':changes,'canonical_rows_derived':len(result['runs']),'sha256':sha(r/'derived-reconciliation.json')}))
