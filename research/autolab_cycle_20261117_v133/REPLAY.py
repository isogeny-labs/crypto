from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys,image_representatives,row_solutions
from research.precision_filtration_20261117_v132b.kernel import actual_family,filtration,evaluate_family
from research.residue_quotient_20261116_v99b.kernel import apply
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
p=json.loads(saved['research/delayed_tangent_20261117_v133b/oracles_profile_raw.json']);assert p['error']is None;o=p['out']['actual'];cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart);K=filtration(cols,17)['kernel'];early=filtration(cols,2)['lower_columns'];reps=image_representatives(early,4);f=Arithmetic(2,[0,1]);rows=[]
for x in reps:
 F=bitpolys(evaluate_family(base,cols,x));row=0
 for j,v in enumerate(K):
  D=bitpolys([apply([col[k]for col in cols],v)for k in range(6)]);row|=jet(F,D,18,18,f)[-1]['tangent'][1]<<j
 rows.append(row)
assert rows==o['tangent_rows'];answers=row_solutions(rows,len(K));assert answers==o['unit_combinations'];assert [hex(apply(K,v))for v in answers]==o['unit_directions_hex']
print('C133 immutable source/raw hashes and complete actual delayed-tangent rows/directions replayed;no qualified all-roots gain.')
