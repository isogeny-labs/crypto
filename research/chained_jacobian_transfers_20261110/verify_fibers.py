#!/usr/bin/env python3
"""Independent geometric fiber tests for genus-2 quotient maps over F_101.
Checks actual degree-two fibers as point divisors; does not implement Cantor arithmetic.
"""
from verify_nonisomorphic import p,add,mul
def onE(P):
    return P is None or (P[1]*P[1]-P[0]**3-1)%p==0
def onC(P):
    x,y=P
    return (y*y-x**6-1)%p==0
def f(P):
    x,y=P
    return (x*x%p,y)
def g(P):
    x,y=P
    if x==0: return None # at x=0, g maps to infinity
    return (pow(x,-2,p),y*pow(x,-3,p)%p)
def main():
    pts=[(x,y) for x in range(p) for y in range(p) if onC((x,y))]
    assert pts
    fibers={}
    for P in pts:
        assert onE(f(P)) and onE(g(P))
        fibers.setdefault(f(P),[]).append(P)
    checked=0
    for Q,points in fibers.items():
        if Q[0]==0:continue # ramified fiber; excluded
        assert len(points)==2
        a,b=points
        assert a[0]==(-b[0])%p and a[1]==b[1]
        # f_* f^* on the fiber: Q+Q=[2]Q.
        assert add(f(a),f(b))==mul(2,Q)
        # g_* f^* on the fiber: g(a)+g(b)=O, a cross-factor zero.
        assert add(g(a),g(b)) is None
        checked+=1
    assert checked>0
    print("PASS: genus-2 rational fibers",checked,"f_*f*=2 and g_*f*=0 on split fibers")
    print("LIMIT: ramified/nonsplit fibers and divisor class arithmetic not independently checked")
if __name__=="__main__":main()
