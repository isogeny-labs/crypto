from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
p=json.loads(saved['research/guarded_block_20261117_v138b/oracles_profile_raw.json'])
if p['error']is not None:
 print('C138immutable source/raw/failure preserved;no accepted compiled result:',p['error']);sys.exit(0)
from research.guarded_block_20261117_v138b.kernel import build,reduction,numeric
from research.coefficient_fusion_20261117_v136b.kernel import tiny_family
from research.correction_basis_checks_20261117_v128b.kernel import tiny
from research.unit_cofactor_20261117_v134b.kernel import word,accepted
from research.precision_filtration_20261117_v132b.kernel import actual_family,filtration,evaluate_family
from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys,image_representatives
from research.delayed_block_20261117_v137b.kernel import coefficients,row_action
from research.residue_quotient_20261116_v99b.kernel import apply
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.local_split_circuit_20261116_v100.kernel import cnf
for code in range(512):
 rows=[code>>(3*i)&7 for i in range(3)];rec=reduction(rows,3)
 for rhs in range(8):
  found=[numeric(rec,rhs,t)[0]for t in range(8)if numeric(rec,rhs,t)[1]];assert len(found)==len(set(found))and set(found)=={z for z in range(8)if row_action(rows,z)==rhs}
base,cols=tiny_family(2);K=[2,64,1024];ns=[2,3];early=filtration(cols,2)['lower_columns'];table=[];f=Arithmetic(2,[0,1])
for x in image_representatives(early,8):
 rows=[0,0];F=evaluate_family(base,cols,x)
 for j,v in enumerate(K):
  q=jet(bitpolys(F),bitpolys(coefficients(cols,v)),2,3,f)
  for i,n in enumerate(ns):rows[i]|=q[n-1]['tangent'][1]<<j
 table.append({'signature_hex':hex(apply(early,x)),'rows':rows})
H,nn,roots,selectors,zz,state,b,ff,meta=build(base,cols,2,3,K,2,ns,table);G,oo,rr,oldmeta=tiny(2,3);reference=set();found=set()
for x in range(1<<G.n):
 values=G.evaluate(x)
 if accepted(values,oo):reference.add((x,tuple(word(values,r)for r in rr)))
for x in range(1<<H.n):
 values=H.evaluate(x)
 if accepted(values,nn):found.add((word(values,selectors),tuple(word(values,r)for r in roots)))
assert found==reference and len(found)==p['out']['tiny']['exact_component_tuples']
a=json.loads(saved['research/delayed_block_20261117_v137b/oracles_profile_raw.json'])['out']['actual'];cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart);K=[int(v,16)for v in a['kernel_hex']];H,nn,roots,selectors,zz,state,b,ff,meta=build(base,cols,18,8,K,17,[18,19,20],a['records']);blob=json.dumps({'nodes':H.nodes,'outputs':nn,'roots':roots,'selector_readback':selectors,'delayed_readback':zz,'state_readback':state,'cofactors':b,'coefficient_readback':ff},separators=(',',':')).encode();assert hashlib.sha256(blob).hexdigest()==p['out']['actual']['graph_sha256'];assert len(cnf(H,nn))==p['out']['actual']['clauses'];print('C138immutable source/raw,allmatrix/rhscontrols,complete tiny component sets and actual guarded graph/clauses replayed;noqualifiedspeedratio.')
