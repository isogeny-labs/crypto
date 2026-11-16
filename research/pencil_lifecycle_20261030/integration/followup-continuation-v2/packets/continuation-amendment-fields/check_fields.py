from pathlib import Path
import json,hashlib
p=Path(__file__).parent;a=json.loads((p/'amendment-fields.json').read_text());refs=json.loads((p/'source-references.json').read_text());old=json.loads(Path(refs['known_c1_v2']['path']).read_text());auth=json.loads(Path(refs['authorization']['path']).read_text())
assert a['cohort']['plans']==old['cohort']['plans']
assert a['cohort']['workloads']==1260 and a['cohort']['base_executions']==22680 and a['cohort']['measured_candidate_reference_pairs']==6300
assert a['proposed_gate']['plan_sha256']==old['proposed_gate']['plan_sha256'] and a['proposed_gate']['base_executions']==18 and not a['proposed_gate']['enrolled_in_full_cohort']
assert a['known_data']==old['known_data']
assert a['resource_envelope']['cumulative_active_worker_seconds']==auth['cumulative_active_worker_seconds']==25200
assert a['resource_envelope']['absolute_deadline_utc']==auth['absolute_deadline_utc']=='2026-10-31T06:00:00Z'
assert a['freeze'] is None and a['launch_request'] is None and a['protocol_id'] is None and a['version'] is None
assert not a['scientific_execution_authorized_by_this_document']
for arm in a['arms']:
 assert arm['instrument']['binary_sha256'] is None and arm['instrument']['commit'] is None and arm['instrument']['image_digest'] is None
for k,v in refs.items():assert hashlib.sha256(Path(v['path']).read_bytes()).hexdigest()==v['sha256']
print(json.dumps({'schema':'research.proposal-consistency-check/v1','status':'passed','checks':'exact retained plans/known outcomes/authorization/unresolved bindings','canonical_protocol_validated':False,'execution_admitted':False}))
