import json,time,sys,hashlib,os
from pathlib import Path
from .common import P,finish
from .kernel import matrix,decode,valid_guard,guarded,prefix_remainder
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
START=time.monotonic();out={'models':[],'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());sys.path.insert(0,d['z3_path']);import z3
 assert z3.get_version_string()==d['z3_version'];z3.set_param('parallel.enable',False);z3.set_param('sat.threads',1);rows=matrix(d['matrix']);C,outputs,delta=guarded(d['graph'],rows);S=z3.Solver();S.set(threads=1,random_seed=d['seed']);V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('v'+str(i-1))for i in range(2,len(C.nodes))]
 for i,node in enumerate(C.nodes[78:],78):
  k,a,*rest=node;expr=z3.And(V[a],V[rest[0]])if k=='and'else z3.Xor(V[a],V[rest[0]])if k=='xor'else z3.Not(V[a]);S.add(V[i]==expr)
 for row in outputs:
  for field in row['wires']:
   for wire in field:S.add(z3.Not(V[wire]))
 out.update({'version':z3.get_version_string(),'cpu_affinity':sorted(os.sched_getaffinity(0)),'formulation_seconds':time.monotonic()-START,'native_timeout_includes': 'solver check;fullworker includes field/native setup,validation,reporting'})
 for iteration in range(d['model_cap']):
  remaining=d['search_deadline_seconds']-(time.monotonic()-START)
  if remaining<=0:out['termination']='worker_search_budget';error='boundedsearchinconclusive';break
  S.set(timeout=max(1,int(1000*remaining)));answer=S.check();out['last_answer']=str(answer)
  if answer==z3.unknown:out.update({'termination':'unknown','reason_unknown':S.reason_unknown()});error='nativeunknown';break
  if answer==z3.unsat:out.update({'termination':'engine_unsat_claim','engine_unsat_claim':True});break
  model=S.model();theta=sum(int(z3.is_true(model.eval(V[2+i],model_completion=True)))<<i for i in range(76));H=decode(rows,theta);assert valid_guard(H,theta>>73)
  lifted=list(H);lifted[0]^=pmul(theta>>73,d['modulus']);local=[]
  for s,m in d['fields']:
   f=Arithmetic(2,[(m>>i)&1 for i in range(s+1)]);ok,_=predicate([rem(v,m)for v in lifted],f);assert ok;local.append(True)
  remainder=prefix_remainder(H,83,d['field_modulus'],18);record={'theta':theta,'H':H,'q':theta>>73,'all_local_checks':local,'prefix_remainder8':remainder};out['models'].append(record)
  if not remainder:out['termination']='supported_H_requires_original_recovery';error='candidatependingoriginalsource/Urecovery';break
  # Only unsupported field H values are blocked; all q aliases are then invalid.
  S.add(z3.Or(*[z3.Not(V[2+i])if theta>>i&1 else V[2+i]for i in range(73)]))
 else:out['termination']='model_cap';error='modelcapinconclusive'
 out['statistics']={k:str(S.statistics().get_key_value(k))for k in S.statistics().keys()};out['engine_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Boundedguarded14-residue necessaryparametersearch;global/originalclosure neveracceptedwithoutfullrecoveryandproof.','Inspectmodelsorunknown;newhypothesisbeforeanyretry. EngineUNSATisnotindependentlycertifiedoriginalUNSAT.')
