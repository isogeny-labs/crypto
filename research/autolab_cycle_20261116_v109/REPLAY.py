from pathlib import Path
import sys,json,hashlib,base64,tarfile,io,ast
C=Path(__file__).resolve().parent;R=C.parents[1];sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():
 b=(R/name).read_bytes();assert hashlib.sha256(b).hexdigest()==h,name
 if name.endswith('.py'):ast.parse(b)
p=R/'research/trace_zero_predicate_20261116_v105/templates.json'
if not p.exists():
 a=R/'research/autolab_cycle_20261116_v105'
 with tarfile.open(fileobj=io.BytesIO(base64.b64decode((a/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:b=t.extractfile(str(p.relative_to(R))).read()
 assert hashlib.sha256(b).hexdigest()=='dc4f32d05c83f23654c725415db7c02107c7a8fa2539fdfc90d92f3ca4f60baf'
 with p.open('xb')as f:f.write(b)
source=R/'research/predicate_dispatch_20261116_v105b/graph.json'
if not source.exists():
 from research.predicate_dispatch_20261116_v105b.kernel import compile_all as old_compile
 d=json.loads((source.parent/'DATA.json').read_text());G,outs,*_=old_compile(d);b=json.dumps({'nodes':G.nodes,'outputs':outs},separators=(',',':')).encode()
 assert hashlib.sha256(b).hexdigest()=='a459425363fe60d7eb8232068277700f5a757ade974f0a56ae0955e2b43a3da4'
 with source.open('xb')as f:f.write(b)
from research.systematic_residue_20261116_v109.kernel import compile_all,old_value,new_value
P='research/systematic_residue_20261116_v109/';d=json.loads((R/(P+'DATA.json')).read_text());G,outs,m,blocks=compile_all(d);b=json.dumps({'nodes':G.nodes,'outputs':outs},separators=(',',':')).encode();receipt=json.loads(saved[P+'compile_raw.json']);assert hashlib.sha256(b).hexdigest()==receipt['out']['graph_sha256'];assert m==json.loads(saved[P+'coordinates.json']);assert len(m['K'])==4 and m['width']==72
for i in range(76):assert new_value(m,old_value(m,1<<i))==1<<i
assert json.loads(saved[P+'oracles_raw.json'])['error']is None
search=json.loads(saved[P+'search_raw.json']);assert search['out']['original_complete']is False
print('C109 immutable bytes, systematic bijection and compiled graph reproduced; native search not rerun; no original-system speedup certified.')
