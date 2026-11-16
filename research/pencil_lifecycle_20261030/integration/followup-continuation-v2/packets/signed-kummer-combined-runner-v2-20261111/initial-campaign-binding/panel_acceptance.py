"""Acceptance of emitted qualification records; no invocation side effects."""
import json,re
from pathlib import Path
JOINTS=[('header',''),('driver_begin','advance'),('driver_end','advance'),('driver_begin','lift'),('driver_end','lift'),('driver_relation',''),('driver_duplicate',''),('terminal','')]
NEGATIVES=['missing_call_result_rejected','changed_call_number_rejected','changed_signed_labels_rejected','missing_lift_disposition_rejected']
def one_test(text):
 assert len(re.findall(r'test result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured;',text))==1,'exact nonzero passing test'
def trace_identity(path,fixture,revision,source_sha256,run_id):
 lines=Path(path).read_text().splitlines(); assert lines,'missing observed header'
 header=json.loads(lines[0]); assert header['kind']=='ObservedHeader'
 identity=header['data']['identity']
 assert identity==dict(source_revision=revision,source_sha256=source_sha256,workload_id=fixture['target']['workload_id'],run_id=run_id),'trace source identity mismatch'
 target=fixture['target'];cases=target['cases'];p=target['p']
 native_expected=dict(kind='header',schema='kummer-driver-events/v1',workload_id=target['workload_id'],target_index=target['target_index'],bits=16,p=p,r=target['r'],g=cases[1]['coordinate_witness']['jacobian_expected'],q=cases[2]['coordinate_witness']['jacobian_expected'],initial_pair=[cases[2]['coordinate_witness']['k_expected'],cases[3]['coordinate_witness']['k_expected']],initial_labels=[0,1],algorithm_seed=target['algorithm_seed'],candidate_cap=100*p+10000,transition_validation_enabled=False)
 assert header['data']['native']==native_expected,'native header mismatch'

 return identity

def verify_driver(directory,fixture,revision,source_sha256):
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
 trace_identity(d/'observed.jsonl',fixture,revision,source_sha256,'observed')
 # Header-write fault occurs before any byte is emitted. Do not invent identity evidence.
 assert r['faults'][0]['position']==0 and (d/'fault-0.jsonl').read_bytes()==b''
 for i in range(1,8):
  trace_identity(d/('fault-'+str(i)+'.jsonl'),fixture,revision,source_sha256,'fault-'+str(i))
 r['trace_identity_acceptance']={'checked_runs':['observed']+['fault-'+str(i) for i in range(1,8)],'header_fault':'expected_empty_prefix_identity_unavailable'}
 return r
def verify_probe(directory,fixture,revision,source_sha256):
 d=Path(directory).with_suffix('.native-probe');r=json.loads((d/'result.json').read_text())
 assert r['schema']=='kummer-driver-native-probe/v1' and r['p']==739 and r['r']==34147
 assert r['passed'] is True and r['forced_fallback_reconstructed_signed_successor'] is True
 assert r['zero_vector_lift_status']=='zero_projective_vector' and r['zero_vector_lift_candidates_tested']==0
 assert r['trace_complete'] is True
 trace_identity(d/'events.jsonl',fixture,revision,source_sha256,'forced-native')
 return r
