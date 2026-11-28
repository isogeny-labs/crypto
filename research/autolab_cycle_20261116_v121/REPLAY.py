from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for n,h in json.loads((C/'RAW_INDEX.json').read_text()).items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
for n,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
from research.prefix_quadratic_checks_20261116_v121b.circuit import compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_text())['records'][1]['chart'];candidate,control=compile_all(cfg,chart)
for variant,h,n,c in[(candidate,'d657261dab0744c512966538c2117ffac5568848e676883d580206dbecb34937',57063,206195),(control,'b5a5ecf01a4bf07797508357c2572839001a773d939947c33213c4b5d5422bd3',62886,227582)]:
 G,oo,A,B,root=variant;b=json.dumps({'nodes':G.nodes,'outputs':oo,'A':A,'B':B,'root':root},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()==h;assert G.n==63 and len(G.nodes)-2==n and len(cnf(G,oo))==c
print('C121immutable source/raw and both exactquadraticprefix graphs replayed;hardsearchUNKNOWN;C122cubicprototype unexecuted;no qualifiedspeedratio.')
