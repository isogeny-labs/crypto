import pathlib,json,hashlib
R=pathlib.Path(__file__).resolve().parents[2];P=R/'research/autolab_cycle_20261115_v62'
for name,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
for directory in ['wide_baseline_local_20261115_v62','inherited_default_policy_20261115_v62']:
 d=json.loads((R/'research'/directory/'raw.json').read_text());assert d['error']is None;assert d['out']['runtime_dependency_bindings_verified']==25
 for record in d['out']['records']:assert not record['complete_matching_roots'];assert record['verified_root_lower_bound']==0
print('Hashes, runtime bindings and retained incomplete baseline statuses verified. No ratio or release claim.')
