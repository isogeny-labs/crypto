from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():assert hashlib.sha256(t.extractfile(name).read()).hexdigest()==h,name
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.domain_pullback_20261117_v140c.kernel import solve_domains
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
f=Arithmetic(5,[0,1]);r=solve_domains(f,[(0,1)]*2,[1,1],2,0,lambda z:(sum(z)-1)%5);assert r['roots']==[(0,1),(1,0)]
print('C140source/raw/child-ledger hashes and explicit missed-root counterexample replayed;native search not rerun;noqualifiedspeedclaim.')
