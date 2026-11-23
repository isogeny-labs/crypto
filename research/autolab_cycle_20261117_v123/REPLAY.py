from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for n,h in json.loads((C/'RAW_INDEX.json').read_text()).items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
for n,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
from research.root_verifier_checks_20261117_v123b.kernel import compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_text())['records'][1]['chart'];candidate,control=compile_all(cfg,chart)
for variant,h,n,c in[(candidate,'1f77cc2df83d9ec0d4d7cb42f6ad0b795b75da40f061cfe4cf78c6a84b6b8733',53794,192485),(control,'1f1fba37b684af3885fb50dfc2e9f123d3f7f6c3ec4dc37c0b79530a933e0793',77953,278047)]:
 G,oo,roots=variant;b=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':roots},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()==h;assert G.n==63 and len(G.nodes)-2==n and len(cnf(G,oo))==c
print('C123immutable source/raw and isolatedexactroot-productverifier graphs replayed;searchUNKNOWN;C124XORprototype unexecuted;no qualifiedspeedratio.')
