from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent
for n,h in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
r=json.loads(Path('research/rationality_image_20261116_v76/raw.json').read_text());assert r['error']is None;o=r['out'];assert o['small_cases']==1976 and len(o['actual'])==36
assert o['old_rejected']==o['fresh_rejected']==16
for v in o['actual']:
 if 'independent_anti_count'in v:assert sum(int(x)<<j for j,x in enumerate(v['e1_e2_e4']))==v['independent_anti_count']
assert o['even_anti_witnesses']['2']['bits']==[0,1,0]and o['even_anti_witnesses']['4']['bits']==[0,0,1]
print('C76 hashes and recorded count-bit/root controls verified; no speedup claim')
