from math import isqrt
from action import mul,code
r=7
def implicit_bsgs(G,Q,z,n):
 m=isqrt(n)+(isqrt(n)**2<n);babies={};calls=0
 for j in range(m):babies[str(code(mul(pow(z,j,r),G)))]=j;calls+=1
 factor=pow(z,-m,r);P=Q
 for i in range(m+1):
  j=babies.get(str(code(P)))
  if j is not None:
   k=i*m+j
   if k<n and mul(pow(z,k,r),G)==Q:return k,{'scalar_calls':calls+1,'baby_entries':len(babies),'giant_index':i}
  P=mul(factor,P);calls+=1
 raise ValueError('implicit BSGS no match')
def solve(G,Q,Pd,d):
 assert 6%d==0;g=3;L=6//d;k2,c1=implicit_bsgs(G,Pd,pow(g,d,r),L);U=mul(pow(g,-k2,r),Q);k1,c2=implicit_bsgs(G,U,pow(g,L,r),d);x=pow(g,k2+L*k1,r)
 if mul(x,G)!=Q or mul(pow(x,d,r),G)!=Pd:raise ValueError('auxiliary input or final scalar mismatch')
 return x,{'divisor':d,'first':c1,'second':c2,'normalization_scalar_calls':1,'final_scalar_checks':2}
