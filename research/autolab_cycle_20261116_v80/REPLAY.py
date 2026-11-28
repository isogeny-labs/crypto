from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent
for n,h in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
r=json.loads(Path('research/exact_poly_square_20261116_v80/raw.json').read_text());assert r['error']is None;o=r['out'];assert(o['square_cases'],o['power_cases'],o['Taylor_cases'],o['large_root_cases'],o['conditional_cases'])==(560,336,120,5,5)
assert len(o['fields'])==14 and len(o['operation_cases'])==28
for v in o['operation_cases']:assert v['new_scalar_mults']+v['new_scalar_squares']<=v['generic_scalar_mults']+v['generic_scalar_squares']
print('C80 hashes and exact regression/operation outcomes verified; no runtime speedup claim')
