#!/usr/bin/env python3
"""Toy exact determinant-locus diagnostic for balanced norm-sign coefficients.

One eighth abscissa varies, seven are fixed; no elliptic-curve DLP solved.
Finite-field Newton interpolation is cross-checked on untouched values.
"""
import itertools as it
import json
import random
from counterexample import det
from run import vand, elimination


def trim(v):
    while len(v)>1 and v[-1]==0: v.pop()
    return v


def add(a,b,p):
    z=[0]*max(len(a),len(b))
    for i,t in enumerate(a): z[i]=(z[i]+t)%p
    for i,t in enumerate(b): z[i]=(z[i]+t)%p
    return trim(z)


def mul(a,b,p):
    z=[0]*(len(a)+len(b)-1)
    for i,t in enumerate(a):
        for j,v in enumerate(b): z[i+j]=(z[i+j]+t*v)%p
    return trim(z)


def evaluate(a,x,p):
    z=0
    for t in reversed(a): z=(z*x+t)%p
    return z


def divide(a,b,p):
    q=[0]*max(1,len(a)-len(b)+1);r=a[:]
    d=pow(b[-1],-1,p)
    while len(r)>=len(b) and any(r):
        i=len(r)-len(b); t=r[-1]*d%p;q[i]=t
        for j,v in enumerate(b): r[i+j]=(r[i+j]-t*v)%p
        trim(r)
    return trim(q),trim(r)


def matrix(xs,p):
    left=list(it.combinations(range(4),2))
    right=list(it.combinations(range(4,8),2))
    return [[pow(vand(xs,tuple(sorted(l+r)),p),2,p) for r in right] for l in left]


def det_at(fixed,z,p):
    return det(matrix(fixed+[z],p),p)


def interpolate(fixed,p):
    """Degree <=18 Newton interpolation plus 32 untouched evaluation checks."""
    poly=[0];prod=[1]
    for z in range(19):
        delta=(det_at(fixed,z,p)-evaluate(poly,z,p))%p
        q=delta*pow(evaluate(prod,z,p),-1,p)%p
        poly=add(poly,[q*t%p for t in prod],p)
        prod=mul(prod,[-z%p,1],p)
    for z in range(19,51):
        assert evaluate(poly,z,p)==det_at(fixed,z,p),z
    assert len(poly)==19,(p,fixed,poly)
    return poly


def square_cubic_from_monic(q,p):
    """Reconstruct monic cubic from normalized degree-six quotient."""
    assert len(q)==7 and q[-1]
    monic=[v*pow(q[-1],-1,p)%p for v in q]
    inv2=pow(2,-1,p)
    a=monic[5]*inv2%p
    b=(monic[4]-a*a)*inv2%p
    c=(monic[3]-2*a*b)*inv2%p
    cub=[c,b,a,1]
    assert mul(cub,cub,p)==monic,(p,monic,cub)
    return cub


def inspect(p,fixed):
    assert p>50 and len(fixed)==7 and len(set(fixed))==7
    poly=interpolate(fixed,p)
    collision=[1]
    for x in fixed[4:7]:
        for _ in range(4): collision=mul(collision,[-x%p,1],p)
    quotient,remainder=divide(poly,collision,p)
    assert remainder==[0],remainder
    cubic=square_cubic_from_monic(quotient,p)
    roots=[z for z in range(p) if evaluate(cubic,z,p)==0 and z not in fixed]
    for z in roots:
        assert len(elimination(matrix(fixed+[z],p),p)[1])<6
    return dict(p=p,fixed=fixed,det_degree=len(poly)-1,cubic=cubic,
                scalar=quotient[-1],noncollision_roots=roots,
                root_ranks=[len(elimination(matrix(fixed+[z],p),p)[1]) for z in roots],
                certificate_checks=32)


def main():
    cases=[(1019,[944,243,432,225,627,387,914])]
    for p in [1019,1019,1019,10007,10007]:
        rng=random.Random(p*997+len(cases)*101)
        cases.append((p,rng.sample(range(10,p-10),7)))
    print(json.dumps([inspect(p,fixed) for p,fixed in cases],sort_keys=True,indent=2))


if __name__=='__main__':
    main()
