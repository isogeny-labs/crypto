"""Trace-dual rank of normalized v coefficient block."""
def derivative(poly):return sum(1<<(i-1)for i in range(1,poly.bit_length(),2)if poly>>i&1)
def rank(cols):
 piv={}
 for x in cols:
  while x:
   p=x.bit_length()-1
   if p not in piv:piv[p]=x;break
   x^=piv[p]
 return len(piv)
def trace(f,x):
 t=0
 for _ in range(f.degree):t^=x;x=f.square(x)
 assert t in(0,1);return t
class Dual:
 def __init__(self,f,w3,w4):
  assert 0<=w3<=w4<=f.degree
  self.f=f;self.w3=w3;self.w4=w4;self.derivative=derivative(f.modulus);inv=f.inv(self.derivative);assert f.mul(inv,self.derivative)==1
  self.mu=[f.mul(inv,1<<j)for j in range(f.degree-w4)];self.half=[]
  for m in self.mu:
   r=m
   for _ in range(f.degree-1):r=f.square(r)
   assert f.square(r)==m;self.half.append(r)
 def terms(self,k):return[a^self.f.mul(k,b)for a,b in zip(self.half,self.mu)]
 def columns(self,h,k,invh=None):
  assert h
  invh=self.f.inv(h)if invh is None else invh;factor=self.f.mul(self.derivative,invh)
  return[self.f.mul(factor,v)>>(self.f.degree-self.w3)for v in self.terms(k)]
 def predicted_v_rank(self,h,k):
  if not h:return rank([(self.f.square(1<<j)^self.f.mul(k,1<<j))>>self.w4 for j in range(self.f.degree)])
  return self.f.degree-self.w3+rank(self.columns(h,k))
 def direct_v_rank(self,h,k):
  return rank([(self.f.mul(h,1<<j)>>self.w3)|(((self.f.square(1<<j)^self.f.mul(k,1<<j))>>self.w4)<<(self.f.degree-self.w3))for j in range(self.f.degree)])
