from pathlib import Path
import json,hashlib,base64,gzip,tarfile,io
p=Path(__file__).resolve().parent
for name,digest in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():
 assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==digest,name
index=json.loads((p/'COLD_RAW_INDEX.json').read_text());z=base64.b64decode((p/'COLD_RAW_ARCHIVE.tar.gz.b64').read_text());assert hashlib.sha256(z).hexdigest()==index['compressed_sha256']
with tarfile.open(fileobj=io.BytesIO(gzip.decompress(z)))as t:
 assert len(t.getmembers())==360
 for name,digest in index['files_sha256'].items():assert hashlib.sha256(t.extractfile(name).read()).hexdigest()==digest
for root,name in [('research/prime_orbit_crossover_20261116_v72','ladder_raw.json'),('research/prime_orbit_crossover_20261116_v72','cold_raw.json'),('research/frobenius_chain_width_20261116_v72','raw.json')]:
 r=json.loads(Path(root,name).read_text());assert r['error']is None
 if name=='ladder_raw.json':assert len(r['out']['rows'])==15 and r['out']['oracle_systems']==96
 if name=='cold_raw.json':assert len(r['out']['rows'])==90 and len(r['out']['summaries'])==10
 if name=='raw.json':assert len(r['out']['cases'])==16
print('C72 hashes, 360 raw archive members and all recorded outcomes verified; no qualified GB speedup')
