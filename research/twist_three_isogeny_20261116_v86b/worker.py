from pathlib import Path
from itertools import product
from collections import defaultdict
from math import isqrt
import json,time,hashlib
from .common import P,finish
from .curve import on,add,scale,neg,sqrt,target,phi
from .proof_backend import prove_prime
START=time.monotonic()
def checks(pts,T,p,A,B):
 Ap,Bp=target(T,p,A,B);assert(4*Ap**3+27*Bp**2)%p;assert on(T,p,A,B)and scale(3,T,p,A)is None and T is not None
 images={U:phi(U,T,p,A)for U in pts};assert all(on(V,p,Ap,Bp)for V in images.values())
 for U,V in product(pts,repeat=2):assert phi(add(U,V,p,A),T,p,A)==add(images[U],images[V],p,Ap)
 return Ap,Bp,images

def main():
 out={'small':[],'NIST':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text())
  for p in d['small_primes']:
   A=B=1;pts=[None]+[(x,y)for x in range(p)for y in range(p)if(y*y-x**3-A*x-B)%p==0];T=next((U for U in pts[1:]if scale(3,U,p,A)is None),None)
   if T is None:out['small'].append({'p':p,'curve_points':len(pts),'rational3kernel':False});continue
   Ap,Bp,images=checks(pts,T,p,A,B);fibers=defaultdict(list)
   for U,V in images.items():fibers[V].append(U)
   assert all(len(v)==3 for v in fibers.values());original={(U,V,neg(add(U,V,p,A),p))for U,V in product(pts,repeat=2)};recovered=set()
   for U,V in product(fibers,repeat=2):
    W=neg(add(U,V,p,Ap),p);assert W in fibers
    for us in product(fibers[U],fibers[V],fibers[W]):
     if add(add(us[0],us[1],p,A),us[2],p,A)is None:recovered.add(us)
   assert original==recovered;out['small'].append({'p':p,'curve_points':len(pts),'rational3kernel':True,'target_A':Ap,'target_B':Bp,'relation_tuples':len(original),'complete_inverse_transfer':True})
  for r in json.loads((P/'NIST.json').read_text())['curves']:
   p,A,B,N=(int(r[k+'_hex'],16)for k in ['p','a','b','n']);G=(int(r['gx_hex'],16),int(r['gy_hex'],16));pp=prove_prime(p,d['library']);np=prove_prime(N,d['library']);assert r['h']==1 and on(G,p,A,B)and scale(N,G,p,A)is None and(4*A**3+27*B**2)%p;assert(N-p-1)**2<=4*p and N>4*(isqrt(p)+1)
   Nt=2*(p+1)-N;row={'name':r['name'],'p':p,'original_order':N,'twist_order':Nt,'proved_p':pp,'proved_N':np,'generator_NG_O':True,'hasse_unique_multiple':True,'original_rational2kernel':False,'twist_rational2kernel':False,'twist_rational3kernel':Nt%3==0}
   if Nt%3:
    out['NIST'].append(row);continue
   delta=next(x for x in range(2,100)if pow(x,(p-1)//2,p)==p-1);At=A*delta**2%p;Bt=B*delta**3%p;co=Nt;valuation=0
   while co%3==0:co//=3;valuation+=1
   samples=[];T=None
   for attempt in range(d['point_attempts']):
    x=int.from_bytes(hashlib.shake_256((str(d['seed'])+r['name']+':'+str(attempt)).encode()).digest((p.bit_length()+7)//8+8),'big')%p;y=sqrt((x**3+At*x+Bt)%p,p)
    if y is None:continue
    R=(x,y)
    if len(samples)<d['sample_points']:samples.append(R)
    if T is None:
     V=scale(co,R,p,At)
     if V is not None:
      for _ in range(valuation):
       W=scale(3,V,p,At)
       if W is None:T=V;break
       V=W
    if T is not None and len(samples)==d['sample_points']:break
   assert T is not None and len(samples)==d['sample_points'],'bounded kernel search unresolved';assert(3*T[0]**4+6*At*T[0]**2+12*Bt*T[0]-At**2)%p==0;Ap,Bp,images=checks([None,T,neg(T,p)]+samples,T,p,At,Bt)
   row.update({'delta':delta,'twist_A':At,'twist_B':Bt,'three_adic_valuation':valuation,'kernel_point':T,'kernel_exact_order3':True,'division_polynomial_zero':True,'auxiliary_A':Ap,'auxiliary_B':Bp,'sample_point_count':len(images),'homomorphism_pair_checks':len(images)**2,'auxiliary_generator_images':[(U,V)for U,V in images.items()if U is not None]});out['NIST'].append(row);assert time.monotonic()-START<65
  out['complete']=True
 except Exception as e:error=repr(e)
 finish(out,error,START,'Exact NIST twist-kernel survey with prime/order proofs and verified3isogenies,plus complete small inverse-fiber oracles.','Use auxiliary3isogenies only on matching twist branches;on original prime-order groups the rational point map is injective. Benchmark a separately declared orbit-closed factor base with all lift/output costs.')
if __name__=='__main__':main()
