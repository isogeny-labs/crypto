from pathlib import Path
import json,hashlib
R=Path(__file__).resolve().parents[2];P=Path(__file__).resolve().parent
for n,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
x=json.loads((R/'research/translation_orbits_20261115_v69/raw.json').read_text());assert x['error']is None
assert x['out']['trace_varying_columns']==0
assert all(r['monic_degree']==12 for r in x['out']['orbit_profiles'])
assert not x['out']['support_noninvariance_control']['shifted_supported']
print('Hashes, orbit contracts and fixed-chart exclusion verified;qualifiedspeedupnull.')
