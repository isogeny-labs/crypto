from pathlib import Path
import json,hashlib,sys,tarfile,io,lzma,base64
C=Path(__file__).resolve().parent;sys.path.insert(0,str(C.parents[1]))
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
assert 'FileExistsError'in(C/'FAILED_STDERR.txt').read_text()
P=Path('research/kernel_lift_solver_20261116_v88b');a=json.loads((P/'oracles_raw.json').read_text());b=json.loads((P/'cold_raw.json').read_text());assert a['error']is None and b['error']is None;assert len(a['out']['tiny'])==8 and sum(r['complete_original_roots']for r in a['out']['tiny'])==5038;assert all(r['exact_root_count']==81 for r in a['out']['actual']);assert len(b['out']['runs'])==27 and all(r['exact_root_count']==81 for r in b['out']['runs'])
idx=json.loads((C/'CHILD_INDEX.json').read_text());raw=lzma.decompress(base64.b64decode((C/'CHILD_EVIDENCE.tar.xz.b64').read_text()));expected={}
with tarfile.open(fileobj=io.BytesIO(raw))as t:
 assert set(t.getnames())==set(idx)and len(idx)==81
 for n,h in idx.items():
  data=t.extractfile(n).read();assert hashlib.sha256(data).hexdigest()==h,n
  if n.endswith('.json'):
   v=json.loads(data)['parsed'];key=int(n.split('_')[1]);assert v['complete']and len(v['roots'])==v['exact_root_count']==81;expected.setdefault(key,v['roots']);assert v['roots']==expected[key]
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
D=json.loads((P/'DATA.json').read_text())
for i,roots in expected.items():
 c=D['cases'][i];f=Field(c['p'],[0,1])
 for r in roots:
  u=r[5:];assert len(r)==8 and s3(r[0],r[1],u[0],c['A'],c['B'],f)==0
  assert s3(u[0],r[2],u[1],c['A'],c['B'],f)==0 and s3(u[1],r[3],u[2],c['A'],c['B'],f)==0 and s3(u[2],r[4],c['Q'][0],c['A'],c['B'],f)==0
assert not json.loads((C/'LEDGER_VALIDATION.txt').read_text())['problems'];print('C88 hashes,81child files,positive original equations and preserved failure verified;no genuineGB ratio')
