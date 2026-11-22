from pathlib import Path
import hashlib,json,statistics
P=Path(__file__).resolve().parent;R=P.parents[1];rows={}
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():
 data=(R/name).read_bytes();assert hashlib.sha256(data).hexdigest()==digest;rows[name]=json.loads(data)
x=rows['research/root_validated_cold_20261115_v50/raw.json'];assert x['error']is None;out=x['out'];expected={r['h_index']:r for r in rows['research/root_validated_cold_20261115_v50/preflight.json']['out']['charts']};assert len(out['records'])==6
for row in out['records']:
 assert row['exit_code']==0 and json.loads(row['stdout'])['charts']==row['charts']
 for c in row['charts']:
  e=expected[c['h_index']];assert c['complete']and c['candidate_sha256']==e['candidate_sha256']and c['norms']==e['norms']and c['candidates']==e['candidates']
 assert row['rss_sum_peak_upper_bound_kib']==row['parent_peak_rss_kib']+max(c['native_peak_rss_kib']for c in row['charts'])
med={m:statistics.median(r['cold_wall_seconds']for r in out['records']if r['method']==m)for m in ['reference','compressed']};aa={m:max(r['cold_wall_seconds']for r in out['records']if r['method']==m)/min(r['cold_wall_seconds']for r in out['records']if r['method']==m)-1 for m in med};assert med==out['median_cold_seconds']and aa==out['AA_spread'];assert med['reference']/med['compressed']==out['conditional_ratio']
print(json.dumps(dict(median_cold_seconds=med,AA_spread=aa,conditional_ratio=out['conditional_ratio'],charts=4,repetitions=3,qualified_speedup=None)))
