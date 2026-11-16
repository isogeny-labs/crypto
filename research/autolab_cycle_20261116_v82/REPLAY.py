from pathlib import Path
import json,hashlib
C=Path(__file__).resolve().parent
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
A=Path('research/semilinear_trace_20261116_v82');B=Path('research/shared_semilinear_trace_20261116_v82b');a=json.loads((A/'oracles_raw.json').read_text());p=json.loads((A/'profile_raw.json').read_text());b=json.loads((B/'raw.json').read_text());assert all(x['error']is None for x in [a,p,b])
for v in [a['out'],b['out']['oracle']]:assert(v['cases'],v['Taylor_cases'],v['literal_points'],v['twist_checks'])==(1008,336,987,644);assert v['omitted_twist_counterexample']['correct_trace']==[1]
for v in [p['out'],b['out']['profile']]:assert len(v['profiles'])==32
r=next(v for v in b['out']['profile']['profiles']if(v['field'],v['degree'],v['trace_length'])==('GF2^83',5,83));assert(r['sequential']['scalar_mul'],r['sequential']['scalar_square'],r['jump']['scalar_mul'],r['jump']['scalar_square'],r['jump']['stats']['map_xors'])==(1968,410,1437,100,18397)
assert not json.loads((C/'LEDGER_VALIDATION.txt').read_text())['problems']
print('C82 hashes,three experiment outcomes,complete panels and retained costs verified; no qualified solver gain')
