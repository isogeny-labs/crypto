from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  raw=t.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h,name;saved[name]=raw
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.correction_basis_checks_20261117_v128b.kernel import compile_all
from research.correction_cone_anf_checks_20261117_v129b.kernel import expand,ExpansionLimit
from research.correction_cone_zdd_20261117_v129c.kernel import compile_cones,ZDDLimit
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];G,oo,rr,meta=compile_all(cfg,chart,8);wires=oo[-1]['wires'][0];flat=json.loads(saved['research/correction_cone_anf_checks_20261117_v129b/profile_raw.json'])['out'];zdd=json.loads(saved['research/correction_cone_zdd_20261117_v129c/oracles_profile_raw.json'])['out']
for count in [1,2,4,8]:
 try:expand(G,wires[:count],max_degree=8,max_monomials=50000,max_stored=1000000,max_work=2000000)
 except ExpansionLimit as e:assert str(e)==flat['variants'][str(count)]['reason']and e.stats==flat['variants'][str(count)]['stats']
 else:raise AssertionError('expected preserved flatcap')
 try:compile_cones(G,wires[:count],max_nodes=100000,max_cache=500000,max_work=2000000)
 except ZDDLimit as e:assert str(e)==zdd['variants'][str(count)]['reason']and e.stats==zdd['variants'][str(count)]['stats']
 else:raise AssertionError('expected preserved compressedcap')
print('C129 immutable source/raw and all8flat/compressed cap reasons+stats replayed;no final actual polynomials/degrees;C130staticonly;noqualifiedspeedratio.')
