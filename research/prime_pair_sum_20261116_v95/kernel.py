"""Two exact fixed-target interval solvers, retaining every polynomial root."""
from math import isqrt
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic,nonsquare,square_root,coefficients,batch_inverse

def solve(spec,method):
 p,A,B,u,M=[spec[k]for k in['p','A','B','u','M']];assert p%2==1 and 0<=M<=4095 and 0<=u<p
 assert 2*M<p and M*M<p,'interval no-wrap bounds'
 f=Arithmetic(p,[0,1]);assert(4*A**3+27*B**2)%p!=0,'smooth curve'
 delta=nonsquare(f);roots=set();stats={'iterations':0,'sqrt_calls':0,'denominator_inverse_calls':0,'repeated_discriminants':0}
 if method=='sum':
  fu=(u**3+A*u+B)%p
  for S in range(2*M+1):
   stats['iterations']+=1;stats['sqrt_calls']+=1;r=square_root(fu*((S+u)%p)%p,f,delta)
   if r is None:continue
   stats['repeated_discriminants']+=int(r==0)
   for T in{(u*S+2*u*u+A+2*r)%p,(u*S+2*u*u+A-2*r)%p}:
    if T>M*M:continue
    disc=S*S-4*T
    if disc<0:continue
    v=isqrt(disc)
    if v*v!=disc or(S-v)%2:continue
    x=(S-v)//2;y=(S+v)//2
    if 0<=x<=M and 0<=y<=M:roots.add((x,y));roots.add((y,x))
 elif method=='source':
  pending=[]
  for x in range(M+1):
   stats['iterations']+=1;a,b,c=coefficients(x,u,A,B,f)
   if not a:
    if b:pending.append((x,(-c)%p,None,b))
    else:assert c!=0
    continue
   stats['sqrt_calls']+=1;r=square_root((b*b-4*a*c)%p,f,delta)
   if r is None:continue
   stats['repeated_discriminants']+=int(r==0);pending.append((x,(-b)%p,r,2*a%p))
  inverses=batch_inverse([t[3]for t in pending],f)if pending else[];stats['denominator_inverse_calls']=int(bool(pending))
  for(x,b,r,den),iv in zip(pending,inverses):
   ys={(b*iv)%p}if r is None else{((b+r)*iv)%p,((b-r)*iv)%p}
   for y in ys:
    if y<=M:roots.add((x,y))
 else:raise ValueError('unknown method')
 return{'complete':True,'roots':sorted(roots),'exact_root_count':len(roots),'stats':stats}
