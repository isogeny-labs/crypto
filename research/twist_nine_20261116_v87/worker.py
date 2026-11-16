from pathlib import Path
from itertools import product
from collections import defaultdict
import json,time,hashlib
from .common import P,finish
from .poly import three_gcd,evaluate,mul
from research.twist_three_isogeny_20261116_v86b.curve import add,scale,neg,on,target,phi
START=time.monotonic()
def witness(pts,p,A,N):
 assert N%9==0 and N%27!=0;co=N//9;ts=[]
 for R in pts:
  V=scale(co,R,p,A)
  if V is None:continue
  T=scale(3,V,p,A)
  if T is not None:
   assert scale(3,T,p,A)is None;return 'cyclic9',V,T
  assert scale(3,V,p,A)is None
  if not ts:ts.append(V)
  elif V not in[ts[0],neg(ts[0],p)]:return 'full3torsion',ts[0],V
 raise RuntimeError('bounded witness set unresolved')

def build(kind,U,V,p,A,B):
 T=scale(3,U,p,A)if kind=='cyclic9'else U;other=U if kind=='cyclic9'else V;A1,B1=target(T,p,A,B);W=phi(other,T,p,A);assert W is not None and scale(3,W,p,A1)is None;A2,B2=target(W,p,A1,B1)
 def image(Q):return phi(phi(Q,T,p,A),W,p,A1)
 K={scale(i,U,p,A)for i in range(9)}if kind=='cyclic9'else{add(scale(i,U,p,A),scale(j,V,p,A),p,A)for i,j in product(range(3),repeat=2)}
 assert len(K)==9 and all(image(Q)is None for Q in K);return image,K,(A1,B1,A2,B2,T,W)

def j(p,A,B):return 1728*4*A**3*pow((4*A**3+27*B**2)%p,-1,p)%p

def main():
 out={'small':[],'P224':None,'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text())
  for p,A,B in d['small_curves']:
   pts=[None]+[(x,y)for x in range(p)for y in range(p)if(y*y-x**3-A*x-B)%p==0];N=len(pts);assert N%9==0 and N%27!=0;F,G=three_gcd(p,A,B);roots=[x for x in range(p)if evaluate(F,x,p)==0];factor=[1]
   for x in roots:factor=mul(factor,[-x%p,1],p)
   assert G==factor;kind,U,V=witness(pts,p,A,N);E3={Q for Q in pts if scale(3,Q,p,A)is None};assert len(E3)==(3 if kind=='cyclic9'else 9);image,K,data=build(kind,U,V,p,A,B);A1,B1,A2,B2,T,W=data;fibers=defaultdict(list)
   for Q in pts:assert on(image(Q),p,A2,B2);fibers[image(Q)].append(Q)
   assert all(len(f)==9 for f in fibers.values())
   for Q,R in product(pts,repeat=2):assert image(add(Q,R,p,A))==add(image(Q),image(R),p,A2)
   original={(Q,R,neg(add(Q,R,p,A),p))for Q,R in product(pts,repeat=2)};recovered=set()
   for Q,R in product(fibers,repeat=2):
    S=neg(add(Q,R,p,A2),p);assert S in fibers
    for us in product(fibers[Q],fibers[R],fibers[S]):
     if add(add(us[0],us[1],p,A),us[2],p,A)is None:recovered.add(us)
   assert original==recovered;out['small'].append({'p':p,'A':A,'B':B,'order':N,'structure':kind,'E3_points':len(E3),'Frobenius_gcd':G,'kernel_points':len(K),'relation_tuples':len(original),'inverse_transfer_equal':True})
  prior=json.load(open(d['prior']))['out']['NIST'];r=next(x for x in prior if x['name']=='P-224');p=r['p'];A=r['twist_A'];B=r['twist_B'];N=r['twist_order'];pts=[tuple(x[0])for x in r['auxiliary_generator_images']];pts +=[tuple(r['kernel_point'])];F,G=three_gcd(p,A,B);assert p%3==1;kind,U,V=witness(pts,p,A,N);assert len(G)-1==(1 if kind=='cyclic9'else 4);image,K,data=build(kind,U,V,p,A,B);A1,B1,A2,B2,T,W=data;sample=list(K)+pts;assert(4*A2**3+27*B2**2)%p
  for Q in sample:assert on(Q,p,A,B)and on(image(Q),p,A2,B2)
  for Q,R in product(sample,repeat=2):assert image(add(Q,R,p,A))==add(image(Q),image(R),p,A2)
  normalized_multiply=None
  if kind=='full3torsion':
   assert(A2,B2)==(81*A%p,729*B%p)
   for Q in sample:
    Z=scale(3,Q,p,A);expected=None if Z is None else(9*Z[0]%p,27*Z[1]%p);assert image(Q)==expected
   normalized_multiply=True
  graph=[]
  generators=[U,V,add(U,V,p,A),add(U,neg(V,p),p,A)]if kind=='full3torsion'else[T]
  for H in generators:
   assert H is not None and scale(3,H,p,A)is None
   AA,BB=target(H,p,A,B);assert(4*AA**3+27*BB**2)%p
   for Q in sample:assert on(phi(Q,H,p,A),p,AA,BB)
   for Q,R in product(sample,repeat=2):assert phi(add(Q,R,p,A),H,p,A)==add(phi(Q,H,p,A),phi(R,H,p,A),p,AA)
   graph.append({'kernel_generator':H,'target_A':AA,'target_B':BB,'target_j':j(p,AA,BB),'homomorphism_pairs':len(sample)**2})
  out['P224']={'p':p,'twist_A':A,'twist_B':B,'twist_order':N,'structure':kind,'witness_U':U,'witness_V':V,'division_polynomial':F,'Frobenius_gcd':G,'kernel':list(K),'first_target':[A1,B1],'second_target':[A2,B2],'first_kernel':T,'second_kernel':W,'homomorphism_pairs':len(sample)**2,'source_j':j(p,A,B),'target_j':j(p,A2,B2),'normalized_multiply3':normalized_multiply,'isogeny_degree':9,'degree3_quotients':graph,'scope':'verified auxiliary map;not a hard all-roots solve'};out['complete']=True
 except Exception as e:error=repr(e)
 finish(out,error,START,'Exact3primary structure and complete small degree9inverse transfer;actual P224 composite verified.','If full3torsion,do not call it a new cyclic9auxiliary curve;rank all four3quotients and measure original-domain fibers/cost,while certified residue-table solving stays queued.')
if __name__=='__main__':main()
