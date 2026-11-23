from pathlib import Path
import json,time,random,itertools,hashlib
from .common import P,finish
from .kernel import tiny,compile_all
from research.product_tree_checks_20261117_v126c.kernel import tiny as old_tiny,compile_all as old_compile
from research.local_split_circuit_20261116_v100.kernel import cnf
from research.prefix_translation_20261116_v115.kernel import product
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
START=time.monotonic();out={'variants':{},'exhaustive_assignments':0,'supported_assignments':0,'random_assignments':0,'actual_acceptance_controls':0,'original_complete':False,'selected_count':None};error=None
TREE=(0,(1,(2,(3,4))))
def catalog(d):
 top=1<<(d-1);return {tuple(product(low+high)):list(low+high)for low in itertools.combinations_with_replacement(range(top),2)for high in itertools.combinations_with_replacement(range(top,1<<d),3)}
def encode(H,d):
 x=0;offset=0
 for j,v in enumerate(H[:5]):x|=(reverse(v,(5-j)*(d-1))>>1)<<offset;offset+=(5-j)*(d-1)
 return x

def evaluate(C,oo,rr,x):
 v=C.evaluate(x);return not any(v[w]for row in oo for field in row['wires']for w in field),sorted(sum(v[w]<<j for j,w in enumerate(row))for row in rr)

def affine_forms(C):
 forms=[0,1]+[1<<(j+1)for j in range(C.n)]
 for node in C.nodes[C.n+2:]:
  kind,a,*rest=node;fa=forms[a];fb=forms[rest[0]]if rest else 1
  if kind=='not':f=None if fa is None else fa^1
  elif kind=='xor':f=None if fa is None or fb is None else fa^fb
  elif fa==0 or fb==0:f=0
  elif fa==1:f=fb
  elif fb==1 or fa==fb and fa is not None:f=fa
  else:f=None
  forms.append(f)
 return forms
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed'])
 for width in [2,3]:
  expected=catalog(width);supported={encode(H,width):roots for H,roots in expected.items()};cases=list(range(1<<(15*(width-1))))if width==2 else list(supported)+[rng.randrange(1<<(15*(width-1)))for _ in range(128)]
  for count in data['counts']:
   G,oo,rr,meta=tiny(width,count)
   for x in cases:
    ok,roots=evaluate(G,oo,rr,x);assert ok==(x in supported)
    if ok:assert roots==supported[x]
   if width==2:out['exhaustive_assignments']+=len(cases)
   else:out['supported_assignments']+=len(supported);out['random_assignments']+=128
 cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];old,co,cr=old_compile(cfg,chart,TREE);samples=[rng.randrange(1<<old.n)for _ in range(32)];wanted=[evaluate(old,co,cr,x)for x in samples]
 for count in data['counts']:
  t=time.monotonic();G,oo,rr,meta=compile_all(cfg,chart,count)
  for x,answer in zip(samples,wanted):
   got=evaluate(G,oo,rr,x);assert got[0]==answer[0]
   if got[0]:assert got[1]==answer[1]
   out['actual_acceptance_controls']+=1
  rows=oo[-1]['wires'][0];forms=affine_forms(G);raw=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode()
  with(P/('graph_'+str(count)+'.json')).open('xb')as f:f.write(raw)
  out['variants'][str(count)]={'variables':len(G.nodes)-2,'clauses':len(cnf(G,oo)),'graph_sha256':hashlib.sha256(raw).hexdigest(),'nonconstant_defects':sum(w>1 for w in rows),'constant_zero_defects':rows.count(0),'constant_one_defects':rows.count(1),'affine_forms':[forms[w]for w in rows],'rows':meta,'compile_and_check_seconds':time.monotonic()-t}
 eligible=[(v['clauses'],int(k))for k,v in out['variants'].items()if v['nonconstant_defects']]
 if eligible:out['selected_count']=min(eligible)[1]
 out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Earlyclipdefects preserve exact support and accepted multisets; fixed2/4/8row calibration and sound affine propagation.','If useful defects survive, freeze a changed hard diagnostic; unknown affine classification is not proof of nonlinearity or a cell cut.')
