from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  raw=t.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h,name;saved[name]=raw
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.product_tree_checks_20261117_v126c.kernel import compile_all,trees
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];profile=json.loads(saved['research/product_tree_checks_20261117_v126c/oracles_profile_raw.json'])['out']
for k,tree in enumerate(trees(list(range(5)))):
 if str(k)not in profile['variants']:continue
 G,outputs,roots=compile_all(cfg,chart,tree);raw=json.dumps({'nodes':G.nodes,'outputs':outputs,'roots':roots},separators=(',',':')).encode();v=profile['variants'][str(k)];assert hashlib.sha256(raw).hexdigest()==v['graph_sha256'];assert len(G.nodes)-2==v['variables']and len(cnf(G,outputs))==v['clauses']
print('C126 immutable source/raw and all completed product trees replayed. Native search remains UNKNOWN; odd component13fields; C127 static only; no qualified speed ratio.')
