import json,time,random
from .common import P,finish
from .kernel import expand,evaluate,ExpansionLimit
from research.local_split_circuit_20261116_v100.kernel import Circuit
START=time.monotonic();out={'circuits':0,'assignments':0,'value_checks':0,'cap_checks':{},'invalid_cap_checks':0,'constant_checks':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_bytes());rng=random.Random(d['seed'])
 for case in range(96):
  n=3+case%6;C=Circuit(n)
  for _ in range(64):
   kind=rng.choice(['and','xor','xor','not']);a=rng.randrange(len(C.nodes));b=rng.randrange(len(C.nodes));C.op(kind,a,None if kind=='not'else b)
  x=C.xor(2,3);y=C.xor(C.xor(2,4),C.xor(3,4));z=C.op('and',x,y);zero=C.op('and',z,C.op('not',x));outputs=[x,y,z,zero]+[rng.randrange(len(C.nodes))for _ in range(8)];before=tuple(C.nodes);polys,stats=expand(C,outputs,max_degree=8,max_monomials=512,max_stored=1000000,max_work=20000000);assert before==tuple(C.nodes);assert polys[0]==polys[1]==polys[2]and not polys[3];out['circuits']+=1
  for value in range(1<<n):
   wanted=C.evaluate(value);assert [evaluate(poly,value)for poly in polys]==[wanted[w]for w in outputs];out['assignments']+=1;out['value_checks']+=len(outputs)
 C=Circuit(0);polys,stats=expand(C,[0,1],max_degree=0,max_stored=1);assert polys==[frozenset(),frozenset([0])];out['constant_checks']+=2
 C=Circuit(3);xor=C.xor(2,3);both=C.op('and',2,3)
 for name,outputs,kw,reason in [('work',[xor],{'max_work':0},'workcap'),('degree',[both],{'max_degree':1},'transientdegreecap'),('monomial',[xor],{'max_monomials':1},'monomialcap'),('storage',[2],{'max_stored':1},'storedmonomialcap'),('initialstorage',[0],{'max_stored':0},'initialstoredcap')]:
  try:expand(C,outputs,**kw)
  except ExpansionLimit as e:assert str(e)==reason;out['cap_checks'][name]=e.stats
  else:raise AssertionError('capfailed:'+name)
 for kw in [{'max_work':-1},{'max_degree':1.5},{'max_monomials':True}]:
  try:expand(C,[0],**kw)
  except ValueError:out['invalid_cap_checks']+=1
  else:raise AssertionError('invalidcapaccepted')
except Exception as e:error=repr(e)
finish(out,error,START,'Exact BooleanANF exhaustive small-circuit values,cancellations,sourceimmutability,all resource caps and constants.','Onlyprofileactualfrozencorrectionconesaftertheseoraclespass;capsremainunknownandcannotprunecells.')
