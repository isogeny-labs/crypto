from pathlib import Path
import json,time,random,itertools,hashlib
from .common import P,finish
from .kernel import tiny,compile_all
from research.product_tree_checks_20261117_v126c.kernel import tiny as old_tiny,compile_all as old_compile
from research.local_split_circuit_20261116_v100.kernel import cnf
from research.prefix_translation_20261116_v115.kernel import product
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
START=time.monotonic();out={'exhaustive_assignments':0,'supported_assignments':0,'random_assignments':0,'actual_value_controls':0,'formulation':None,'original_complete':False};error=None
TREE=(0,(1,(2,(3,4))))
def catalog(d):
 top=1<<(d-1);return {tuple(product(low+high)):list(low+high)for low in itertools.combinations_with_replacement(range(top),2)for high in itertools.combinations_with_replacement(range(top,1<<d),3)}
def encode(H,d):
 x=0;offset=0
 for j,v in enumerate(H[:5]):x|=(reverse(v,(5-j)*(d-1))>>1)<<offset;offset+=(5-j)*(d-1)
 return x

def values(C,oo,rr,x):
 v=C.evaluate(x);return [v[w]for row in oo for field in row['wires']for w in field]+[v[w]for row in rr for w in row]
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed'])
 for width in [2,3]:
  G,oo,rr=tiny(width);old,co,cr=old_tiny(width,TREE);expected=catalog(width);supported={encode(H,width):roots for H,roots in expected.items()};cases=list(range(1<<G.n))if width==2 else list(supported)+[rng.randrange(1<<G.n)for _ in range(128)]
  for i,x in enumerate(cases):
   assert values(G,oo,rr,x)==values(old,co,cr,x);v=G.evaluate(x);ok=not any(v[w]for row in oo for field in row['wires']for w in field);assert ok==(x in supported)
   if ok:assert sorted(sum(v[w]<<j for j,w in enumerate(row))for row in rr)==supported[x]
  if width==2:out['exhaustive_assignments']+=len(cases)
  else:out['supported_assignments']+=len(supported);out['random_assignments']+=128
 cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];t=time.monotonic();G,oo,rr=compile_all(cfg,chart);out['cold_compile_seconds']=time.monotonic()-t;old,co,cr=old_compile(cfg,chart,TREE);receipt=json.loads(Path(data['control_receipt']).read_bytes())['out']['variants']['0'];raw=json.dumps({'nodes':old.nodes,'outputs':co,'roots':cr},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==receipt['graph_sha256']
 for _ in range(32):
  x=rng.randrange(1<<G.n);assert values(G,oo,rr,x)==values(old,co,cr,x);out['actual_value_controls']+=1
 raw=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode()
 with(P/'graph.json').open('xb')as f:f.write(raw)
 out['formulation']={'inputs':G.n,'variables':len(G.nodes)-2,'clauses':len(cnf(G,oo)),'graph_sha256':hashlib.sha256(raw).hexdigest(),'control_variables':len(old.nodes)-2,'control_clauses':len(cnf(old,co))};out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Frobeniusrootpair exact observable equivalence and actual63parameter profile.','If size/cost regresses, preserve result and use early clipping defects; no identical hard timeout retry.')
