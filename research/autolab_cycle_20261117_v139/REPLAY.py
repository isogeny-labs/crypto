from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64,random
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.guarded_block_20261117_v138b.kernel import build
from research.precision_filtration_20261117_v132b.kernel import actual_family
from research.field_tangent_20261117_v139c.kernel import family,tangent_row,certificate,dot,evaluate,correction
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
n=json.loads(saved['research/guarded_block_search_20261117_v139b/search_raw.json']);rank=json.loads(saved['research/delayed_block_20261117_v137b/oracles_profile_raw.json'])['out']['actual'];cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart);K=[int(v,16)for v in rank['kernel_hex']];H,nn,roots,selectors,zz,state,b,ff,meta=build(base,cols,18,8,K,17,[18,19,20],rank['records']);raw=json.dumps({'nodes':H.nodes,'outputs':nn,'roots':roots,'selector_readback':selectors,'delayed_readback':zz,'state_readback':state,'cofactors':b,'coefficient_readback':ff},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==n['out']['graph_sha256']
fraw=json.loads(saved['research/field_tangent_20261117_v139c/oracles_raw.json']);data=json.loads((R/'research/field_tangent_20261117_v139c/DATA.json').read_bytes());specs=json.loads((R/data['field_data']).read_bytes())['fields'];rng=random.Random(data['seed'])
for spec,rec in zip(specs,fraw['out']['contexts']):
 f=Arithmetic(spec['p'],spec['mod']);base,E,D=family(f,rng);R0=tangent_row(base,D,f);variations=[]
 for i in range(2):
  y=[0,0];y[i]=1;r=tangent_row(evaluate(base,E,y,f),D,f);variations.append([f.sub(a,b)for a,b in zip(r,R0)])
 v,slope=certificate(R0,variations,f);scale=rng.randrange(1,f.q);v=[f.mul(a,scale)for a in v];slope=dot(R0,v,f);assert R0==rec['R0']and variations==rec['variations']and v==rec['direction']and slope==rec['slope'];assert slope and all(dot(r,v,f)==0 for r in variations);F=evaluate(base,E,[1,1],f);assert f.sub(correction(evaluate(F,D,v,f),f),correction(F,f))==slope
 for _ in range(data['samples_per_field']*9):rng.randrange(f.q)
assert fraw['error']is None and fraw['out']['all_registered_contexts_complete']and len(fraw['out']['contexts'])==17
print('C139immutable source/raw,guarded graph and17symbolic field certificates/independent differences replayed;native search not rerun;noqualifiedspeedratio.')
