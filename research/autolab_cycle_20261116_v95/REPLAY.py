from pathlib import Path
import json,hashlib,sys,tarfile,io,lzma,base64
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT));members={};index=json.loads((C/'RAW_INDEX.json').read_text())
with tarfile.open(fileobj=io.BytesIO(lzma.decompress(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_text()))))as tf:
 assert set(tf.getnames())==set(index)
 for name,h in index.items():
  raw=tf.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h;members[name]=raw
for name,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
def data(name):return json.loads(members[name])
def exhaustive(spec,targets):
 p,A,B,M=[spec[k]for k in['p','A','B','M']];pairs=[]
 for x in range(M+1):
  for y in range(M+1):
   xy=x*y;s=x+y;pairs.append((x,y,(x-y)**2,(-2*(s*(xy+A)+2*B))%p,((xy-A)**2-4*B*s)%p))
 return [[(x,y)for x,y,a,b,c in pairs if(a*(u*u%p)+b*u+c)%p==0]for u in targets]
P='research/prime_pair_sum_20261116_v95';D=json.loads((ROOT/P/'DATA.json').read_text());a=data(P+'/oracles_raw.json');assert a['error']is None and len(a['out']['tiny'])==336 and a['out']['identity_checks']==2046 and a['out']['wrap_rejections']==4
from research.prime_pair_sum_20261116_v95.kernel import solve
for spec in D['tiny']:
 want=exhaustive(spec,[spec['u']])[0];assert solve(spec,'sum')['roots']==solve(spec,'source')['roots']==want
b=data(P+'/cold_raw.json');assert b['error']is None and b['out']['complete'];b=b['out'];assert len(b['runs'])==90;expected={spec['id']:[list(v)for v in exhaustive(spec,[spec['u']])[0]]for spec in D['cold']}
for r in b['runs']:
 v=r['parsed'];assert v['complete']and v['exact_root_count']==len(v['roots'])and v['roots']==expected[r['id']]and len(v['cpu_affinity'])==1
P='research/prime_target_batch_20261116_v95b';D=json.loads((ROOT/P/'DATA.json').read_text());b=data(P+'/raw.json');assert b['error']is None and len(b['out']['tiny'])==10 and sum(r['root_count']for r in b['out']['tiny'])==133;b=b['out'];assert b['complete']and len(b['runs'])==30
from research.prime_target_batch_20261116_v95b.kernel import solve as batch
for spec in D['tiny']:assert batch(spec,'cached')['roots']==exhaustive(spec,spec['targets'])
expected={spec['id']:[[list(t)for t in pairs]for pairs in exhaustive(spec,spec['targets'])]for spec in D['cases']}
for r in b['runs']:
 v=r['parsed'];assert v['complete']and v['roots']==expected[r['id']]and v['exact_root_counts']==[len(t)for t in v['roots']]and len(v['cpu_affinity'])==1
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed';print('C95 hashes/raw archive,336conic+10batchtinyoracles,120coldreports,and independentlyEXHAUSTIVEall5NIST original rootsets verified. No GB release claim.')
