"""Acceptance of emitted qualification records; no invocation side effects."""
import json,re
from pathlib import Path
JOINTS=[('header',''),('driver_begin','advance'),('driver_end','advance'),('driver_begin','lift'),('driver_end','lift'),('driver_relation',''),('driver_duplicate',''),('terminal','')]
NEGATIVES=['missing_call_result_rejected','changed_call_number_rejected','changed_signed_labels_rejected','missing_lift_disposition_rejected']
def one_test(text):
 assert len(re.findall(r'test result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured;',text))==1,'exact nonzero passing test'
def verify_driver(directory,fixture):
 d=Path(directory);r=json.loads((d/'result.json').read_text())
 assert r['schema']=='kummer-driver-software-qualification/v1'
 assert r['original_disabled_observed_equal'] is True and r['native_runs']==11
 expected=r['complete_original_row']
 assert expected['status']=='verified' and expected['source_verified'] is True
 assert expected['recovered']==fixture['target']['planted']
 for arm in ['original','disabled','observed']:
  lines=(d/arm/'collector-16.jsonl').read_text().splitlines();assert len(lines)==1
  assert json.loads(lines[0])==expected,'complete row equality'
 assert [(x['kind'],x['operation']) for x in r['faults']]==JOINTS
 assert all(x['passed'] is True and type(x['position']) is int and x['position']>=0 for x in r['faults'])
 for name in ['observed.jsonl']+['fault-'+str(i)+'.jsonl' for i in range(8)]:
  assert (d/name).is_file() and (d/name).stat().st_size>0
 return r
def verify_probe(directory):
 d=Path(directory).with_suffix('.native-probe');r=json.loads((d/'result.json').read_text())
 assert r['schema']=='kummer-driver-native-probe/v1' and r['p']==739 and r['r']==34147
 assert r['passed'] is True and r['forced_fallback_reconstructed_signed_successor'] is True
 assert r['zero_vector_lift_status']=='zero_projective_vector' and r['zero_vector_lift_candidates_tested']==0
 assert r['trace_complete'] is True and (d/'events.jsonl').stat().st_size>0
 return r
