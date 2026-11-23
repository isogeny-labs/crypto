"""Exact target-first binary chain join; conditional source-bag contract."""
from research.original_chain_lift_20261115_v55.solver import Lift
class Join(Lift):
 def __init__(self,f,B):
  super().__init__(f,B);self.common_cache={};self.join_branches={'linear':0,'inconsistent':0,'proportional':0}
 def common(self,a,b,c,d):
  f=self.f;p=tuple(sorted((a,b)));q=tuple(sorted((c,d)));key=tuple(sorted((p,q)))
  if key in self.common_cache:return self.common_cache[key]
  L=f.mul(a,b);A=f.square(a^b);C=f.square(L)^self.B
  M=f.mul(c,d);D=f.square(c^d);E=f.square(M)^self.B
  den=f.mul(D,L)^f.mul(A,M);num=f.mul(D,C)^f.mul(A,E)
  if den:
   self.join_branches['linear']+=1;v=f.mul(num,f.inv(den));out=(v,)if not self.residual(a,b,v)and not self.residual(c,d,v)else()
  elif num:
   self.join_branches['inconsistent']+=1;out=()
  else:
   self.join_branches['proportional']+=1;out=tuple(v for v in self.roots(a,b)if not self.residual(c,d,v))
  self.common_cache[key]=out;return out
 def ordered(self,xs,target):
  if len(xs)!=5:raise ValueError('five source coordinates required')
  xs=tuple(self.f.validate(x)for x in xs);target=self.f.validate(target);out=[]
  U=self.roots(xs[0],xs[1]);W=self.roots(xs[4],target)
  for u in U:
   for w in W:
    for v in self.common(u,xs[2],w,xs[3]):out.append(xs+(u,v,w))
  return sorted(out)
