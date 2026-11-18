import json,time,hashlib,random
from collections import Counter
from .common import P,finish
from .kernel import compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
START=time.monotonic();out={'actual_controls':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());C,outputs,_=compile_all(d);rows=matrix(d['matrix']);rng=random.Random(d['oracle_seed'])
 for _ in range(64):
  theta=rng.randrange(1<<76);values=C.evaluate(theta);H=decode(rows,theta);H[0]^=pmul(theta>>73,d['modulus'])
  for r in outputs[:-1]:
   s,m=r['s'],r['m'];ok,_=predicate([rem(x,m)for x in H],Arithmetic(2,[(m>>i)&1 for i in range(s+1)]));assert (not any(values[w]for field in r['wires']for w in field))==ok
  assert (not any(values[w]for field in outputs[-1]['wires']for w in field))==valid_guard(decode(rows,theta),theta>>73);out['actual_controls']+=1
 graph=json.dumps({'nodes':C.nodes,'outputs':outputs},separators=(',',':')).encode();(P/'graph.json').open('xb').write(graph);out.update({'variables':len(C.nodes)-2,'clauses':len(cnf(C,outputs)),'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'graph_sha256':hashlib.sha256(graph).hexdigest(),'compile_and_validation_seconds':time.monotonic()-START})
 finish(out,None,START,'Trace-zero canonical templates compiled through exact affine wires and checked against old fourteen local numeric predicates.','If total native formulation improves, freeze changed45second cold search; if not, change parity representation before another timeout.')
except Exception as e:finish(out,repr(e),START,'Trace-zero compiler.','Preserve failure and repair a new immutable variant.')
