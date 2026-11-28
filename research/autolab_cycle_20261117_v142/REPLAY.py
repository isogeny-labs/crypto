from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64,itertools
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  z=t.extractfile(name).read();assert hashlib.sha256(z).hexdigest()==h,name;saved[name]=z
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.joint_translation_20261117_v142c.kernel import anchors,anchor_count,solve_interval,solve_subspace
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
for p in [3,5,7]:
 for M in range(p):
  for n in [1,2,3]:
   got=list(anchors(n,M));direct={tuple(x-min(z)for x in z)for z in itertools.product(range(M+1),repeat=n)};assert set(got)==direct and len(got)==anchor_count(n,M)
   for k in [0,1]:
    c=lambda z,p=p,k=k:(k*z[0]+sum((z[j]-z[0])**2 for j in range(1,len(z))))%p
    assert solve_interval(p,n,M,k,c)['roots']==sorted(z for z in itertools.product(range(M+1),repeat=n)if c(z)==0)
f=Arithmetic(2,[1,1,1]);D=list(range(4));c=lambda z:f.add(z[0],f.mul(f.add(z[1],z[0]),f.add(z[2],z[0])));assert solve_subspace(f,3,D,1,c)['roots']==sorted(z for z in itertools.product(D,repeat=3)if c(z)==0)
d=json.loads(saved['research/joint_translation_20261117_v142c/oracles_raw.json']);assert d['error']is None and d['out']['all_registered_contexts_complete'];assert len(d['out']['cohort'])==20
print('C142 source/raw/child-ledger hashes,prime joint frontiers,complete zero/nonzero-slope roots and GF4subspace oracle replayed;paired20-case evidence retained;noqualifiedspeedclaim.')
