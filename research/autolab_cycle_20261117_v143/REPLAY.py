from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64,itertools
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  z=t.extractfile(name).read();assert hashlib.sha256(z).hexdigest()==h,name;saved[name]=z
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.weighted_frontier_20261117_v143b.kernel import solve,frontier_count,frontier
for p in [3,5,7]:
 for upper in itertools.product(range(min(p,3)),repeat=2):
  domain=list(itertools.product(*[range(x+1)for x in upper]))
  for v in itertools.product(range(-2,3),repeat=2):
   if not any(v):continue
   j=next(i for i,a in enumerate(v)if a)
   for s in [0,1]:
    c=lambda z,p=p,v=v,j=j,s=s:(s*z[j]+(v[j]*z[1-j]-v[1-j]*z[j])**2)%p
    got=solve(p,upper,v,s*v[j],c);assert got['complete']and got['roots']==sorted(z for z in domain if c(z)==0)
a=json.loads(saved['research/weighted_frontier_20261117_v143b/oracles_raw.json']);d=json.loads(saved['research/weighted_frontier_profile_20261117_v143c/profile_raw.json']);assert a['error']is None and a['out']['all_registered_contexts_complete'];assert d['error']is None and d['out']['all_registered_contexts_complete'];assert len(d['out']['contexts'])==17
print('C143 source/raw/child-ledger hashes and independent signed/zero-slope weighted original-root oracles replayed;17coefficient-family profile evidence retained;noqualifiedspeedclaim.')
