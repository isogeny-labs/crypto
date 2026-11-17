from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for n,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
  b=t.extractfile(n).read();assert hashlib.sha256(b).hexdigest()==h,n;saved[n]=b
for n,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
from research.root_verifier_checks_20261117_v123b.kernel import compile_all
from research.xor_atom_checks_20261117_v124b.kernel import rewrite
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_text())['records'][1]['chart'];source,_=compile_all(cfg,chart);S,oo,rr=source;profile=json.loads(saved['research/xor_atom_checks_20261117_v124b/profile_raw.json'])['out']
for k in [4,8,16]:
 G,outputs,roots,stats=rewrite(S,oo,rr,k,8000000);b=json.dumps({'nodes':G.nodes,'outputs':outputs,'roots':roots},separators=(',',':')).encode();wanted=profile['variants'][str(k)];assert hashlib.sha256(b).hexdigest()==wanted['graph_sha256'];assert G.n==63 and len(G.nodes)-2==wanted['variables']and len(cnf(G,outputs))==wanted['clauses']
print('C124immutable source/raw and allthree nonlinearXOR calibrationgraphs replayed;hardsearchUNKNOWN;C125recipe prototype unexecuted;no qualifiedspeedratio.')
