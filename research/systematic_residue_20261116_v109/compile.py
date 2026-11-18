import json,time,hashlib,random
from collections import Counter
from .common import P,finish
from .kernel import compile_all,old_value,new_value
from research.local_split_circuit_20261116_v100.kernel import cnf
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
START=time.monotonic();out={'actual_controls':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());C,outputs,mapping,blocks=compile_all(d);assert mapping==json.loads((P/'coordinates.json').read_text());rows=matrix(d['matrix']);rng=random.Random(d['oracle_seed'])
 for _ in range(64):
  rho=rng.randrange(1<<76);theta=old_value(mapping,rho);assert new_value(mapping,theta)==rho;values=C.evaluate(rho);H=decode(rows,theta);H[0]^=pmul(theta>>73,d['modulus'])
  for r in outputs[:-1]:
   s,m=r['s'],r['m'];ok,_=predicate([rem(x,m)for x in H],Arithmetic(2,[(m>>i)&1 for i in range(s+1)]));assert (not any(values[w]for field in r['wires']for w in field))==ok
  assert (not any(values[w]for field in outputs[-1]['wires']for w in field))==valid_guard(decode(rows,theta),theta>>73);out['actual_controls']+=1
 graph=json.dumps({'nodes':C.nodes,'outputs':outputs},separators=(',',':')).encode();(P/'graph.json').open('xb').write(graph);out.update({'variables':len(C.nodes)-2,'clauses':len(cnf(C,outputs)),'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'blocks':blocks,'graph_sha256':hashlib.sha256(graph).hexdigest(),'compile_and_validation_seconds':time.monotonic()-START})
 finish(out,None,START,'Actual systematic reparameterization verifies seven disjoint local blocks and same original14numeric predicates/q guards.','Freeze graph and coordinates; new cold search must back-map every model before support/recovery or exclusions.')
except Exception as e:finish(out,repr(e),START,'Systematic hard-family compiler.','Preserve counterexample and change a concrete assumption before a new variant.')
