import json,time,hashlib,random
from collections import Counter
from pathlib import Path
from .common import P,finish
from .kernel import compile_all,local
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.residue_tables_20261116_v102b.kernel import multiplication,literal_split,allowed
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
START=time.monotonic();out={'local_controls':0,'actual_controls':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());prior=json.loads(Path(d['relations_raw']).read_text());rows=matrix(d['matrix']);rng=random.Random(d['oracle_seed'])
 for record in prior['out']['fields']:
  s,m=record['s'],record['m'];t=rem(rows[0][4],m);C=Circuit(4*s);bits=[[2+s*k+j for j in range(s)]for k in range(4)];rel=next(r['relations']for r in record['degrees']if r['degree']==3);zero,_=local(C,s,m,t,rel,bits,d['sparsify_passes'],d['parity_chunk']);G=multiplication(s,m);table=sorted(allowed(s,t,G));codes=list(range(1<<(4*s)))if s<=2 else table[:16]+[rng.randrange(1<<(4*s))for _ in range(32)]
  for code in codes:
   value=C.evaluate(code);H=[code>>(s*i)&((1<<s)-1)for i in range(4)]+[t,1];assert (not any(value[w]for w in zero))==literal_split(H,1<<s,G);out['local_controls']+=1
 C,outputs,stats=compile_all(d)
 for _ in range(32):
  theta=rng.randrange(1<<76);values=C.evaluate(theta);H=decode(rows,theta);H[0]^=pmul(theta>>73,d['modulus'])
  for r in outputs[:-1]:
   s,m=r['s'],r['m'];ok,_=predicate([rem(x,m)for x in H],Arithmetic(2,[(m>>i)&1 for i in range(s+1)]));assert (not any(values[w]for field in r['wires']for w in field))==ok
  assert (not any(values[w]for field in outputs[-1]['wires']for w in field))==valid_guard(decode(rows,theta),theta>>73);out['actual_controls']+=1
 graph=json.dumps({'nodes':C.nodes,'outputs':outputs},separators=(',',':')).encode();(P/'graph.json').open('xb').write(graph);clauses=cnf(C,outputs);out.update({'variables':len(C.nodes)-2,'clauses':len(clauses),'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'fields':stats,'graph_sha256':hashlib.sha256(graph).hexdigest(),'compile_and_validation_seconds':time.monotonic()-START})
 finish(out,None,START,'Exact local cubic+guard circuit with span-preserving weight reduction and shared76parameter mapping checked independently.','Freeze graph hash and pinned native sources; next search must reconstruct circuit cold and validate every model in original predicates.')
except Exception as e:finish(out,repr(e),START,'Cubic/guard compiler.','Preserve failure and repair a new immutable variant; no speed claim from representation alone.')
