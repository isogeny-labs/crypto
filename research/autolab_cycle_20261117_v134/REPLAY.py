from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.unit_cofactor_20261117_v134b.kernel import transform,word,accepted
from research.correction_basis_checks_20261117_v128b.kernel import tiny,compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
p=json.loads(saved['research/unit_cofactor_20261117_v134b/oracles_profile_raw.json']);assert p['error']is None;o=p['out'];G,oo,rr,meta=tiny(2,1);target=oo[-1]['wires'][0][0];reference=set()
for x in range(1<<G.n):
 values=G.evaluate(x)
 if accepted(values,oo):reference.add((x,tuple(word(values,r)for r in rr)))
for v,rec in zip([2,6],o['tiny_variants']):
 H,nn,roots,selectors,h,stats=transform(G,oo,rr,target,v,1);found=set()
 for z in range(1<<H.n):
  values=H.evaluate(z)
  if accepted(values,nn):found.add((word(values,selectors),tuple(word(values,r)for r in roots)))
 assert found==reference and len(found)==rec['exact_component_tuple_count'];assert hashlib.sha256(json.dumps(sorted(found),separators=(',',':')).encode()).hexdigest()==rec['component_tuple_sha256']
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];G,oo,rr,meta=compile_all(cfg,chart,8);H,nn,roots,selectors,h,stats=transform(G,oo,rr,oo[-1]['wires'][0][0],int('25bc29bd1e36b336',16),1);blob=json.dumps({'nodes':H.nodes,'outputs':nn,'roots':roots,'selector_readback':selectors,'cofactor':h},separators=(',',':')).encode();assert hashlib.sha256(blob).hexdigest()==o['actual']['graph_sha256'];assert len(cnf(H,nn))==o['actual']['new_clauses']
print('C134 immutable source/raw hashes,complete tiny component tuple sets and actual transformed graph hash/clauses replayed;no qualified speed gain.')
