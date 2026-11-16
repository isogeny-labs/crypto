"""Complete finite binary S3 chain lifts, conditional on supplied source bags."""
from itertools import permutations

class Lift:
 def __init__(self,field,B):
  self.f=field;self.B=field.validate(B)
  if not B:raise ValueError('nonsingular binary model requires B != 0')
  self.piv={};self.cache={};self.branches={'equal':0,'zero_product':0,'regular':0,'inconsistent':0}
  for i in range(1,field.degree):
   v=field.square(1<<i)^(1<<i);tag=1<<i
   while v:
    p=v.bit_length()-1
    if p in self.piv:a,b=self.piv[p];v^=a;tag^=b
    else:self.piv[p]=(v,tag);break
   if not v:raise ArithmeticError('Artin-Schreier rank')
  if len(self.piv)!=field.degree-1:raise ArithmeticError('Artin-Schreier rank')
 def AS(self,q):
  value=q;tag=0
  for p,(v,t) in sorted(self.piv.items(),reverse=True):
   if value>>p&1:value^=v;tag^=t
  return None if value else tag
 def sqrt(self,v):
  for _ in range(self.f.degree-1):v=self.f.square(v)
  return v
 def residual(self,a,b,c):
  f=self.f;v=f.mul(a,b)^f.mul(a,c)^f.mul(b,c)
  return f.square(v)^f.mul(f.mul(a,b),c)^self.B
 def roots(self,a,b):
  f=self.f;a=f.validate(a);b=f.validate(b);key=(min(a,b),max(a,b))
  if key in self.cache:return self.cache[key]
  L=f.mul(a,b);A=f.square(a^b);C=f.square(L)^self.B
  if not A:
   self.branches['equal']+=1
   out=() if not L else (f.mul(C,f.inv(L)),)
  elif not L:
   self.branches['zero_product']+=1;out=(self.sqrt(f.mul(C,f.inv(A))),)
  else:
   self.branches['regular']+=1;u=self.AS(f.mul(f.mul(A,C),f.inv(f.square(L))))
   if u is None:self.branches['inconsistent']+=1;out=()
   else:
    scale=f.mul(L,f.inv(A));v=f.mul(scale,u);out=tuple(sorted((v,v^scale)))
  if any(self.residual(a,b,x) for x in out):raise ArithmeticError('quadratic membership')
  self.cache[key]=out;return out
 def ordered(self,xs,target):
  if len(xs)!=5:raise ValueError('five source coordinates required')
  xs=tuple(self.f.validate(x)for x in xs);target=self.f.validate(target);out=[]
  for u in self.roots(xs[0],xs[1]):
   for v in self.roots(u,xs[2]):
    for w in self.roots(v,xs[3]):
     if not self.residual(w,xs[4],target):out.append(xs+(u,v,w))
  return sorted(out)
 def bag(self,roots,target,max_sources=5,max_outputs=100000):
  xs=[]
  for x,count in roots:
   if type(count)is not int or count<1:raise ValueError('positive multiplicities')
   xs.extend([self.f.validate(x)]*count)
  if len(xs)!=5 or len(xs)>max_sources:raise ValueError('source-degree cap')
  answer=[]
  for order in sorted(set(permutations(xs))):
   answer.extend(self.ordered(order,target))
   if len(answer)>max_outputs:raise RuntimeError('output cap; no completeness claim')
  return sorted(set(answer))
