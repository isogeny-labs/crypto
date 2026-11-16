import json,time,random,itertools
from .common import P,finish
from .kernel import atoms,quotient,restore,accepts,evaluate
from research.local_split_circuit_20261116_v100.kernel import Circuit
START=time.monotonic();out={'circuits':0,'literal_assignments':0,'quotient_assignments':0,'original_complete':False}
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['toy_seed'])
 for case in range(72):
  n=1+case%5;C=Circuit(n)
  for i in range(12):
   kind=['xor','not','and'][i%3];a=rng.randrange(len(C.nodes));b=rng.randrange(len(C.nodes));C.op(kind,a,None if kind=='not'else b)
  zero_nodes=[rng.randrange(len(C.nodes))for _ in range(1+case%3)]
  if case%9==0:zero_nodes=[1]
  if case%9==1:zero_nodes=[0]
  forms,products,count=atoms(C.nodes,n);q=quotient(count,products,[forms[w]for w in zero_nodes],d['max_rounds'])
  expected={x for x in range(1<<n)if not any(C.evaluate(x)[w]for w in zero_nodes)};out['literal_assignments']+=1<<n
  observed=[]
  assert len(q['free'])<=14
  for x in range(1<<len(q['free'])):
   a=sum(((x>>i)&1)<<j for i,j in enumerate(q['free']));out['quotient_assignments']+=1
   if accepts(q,a):
    full=restore(q,a);theta=(full>>1)&((1<<n)-1);observed.append(theta)
    assert all(evaluate(f,full)==0 for f in [forms[w]for w in zero_nodes])
    assert all(evaluate(z,full)==(evaluate(u,full)&evaluate(v,full))for z,u,v in products)
  assert set(observed)==expected and len(observed)==len(expected),(case,observed,expected)
  out['circuits']+=1
 finish(out,None,START,'Exhaustive independent truth-table comparison, affine projection and unique exact back substitution.','Profile actual guarded family, then changed bounded search; no identical timeout retry.')
except Exception as e:finish(out,repr(e),START,'Affine quotient oracle.','Preserve failure and register a new repaired variant.')
