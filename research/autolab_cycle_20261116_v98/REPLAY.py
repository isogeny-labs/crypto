from pathlib import Path
import sys,json,hashlib,tarfile,io,lzma,base64
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT));index=json.loads((C/'RAW_INDEX.json').read_text());members={}
with tarfile.open(fileobj=io.BytesIO(lzma.decompress(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_text()))))as tf:
 assert set(tf.getnames())==set(index)
 for name,h in index.items():
  raw=tf.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h,name;members[name]=raw
for name,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
def data(name):return json.loads(members[name])
P='research/outer_alignment_20261116_v98';D=json.loads((ROOT/P/'DATA.json').read_text());a=data(P+'/oracles_raw.json');assert a['error']is None and len(a['out']['cases'])==28 and sum(r['root_count']for r in a['out']['cases'])==2219
from research.outer_alignment_20261116_v98.kernel import solve,family,covered,zero_pair
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
for n,mod,dim in D['tiny']:
 f=Field(2,mod);q=1<<n;domain=range(1<<dim);adj={(x,y):[z for z in range(q)if s3(x,y,z,0,1,f)==0]for x in range(q)for y in range(q)}
 for T in range(q):
  roots=[]
  for x1 in domain:
   for x2 in domain:
    for a in adj[x1,x2]:
     for x3 in domain:
      for z in adj[a,x3]:
       for x4 in domain:
        for b in adj[z,x4]:
         for x5 in domain:
          if T in adj[b,x5]:roots.append((x1,x2,x3,x4,x5,a,z,b))
  roots.sort()
  for method in['join','block','filtered']:assert solve({'n':n,'mod':mod,'d':dim,'T':T},method)['roots']==roots
b=data(P+'/profile_raw.json');assert b['error']is None;b=b['out'];assert len(b['outer'])==128 and b['endpoint_blocks']==136 and b['finite_w']==117 and b['covered_w']==b['cell_only_rejects']==0 and b['no_w_rejects']==76 and b['verified_sample_roots']==0
from research.separator_images_20261116_v91.kernel import Solver
c=D['actual'];F=Solver(c['n'],c['mod'],c['n']);_,cells=family(c['n'],c['mod'],c['d']);assert len(cells)==62
for row in b['outer']:
 x1,x2,x5=row['sources'];assert row['left_endpoints']==F.solve(*F.coefficients(x1,x2,1),'stream')and row['right_endpoints']==F.solve(*F.coefficients(x5,c['T'],1),'stream')
 for v in row['blocks']:
  assert v['w']==F.solve(*F.coefficients(v['a'],v['b'],1),'stream');assert v['covered']==[covered(w,cells)for w in v['w']];assert v['zero_branch']==zero_pair(F,v['a'],v['b'],c['d']);assert v['diagonal']==(v['a']==v['b']);assert v['complete']and v['root_count']==len(v['roots'])
  if v['reject']:assert not v['w']and not v['zero_branch']and not v['diagonal']and not v['roots']
b=data(P+'/cold_raw.json');assert b['error']is None;b=b['out'];assert b['complete']and len(b['runs'])==18;expected={}
for r in b['runs']:
 v=r['parsed'];assert v['complete']and v['exact_root_count']==len(v['roots'])and len(v['cpu_affinity'])==1;expected.setdefault(r['id'],v['roots']);assert expected[r['id']]==v['roots'];spec=next(c for c in D['cold']if c['id']==r['id']);f=Field(2,spec['mod'])
 for x1,x2,x3,x4,x5,a,z,bb in v['roots']:assert all(s3(u,vv,w,0,1,f)==0 for u,vv,w in[(x1,x2,a),(a,x3,z),(z,x4,bb),(bb,x5,spec['T'])])
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed';print('C98 hashes/archive,28completechainoracles2219roots,actual136blockcoverageand18coldoriginalrootreports verified. Slower integrations retained;no global speedup.')
