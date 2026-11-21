"""Independent Fraction enumeration and public-input root verification."""
from fractions import Fraction as F
from itertools import permutations
from math import isqrt

def det(A):
 return sum((-1)**sum(p[i]>p[j]for i in range(4)for j in range(i+1,4))*__import__('functools').reduce(lambda x,i:x*A[i][p[i]],range(4),1)for p in permutations(range(4)))
def audit(public,row):
 p=int(public['p']);M=int(public['M']);t=list(map(int,public['T']));u=list(map(int,public['U']));B=[list(map(int,r))for r in row['basis']];w=list(map(int,row['weights']));O=list(map(int,row['offset']));hi=[3*M,9*M*M,3*M*M,M**3];assert O==[0,0,t[2],u[2]] and all(x>0 for x in w)
 raw=[]
 for b in B:
  assert all(b[j]%(2*w[j])==0 for j in range(4));v=[b[j]//(2*w[j])for j in range(4)];assert(v[2]-t[0]*v[1]-t[1]*v[0])%p==0 and(v[3]-u[0]*v[1]-u[1]*v[0])%p==0;raw.append(v)
 assert abs(det(raw))==p*p
 star=[];mu=[[F(0)for _ in range(4)]for _ in range(4)];norm=[]
 for i,b in enumerate(B):
  v=list(map(F,b))
  for j in range(i):
   mu[i][j]=sum(F(b[k])*star[j][k]for k in range(4))/norm[j];v=[x-mu[i][j]*y for x,y in zip(v,star[j])]
  star.append(v);norm.append(sum(x*x for x in v));assert norm[-1]>0
 C=[w[j]*(hi[j]-2*O[j])for j in range(4)];R=sum((w[j]*hi[j])**2 for j in range(4));target=[sum(F(C[j])*star[i][j]for j in range(4))/norm[i]for i in range(4)];coeff=[0]*4;points=set();nodes=0
 def rec(i,used):
  nonlocal nodes
  if i<0:
   v=[sum(coeff[k]*B[k][j]for k in range(4))for j in range(4)];assert all(v[j]%(2*w[j])==0 for j in range(4));x=tuple(O[j]+v[j]//(2*w[j])for j in range(4))
   if all(0<=x[j]<=hi[j]for j in range(4)):points.add(x)
   return
  remain=F(R)-used
  if remain<0:return
  center=target[i]-sum(coeff[j]*mu[j][i]for j in range(i+1,4));bound=remain/norm[i];A,D=center.numerator,center.denominator;q=F(D*D)*bound;T=isqrt(q.numerator//q.denominator);lo=-((T-A)//D);high=(A+T)//D
  for z in range(lo,high+1):
   nodes+=1
   if nodes>100000:raise RuntimeError('independent enumeration node cap')
   coeff[i]=z;rec(i-1,used+norm[i]*(z-center)**2)
 rec(3,F(0));assert row['enumeration_complete'];points={x for x in points if x[1]==x[0]*x[0]};assert points=={tuple(map(int,v))for v in row['points']}
 solutions=set();unresolved=0
 for S,V,T,U in points:
  assert(T-t[0]*V-t[1]*S-t[2])%p==0 and(U-u[0]*V-u[1]*S-u[2])%p==0
  if V!=S*S:continue
  if S>4096:unresolved+=1;continue
  poly=[-U,T,-S,1];roots=[]
  for x in range(min(S,M)+1):
   def value():
    out=0
    for c in poly[::-1]:out=out*x+c
    return out
   while len(poly)>1 and value()==0:
    q=[0]*(len(poly)-1);q[-1]=poly[-1]
    for j in range(len(q)-2,-1,-1):q[j]=poly[j+1]+x*q[j+1]
    assert poly[0]+x*q[0]==0;poly=q;roots.append(x)
  if len(poly)==1:
   assert len(roots)==3
   solutions.update((S,*v)for v in set(permutations(roots)))
 assert unresolved==row['unresolved_root_candidates'];assert solutions=={tuple(map(int,v))for v in row['solutions']};assert row['complete']==(unresolved==0)
 return sorted(solutions),nodes

def norm_verify(public,solution,coeff_only=False):
 p=int(public['p']);a=int(public['curve_a']);b=int(public['curve_b']);z,zy=map(int,public['target']);x,xy=map(int,public['anchor_x']);y,yy=map(int,public['anchor_y']);h=int(public['h']);S,r0,r1,r2=solution;assert x!=y and x!=z and y!=z
 for xx,vv in [(x,xy),(y,yy),(z,zy)]:assert(vv*vv-xx**3-a*xx-b)%p==0
 def ab(xx,vv):
  iz=pow((xx-z)%p,-1,p);return((zy-vv)*iz%p,(-(xx*xx+xx*z+z*z)+h*(z*zy-xx*vv)*iz)%p)
 ax,bx=ab(x,xy);ay,by=ab(y,yy);iv=pow((x-y)%p,-1,p);A=(ax-ay)*iv%p;B=(bx-by)*iv%p;C=(ax-x*A)%p;D=(bx-x*B)%p;kap=-2*A%p;assert kap;k=(S-(h*h-z-2*B-x-y))*pow(kap,-1,p)%p;r=(A*k+B)%p;s=(C*k+D-z*r)%p;t=-(z**3+r*z*z+s*z+(h*z+k)*zy)%p
 def mul(v,w):
  o=[0]*(len(v)+len(w)-1)
  for i,vi in enumerate(v):
   for j,wj in enumerate(w):o[i+j]=(o[i+j]+vi*wj)%p
  return o
 P=[t,s,r,1];N=mul(P,P);FQ=mul([b,a,0,1],mul([k,h],[k,h]));N=[(v-(FQ[j]if j<len(FQ)else 0))%p for j,v in enumerate(N)];expect=[1]
 if coeff_only:
  def divide(v,xx):
   q=[0]*(len(v)-1);q[-1]=v[-1]
   for j in range(len(q)-2,-1,-1):q[j]=(v[j+1]+xx*q[j+1])%p
   assert(v[0]+xx*q[0])%p==0;return q
  C=divide(divide(divide(N,z),x),y);return(C[1],-C[0]%p)
 for root in [z,x,y,r0,r1,r2]:expect=mul(expect,[-root%p,1])
 assert N==expect
 return [r0,r1,r2,k]

def verify_public(public):
 p=int(public['p']);M=int(public['M']);assert 0<=M<p and 3*M<p and 3*M*M<p and M**3<p
 assert all(0<=int(public[k][0])<=M for k in ['anchor_x','anchor_y'])
 vals=[norm_verify(public,(S,0,0,0),True)for S in range(3)]
 for j,key in enumerate(['T','U']):
  f0,f1,f2=[v[j]for v in vals];aa=(f2-2*f1+f0)*pow(2,-1,p)%p;bb=(f1-f0-aa)%p;assert [int(x)%p for x in public[key]]==[aa,bb,f0]
 return True
