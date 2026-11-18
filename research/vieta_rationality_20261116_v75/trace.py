from itertools import combinations_with_replacement
from research.image_cells_20261115_v67.algebra import BField,pmul,roots_poly

def power(x,e,f):
 r=1
 while e:
  if e&1:r=f.mul(r,x)
  e//=2
  if e:x=f.sq(x)
 return r

def inv(x,f):
 assert x
 return power(x,(1<<f.n)-2,f)
def tr(x,f):
 r=x
 for _ in range(1,f.n):x=f.sq(x);r^=x
 assert r in [0,1]
 return r

def trace_mask(f):return sum(tr(1<<i,f)<<i for i in range(f.n))
def batch(values,f):
 pre=[1]
 for x in values:
  assert x;pre.append(f.mul(pre[-1],x))
 v=inv(pre[-1],f);out=[0]*len(values)
 for i in range(len(values)-1,-1,-1):out[i]=f.mul(v,pre[i]);v=f.mul(v,values[i])
 return out

def stripped(H):
 H=list(H);removed=0
 while len(H)>1 and H[0]==0:H=H[1:];removed+=1
 assert H and H[-1]==1
 return H,removed

def bit(H,A,B,f,mask=None,iv=None):
 H,k=stripped(H);m=len(H)-1
 if not m:return 0
 b=power(B,1<<(f.n-1),f);r=H[-2]^(A if m&1 else 0)^f.mul(b,f.mul(H[1],inv(H[0],f)if iv is None else iv))
 return tr(r,f)if mask is None else(r&mask).bit_count()%2

def small():
 out={'cases':0,'all_rational':0,'mixed_even_false_positive':None,'mixed_odd_rejection':None,'zero_root_cases':0,'norm_identity_cases':0}
 for n,M in [(2,7),(3,11),(4,19)]:
  f=BField(n,M);mask=trace_mask(f)
  for A in [0,1]:
   rat={x:any(f.sq(y)^f.mul(x,y)==f.mul(f.sq(x),x)^f.mul(A,f.sq(x))^1 for y in range(1<<n))for x in range(1<<n)}
   for degree in [3,5]:
    for xs in combinations_with_replacement(range(1<<n),degree):
     H=roots_poly(xs,f);v=bit(H,A,1,f,mask);odd=sum(not rat[x]for x in xs)%2;assert v==odd;out['cases']+=1;out['zero_root_cases']+=0 in xs
     if all(rat[x]for x in xs):assert v==0;out['all_rational']+=1
     elif not v and out['mixed_even_false_positive']is None:out['mixed_even_false_positive']={'n':n,'A':A,'roots':xs,'H':H}
     elif v and out['mixed_odd_rejection']is None:out['mixed_odd_rejection']={'n':n,'A':A,'roots':xs,'H':H}
   for a0 in range(1<<n):
    for b0 in range(1<<n):
     if a0==b0:continue
     a=[a0,1,1];b=[b0,1];F=[1,0,A,1];N=pmul(a,a,f);cross=[0]+pmul(a,b,f);last=pmul(F,pmul(b,b,f),f)
     for poly in [cross,last]:
      if len(N)<len(poly):N+=[0]*(len(poly)-len(N))
      for i,x in enumerate(poly):N[i]^=x
     assert N[-1]==1 and N[0]and bit(N,A,1,f,mask)==0;out['norm_identity_cases']+=1
 return out
