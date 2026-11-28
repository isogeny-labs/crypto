import time
START=time.monotonic()
from pathlib import Path
import json,lzma,hashlib,sys,os
from .common import P,finish
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.unit_cofactor_search_20261117_v135b.recovery import oracles,recover
from research.unit_cofactor_20261117_v134b.kernel import original_readback,word
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard,prefix_remainder
from research.residue_quotient_20261116_v99b.kernel import pmul
from research.prefix_translation_20261116_v115.kernel import product,translate
from research.leading_cover_20261116_v116.kernel import fits,limits
from research.xor_cut_20261116_v114b.backend import make_solver,readback
out={'models':[],'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False,'verified_original_tuple_lower_bound':0,'scope':'fixed necessary k3family;otherstrata/source/U/ghost/outercoverage remains incomplete'};error=None

def encode(C,checks,z3,seed):
 S=make_solver(z3,seed);V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('v'+str(i-1))for i in range(2,len(C.nodes))]
 for i,node in enumerate(C.nodes[C.n+2:],C.n+2):
  kind,a,*rest=node;value=z3.And(V[a],V[rest[0]])if kind=='and'else z3.Xor(V[a],V[rest[0]])if kind=='xor'else z3.Not(V[a]);S.add(V[i]==value)
 for row in checks:
  for field in row['wires']:
   for wire in field:S.add(z3.Not(V[wire]))
 return S,V
try:
 d=json.loads((P/'DATA.json').read_bytes());profile_process=json.loads(Path(d['profile_process']).read_bytes());assert profile_process['status']=='ok'and profile_process['wall_seconds']<90;prior=json.loads(Path(d['profile']).read_bytes());assert prior['error']is None and prior['out']['all_registered_contexts_complete']and prior['out']['actual']['development_cost_target_met'],'cost_or_oracle_not_admitted';b=Path(d['graph']).read_bytes();assert hashlib.sha256(b).hexdigest()==prior['out']['graph_artifact_sha256'];graph=json.loads(lzma.decompress(b));assert graph['status']=='complete';fingerprint=hashlib.sha256(json.dumps({'nodes':graph['nodes'],'outputs':graph['outputs']},separators=(',',':')).encode()).hexdigest();assert fingerprint==graph['graph_sha256']==prior['out']['actual']['graph_sha256'];C=Circuit(graph['n']);C.nodes=[tuple(v)for v in graph['nodes']];oo=graph['outputs'];checks=oo[:1];selector_wires=oo[1]['wires'][0];source_roots=oo[1]['wires'][1:];assert C.n==61 and len(selector_wires)==63 and len(source_roots)==5;assert len(cnf(C,checks))==prior['out']['actual']['clauses'];out['recovery_oracles']=oracles()
 instance_raw=lzma.decompress((P/d['instance']).read_bytes());assert hashlib.sha256(instance_raw).hexdigest()==d['instance_sha256'];original=json.loads(instance_raw);assert original['n_vars']==339 and original['equation_count']==332;out['original_instance_sha256']=d['instance_sha256'];del original,instance_raw
 cfg=json.loads(Path(d['config']).read_bytes());chart=json.loads(Path(d['cover']).read_bytes())['records'][1]['chart'];sys.path.insert(0,cfg['z3_path']);import z3
 assert z3.get_version_string()==cfg['z3_version'];T=Circuit(3);wire=T.op('and',T.xor(2,3),4);toy,TV=encode(T,[{'wires':[[wire]]}],z3,d['seed']);found=set()
 while toy.check()==z3.sat:
  model=toy.model();v=sum(int(z3.is_true(model.eval(TV[i+2],model_completion=True)))<<i for i in range(3));assert v not in found;found.add(v);toy.add(z3.Or(*[z3.Not(TV[i+2])if v>>i&1 else TV[i+2]for i in range(3)]))
 assert found=={v for v in range(8)if not(((v&1)^((v>>1)&1))&((v>>2)&1))};out['native_gate_oracle_models']=len(found);S,V=encode(C,checks,z3,d['seed']);out.update(inputs=C.n,variables=len(C.nodes)-2,clauses=prior['out']['actual']['clauses'],graph_sha256=fingerprint,version=z3.get_version_string(),backend_configuration_readback=readback(z3),cpu_affinity=sorted(os.sched_getaffinity(0)),formulation_seconds=time.monotonic()-START)
 remaining=d['soft_total_seconds']-(time.monotonic()-START)
 if remaining<=0:out['termination']='formulation_budget';error='formulation_inconclusive'
 else:
  S.set(timeout=max(1,int(remaining*1000)));stamp=time.monotonic();answer=S.check();out['engine_check_seconds']=time.monotonic()-stamp;out['last_answer']=str(answer)
  if answer==z3.unknown:out['termination']='unknown';out['reason_unknown']=S.reason_unknown();error='nativeunknown'
  elif answer==z3.unsat:out['termination']='engine_unsat_claim_uncertified';error='independent_certificate_and_other_branches_pending'
  else:
   model=S.model();x=sum(int(z3.is_true(model.eval(V[i+2],model_completion=True)))<<i for i in range(C.n));values=C.evaluate(x);assert not any(values[w]for row in checks for field in row['wires']for w in field);old_x=word(values,selector_wires);assert all(int(z3.is_true(model.eval(V[w],model_completion=True)))==values[w]for w in selector_wires);theta=original_readback(chart,old_x);H=decode(matrix(cfg['matrix']),theta);assert valid_guard(H,theta>>73);rawH=list(H);rawH[0]^=pmul(theta>>73,cfg['modulus']);assert fits(rawH,limits(18,3))and fits(translate(rawH,1<<17),limits(18,2));sources=[word(values,row)for row in source_roots];assert all(0<=s<1<<18 for s in sources)and product(sources)==rawH;assert not prefix_remainder(H,83,cfg['field_modulus'],18);recovery=recover(sources,P,d,START);out['models'].append({'encoding_hex':hex(x),'old_selector_hex':hex(old_x),'theta_hex':hex(theta),'source_multiset':sorted(sources),'original_source_U_recovery':recovery});out['verified_original_tuple_lower_bound']=recovery['verified_original_tuple_lower_bound'];out['termination']='one_model_remaining_coverage'if recovery['complete_for_this_source_multiset']else'original_recovery_budget';error='modelcap_or_original_recovery_incomplete'
 out['statistics']={name:str(S.statistics().get_key_value(name))for name in S.statistics().keys()};out['full_diagnostic_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
finish(out,error,START,'Changed61inputstatic-pivot source-product hard diagnostic with original332equation per-model verification;40ssoft90souter','UNKNOWN/uncertifiedUNSATareinconclusive. Diagnose exact XOR/formulation/search costs before any different encoding;retain all original branches and frozen matched baseline obligations. Never repeat this same failed execution unchanged.')
