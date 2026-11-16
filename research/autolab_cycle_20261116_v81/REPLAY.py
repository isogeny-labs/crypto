from pathlib import Path
import json,hashlib,base64,lzma,tarfile,io
C=Path(__file__).resolve().parent
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
idx=json.loads((C/'CHILD_INDEX.json').read_text());raw=lzma.decompress(base64.b64decode((C/'CHILD_EVIDENCE.tar.xz.b64').read_text()))
with tarfile.open(fileobj=io.BytesIO(raw))as t:
 assert set(t.getnames())==set(idx)
 for n,h in idx.items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
p=Path('research/aa_amortization_20261116_v81');o=json.loads((p/'oracles_raw.json').read_text());assert o['error']is None and o['out']['cases']==6 and o['out']['digest_checks']==5
a=json.loads((p/'aa_raw.json').read_text());assert a['error']is None and len(a['out']['pairs'])==15 and a['out']['failed_pairs']==2 and not a['out']['all_pairs_within10percent']
assert {x['batch_size']for x in a['out']['pairs']}=={1,5,25,125,625}
print('C81 source/evidence hashes,120 child files and retained A/A failures verified; no qualified speedup')
