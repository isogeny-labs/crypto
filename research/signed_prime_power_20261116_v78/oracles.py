from itertools import product
import json,time
from lab.research.nist_fields import preset
from research.prime_orbit_roots_20261116_v71.field import Prime
from research.prime_orbit_roots_20261116_v71.solver import power as reference,solve as original
from research.image_cells_20261115_v67.algebra import roots_poly
from research.mersenne_power_20261116_v77.oracles import taylor
from . import kernel
from .solver import field_roots
from .router import engine
from .common import P,finish
START=time.monotonic()
def main():
 out={'taylor_cases':0,'large_root_cases':0,'signed_exponents':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());N=[preset(n)['p']for n in d['fields']if n.startswith('P')];exponents=[e for p in N for e in [p,(p-1)//2]];kernel.reset()
  for p in N:
   f=Prime(p)
   for n in [p,(p-1)//2]:
    ds=kernel.naf(n);assert sum(v*(1<<i)for i,v in ds)==n;out['signed_exponents'].append({'field_bits':p.bit_length(),'n':n,'digits':ds})
    for r,c,t in product(range(3),range(3),[1,2,3,5]):
     H=roots_poly([r]*t,f);a=[c,1];assert kernel.power(a,n,H,f)==reference(a,n,H,f)==taylor(c,r,n,t,p);out['taylor_cases']+=1
   for xs in [[0,0,1,2,2,3],[1,2,3,4,5],[0,1,1,1,2]]:
    H=roots_poly(xs,f);assert field_roots(H,f)==sorted(set(xs));out['large_root_cases']+=1
   assert time.monotonic()-START<40
  for p in d['oracle_primes']:
   f=Prime(p)
   for n in exponents:
    for r,c,t in product(range(2),range(2),[1,2,3,5]):
     H=roots_poly([r]*t,f);a=[c,1];assert kernel.power(a,n,H,f)==reference(a,n,H,f)==taylor(c,r,n,t,p);out['taylor_cases']+=1
  assert kernel.COUNTS['signed_calls']and kernel.COUNTS['nonunit_fallbacks'];out['counts']=dict(kernel.COUNTS)
  assert all(65537%v for v in range(2,257));assert engine(65537)is original;assert engine(17)is original;out['direct_reference_routes_verified']=[17,65537]
 except Exception as e:error=type(e).__name__+': '+str(e)
 finish(out,error,START,'Independent Taylor and full large-field root checks for guarded sparse signed powers.','Proceed to frozen batch comparison, preserving all fields and prior noisy evidence.')
if __name__=='__main__':main()
