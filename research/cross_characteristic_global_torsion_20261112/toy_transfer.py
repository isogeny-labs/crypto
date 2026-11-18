#!/usr/bin/env python3
"""Bounded cross-characteristic torsion specialization on Cremona 11a1.

No discrete log is used to apply the lookup, but constructing the complete
preimage table has O(N) size. This does not scale to cryptographic N.
"""
from fractions import Fraction as F
from itertools import product

A1,A2,A3,A4,A6 = 0,-1,1,-10,-20
O = None
P = (F(5),F(5))

def require(ok, msg="failed check"):
    if not ok: raise AssertionError(msg)

def on(pt,p=None):
    if pt is None: return True
    x,y=pt
    v=y*y+A1*x*y+A3*y-x**3-A2*x*x-A4*x-A6
    return v%p==0 if p else v==0

def inverse(a,p):
    return pow(int(a)%p,-1,p) if p else 1/a

def add(P,Q,p=None):
    if P is None: return Q
    if Q is None: return P
    x,y=P;u,v=Q
    if p:
        x,y,u,v=[int(t)%p for t in (x,y,u,v)]
        if x==u and (y+v+A1*x+A3)%p==0:return None
        if (x,y)==(u,v):
            den=(2*y+A1*x+A3)%p
            lam=(3*x*x+2*A2*x+A4-A1*y)*inverse(den,p)%p
            nu=(-x**3+A4*x+2*A6-A3*y)*inverse(den,p)%p
        else:
            den=(u-x)%p
            lam=(v-y)*inverse(den,p)%p
            nu=(y*u-v*x)*inverse(den,p)%p
        X=(lam*lam+A1*lam-A2-x-u)%p
        Y=(-(lam+A1)*X-nu-A3)%p
    else:
        if x==u and y+v+A1*x+A3==0:return None
        if P==Q:
            den=2*y+A1*x+A3
            lam=(3*x*x+2*A2*x+A4-A1*y)/den
            nu=(-x**3+A4*x+2*A6-A3*y)/den
        else:
            lam=(v-y)/(u-x)
            nu=(y*u-v*x)/(u-x)
        X=lam*lam+A1*lam-A2-x-u
        Y=-(lam+A1)*X-nu-A3
    R=(X,Y)
    require(on(R,p),"addition left curve")
    return R

def global_torsion():
    T=[O]
    for i in range(4):T.append(add(T[-1],P))
    require(add(T[-1],P) is O,"not order five")
    require(len(set(T))==5,"torsion collision")
    require(all(on(t) for t in T),"bad rational point")
    require(T==[None,(F(5),F(5)),(F(16),F(-61)),(F(16),F(60)),(F(5),F(-6))],"reference mismatch")
    return T

def reduce(Q,p):
    if Q is None:return None
    require(all(v.denominator%p for v in Q),"bad denominator")
    return tuple(int(v.numerator*pow(v.denominator,-1,p))%p for v in Q)

def group(p):
    return {O}|{(x,y) for x,y in product(range(p),repeat=2) if on((x,y),p)}

def bridge(source,target,T):
    left={reduce(Q,source):Q for Q in T}
    right={Q:reduce(Q,target) for Q in T}
    require(len(left)==5 and len(set(right.values()))==5,"noninjective reduction")
    return {x:right[Q] for x,Q in left.items()}

def validate():
    T=global_torsion()
    require(len(group(2))==len(group(3))==5,"source full-group order")
    for p in (2,3,7,23):
        table=bridge(p,2,T)
        require(len(table)==5,"not five map entries")
        for i in range(5):
            for j in range(5):
                require(table[reduce(T[(i+j)%5],p)]==add(table[reduce(T[i],p)],table[reduce(T[j],p)],2),"not a homomorphism")
        for i in range(5):
            Q=reduce(T[i],p)
            require(table[Q]==reduce(T[i],2),"wrong lift")
            R=None
            for n in range(5):
                if R==table[Q]:break
                R=add(R,table[reduce(T[1],p)],2)
            else:raise AssertionError("no recovered log")
            require(n==i,"wrong recovered log")
        wrong=dict(table)
        x,y=reduce(T[2],p),reduce(T[3],p)
        wrong[x],wrong[y]=wrong[y],wrong[x]
        require(any(wrong[reduce(T[(i+j)%5],p)]!=add(wrong[reduce(T[i],p)],wrong[reduce(T[j],p)],2) for i in range(5) for j in range(5)),"mutant not rejected")
        print("source",p,"target 2","5/5 lifts","25/25 homomorphism pairs","5/5 public toy DLPs","mutant rejected")
    print("3->2",bridge(3,2,T))
    print("LIMIT: five-entry precomputed global-torsion lookup; no asymptotic advantage")
if __name__=="__main__":validate()
