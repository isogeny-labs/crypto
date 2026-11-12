#!/usr/bin/env python3
"""CT38: public rational 5-torsion reduction across characteristics.

This is not an arbitrary-point characteristic-changing map or DLP speedup.
"""
import json
from itertools import product

def on_curve(P,p):
    return P is None or (P[1]**2-P[1]-P[0]**3+P[0]**2)%p==0

def add(P,Q,p):
    if P is None:return Q
    if Q is None:return P
    if not on_curve(P,p) or not on_curve(Q,p):raise ValueError("off curve")
    x,y=P;u,v=Q
    if x==u and (y+v-1)%p==0:return None
    if P==Q:
        d=pow(2*y-1,-1,p)
        slope=(3*x*x-2*x)*d%p
        intercept=(-x**3+y)*d%p
    else:
        d=pow(u-x,-1,p)
        slope=(v-y)*d%p
        intercept=(y*u-v*x)*d%p
    X=(slope*slope+1-x-u)%p
    Y=(1-slope*X-intercept)%p
    R=(X,Y)
    if not on_curve(R,p):raise ArithmeticError("bad addition")
    return R

def mul(k,P,p):
    R=None
    while k:
        if k&1:R=add(R,P,p)
        P=add(P,P,p)
        k//=2
    return R

def check(cond,what):
    if not cond:raise AssertionError(what)

def points(p):
    return [None]+[(x,y) for x in range(p) for y in range(p) if on_curve((x,y),p)]

def main():
    primes=(7,13,17,101,103)
    expected=(None,(0,0),(1,1),(1,0),(0,1))
    counts={}
    for p in primes:
        ps=points(p)
        counts[p]=len(ps)
        T=tuple(mul(k,(0,0),p) for k in range(5))
        check(T==expected,f"torsion table p={p}")
        check(mul(5,(0,0),p) is None and counts[p]%5==0,f"order p={p}")
        for i,j in product(range(5),repeat=2):
            check(add(T[i],T[j],p)==T[(i+j)%5],f"law p={p}")
    for p,q in ((7,13),(17,101),(101,103)):
        for a,b in product(range(5),repeat=2):
            check(mul(a,mul(b,(0,0),p),p)==mul(a*b%5,(0,0),p),"source")
            check(mul(a,mul(b,(0,0),q),q)==mul(a*b%5,(0,0),q),"target")
    check(on_curve((2,29),101) and not on_curve((2,29),103),"off-curve negative control")
    check(101%101==0 and 101%103!=0,"no unital field hom")
    print(json.dumps({"torsion_order":5,"point_counts":counts,
      "torsion":[None,[0,0],[1,1],[1,0],[0,1]],"negative_control":[2,29],
      "standing":"[pcc:unverified]; no arbitrary-point transfer"},sort_keys=True))

if __name__=="__main__":main()
