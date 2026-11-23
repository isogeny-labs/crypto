from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent
for n,h in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
r=json.loads(Path('research/frobenius_split_20261116_v73/raw.json').read_text());assert r['error']is None and r['out']['guards_hold']
assert r['out']['small']['original_tuple_count']==110856 and r['out']['small']['complete_original_set_equality']
assert all(t['pair_count']==0 and t['visited_nonzero_x']==262143 for t in r['out']['actual']['targets'])
print('C73 hashes and recorded exact guards verified; full five-source solve and speedup not claimed')
