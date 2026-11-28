#!/usr/bin/env python3
"""Toy genus-2 Cantor arithmetic over F_101 for y²=x⁶+1."""
from math import comb
import random
p=101;root=10
F=[comb(6,i)*pow(root,i,p)%p for i in range(7)]
F[-1]=(F[-1]+1)%p
assert F[-1]==0
F=F[:-1]
def trim(a):
 a=[x%p for x in a]
 while len(a)>1 and a[-1]==0:a.pop()
 return a
def add(a,b):
 c=[0]*max(len(a),len(b))
 for i,x in enumerate(a):c[i]+=x
 for i,x in enumerate(b):c[i]+=x
 return trim(c)
def neg(a):return trim([-x for x in a])
def sub(a,b):return add(a,neg(b))
def mul(a,b):
 c=[0]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):c[i+j]+=x*y
 return trim(c)
def divmodp(a,b):
 a=trim(a);b=trim(b);assert b!=[0]
 q=[0]*max(1,len(a)-len(b)+1)
 while a!=[0] and len(a)>=len(b):
  k=len(a)-len(b);t=a[-1]*pow(b[-1],-1,p)%p;q[k]=t
  a=sub(a,[0]*k+mul([t],b))
 return trim(q),a
def exact(a,b):
 q,r=divmodp(a,b);assert r==[0],(a,b,r)
 return q
def mod(a,b):return divmodp(a,b)[1]
def monic(a):return mul(a,[pow(a[-1],-1,p)])
def xgcd(a,b):
 s0,s1=[1],[0];t0,t1=[0],[1]
 while b!=[0]:
  q,r=divmodp(a,b);a,b=b,r;s0,s1=s1,sub(s0,mul(q,s1));t0,t1=t1,sub(t0,mul(q,t1))
 z=pow(a[-1],-1,p)
 return mul(a,[z]),mul(s0,[z]),mul(t0,[z])
def normalize(u,v):
 u=monic(u);v=mod(v,u)
 assert mod(sub(mul(v,v),F),u)==[0],(u,v)
 while len(u)-1>2:
  u=monic(exact(sub(F,mul(v,v)),u));v=mod(neg(v),u)
 assert len(u)-1<=2
 return tuple(u),tuple(v)
def compose(A,B):
 u1,v1=map(list,A);u2,v2=map(list,B)
 d1,a,b=xgcd(u1,u2)
 d,c,e=xgcd(d1,add(v1,v2))
 a=mul(a,c);b=mul(b,c)
 u=exact(mul(u1,u2),mul(d,d))
 numerator=add(add(mul(mul(a,u1),v2),mul(mul(b,u2),v1)),mul(e,add(mul(v1,v2),F)))
 v=mod(exact(numerator,d),u)
 return normalize(u,v)
O=((1,),(0,))
def point(x,y):
 assert (y*y-x**6-1)%p==0 and x!=root
 t=pow((x-root)%p,-1,p);Y=y*pow(t,3,p)%p
 assert (Y*Y-sum(c*pow(t,i,p) for i,c in enumerate(F)))%p==0
 return normalize([(-t)%p,1],[Y])
def inverse(A):return normalize(list(A[0]),neg(list(A[1])))
def scale(n,A):
 if n<0:return scale(-n,inverse(A))
 R=O
 while n:
  if n&1:R=compose(R,A)
  A=compose(A,A);n//=2
 return R
if __name__=='__main__':
 pts=[(x,y) for x in range(p) for y in range(p) if (y*y-x**6-1)%p==0 and x!=root]
 rng=random.Random(20261110)
 for i in range(1500):
  P=point(*rng.choice(pts));Q=point(*rng.choice(pts));R=point(*rng.choice(pts))
  assert compose(P,O)==P
  assert compose(P,inverse(P))==O
  assert compose(P,Q)==compose(Q,P)
  assert compose(compose(P,Q),R)==compose(P,compose(Q,R))
  assert scale(3,P)==compose(P,compose(P,P))
 print('PASS 1500 random triples: identity inverse commutativity associativity scalar')
