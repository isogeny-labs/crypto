import json,time,sys,os
from .common import P,finish
from .kernel import atoms,quotient,normalize,bits,profile,evaluate,restore,accepts
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard,guarded,prefix_remainder
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
START=time.monotonic();out={'models':[],'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rows=matrix(d['matrix']);C,outputs,_=guarded(d['graph'],rows);forms,products,count=atoms(C.nodes,76)
 q=quotient(count,products,[forms[w]for r in outputs for field in r['wires']for w in field],d['max_rounds']);out['profile']=profile(q,forms,products);out['preprocess_seconds']=time.monotonic()-START
 assert out['profile']['affine_terms']<=d['term_cap'],'affine-term cap'
 sys.path.insert(0,d['z3_path']);import z3
 assert z3.get_version_string()==d['z3_version'];z3.set_param('parallel.enable',False);z3.set_param('sat.threads',1);S=z3.Solver();S.set(threads=1,random_seed=d['seed']);V={j:z3.Bool('a'+str(j))for j in q['free']};cache={0:z3.BoolVal(False),1:z3.BoolVal(True)}
 def expr(f):
  if f not in cache:
   values=[V[j]for j in bits(f&~1)]
   while len(values)>1:values=[z3.Xor(values[i],values[i+1])if i+1<len(values)else values[i]for i in range(0,len(values),2)]
   v=values[0]if values else z3.BoolVal(False);cache[f]=z3.Not(v)if f&1 else v
  return cache[f]
 if q['bad']:S.add(z3.BoolVal(False))
 for z,a,b in q['products']:S.add(expr(z)==z3.And(expr(a),expr(b)))
 out.update({'version':z3.get_version_string(),'cpu_affinity':sorted(os.sched_getaffinity(0)),'formulation_seconds':time.monotonic()-START})
 for iteration in range(d['model_cap']):
  remaining=d['search_deadline_seconds']-(time.monotonic()-START)
  if remaining<=0:out['termination']='worker_search_budget';error='boundedsearchinconclusive';break
  S.set(timeout=max(1,int(1000*remaining)));answer=S.check();out['last_answer']=str(answer)
  if answer==z3.unknown:out.update({'termination':'unknown','reason_unknown':S.reason_unknown()});error='nativeunknown';break
  if answer==z3.unsat:out.update({'termination':'engine_unsat_claim','engine_unsat_claim':True});break
  model=S.model();assignment=sum(int(z3.is_true(model.eval(v,model_completion=True)))<<j for j,v in V.items());assert accepts(q,assignment);full=restore(q,assignment);theta=(full>>1)&((1<<76)-1)
  literal=C.evaluate(theta);assert not any(literal[w]for r in outputs for field in r['wires']for w in field)
  H=decode(rows,theta);assert valid_guard(H,theta>>73);lifted=list(H);lifted[0]^=pmul(theta>>73,d['modulus'])
  for s,m in d['fields']:
   ok,_=predicate([rem(v,m)for v in lifted],Arithmetic(2,[(m>>i)&1 for i in range(s+1)]));assert ok
  remainder=prefix_remainder(H,83,d['field_modulus'],18);out['models'].append({'theta':theta,'H':H,'q':theta>>73,'prefix_remainder8':remainder})
  if not remainder:out['termination']='supported_H_requires_original_recovery';error='candidatependingoriginalsource/Urecovery';break
  S.add(z3.Or(*[z3.Not(expr(normalize(forms[2+i],q['sub'],q['mask'])))if theta>>i&1 else expr(normalize(forms[2+i],q['sub'],q['mask']))for i in range(73)]))
 else:out['termination']='model_cap';error='modelcapinconclusive'
 out['statistics']={k:str(S.statistics().get_key_value(k))for k in S.statistics().keys()};out['engine_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Affine-first exact quotient then bounded necessary search; no original closure accepted.','If no model progress, measure XOR-support fill and use residue-local separators or semantic degree constraints rather than repeat this encoding.')
