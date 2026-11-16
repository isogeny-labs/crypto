#!/usr/bin/env python3
"""Split-fiber checks of explicit f* and f_* on Mumford divisor classes."""
from cantor import *
from verify_nonisomorphic import add as eadd,mul as emul
def roots(u):return [t for t in range(p) if sum(c*pow(t,i,p) for i,c in enumerate(u))%p==0]
def onE(Q):return Q is None or (Q[1]*Q[1]-Q[0]**3-1)%p==0
Q0=(p-1,0)
def ef(t,Y):
 x=(root+pow(t,-1,p))%p
 y=Y*pow(t,-3,p)%p
 return (x*x%p,y)
def push(A):
 u,v=A;rr=roots(u)
 if len(rr)!=len(u)-1 or 0 in rr:return None
 R=None
 for t in rr:
  y=sum(c*pow(t,i,p) for i,c in enumerate(v))%p
  R=eadd(R,ef(t,y))
 return eadd(R,emul(-len(rr)%102,Q0))
def pull(Q):
 x,y=Q;xs=[z for z in range(p) if z*z%p==x]
 if len(xs)!=2:return None
 D=O
 for z in xs:
  if z==root:continue
  D=compose(D,point(z,y))
 return D
def main():
 pts=[(x,y) for x in range(p) for y in range(p) if onE((x,y))]
 B=pull(Q0);assert B is not None
 count=0
 for Q in pts:
  A=pull(Q)
  if A is None:continue
  D=compose(A,inverse(B));got=push(D)
  if got is None:continue
  expected=eadd(emul(2,Q),emul(-2%102,Q0))
  assert got==expected,(Q,D,got,expected)
  count+=1
 assert count>0
 print('PASS independently represented genus-2 divisor pullback/pushforward for',count,'rational split cases')
if __name__=='__main__':main()
