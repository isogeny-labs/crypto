from pathlib import Path
import os,sys,json,hashlib,tarfile,base64,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.coefficient_fusion_20261117_v136b.kernel import fused,tiny_family
from research.correction_basis_checks_20261117_v128b.kernel import tiny
from research.precision_filtration_20261117_v132b.kernel import actual_family
from research.unit_cofactor_20261117_v134b.kernel import accepted,word
from research.local_split_circuit_20261116_v100.kernel import cnf
p=json.loads(saved['research/coefficient_fusion_20261117_v136b/oracles_profile_raw.json']);assert p['error']is None;o=p['out'];base,cols=tiny_family(2);G,oo,rr,meta=tiny(2,1);reference=set()
for x in range(1<<G.n):
 values=G.evaluate(x)
 if accepted(values,oo):reference.add((x,tuple(word(values,r)for r in rr)))
for v,rec in zip([2,6],o['tiny_variants']):
 H,nn,roots,selectors,h,coeffs,stats=fused(base,cols,2,1,v,1);found=set()
 for z in range(1<<H.n):
  values=H.evaluate(z)
  if accepted(values,nn):found.add((word(values,selectors),tuple(word(values,r)for r in roots)))
 assert found==reference and len(found)==rec['exact_component_tuples'];assert hashlib.sha256(json.dumps(sorted(found),separators=(',',':')).encode()).hexdigest()==rec['tuple_sha256']
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart);H,nn,roots,selectors,h,coeffs,stats=fused(base,cols,18,8,int('25bc29bd1e36b336',16),1);b=json.dumps({'nodes':H.nodes,'outputs':nn,'roots':roots,'selector_readback':selectors,'cofactor':h,'coefficient_readback':coeffs},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()==o['actual']['graph_sha256'];assert len(cnf(H,nn))==o['actual']['clauses']
print('C136 immutable source/raw hashes,complete tiny component tuple sets and fused actual graph/clauses replayed;no qualified speed gain.')
