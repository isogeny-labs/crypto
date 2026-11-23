from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  raw=t.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h,name;saved[name]=raw
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.frobenius_pair_checks_20261117_v127b.kernel import compile_all as frob
from research.clip_defect_checks_20261117_v127c.kernel import compile_all as defects
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart']
def check(G,oo,rr,v):
 raw=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==v['graph_sha256'];assert len(G.nodes)-2==v['variables']and len(cnf(G,oo))==v['clauses']
v=json.loads(saved['research/frobenius_pair_checks_20261117_v127b/oracles_profile_raw.json'])['out']['formulation'];G,oo,rr=frob(cfg,chart);check(G,oo,rr,v)
v=json.loads(saved['research/clip_defect_checks_20261117_v127c/oracles_profile_raw.json'])['out']['variants']
for count in [2,4,8]:
 G,oo,rr,meta=defects(cfg,chart,count);check(G,oo,rr,v[str(count)]);w=oo[-1]['wires'][0];assert len(set(w))==count//2 and all(w[i]==w[i+1]for i in range(0,count,2))
print('C127 immutable source/raw,Frobenius graph,three defect graphs and exact duplicate identities replayed. Native UNKNOWN; C128 static only; no qualified speed ratio.')
