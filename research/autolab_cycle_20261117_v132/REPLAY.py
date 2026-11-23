from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.precision_filtration_20261117_v132b.kernel import actual_family,filtration,numeric_correction
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart)
o=json.loads(saved['research/precision_filtration_20261117_v132b/oracles_profile_raw.json'])['out']
for r in o['records']:
 q=filtration(cols,r['precision']);assert q['rank']==r['rank']and q['augmented_rank']==r['augmented_rank']and q['witness']is None
assert numeric_correction([0,0,1,1,1,1],[[0,2,0,0,0,0]],0,2,2)==0
assert numeric_correction([0,0,1,1,1,1],[[0,2,0,0,0,0]],1,2,2)==1
print('C132 immutable source/raw hashes, all8 actual rank results and rank-equality counterexample replayed; no qualified gain.')
