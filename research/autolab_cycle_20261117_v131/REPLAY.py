from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.correction_basis_checks_20261117_v128b.kernel import compile_all
from research.live_correction_zdd_20261117_v130c.kernel import compile_live
from research.correction_cone_zdd_20261117_v129c.kernel import ZDDLimit
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];G,oo,rr,meta=compile_all(cfg,chart,8);wires=oo[-1]['wires'][0];p=json.loads(saved['research/live_correction_zdd_20261117_v130c/oracles_profile_raw.json'])['out']
for count in [1,2,4,8]:
 try:compile_live(G,wires[:count],collect_every=128,max_nodes=100000,max_cache=500000,max_work=2000000,max_total_nodes=500000,max_gc_visits=5000000)
 except ZDDLimit as e:assert str(e)==p['variants'][str(count)]['reason']and e.stats==p['variants'][str(count)]['stats']
 else:raise AssertionError('expectedpreservedcap')
print('C131immutable source/raw and all4collector capcounters replayed;unit-direction lead unexecuted;noqualifiedspeedratio.')
