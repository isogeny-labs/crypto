from research.twist_auxiliary_20261116_v86.algebra import Field
from research.linearized_bridge_20261116_v90c.subspace import value

class Solver:
 def __init__(self,n,mod,d):
  self.f=Field(2,mod);self.n=n;self.d=d;self.mask=(1<<n)-1;self.reduction=sum(c<<i for i,c in enumerate(mod))^(1<<n);self.basis=[1<<i for i in range(d)];self.squares=[self.f.sq(x)for x in self.basis];self.AS=self.prepare([self.f.sq(1<<j)^(1<<j)for j in range(n)])
 def timesT(self,x):return ((x<<1)&self.mask)^(self.reduction if x>>(self.n-1)else 0)
 @staticmethod
 def prepare(columns):
  rows={};kernel=[]
  for j,v in enumerate(columns):
   witness=1<<j
   while v:
    p=v.bit_length()-1
    if p not in rows:rows[p]=(v,witness);break
    w,k=rows[p];v^=w;witness^=k
   else:kernel.append(witness)
  return rows,kernel
 @staticmethod
 def particular(c,rows):
  witness=0
  while c:
   p=c.bit_length()-1
   if p not in rows:return None
   v,k=rows[p];c^=v;witness^=k
  return witness
 def roots(self,c,prepared):
  rows,kernel=prepared;x=self.particular(c,rows)
  if x is None:return []
  if 1<<len(kernel)>1024:raise OverflowError('explicit fiber cap')
  out=[x]
  for k in kernel:out +=[u^k for u in out]
  return sorted(out)
 def columns(self,a,b,stream):
  if not stream:return [self.f.mul(a,s)^self.f.mul(b,x)for x,s in zip(self.basis,self.squares)]
  out=[]
  for _ in range(self.d):out.append(a^b);a=self.timesT(self.timesT(a));b=self.timesT(b)
  return out
 def solve(self,a,b,c,method):
  f=self.f
  if method in ['matrix','stream']:return self.roots(c,self.prepare(self.columns(a,b,method=='stream')))
  if method=='annihilator':
   cs=[1]
   for v in self.columns(a,b,True):
    w=value(cs,v,f)
    if not w:continue
    new=[0]*(len(cs)+1)
    for j,h in enumerate(cs):new[j]^=f.mul(w,h);new[j+1]^=f.sq(h)
    cs=new
   if value(cs,c,f):return []
   return self.roots(c,self.prepare(self.columns(a,b,True)))
  assert method=='normalized'
  if not a:
   if not b:return self.roots(c,({},[1<<i for i in range(self.d)]))
   x=f.mul(c,f.inv(b));return [x]if x<1<<self.d else[]
  if not b:
   x=f.power(f.mul(c,f.inv(a)),1<<(self.n-1));return [x]if x<1<<self.d else[]
  invab=f.inv(f.mul(a,b));ia=f.mul(b,invab);ib=f.mul(a,invab);t=f.mul(b,ia);w=f.mul(f.mul(c,a),f.sq(ib));u=self.particular(w,self.AS[0])
  if u is None:return []
  xs=[f.mul(t,u),f.mul(t,u^1)];return sorted(x for x in xs if x<1<<self.d)
 def coefficients(self,y,u,B=1):
  v=self.f.mul(y,u);return self.f.sq(y^u),v,self.f.sq(v)^B
