"""Exact univariate constraints for quadratic parameter maps over odd extensions."""
from research.subspace_factor_20261115_v29 import factor as p

def constraints(F,maps):
 G=None
 for a,b,c in maps:
  C=[0]*(2*F.p+1)
  for i,v in enumerate([a,b,c]):C[i*F.p]=F.add(C[i*F.p],p.power(F,v,F.p));C[i]=F.sub(C[i],v)
  C=p.trim(C)
  if C:G=p.monic(F,C)if G is None else p.gcd(F,G,C)
 if G is None:return dict(status='whole_field',polynomial=None)
 if len(G)==1:return dict(status='finite_constraint_roots',polynomial=[1])
 R=p.add(F,p.powmod(F,[0,1],F.order,G),[0,F.neg(1)])
 return dict(status='finite_constraint_roots',polynomial=p.gcd(F,G,R))

def value(F,map,t):return F.add(map[0],F.add(F.mul(map[1],t),F.mul(map[2],F.mul(t,t))))
def evaluate(F,P,t):
 v=0
 for c in reversed(P):v=F.add(F.mul(v,t),c)
 return v
