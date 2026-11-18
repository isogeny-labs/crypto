"""Portable evidence replay; run from repository root. No native binary needed."""
import hashlib,json,pathlib,statistics
root=pathlib.Path(__file__).resolve().parents[2]
p=root/'research/autolab_cycle_20261115_v54'
for name,h in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():
 assert hashlib.sha256((root/name).read_bytes()).hexdigest()==h,name
raw=json.loads((root/'research/general_native_20261115_v54/raw.json').read_text());assert raw['error'] is None
for case in raw['out']['cases']:
 d=root/'research/general_native_20261115_v54';name=case['name']
 a=json.loads((d/(name+'_reference.json')).read_text());b=json.loads((d/(name+'_compressed.json')).read_text())
 assert a['candidate_pairs']==b['candidate_pairs']
 assert a['solutions']==b['solutions']==case['solutions']
 for s in a['solutions']:assert sum(v for x,v in s['roots'])==5
cold=json.loads((root/'research/general_native_cold_20261115_v54/raw.json').read_text());assert cold['error'] is None
out=cold['out']
for method in ['reference','compressed']:
 rows=[r for r in out['records'] if r['method']==method];assert len(rows)==3
 assert statistics.median(r['cold_wall_seconds'] for r in rows)==out['medians'][method]
 assert all(r['exit_code']==0 and all(c['complete'] for c in r['cases']) for r in rows)
assert out['conditional_ratio']==out['medians']['reference']/out['medians']['compressed']
print('Hashes, complete conditional candidates/solutions, multiplicities and cold timing arithmetic verified; genuine-GB qualification remains open.')
