from pathlib import Path
import json,time,random,itertools,hashlib
from .common import P,finish
from .kernel import tiny,compile_all
from research.clip_defect_checks_20261117_v127c.kernel import defects
from research.product_tree_checks_20261117_v126c.kernel import tiny as old_tiny,compile_all as old_compile
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.prefix_translation_20261116_v115.kernel import product
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
START=time.monotonic();out={'variants':{},'exhaustive_assignments':0,'supported_assignments':0,'random_assignments':0,'basis_equivalence_checks':0,'coefficient_identity_checks':0,'counterexample_checks':0,'actual_acceptance_controls':0,'selected_count':None,'original_complete':False};error=None
TREE=(0,(1,(2,(3,4))))
def catalog(d):
 top=1<<(d-1);return {tuple(product(low+high)):list(low+high)for low in itertools.combinations_with_replacement(range(top),2)for high in itertools.combinations_with_replacement(range(top,1<<d),3)}
def encode(H,d):
 x=0;offset=0
 for j,v in enumerate(H[:5]):x|=(reverse(v,(5-j)*(d-1))>>1)<<offset;offset+=(5-j)*(d-1)
 return x

def evaluate(C,oo,rr,x):
 v=C.evaluate(x);return not any(v[w]for row in oo for field in row['wires']for w in field),sorted(sum(v[w]<<j for j,w in enumerate(row))for row in rr)

def audit_circuit(d):
 C=Circuit(15*(d-1));F=[];offset=2
 for j in range(5):
  cap=(5-j)*(d-1);F.append([int(j>=2)]+list(range(offset,offset+cap)));offset+=cap
 F.append([1]);return C,defects(F,d,C)
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed'])
 for width in [2,3]:
  expected=catalog(width);supported={encode(H,width):roots for H,roots in expected.items()};cases=list(range(1<<(15*(width-1))))if width==2 else list(supported)+[rng.randrange(1<<(15*(width-1)))for _ in range(128)];A,rows=audit_circuit(width);m=width-1
  for x in cases:
   v=A.evaluate(x);allzero=not any(v[r['wire']]for r in rows);basiszero=not any(v[r['wire']]for r in rows if r['factor']=='A');assert allzero==basiszero;out['basis_equivalence_checks']+=1;lookup={(r['precision'],r['factor'],r['coefficient']):v[r['wire']]for r in rows}
   for n in range(m+1,3*m+1):
    a1=lookup[n,'A',1];assert lookup[n,'B',2]==a1;out['coefficient_identity_checks']+=1
    if n>2*m:assert lookup[n,'B',1]==lookup[n,'A',0]^a1;out['coefficient_identity_checks']+=1
  for count in data['counts']:
   G,oo,rr,meta=tiny(width,count)
   for x in cases:
    ok,roots=evaluate(G,oo,rr,x);assert ok==(x in supported)
    if ok:assert roots==supported[x]
   if width==2:out['exhaustive_assignments']+=len(cases)
   else:out['supported_assignments']+=len(supported);out['random_assignments']+=128
 G,oo,rr,meta=tiny(2,999);v=G.evaluate(data['counterexample_x']);assert not any(v[w]for w in oo[-1]['wires'][0])and any(v[w]for w in oo[0]['wires'][0]);A,rows=audit_circuit(2);v=A.evaluate(data['counterexample_x']);assert not any(v[r['wire']]for r in rows);out['counterexample_checks']+=1
 cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];old,co,cr=old_compile(cfg,chart,TREE);receipt=json.loads(Path(data['control_receipt']).read_bytes())['out']['variants']['0'];raw=json.dumps({'nodes':old.nodes,'outputs':co,'roots':cr},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==receipt['graph_sha256'];source_clauses=len(cnf(old,co));out['source']={'variables':len(old.nodes)-2,'clauses':source_clauses,'graph_sha256':hashlib.sha256(raw).hexdigest()};samples=[rng.randrange(1<<old.n)for _ in range(32)];wanted=[evaluate(old,co,cr,x)for x in samples]
 for count in data['counts']:
  t=time.monotonic();G,oo,rr,meta=compile_all(cfg,chart,count)
  for x,answer in zip(samples,wanted):
   got=evaluate(G,oo,rr,x);assert got[0]==answer[0]
   if got[0]:assert got[1]==answer[1]
   out['actual_acceptance_controls']+=1
  raw=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode()
  with(P/('graph_'+str(count)+'.json')).open('xb')as f:f.write(raw)
  wires=oo[-1]['wires'][0];out['variants'][str(count)]={'variables':len(G.nodes)-2,'clauses':len(cnf(G,oo)),'unique_nonconstant_rows':len({w for w in wires if w>1}),'graph_sha256':hashlib.sha256(raw).hexdigest(),'rows':meta,'compile_and_check_seconds':time.monotonic()-t}
 eligible=[int(k)for k,v in out['variants'].items()if int(k)>1 and v['clauses']<=source_clauses+data['max_extra_clauses']and v['variables']<=len(old.nodes)-2+data['max_extra_variables']and v['unique_nonconstant_rows']>=int(k)]
 if eligible:out['selected_count']=max(eligible)
 out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Compact correction basis exact support and full-basis identities;mandatory defect-only counterexample;fixed1/2/4/8profiles.','Freeze a stronger correct basis prefix only under declared overhead bounds;retain final raw product and all original recovery obligations.')
