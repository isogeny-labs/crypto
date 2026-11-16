from pathlib import Path
import json,hashlib,base64,gzip,tarfile,io
p=Path(__file__).resolve().parent
for n,h in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
idx=json.loads((p/'RAW_INDEX.json').read_text());z=base64.b64decode((p/'RAW.tar.gz.b64').read_text());assert hashlib.sha256(z).hexdigest()==idx['compressed_sha256']
with tarfile.open(fileobj=io.BytesIO(gzip.decompress(z)))as t:
 assert len(t.getmembers())==216
 for n,h in idx['files_sha256'].items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h
f=json.loads(Path('research/pidfd_observer_20261116_v79/fidelity_raw.json').read_text());assert f['error']is None and len(f['out']['runs'])==24 and len(f['out']['controls'])==8
a=json.loads(Path('research/pidfd_observer_20261116_v79/aa_raw.json').read_text());assert a['error']is None and len(a['out']['pairs'])==11 and a['out']['failed_pairs']==1 and not a['out']['all_pairs_within10percent']
print('C79 hashes,216 raw members, fidelity controls and retained A/A failure verified')
