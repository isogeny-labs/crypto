from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent
for n,h in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
r=json.loads(Path('research/vieta_rationality_20261116_v75/raw.json').read_text());assert r['error']is None;o=r['out'];assert o['small']['cases']==34616 and o['small']['norm_identity_cases']==616
assert o['fresh_rejected']==0 and o['old_relaxed_rejected']==0
q=o['join_preflight'];rr=q['nonzero_rational_x'];assert q['source_points']==2*rr+1 and q['finite_unordered_pair_records']==2*rr*(rr+1)
assert q['flat_pair_bytes']==16*q['finite_unordered_pair_records']>q['cap_bytes'] and q['refuse_flat_materialization']
print('C75 hashes, parity non-improvement and exact pair-format refusal verified; no speedup claim')
