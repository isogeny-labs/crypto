from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.delayed_block_20261117_v137b.kernel import solve,solutions,row_action,coefficients
from research.residue_quotient_20261116_v99b.kernel import apply,basis
from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys,image_representatives
from research.precision_filtration_20261117_v132b.kernel import actual_family,filtration,evaluate_family
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
for code in range(512):
 rows=[code>>(3*i)&7 for i in range(3)]
 for rhs in range(8):assert solutions(solve(rows,rhs,3))=={x for x in range(8)if row_action(rows,x)==rhs}
p=json.loads(saved['research/delayed_block_20261117_v137b/oracles_profile_raw.json']);assert p['error']is None;a=p['out']['actual'];assert a['complete'];cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart);K=filtration(cols,17)['kernel'];early=filtration(cols,4)['lower_columns'];reps=image_representatives(early,8);assert len(reps)==len(a['records'])==256;f=Arithmetic(2,[0,1]);bound=0
for x,record in zip(reps,a['records']):
 F=[v&15 for v in evaluate_family(base,cols,x)];rows=[0]*3
 for j,v in enumerate(K):
  q=jet(bitpolys(F),bitpolys(coefficients(cols,v)),18,20,f)
  for i,n in enumerate([18,19,20]):rows[i]|=q[n-1]['tangent'][1]<<j
 assert rows==record['rows'];r=len(basis(rows)[0]);assert r==record['rank'];bound+=1<<(55-r)
assert bound==a['conditional_parameter_upper_bound'];print('C137 immutable source/raw hashes,all4096matrix/rhsoracles,all256actual tangent matrices/ranks and conditional bound replayed;noqualifiedspeedratio.')
