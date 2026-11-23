from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  raw=t.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h,name;saved[name]=raw
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.root_verifier_checks_20261117_v123b.kernel import compile_all
from research.xor_recipe_checks_20261117_v125b.kernel import rewrite
from research.degree_clipped_checks_20261117_v125c.kernel import compile_all as clipped_compile
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];source,_=compile_all(cfg,chart);S,oo,rr=source;profile=json.loads(saved['research/xor_recipe_checks_20261117_v125b/profile_raw.json'])['out']
for k in [32,64,128]:
 G,outputs,roots,stats=rewrite(S,oo,rr,k,8000000);raw=json.dumps({'nodes':G.nodes,'outputs':outputs,'roots':roots},separators=(',',':')).encode();v=profile['variants'][str(k)];assert hashlib.sha256(raw).hexdigest()==v['graph_sha256'];assert len(G.nodes)-2==v['variables']and len(cnf(G,outputs))==v['clauses']
receipt=json.loads(saved['research/degree_clipped_checks_20261117_v125c/oracles_profile_raw.json']);wanted=receipt['out'].get('formulation')
if wanted:
 (G,outputs,roots),_=clipped_compile(cfg,chart);raw=json.dumps({'nodes':G.nodes,'outputs':outputs,'roots':roots},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==wanted['graph_sha256'];assert len(G.nodes)-2==wanted['variables']and len(cnf(G,outputs))==wanted['clauses']
print('C125 immutable source/raw, three recipe graphs and any completed clipped graph replayed. Conditional support only; no qualified speed ratio.')
