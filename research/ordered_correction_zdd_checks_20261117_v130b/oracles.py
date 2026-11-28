import json,time,random
from .common import P,finish
from .kernel import OrderedManager,compile_cones,fixed_orders
from research.correction_cone_anf_checks_20261117_v129b.kernel import expand
from research.local_split_circuit_20261116_v100.kernel import Circuit
START=time.monotonic();out={'circuits':0,'configurations':0,'assignments':0,'value_checks':0,'polynomial_checks':0,'invalid_order_checks':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_bytes());rng=random.Random(d['seed'])
 for case in range(48):
  n=3+case%5;C=Circuit(n)
  for _ in range(48):
   kind=rng.choice(['and','xor','xor','not']);a=rng.randrange(len(C.nodes));b=rng.randrange(len(C.nodes));C.op(kind,a,None if kind=='not'else b)
  x=C.xor(2,3);y=C.xor(C.xor(2,4),C.xor(3,4));outputs=[0,1,x,y,C.op('and',x,y)]+[rng.randrange(len(C.nodes))for _ in range(5)];before=tuple(C.nodes);polys,stats=expand(C,outputs,max_degree=8,max_monomials=512,max_stored=1000000,max_work=20000000);orders=list(fixed_orders(C,outputs).values())
  for _ in range(2):order=list(range(n));rng.shuffle(order);orders.append(order)
  for order in orders:
   M,roots=compile_cones(C,outputs,order,max_nodes=100000,max_cache=500000,max_work=2000000);assert tuple(C.nodes)==before;out['configurations']+=1
   for r,p in zip(roots,polys):assert M.original_monomials(r)==p;out['polynomial_checks']+=1
   degrees,terms=M.degrees_terms(roots);assert terms==[len(p)for p in polys]and degrees==[max((m.bit_count()for m in p),default=-1)for p in polys]
   for value in range(1<<n):
    v=C.evaluate(value);assert M.evaluate(roots,value)==[v[w]for w in outputs];out['assignments']+=1;out['value_checks']+=len(outputs)
  out['circuits']+=1
 for order in [[0,0,2],[-1,1,2],[0,1],[0,1,3],[0.0,1,2],[False,1,2]]:
  try:OrderedManager(3,order)
  except ValueError:out['invalid_order_checks']+=1
  else:raise AssertionError('invalidorderaccepted')
except Exception as e:error=repr(e)
finish(out,error,START,'Ordered ZDD original-coordinate polynomial/value oracles,all fixed/randomorders,immutability and invalid permutations.','Only profile frozen actual cones after map oracles pass;order heuristics supply no rank or pruning proof.')
