from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for n,h in json.loads((C/'RAW_INDEX.json').read_text()).items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
for n,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
from research.factor_support_checks_20261116_v120b.kernel import compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_text())['records'][1]['chart'];candidate,control,stats=compile_all(cfg,chart)
for variant,h,n,c in[(candidate,'764e379ad846e2279e7667cc3130a9c0ec1e18beec4e2701af6d91ab155054d1',58024,210289),(control,'383c6f0b28995d02d0105ab6aa1f8778c3f2fd64f04f71803f8461162790d698',81218,296189)]:
 G,oo,A,B=variant;b=json.dumps({'nodes':G.nodes,'outputs':oo,'A':A,'B':B},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()==h;assert G.n==63 and len(G.nodes)-2==n and len(cnf(G,oo))==c
print('C120immutable source/raw and both exactconditionalfactor-support graphs replayed;hardsearchUNKNOWN;C121quadraticprototype unexecuted;no qualifiedspeedratio.')
