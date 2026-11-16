import json,time,random,itertools,hashlib
from pathlib import Path
from .common import P,finish
from .kernel import count,apply,basis,poly,rem
START=time.monotonic();out={'toy':[],'recovered':[]};error=None
try:
 for r in json.loads((P/'RECOVERY.json').read_text()):
  raw=Path(r['local_path']).read_bytes();assert hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()==r['git_blob_sha'];out['recovered'].append({'path':r['local_path'],'sha256':hashlib.sha256(raw).hexdigest()})
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
 for i in range(80):
  for _ in range(32):
   E=[rng.randrange(64)for _ in range(8)]
   if len(basis(E)[0])==6:break
  else:raise AssertionError('rank generation cap')
  F=[rng.randrange(16)for _ in range(8)];A=set(rng.randrange(8)for _ in range(5));B=set(rng.randrange(8)for _ in range(5));L=set(rng.randrange(16)for _ in range(7));eb=rng.randrange(64);fb=rng.randrange(16);want=0
  for z in range(256):
   y=apply(E,z)^eb;want+=int(y&7 in A and y>>3 in B and(apply(F,z)^fb)in L)
  a=count(E,eb,[3,3],[A,B],F,fb,L,'dp');b=count(E,eb,[3,3],[A,B],F,fb,L,'walsh');assert a['exact_necessary_parameter_count']==b['exact_necessary_parameter_count']==want and a['distribution']==b['distribution'];out['toy'].append({'count':want,'quotient_bits':a['quotient_bits']})
 finish(out,None,START,'Recovered exact predecessor gitblobs and independent exhaustive quotient-convolution all-parameter toy oracles.','Buildactual8thresiduequotientwithsame3globalqbits;countnecessaryparameters only,then strongerconstraints/source recovery.')
except Exception as e:finish(out,repr(e),START,'Recovery/quotientoracle.','Preservefailedsource;repairnewvariant only;no actualcountacceptedbeforehashandoraclechecks.')
