import json,time,math
from .common import P,finish
from .inputs import materialize,digest
from research.prime_orbit_roots_20261116_v71.solver import solve
START=time.monotonic()
out={'cases':0,'digest_checks':0};error=None
try:
 d=json.loads((P/'DATA.json').read_text());p=d['base']['p'];base=solve(d['base']);assert base['complete']and base['roots']==d['base_roots'];R=base['orbit_polynomial']
 for n in d['batch_sizes']:assert digest(materialize(n,d))==d['input_hashes'][str(n)];out['digest_checks']+=1
 for delta in [0,1,4,24,124,624]:
  inst=materialize(delta+1,d)[-1];v=solve(inst);expect=[[t-delta,u]for t,u in d['base_roots']];assert v['complete']and v['roots']==expect
  translated=[sum(math.comb(j,i)*R[j]*pow(delta,j-i,p)for j in range(i,len(R)))%p for i in range(len(R))];assert v['orbit_polynomial']==translated;out['cases']+=1
 out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Exact translation roots and independent binomial resultant identity.','Run every fixed A/A ladder size without dropping failed pairs.')
