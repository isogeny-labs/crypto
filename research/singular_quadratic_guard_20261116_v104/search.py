import json,time,sys,hashlib,os
from pathlib import Path
from .common import P,finish
from .kernel import compile_all
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard,prefix_remainder
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
START=time.monotonic();out={'models':[],'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());compiled=json.loads((P/'compile_raw.json').read_text());assert compiled['error']is None;C,outputs,_=compile_all(d);raw=json.dumps({'nodes':C.nodes,'outputs':outputs},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==compiled['out']['graph_sha256'];out['cold_compile_seconds']=time.monotonic()-START
 sys.path.insert(0,d['z3_path']);import z3
 assert z3.get_version_string()==d['z3_version'];z3.set_param('parallel.enable',False);z3.set_param('sat.threads',1);S=z3.Solver();S.set(threads=1,random_seed=d['seed']);V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('v'+str(i-1))for i in range(2,len(C.nodes))]
 for i,node in enumerate(C.nodes[78:],78):
  kind,a,*rest=node;expr=z3.And(V[a],V[rest[0]])if kind=='and'else z3.Xor(V[a],V[rest[0]])if kind=='xor'else z3.Not(V[a]);S.add(V[i]==expr)
 for r in outputs:
  for field in r['wires']:
   for w in field:S.add(z3.Not(V[w]))
 out.update({'formulation_seconds':time.monotonic()-START,'version':z3.get_version_string(),'cpu_affinity':sorted(os.sched_getaffinity(0))});rows=matrix(d['matrix'])
 for iteration in range(d['model_cap']):
  remaining=d['search_deadline_seconds']-(time.monotonic()-START)
  if remaining<=0:out['termination']='worker_search_budget';error='boundedsearchinconclusive';break
  S.set(timeout=max(1,int(1000*remaining)));answer=S.check();out['last_answer']=str(answer)
  if answer==z3.unknown:out.update({'termination':'unknown','reason_unknown':S.reason_unknown()});error='nativeunknown';break
  if answer==z3.unsat:out.update({'termination':'engine_unsat_claim','engine_unsat_claim':True});break
  model=S.model();theta=sum(int(z3.is_true(model.eval(V[2+i],model_completion=True)))<<i for i in range(76));values=C.evaluate(theta);assert not any(values[w]for r in outputs for field in r['wires']for w in field);H=decode(rows,theta);assert valid_guard(H,theta>>73);lifted=list(H);lifted[0]^=pmul(theta>>73,d['modulus'])
  for s,m in d['fields']:
   ok,_=predicate([rem(x,m)for x in lifted],Arithmetic(2,[(m>>i)&1 for i in range(s+1)]));assert ok
  remainder=prefix_remainder(H,83,d['field_modulus'],18);out['models'].append({'theta':theta,'H':H,'q':theta>>73,'prefix_remainder8':remainder})
  if not remainder:out['termination']='supported_H_requires_original_recovery';error='candidatependingoriginalsource/Urecovery';break
  S.add(z3.Or(*[z3.Not(V[2+i])if theta>>i&1 else V[2+i]for i in range(73)]))
 else:out['termination']='model_cap';error='modelcapinconclusive'
 out['statistics']={k:str(S.statistics().get_key_value(k))for k in S.statistics().keys()};out['engine_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Cold cubic-plus-singular-guard necessary search; compilation and formulation charged inside same45second budget.','Use circuit cost/model evidence to choose a changed basis or direct algebraic block method; unknown is not original UNSAT or speed gain.')
