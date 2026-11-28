#!/usr/bin/env python3
"""Bounded Las Vegas root selector for the signed norm-branch cubic locus.

Imports the earlier, independent degree-18 interpolation and finite-field
rank checks. This script verifies the linear factor count by gcd(f,X^p-X),
extracts each root using bounded Cantor--Zassenhaus, and fails closed if the
factorization is incomplete. p must be a certified odd prime > 50.

This locates a *toy coefficient-rank defect*, not an ECDLP relation or solver.
"""
import argparse
import json
import random
from locus_selector import (mul, divide, square_cubic_from_monic, matrix)
from quotient_interpolate import interpolate_quotient
from run import elimination


def clean(a, p):
    a=[t%p for t in a]
    while len(a)>1 and not a[-1]: a.pop()
    return a


def plus(a,b,p):
    v=[0]*max(len(a),len(b))
    for i,x in enumerate(a): v[i]=(v[i]+x)%p
    for i,x in enumerate(b): v[i]=(v[i]+x)%p
    return clean(v,p)


def monic(a,p):
    a=clean(a,p)
    if a==[0]: raise ValueError('zero polynomial')
    return clean([x*pow(a[-1],-1,p)%p for x in a],p)


def rem(a,b,p): return divide(a,b,p)[1]


def power_mod(a,e,f,p):
    y=[1];a=rem(a,f,p)
    while e:
        if e&1: y=rem(mul(y,a,p),f,p)
        e>>=1
        if e: a=rem(mul(a,a,p),f,p)
    return y


def pgcd(a,b,p):
    while clean(b,p)!=[0]: a,b=b,rem(a,b,p)
    return monic(a,p)


def all_roots(f,p,seed=1,split_limit=128):
    """List all distinct F_p roots of a nonzero polynomial of degree <=3.

    Count is certified algebraically by the degree of gcd(f, X^p-X).
    A repeated or irreducible factor is handled without enumerating F_p.
    """
    if p<3 or p%2==0: raise ValueError('odd prime required')
    f=clean(f,p)
    if f==[0] or len(f)>4: raise ValueError('nonzero degree <=3 required')
    if len(f)==1: return []
    f=monic(f,p)
    h=pgcd(f,plus(power_mod([0,1],p,f,p),[0,-1],p),p)
    expected=len(h)-1
    rng=random.Random(seed)
    def split(g):
        if len(g)==2: return [(-g[0]*pow(g[1],-1,p))%p]
        for _ in range(split_limit):
            z=rng.randrange(p)
            s=power_mod([z,1],(p-1)//2,g,p)
            for constant in (-1,1):
                d=pgcd(g,plus(s,[constant],p),p)
                if 1<len(d)<len(g):
                    q,r=divide(g,d,p)
                    assert r==[0]
                    return split(monic(d,p))+split(monic(q,p))
        raise RuntimeError('polynomial split cap exhausted; incomplete roots refused')
    roots=sorted(set(split(h))) if expected else []
    assert len(roots)==expected
    assert all(sum(c*pow(z,i,p) for i,c in enumerate(f))%p==0 for z in roots)
    return roots


def select(p,fixed,seed=1):
    if p<=50 or p%2==0 or len(fixed)!=7 or len(set(v%p for v in fixed))!=7:
        raise ValueError('odd prime p>50 and seven distinct residues required')
    fixed=[v%p for v in fixed]
    # Universal column-operation argument: (z-x_j)^4 divides det K for
    # each j=4,5,6 over Z[x_0,...,x_7]. The quotient has degree <=6 in z,
    # so seven noncollision determinant evaluations suffice for interpolation.
    q=interpolate_quotient(fixed,p,heldout_checks=32)
    cubic=square_cubic_from_monic(q,p)  # remaining cubic-square hypothesis; fails closed
    roots=all_roots(cubic,p,seed=seed)
    good=[z for z in roots if z not in fixed]
    ranks=[len(elimination(matrix(fixed+[z],p),p)[1]) for z in good]
    if any(x>=6 for x in ranks): raise AssertionError('noncollision root retained full rank')
    return {'p':p,'fixed':fixed,'cubic':cubic,'noncollision_roots':good,
            'ranks':ranks,'root_count':len(roots),'gcd_root_count':len(roots),
            'det_degree':12+len(q)-1,'quotient_degree':len(q)-1,
            'interpolation_points':7,'heldout_determinant_checks':32,
            'scope':'coefficient-matrix diagnostic only'}


def selftest():
    for p in (3,7,1019,10007,2**61-1):
        for vals in ((1,), (1,2), (1,2,3), (1,1,2)):
            f=[1]
            for z in vals: f=mul(f,[-z,1],p)
            assert all_roots(f,p)==sorted(set(v%p for v in vals))
        if p%4==3: assert all_roots([1,0,1],p)==[]
    out=[select(1019,[944,243,432,225,627,387,914])]
    assert out[0]['cubic']==[609,79,1017,1]
    assert out[0]['noncollision_roots']==[764] and out[0]['ranks']==[4]
    for p in (10007,2**61-1,2**127-1,2**256-2**32-977):
        out.append(select(p,[944,243,432,225,627,387,914]))
    return {'selftest':'passed','cases':out}


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--selftest',action='store_true')
    parser.add_argument('--p',type=int)
    parser.add_argument('--seven-x',type=int,nargs=7)
    args=parser.parse_args()
    if args.selftest: print(json.dumps(selftest(),sort_keys=True,indent=2))
    else:
        if args.p is None or args.seven_x is None: parser.error('--p and --seven-x required')
        print(json.dumps(select(args.p,args.seven_x),sort_keys=True,indent=2))
