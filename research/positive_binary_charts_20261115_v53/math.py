"""Deterministic binary-curve positive chart construction."""
from research.generic_block_20261115_v27 import backend as linear
from research.three_point_line_20261115_v46 import solver as polynomial

class Ops:
 def __init__(self,f):self.f=f
 def add(self,a,b):return a^b
 sub=add
 def neg(self,a):return a
 def mul(self,a,b):return self.f.mul(a,b)
 def inv(self,a):return self.f.inv(a)

def on(f,A,B,P):
 if P is None:return True
 x,y=P;return f.square(y)^f.mul(x,y)==f.mul(f.square(x),x)^f.mul(A,f.square(x))^B

def add(f,A,B,P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u:
  if y^v==x or not x:return None
  assert y==v;lam=x^f.mul(y,f.inv(x));xx=f.square(lam)^lam^A;yy=f.square(x)^f.mul(lam^1,xx)
 else:
  lam=f.mul(y^v,f.inv(x^u));xx=f.square(lam)^lam^x^u^A;yy=f.mul(lam,x^xx)^xx^y
 R=(xx,yy);assert on(f,A,B,R);return R

def total(f,A,B,points):
 P=None
 for Q in points:P=add(f,A,B,P,Q)
 return P

def trace(f,v):
 t=0
 for _ in range(f.degree):t^=v;v=f.square(v)
 assert t in [0,1];return t

def sqrt(f,v):
 for _ in range(f.degree-1):v=f.square(v)
 return v

def lift(f,A,B,x):
 if not x:return (0,sqrt(f,B))
 v=x^A^f.mul(B,f.square(f.inv(x)))
 if trace(f,v):return None
 ht=0
 for _ in range((f.degree+1)//2):ht^=v;v=f.square(f.square(v))
 P=(x,f.mul(x,ht));assert on(f,A,B,P);return P

class Generator:
 def __init__(self,f,A,B,seed,d):self.f=f;self.A=A;self.B=B;self.state=seed;self.mask=(1<<d)-1
 def word(self):self.state=(self.state*6364136223846793005+1442695040888963407)&((1<<64)-1);return self.state
 def point(self,used):
  for _ in range(2048):
   v=self.word();x=v&self.mask
   if not x or x in used:continue
   P=lift(self.f,self.A,self.B,x)
   if P is not None:return(x,P[1]^(x if(v>>18)&1 else 0))
  raise RuntimeError('point generation cap')
 def ghost_x(self,used):
  for _ in range(2048):
   x=self.word()&self.mask
   if x and x not in used and lift(self.f,self.A,self.B,x)is None:return x
  raise RuntimeError('ghost generation cap')

def field_solution(f,M,rhs,columns):
 result=linear.solve(Ops(f),M,rhs,columns);assert result is not None and not result['basis'];return result['particular']

def F6(f,points):return field_solution(f,[[f.square(x),x,1,f.mul(x,y),y]for x,y in points],[f.mul(f.square(x),x)for x,y in points],5)

def F4(f,points):return field_solution(f,[[x,1,y]for x,y in points],[f.square(x)for x,y in points],3)

def coordinates(basis,value):
 piv={}
 for i,v in enumerate(basis):
  tag=1<<i
  while v:
   p=v.bit_length()-1
   if p in piv:a,b=piv[p];v^=a;tag^=b
   else:piv[p]=(v,tag);break
  assert v
 tag=0
 for p,(v,t)in sorted(piv.items(),reverse=True):
  if value>>p&1:value^=v;tag^=t
 return None if value else tag

def S3(f,B,a,b,c):return f.square(f.mul(a,b)^f.mul(a,c)^f.mul(b,c))^f.mul(f.mul(a,b),c)^B

def literal_mul(a,b,mod):
 out=0
 while b:
  low=b&-b;out^=a<<(low.bit_length()-1);b^=low
 while out.bit_length()>=mod.bit_length():out^=mod<<(out.bit_length()-mod.bit_length())
 return out

def residuals_literal(values,target,B,mod):
 x1,x2,x3,x4,x5,u,v,w=values;mul=lambda a,b:literal_mul(a,b,mod)
 def eq(a,b,c):
  value=mul(a,b)^mul(a,c)^mul(b,c);return mul(value,value)^mul(mul(a,b),c)^B
 return [eq(x1,x2,u),eq(u,x3,v),eq(v,x4,w),eq(w,x5,target)]
