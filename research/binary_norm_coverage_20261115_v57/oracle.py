"""Independent small quadratic-extension group and Hermite oracle."""
from collections import Counter
from research.original_chain_lift_20261115_v55.worker import literal
from research.positive_binary_charts_20261115_v53 import math as curve
from research.generic_block_20261115_v27 import backend as linear
class Extension:
 def __init__(self,f):
  if f.degree%2!=1:raise ValueError('U^2+U+1 irreducibility requires odd n')
  self.f=f;self.n=f.degree;self.mask=f.order-1
 def mul(self,x,y):
  a,b=x&self.mask,x>>self.n;c,d=y&self.mask,y>>self.n;m=lambda u,v:literal(u,v,self.f.modulus);bd=m(b,d)
  return(m(a,c)^bd)|((m(a,d)^m(b,c)^bd)<<self.n)
 def square(self,x):return self.mul(x,x)
 def inv(self,x):
  a,b=x&self.mask,x>>self.n;norm=self.mul(x,(a^b)|(b<<self.n));assert norm>>self.n==0;v=self.f.inv(norm);assert literal(norm,v,self.f.modulus)==1
  return literal(a^b,v,self.f.modulus)|(literal(b,v,self.f.modulus)<<self.n)
 def real(self,x):return x>>self.n==0
 def conjugate(self,x):return(x&self.mask)^(x>>self.n)|((x>>self.n)<<self.n)

def points(f,A,B,d):
 e=Extension(f);out=[]
 beta=B
 for _ in range(f.degree-1):beta=f.square(beta)
 for x in range(1<<d):
  if not x:out.append((0,beta));continue
  inv=f.inv(x);v=x^A^f.mul(B,f.square(inv));roots=[u for u in range(f.order)if f.square(u)^u==v]
  if roots:ys=[f.mul(x,u)for u in roots]
  else:
   roots=[u for u in range(f.order)if f.square(u)^u==v^1];assert len(roots)==2;ys=[f.mul(x,u)|(x<<f.degree)for u in roots]
  for y in ys:assert curve.on(e,A,B,(x,y));out.append((x,y))
 return e,out

def jets(e,A,P,m):
 x,y=P;X=[0]*m;Y=[0]*m;X[0]=x;Y[0]=y
 if x:
  if m>1:X[1]=1
  inv=e.inv(x)
  for j in range(1,m):
   rhs=e.square(x)if j==1 else(x^A if j==2 else(1 if j==3 else 0));v=rhs^Y[j-1]
   if j%2==0:v^=e.square(Y[j//2])
   Y[j]=e.mul(v,inv)
 else:
  if m>1:Y[1]=1
  inv=e.inv(y)
  for j in range(1,m):
   v=(1 if j==2 else 0)^X[j-1]
   if j%2==0:v^=e.mul(A,e.square(X[j//2]))
   for i in range(1,j):
    k=j-2*i
    if k>0:v^=e.mul(e.square(X[i]),X[k])
   X[j]=e.mul(v,inv)
 return X,Y

def hermite_norm(f,e,A,B,sources,Q):
 rows=[];rhs=[]
 for P,m in Counter(sources+[Q]).items():
  X,Y=jets(e,A,P,m)
  def product(a,b):return [sumxor(e.mul(a[i],b[j-i])for i in range(j+1))for j in range(m)]
  X2=product(X,X);X3=product(X2,X);XY=product(X,Y);one=[1]+[0]*(m-1)
  for j in range(m):
   values=[X2[j],X[j],one[j],XY[j],Y[j]]
   for shift in[0,f.degree]:
    row=[(v>>shift)&e.mask for v in values];value=(X3[j]>>shift)&e.mask
    if any(row)or value:rows.append(row);rhs.append(value)
 result=linear.solve(curve.Ops(f),rows,rhs,5);assert result is not None and not result['basis'];return result['particular']
def sumxor(values):
 out=0
 for v in values:out^=v
 return out
