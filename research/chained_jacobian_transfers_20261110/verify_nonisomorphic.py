#!/usr/bin/env python3
"""Deterministic check: genus-2 push-pull [2] followed by a 2-isogeny.
The genus-2 identity is proved algebraically, not implemented as divisor arithmetic.
"""
p=101
def inv(a):return pow(a%p,-1,p)
def add(P,Q,a=0,b=0):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u and (y+v)%p==0:return None
    if P==Q:
        if y==0:return None
        m=(3*x*x+2*a*x+b)*inv(2*y)%p
    else:m=(v-y)*inv(u-x)%p
    z=(m*m-a-x-u)%p
    return z,(m*(x-z)-y)%p
def mul(n,P,a=0,b=0):
    R=None
    while n:
        if n&1:R=add(R,P,a,b)
        P=add(P,P,a,b);n//=2
    return R
def phi(P):
    # Shift x->X=x+1: y²=X³-3X²+3X; kernel (X,y)=(0,0).
    # Vélu 2-isogeny to Y²=U³+6U²-3U:
    # U=X-3+3/X; V=y(1-3/X²).
    if P is None:return None
    x,y=P;X=(x+1)%p
    if X==0:return None
    return (X-3+3*inv(X))%p, y*(1-3*inv(X*X))%p
def main():
    pts=[(x,y) for x in range(p) for y in range(p) if (y*y-x**3-1)%p==0]
    assert len(pts)+1==102
    for P in pts:
        T=phi(P)
        assert T is None or (T[1]**2-(T[0]**3+6*T[0]**2-3*T[0]))%p==0
        for Q in pts:
            assert phi(add(P,Q))==add(phi(P),phi(Q),6,-3)
    P=next(Q for Q in pts if mul(17,Q) is None)
    assert all(mul(j,P) is not None for j in range(1,17))
    for j in range(17):
        assert phi(mul(2*j,P))==mul(j,phi(mul(2,P)),6,-3)
    # j(E)=0; E2 has c4=16(a2²-3a4)=720 nonzero mod 101.
    assert 720%p!=0
    print("PASS: p=101, #E=102, r=17, P=",P,"image=",phi(P))
    print("PASS: all 17 logs, all curve points, all ordered pair homomorphism checks")
    print("NOTE: genus-2 divisor arithmetic not independently implemented")
if __name__=="__main__":main()
