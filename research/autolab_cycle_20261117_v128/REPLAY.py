from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  raw=t.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h,name;saved[name]=raw
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.correction_basis_checks_20261117_v128b.kernel import compile_all,tiny
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];profile=json.loads(saved['research/correction_basis_checks_20261117_v128b/oracles_profile_raw.json'])['out']
for count in [1,2,4,8]:
 G,oo,rr,meta=compile_all(cfg,chart,count);raw=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode();v=profile['variants'][str(count)];assert hashlib.sha256(raw).hexdigest()==v['graph_sha256'];assert len(G.nodes)-2==v['variables']and len(cnf(G,oo))==v['clauses'];assert len(set(oo[-1]['wires'][0]))==count
G,oo,rr,meta=tiny(2,999);v=G.evaluate(16);assert not any(v[w]for w in oo[-1]['wires'][0])and any(v[w]for w in oo[0]['wires'][0])
print('C128 immutable raw/source,four compact basis graphs and mandatory counterexample replayed;fieldcomponent17contexts;C129staticonly;noqualifiedspeedratio.')
