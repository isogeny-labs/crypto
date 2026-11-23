from pathlib import Path
import json,hashlib,base64,gzip,tarfile,io
p=Path(__file__).resolve().parent
for n,h in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
idx=json.loads((p/'COLD_RAW_INDEX.json').read_text());z=base64.b64decode((p/'COLD_RAW.tar.gz.b64').read_text());assert hashlib.sha256(z).hexdigest()==idx['compressed_sha256']
with tarfile.open(fileobj=io.BytesIO(gzip.decompress(z)))as t:
 assert len(t.getmembers())==120
 for n,h in idx['files_sha256'].items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h
r=json.loads(Path('research/mersenne_power_20261116_v77/oracles_raw.json').read_text());assert r['error']is None and r['out']['taylor_cases']==2232 and r['out']['root_polynomials']==240
v=json.loads(Path('research/mersenne_power_20261116_v77/cold_raw.json').read_text());assert v['error']is None and len(v['out']['rows'])==30 and len(v['out']['summaries'])==5
print('C77 source/evidence hashes and120 raw members verified; complete-root cold panel preserved, no qualified GB gain')
