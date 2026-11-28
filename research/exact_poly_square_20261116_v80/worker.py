from pathlib import Path
import json,time,hashlib,sys
from research.image_cells_20261115_v67.algebra import BField,pmul,rem,roots_poly
from research.torsion_cut_20261116_v74.core import PF,F9
from research.prime_orbit_roots_20261116_v71.field import Prime
from research.prime_orbit_roots_20261116_v71.solver import power as reference
from research.mersenne_power_20261116_v77.oracles import taylor
from research.signed_prime_power_20261116_v78.router import solve as prior_solve
from lab.research.nist_fields import preset
from .square import square_mod
from .kernel import power
from .solver import field_roots,solve
from .common import P,finish
START=time.monotonic()
class Count:
 def __init__(self,f):self.f=f;self.p=f.p;self.M=0;self.S=0
 def add(self,a,b):return self.f.add(a,b)
 def neg(self,a):return self.f.neg(a)
 def mul(self,a,b):self.M+=1;return self.f.mul(a,b)
 def sq(self,a):self.S+=1;return self.f.sq(a)
def main():
 out={'square_cases':0,'power_cases':0,'Taylor_cases':0,'large_root_cases':0,'conditional_cases':0,'operation_cases':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());fields=[(PF(p),p,'F'+str(p))for p in d['prime_fields']]+[(F9(),9,'F9')]+[(BField(n,M),1<<n,'GF2^'+str(n))for n,M in d['binary_fields']]+[(Prime(preset(n)['p']),preset(n)['p'],n)for n in d['NIST']];counter=0
  for f,q,name in fields:
   def rand():
    nonlocal counter
    counter+=1;return int.from_bytes(hashlib.shake_256((str(d['seed'])+':'+name+':'+str(counter)).encode()).digest(((q-1).bit_length()+7)//8),'big')%q
   for i in range(d['random_cases_per_field']):
    m=d['degrees'][i%len(d['degrees'])];H=roots_poly([i%q]*m,f)if i%3==0 else[rand()for _ in range(m)]+[1];a=[rand()for _ in range((i*7)%13)];assert square_mod(a,H,f)==rem(pmul(a,a,f),H,f);out['square_cases']+=1
    if i<6:
     for n in [0,1,7,31]:assert power(a,n,H,f)==reference(a,n,H,f);out['power_cases']+=1
   for m in [5,12]:
    a=[i%(q-1)+1 for i in range(m)];H=[1]*m+[1];old=Count(f);new=Count(f);assert rem(pmul(a,a,old),H,old)==square_mod(a,H,new);assert new.M+new.S<=old.M+old.S
    out['operation_cases'].append({'field':name,'coefficients':m,'generic_scalar_mults':old.M,'generic_scalar_squares':old.S,'new_scalar_mults':new.M,'new_scalar_squares':new.S,'scope':'includes identical remainder work; operation counts are not runtime'})
   assert time.monotonic()-START<55
  for name in d['NIST']:
   p=preset(name)['p'];f=Prime(p)
   for n in [p,(p-1)//2]:
    for r in [0,1]:
     for c in [0,1]:
      for t in [2,3,5]:
       H=roots_poly([r]*t,f);a=[c,1];assert power(a,n,H,f)==taylor(c,r,n,t,p);out['Taylor_cases']+=1
   xs=[0,0,1,2,2,3];assert field_roots(roots_poly(xs,f),f)==sorted(set(xs));out['large_root_cases']+=1
   inst=json.loads(Path('research/signed_prime_power_20261116_v78',name+'_BATCH.json').read_text())[0];a=solve(inst);b=prior_solve(inst);assert(a['roots'],a['candidate_shifts'],a['orbit_polynomial'])==(b['roots'],b['candidate_shifts'],b['orbit_polynomial']);out['conditional_cases']+=1
  out['fields']=[name for f,q,name in fields]
 except Exception as e:error=type(e).__name__+': '+str(e)
 finish(out,error,START,'Field-aware exact polynomial squaring across prime,binary and odd-extension quotients with complete-root regression.','Keep performance claims gated on accepted observation; test binary trace jumps or charged solve cost in a distinct variant.')
if __name__=='__main__':main()
