from pathlib import Path
import json,hashlib,sys,tarfile,io,lzma,base64
C=Path(__file__).resolve().parent;sys.path.insert(0,str(C.parents[1]))
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
v=json.loads(Path('research/twist_nine_20261116_v87/raw.json').read_text());assert v['error']is None;r=v['out']['P224'];assert r['structure']=='full3torsion'and r['normalized_multiply3']and len(r['Frobenius_gcd'])==5 and len(r['kernel'])==9 and len({q['target_j']for q in r['degree3_quotients']})==4
P=Path('research/isogeny_root_solver_20261116_v87b');o=json.loads((P/'oracles_raw.json').read_text());b=json.loads((P/'cold_raw.json').read_text());assert o['error']is None and b['error']is None;assert sum(x['complete_original_roots']for x in o['out']['tiny'])==4900;assert len(b['out']['runs'])==18 and all(x['exact_root_count']==0 for x in b['out']['runs'])
idx=json.loads((C/'CHILD_INDEX.json').read_text());raw=lzma.decompress(base64.b64decode((C/'CHILD_EVIDENCE.tar.xz.b64').read_text()))
with tarfile.open(fileobj=io.BytesIO(raw))as t:
 assert set(t.getnames())==set(idx)and len(idx)==54
 for n,h in idx.items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
assert not json.loads((C/'LEDGER_VALIDATION.txt').read_text())['problems'];print('C87 hashes,complete tiny roots,actual calibration outcomes and54child files verified;no genuineGB speed ratio')
