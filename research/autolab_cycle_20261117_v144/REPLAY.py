from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64,itertools
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.shifted_tangent_20261117_v144b.kernel import packed
from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
F=[2,4,1,3,1,1];Ds=[[4,8,0,0,0,0],[0]*6];q=packed(F,Ds,3,2,3)['correction_masks'];f=Arithmetic(2,[0,1])
for lane,D in enumerate(Ds):
 a=jet(bitpolys(F),bitpolys(D),3,3,f)
 for k in [2,3]:assert [(v>>lane)&1 for v in q[k-2]]==a[k-1]['tangent']
a=json.loads(saved['research/shifted_tangent_20261117_v144b/oracles_raw.json']);assert a['error']is None
c=json.loads(saved['research/whole_cell_planes_20261117_v144c/census_raw.json'])
if c['error']is None and c['out'].get('complete_early_census'):
 o=c['out'];assert sum(o['rank_distribution'].values())==o['early_states'];assert sum(v*(1<<(63-o['early_rank']-int(r)))for r,v in o['rank_distribution'].items())==o['conditional_parameter_upper_bound']
print('C144 source/raw/child-ledger hashes,independent short/full tangent controls and census arithmetic replayed;truth planes deterministically regenerable;noqualifiedspeedratio.')
