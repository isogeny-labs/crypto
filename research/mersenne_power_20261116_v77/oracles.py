from itertools import product
from math import comb
import json,time
from research.prime_orbit_roots_20261116_v71.field import Prime
from research.prime_orbit_roots_20261116_v71.solver import power as reference
from research.image_cells_20261115_v67.algebra import roots_poly,evaluate
from research.translation_orbits_20261115_v69.algebra import literal_s3
from . import kernel
from .solver import field_roots,solve
from .common import P,finish
START=time.monotonic()
def taylor(c,r,n,degree,p):
 out=[0]*degree
 for j in range(min(n,degree-1)+1):
  v=comb(n,j)*pow((r+c)%p,n-j,p)%p
  for i in range(j+1):out[i]=(out[i]+v*comb(j,i)*pow(-r,j-i,p))%p
 while out and not out[-1]:out.pop()
 return out

def main():
 out={'taylor_cases':0,'root_polynomials':0,'original_systems':0,'literal_checks':0,'large_counts':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());cases=[(p,k)for p in d['oracle_primes']for k in d['oracle_k']]+[((1<<521)-1,k)for k in d['large_k']]
  for p,k in cases:
   f=Prime(p);n=(1<<k)-1
   for r,c,t in product(range(3),range(3),[1,2,3,5]):
    H=roots_poly([r]*t,f);a=[c,1];kernel.reset();got=kernel.power(a,n,H,f);assert got==reference(a,n,H,f)==taylor(c,r,n,t,p);assert kernel.COUNTS['squarings']==max(0,k-1);assert kernel.COUNTS['multiplications']==(k.bit_length()-1+k.bit_count()-1 if k else 0);out['taylor_cases']+=1
   if k>=520:out['large_counts'].append({'k':k,**kernel.COUNTS})
  state=77
  for p in d['oracle_primes']:
   f=Prime(p)
   for j in range(40):
    H=[]
    for _ in range(1+j%5):state=(state*6364136223846793005+1442695040888963407)&((1<<128)-1);H.append(state%p)
    H.append(1);assert field_roots(H,f)==[x for x in range(p)if evaluate(H,x,f)==0];out['root_polynomials']+=1
  for p,A,B,z in [(3,1,1,1),(7,1,1,1),(17,1,1,1),(7,0,0,0)]:
   f=Prime(p)
   for beta in product(range(2),repeat=3):
    inst={'p':p,'A':A,'B':B,'z':z,'beta':list(beta),'shift_low':0,'shift_high':p-1};expected=[]
    for x,u in product(range(p),repeat=2):
     a,b,c=[f.add(v,x)for v in beta];out['literal_checks']+=1
     if literal_s3(a,b,u,A,B,f)==0 and literal_s3(u,c,z,A,B,f)==0:expected.append([x,u])
    assert solve(inst)['roots']==expected;out['original_systems']+=1
   assert time.monotonic()-START<65
 except Exception as e:error=type(e).__name__+': '+str(e)
 finish(out,error,START,'Exact Mersenne power chains against independent Hasse/Taylor remainder and full small-root oracles.','Proceed to the frozen all-five-field cold panel only after all coefficient and complete-root checks pass.')
if __name__=='__main__':main()
