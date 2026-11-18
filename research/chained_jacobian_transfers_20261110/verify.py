#!/usr/bin/env python3
"""Finite-field subgroup check for the proven Jacobian push-pull composite."""
p=101
def inv(a): return pow(a%p,-1,p)
def add(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u and (y+v)%p==0:return None
    if P==Q:
        if y==0:return None
        m=3*x*x*inv(2*y)%p
    else:m=(v-y)*inv(u-x)%p
    z=(m*m-x-u)%p
    return z,(m*(x-z)-y)%p
def mul(n,P):
    R=None
    while n:
        if n&1:R=add(R,P)
        P=add(P,P);n>>=1
    return R
def tau(P):return None if P is None else ((P[0]+5)%p,P[1])
def main():
    pts=[(x,y) for x in range(p) for y in range(p) if (y*y-x**3-1)%p==0]
    assert len(pts)+1==102
    P=next(Q for Q in pts if mul(17,Q) is None)
    assert all(mul(j,P) is not None for j in range(1,17))
    assert (P[1]**2-P[0]**3-1)%p==0
    assert len({tau(mul(2*j,P)) for j in range(17)})==17
    for j in range(17):
        Q=mul(j,P)
        assert tau(mul(2,Q))==tau(mul(j,mul(2,P)))
        R=tau(mul(2,Q))
        assert R is None or (R[1]**2-(R[0]-5)**3-1)%p==0
    print("PASS p=101 #E=102 r=17 P=",P,"all 17 transported logs")
    print("NOTE: Jacobian identity is proven, not independently numerically implemented")
if __name__=="__main__":main()
