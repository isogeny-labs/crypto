import json,time,hashlib,random,sys,os
from collections import Counter
from .common import P,finish
from .kernel import compile_all,merge
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard,prefix_remainder
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
START=time.monotonic();out={'models':[],'merge_oracles':0,'actual_controls':0,'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False,'search_executed':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['oracle_seed'])
 for case in range(12):
  source=Circuit(76)
  for _ in range(20):
   kind=rng.choice(['and','xor','not']);a=rng.randrange(len(source.nodes));b=rng.randrange(len(source.nodes));source.op(kind,a,None if kind=='not'else b)
  targets=[{'s':1,'m':3,'wires':[[rng.randrange(len(source.nodes))for _ in range(3)]]}];dest=Circuit(76);mapped=merge(dest,source,targets)
  for value in range(64):
   a=source.evaluate(value);b=dest.evaluate(value);assert [a[w]for field in targets[0]['wires']for w in field]==[b[w]for field in mapped[0]['wires']for w in field]
  out['merge_oracles']+=1
 C,outputs,choices=compile_all(d);rows=matrix(d['matrix']);out.update({'choices':choices,'variables':len(C.nodes)-2,'clauses':len(cnf(C,outputs)),'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'cold_compile_seconds':time.monotonic()-START})
 for _ in range(32):
  theta=rng.randrange(1<<76);values=C.evaluate(theta);H=decode(rows,theta);H[0]^=pmul(theta>>73,d['modulus'])
  for r in outputs[:-1]:
   s,m=r['s'],r['m'];ok,_=predicate([rem(x,m)for x in H],Arithmetic(2,[(m>>i)&1 for i in range(s+1)]));assert (not any(values[w]for field in r['wires']for w in field))==ok
  assert (not any(values[w]for field in outputs[-1]['wires']for w in field))==valid_guard(decode(rows,theta),theta>>73);out['actual_controls']+=1
 graph=json.dumps({'nodes':C.nodes,'outputs':outputs},separators=(',',':')).encode();(P/'graph.json').open('xb').write(graph);out['graph_sha256']=hashlib.sha256(graph).hexdigest()
 if out['clauses']>=d['maximum_search_clauses']:out['termination']='preflight_no_size_gain';finish(out,None,START,'Exact dispatcher profile; preflight prevented a non-improving hard retry.','Change basis or exact table/Gaussian propagation; no native search occurred.');raise SystemExit
 sys.path.insert(0,d['z3_path']);import z3
 assert z3.get_version_string()==d['z3_version'];z3.set_param('parallel.enable',False);z3.set_param('sat.threads',1);S=z3.Solver();S.set(threads=1,random_seed=d['seed']);V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('v'+str(i-1))for i in range(2,len(C.nodes))]
 for i,node in enumerate(C.nodes[78:],78):
  kind,a,*rest=node;expr=z3.And(V[a],V[rest[0]])if kind=='and'else z3.Xor(V[a],V[rest[0]])if kind=='xor'else z3.Not(V[a]);S.add(V[i]==expr)
 for r in outputs:
  for field in r['wires']:
   for w in field:S.add(z3.Not(V[w]))
 out.update({'search_executed':True,'formulation_seconds':time.monotonic()-START,'version':z3.get_version_string(),'cpu_affinity':sorted(os.sched_getaffinity(0))})
 for iteration in range(d['model_cap']):
  remaining=d['search_deadline_seconds']-(time.monotonic()-START)
  if remaining<=0:out['termination']='worker_search_budget';error='boundedsearchinconclusive';break
  S.set(timeout=max(1,int(1000*remaining)));answer=S.check();out['last_answer']=str(answer)
  if answer==z3.unknown:out.update({'termination':'unknown','reason_unknown':S.reason_unknown()});error='nativeunknown';break
  if answer==z3.unsat:out.update({'termination':'engine_unsat_claim','engine_unsat_claim':True});break
  model=S.model();theta=sum(int(z3.is_true(model.eval(V[2+i],model_completion=True)))<<i for i in range(76));H=decode(rows,theta);assert valid_guard(H,theta>>73);lifted=list(H);lifted[0]^=pmul(theta>>73,d['modulus'])
  for s,m in d['fields']:
   ok,_=predicate([rem(x,m)for x in lifted],Arithmetic(2,[(m>>i)&1 for i in range(s+1)]));assert ok
  remainder=prefix_remainder(H,83,d['field_modulus'],18);out['models'].append({'theta':theta,'H':H,'q':theta>>73,'prefix_remainder8':remainder})
  if not remainder:out['termination']='supported_H_requires_original_recovery';error='candidatependingoriginalsource/Urecovery';break
  S.add(z3.Or(*[z3.Not(V[2+i])if theta>>i&1 else V[2+i]for i in range(73)]))
 else:out['termination']='model_cap';error='modelcapinconclusive'
 out['statistics']={k:str(S.statistics().get_key_value(k))for k in S.statistics().keys()};out['engine_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Cold exact predicate dispatch with charged construction/oracles, then bounded necessary search only after preflight.','If unknown persists, move to exact table plus incremental affine-hull propagation or factorized parity networks; no identical failed retry.')
