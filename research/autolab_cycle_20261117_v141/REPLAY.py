from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64,itertools
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.implicit_projection_20261117_v141b.kernel import floor_sum,PrimeProjection,solve_prime_boxes
for n,m,a,b in itertools.product(range(5),range(1,8),range(-5,6),range(-5,6)):assert floor_sum(n,m,a,b)[0]==sum((a*i+b)//m for i in range(n))
for p in [2,3,5,7]:
 for r in range(p):
  S=PrimeProjection(p,(0,p//2),(0,p-1),r)
  for x in range(p):assert S.witness_count(x)==sum((x+r*b)%p<=p//2 for b in range(p))
assert solve_prime_boxes(5,[(0,1)]*2,[1,1],2,0,lambda z:(sum(z)-1)%5)['roots']==[(0,1),(1,0)]
d=json.loads(saved['research/implicit_projection_20261117_v141b/oracles_raw.json']);assert d['error']is None and d['out']['all_registered_contexts_complete'];assert len(d['out']['contexts'])==17 and len(d['out']['nist'])==5;print('C141 source/raw hashes,independent signed floor-sum and projected witness controls,F5complete-root counterexample replayed;17context evidence retained;noqualifiedspeedup.')
