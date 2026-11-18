from pathlib import Path
import json,hashlib,sys,tarfile,io,lzma,base64
from fractions import Fraction
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT));index=json.loads((C/'RAW_INDEX.json').read_text());members={}
with tarfile.open(fileobj=io.BytesIO(lzma.decompress(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_text()))))as tf:
 assert set(tf.getnames())==set(index)
 for name,h in index.items():
  raw=tf.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h;members[name]=raw
for name,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
def data(name):return json.loads(members[name])
from research.affine_dual_cells_20261116_v97.kernel import Cells,parity
from research.affine_dual_cells_20261116_v97.checker import verify
from research.twist_auxiliary_20261116_v86.algebra import s3
P='research/affine_dual_cells_20261116_v97';D=json.loads((ROOT/P/'DATA.json').read_text());a=data(P+'/oracles_raw.json');assert a['error']is None and len(a['out']['cells'])==60 and a['out']['whole_field_checks']==1360 and a['out']['original_pair_checks']==740
for n,mod,dim in D['tiny']:
 for B in[1,2]:
  K=Cells(n,mod,dim,B)
  for dual in range(1,1<<n):
   if not parity(dual&B):continue
   r=K.cell(dual);assert verify(K,r);ws=[w for w in range(1<<n)if all(parity(dual&v)==0 for v in K.columns(w))];assert len(ws)==(0 if r['empty_cell']else 1<<r['dimension'])
   for w in ws:
    for x in range(1<<dim):
     for y in range(1<<dim):assert s3(x,y,w,0,B,K.S.f)!=0
a=data(P+'/profile_raw.json');assert a['error']is None;a=a['out'];assert len(a['queries'])==128 and a['rank_histogram']=={'53':4032};c=D['actual'];K=Cells(c['n'],c['mod'],c['d'],c['B'])
# Independently replay one whole-cell certificate per query; every dual's recorded rank is reconstructed.
for q in a['queries']:
 for i,v in enumerate(q['cells']):
  r=K.cell(v['dual']);assert not r['empty_cell']and r['rank']==v['rank']and r['dimension']==v['dimension'];assert all(parity(row&q['w'])==rhs for row,rhs in r['equations'])
  if i==0:assert verify(K,r)
P='research/structured_duals_20261116_v97b';b=data(P+'/raw.json');assert b['error']is None;b=b['out'];assert len(b['cells'])==62 and b['rank_histogram']=={'36':32,'37':30}and b['query_coverage']==0
for v in b['cells']:
 r=K.cell(v['dual']);assert verify(K,r)and not r['empty_cell']and r['rank']==35+v['numerator_degree']
assert b['union_upper_bound']==sum(1<<v['dimension']for v in b['cells'])and b['union_upper_bound']<Fraction(7,10**10)*(1<<83)
U=sum(Fraction(2)**(r+q-2-max(35+r,18+q))for r in range(1,18)for q in range(1,35));assert U==Fraction(3,4)-Fraction(10,1<<18)-Fraction(17,1<<36)and U<Fraction(3,4)
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed';print('C97 hashes,rawcells,tinywhole-fieldemptiness,GF83cellranks,structuredfamilyunionbound,and exactwhole-fieldcoverbarrier verified. No global solve/speedup claim.')
