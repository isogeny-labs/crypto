"""Exact binary trace by semilinear modular composition; no squarefree assumption."""
from research.image_cells_20261115_v67.algebra import rem,trim,compose_mod
from research.exact_poly_square_20261116_v80.square import square_mod
class Frobenius:
 def __init__(self,f):
  if f.p!=2:raise ValueError('binary field required')
  self.f=f;self.n=f.n;self.maps=[];self.xors=0;self.calls=0
  if self.n>1:
   self.maps.append([f.sq(1<<j)for j in range(self.n)])
   for i in range(1,(self.n-1).bit_length()):
    prev=self.maps[-1];self.maps.append([self.linear(x,prev)for x in prev])
 def linear(self,x,cols):
  v=0
  while x:
   b=x&-x;v^=cols[b.bit_length()-1];x^=b;self.xors+=1
  return v
 def twist(self,x,k):
  self.calls+=1;k%=self.n;i=0
  while k:
   if k&1:x=self.linear(x,self.maps[i])
   k>>=1;i+=1
  return x

def add(a,b):
 c=list(a)+[0]*max(0,len(b)-len(a))
 for i,x in enumerate(b):c[i]^=x
 return trim(c)

def trace_sequential(t,k,H,f):
 if k<0:raise ValueError('negative length')
 q=rem(t,H,f);out=[]
 for i in range(k):
  out=add(out,q)
  if i+1<k:q=square_mod(q,H,f)
 return out

def trace_jump(t,k,H,f,linear=None,stats=None):
 if k<0:raise ValueError('negative length')
 if len(H)<2 or H[-1]!=1:raise ValueError('positive monic modulus')
 if f.p!=2:raise ValueError('binary field required')
 t=rem(t,H,f)
 if not k:return []
 if k==1:return t
 linear=Frobenius(f)if linear is None else linear
 if linear.f is not f:raise ValueError('field-specific map')
 stats={}if stats is None else stats
 def act(a,j,P):
  stats['actions']=stats.get('actions',0)+1
  if j==1:return square_mod(a,H,f)
  stats['compositions']=stats.get('compositions',0)+1
  return compose_mod([linear.twist(c,j)for c in a],P,H,f)
 def block(j):
  if j==1:return square_mod([0,1],H,f),t
  half=j//2;P,T=block(half);newT=add(T,act(T,half,P));newP=act(P,half,P)
  if j&1:
   newT=add(newT,act(t,2*half,newP));newP=square_mod(newP,H,f)
  return newP,newT
 P,T=block(k);stats['map_xors']=linear.xors;stats['twist_calls']=linear.calls
 return T
