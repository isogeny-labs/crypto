from pathlib import Path
import json,hashlib
r=Path(__file__).resolve().parents[2];p=r/'research/autolab_cycle_20261115_v62'
for n,h in json.loads((p/'CLOSURE_SHA256.json').read_text()).items():assert hashlib.sha256((r/n).read_bytes()).hexdigest()==h,n
for d in ['wide_baseline_local_20261115_v62','inherited_default_policy_20261115_v62']:
 s=json.loads((r/'research'/d/'raw.json').read_text());assert s['error'] is None
 assert s['out']['runtime_dependency_bindings_verified']==25
 assert all(not x['complete_matching_roots'] for x in s['out']['records'])
assert not json.loads((p/'BUDGET_CLOSURE.json').read_text())['within_budget']
print('Current hashes, incomplete results and session-budget disclosure verified.')
