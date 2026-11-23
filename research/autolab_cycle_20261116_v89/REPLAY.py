from pathlib import Path
import json,hashlib,sys,tarfile,io,lzma,base64
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h,n
P=ROOT/'research/kernel_trellis_20261116_v89';F=ROOT/'research/full_kernel_join_20261116_v89c'
a=json.loads((P/'oracles_raw.json').read_text());b=json.loads((P/'cold_raw.json').read_text());d=json.loads((F/'cold_raw.json').read_text());assert all(v['error']is None and v['out']['complete']for v in[a,b,d]);assert len(a['out']['groups'])==144 and len(a['out']['curves'])==32 and sum(v['complete_original_roots']for v in a['out']['curves'])==371;assert len(b['out']['runs'])==18 and len(d['out']['runs'])==27
idx=json.loads((C/'CHILD_INDEX.json').read_text());raw=lzma.decompress(base64.b64decode((C/'CHILD_EVIDENCE.tar.xz.b64').read_text()));expected={};assert len(idx)==135
with tarfile.open(fileobj=io.BytesIO(raw))as t:
 assert set(t.getnames())==set(idx)
 for n,h in idx.items():
  data=t.extractfile(n).read();assert hashlib.sha256(data).hexdigest()==h,n
  if n.endswith('.json'):
   v=json.loads(data)['parsed'];parts=Path(n).stem.split('_');i=int(parts[1]);kind='full'if'full_kernel_join'in n else'trellis';key=kind,i;assert v['complete']and v['exact_root_count']==len(v['roots']);assert v['exact_root_count']==(6561 if kind=='full'and i==0 else 0 if kind=='full'else81);expected.setdefault(key,v['roots']);assert expected[key]==v['roots']
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
from research.twist_three_isogeny_20261116_v86b.curve import add,scale,on,neg
for(kind,i),roots in expected.items():
 c=json.loads(((F if kind=='full'else P)/'DATA.json').read_text())['cases'][i];f=Field(c['p'],[0,1]);domain={U[0]for U in c['R']}
 for r in roots:
  assert len(r)==8 and all(x in domain for x in r[:5]);u=r[5:];assert s3(r[0],r[1],u[0],c['A'],c['B'],f)==0
  assert s3(u[0],r[2],u[1],c['A'],c['B'],f)==0 and s3(u[1],r[3],u[2],c['A'],c['B'],f)==0 and s3(u[2],r[4],c['Q'][0],c['A'],c['B'],f)==0
 if kind=='full':
  K=[None if U is None else tuple(U)for U in c['K']];R={tuple(U)for U in c['R']};p,A,B=c['p'],c['A'],c['B'];assert len(set(K))==9 and K[0]is None;assert all(on(U,p,A,B)and scale(3,U,p,A)is None for U in K);assert all(add(U,V,p,A)in K for U in K for V in K);assert all(add(U,k,p,A)in R for U in R for k in K)
assert not json.loads((C/'LEDGER_VALIDATION.txt').read_text())['problems'];print('C89 source/input hashes,135child files,45cold results and complete original equations verified;no qualifiedGBspeedup')
