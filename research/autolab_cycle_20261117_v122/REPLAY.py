from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for n,h in json.loads((C/'RAW_INDEX.json').read_text()).items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
for n,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
from research.shared_cubic_prefix_20261117_v122c.circuit import compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_text())['records'][1]['chart'];candidate,control=compile_all(cfg,chart)
for variant,h,n,c in[(candidate,'92264f64fff1821728233980c8f30858c1c1775dccf43ecb2d8739cf2c44c53f',77953,278047),(control,'dd2571f19f390171f6482bb6261a44a3e5813aabb2bea47b7dd5a35ba73b61a2',101646,364488)]:
 G,oo,A,B,roots=variant;b=json.dumps({'nodes':G.nodes,'outputs':oo,'A':A,'B':B,'roots':roots},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()==h;assert G.n==63 and len(G.nodes)-2==n and len(cnf(G,oo))==c
print('C122immutable source/raw and exactallfiveprefixsupport graphs replayed;searchUNKNOWN;C123verifier prototype unexecuted;no qualifiedspeedratio.')
