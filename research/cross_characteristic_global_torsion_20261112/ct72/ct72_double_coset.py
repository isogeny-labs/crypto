#!/usr/bin/env python3
"""CT72: full GL2-conjugation audit of split-vs-irreducible Frobenius alignments.
Pure finite combinatorics. Not a DLP solver or a global lift.
"""
import json
from itertools import product

PRIMES=(3,5,7,11)

def check(condition,message="CT72 assertion failed"):
    if not condition:
        raise AssertionError(message)

def nonsquare(a,p):
    return a%p not in {x*x%p for x in range(p)}

def choose(p):
    for det in range(1,p):
        if nonsquare(1-4*det,p):
            return (1,det)
    raise AssertionError('no nonsplit charpoly')

def mm(A,B,p):
    return ((A[0]*B[0]+A[1]*B[2])%p,(A[0]*B[1]+A[1]*B[3])%p,
            (A[2]*B[0]+A[3]*B[2])%p,(A[2]*B[1]+A[3]*B[3])%p)

def determinant(A,p):
    return (A[0]*A[3]-A[1]*A[2])%p

def inverse(A,p):
    d=pow(determinant(A,p),-1,p)
    return ((A[3]*d)%p,(-A[1]*d)%p,(-A[2]*d)%p,(A[0]*d)%p)

def conjugate(G,F,p):
    return mm(mm(G,F,p),inverse(G,p),p)

def run(p):
    tr,det=choose(p)
    T=(0,(-det)%p,1,tr)
    S=(1,0,0,p-1)
    group=[a for a in product(range(p),repeat=4) if determinant(a,p)]
    check(len(group)==p*(p-1)**2*(p+1))
    Cs=[g for g in group if mm(g,S,p)==mm(S,g,p)]
    Ct=[g for g in group if mm(g,T,p)==mm(T,g,p)]
    check(len(Cs)==(p-1)**2 and len(Ct)==p*p-1)
    check(len({(2*u-tr)%p for u in range(p)})==p)
    images={conjugate(g,T,p) for g in group}
    check(len(images)==p*(p-1), 'GL2 conjugacy-class count')
    seen=set()
    all_us=set()
    for A in sorted(images):
        if A in seen:continue
        orbit={conjugate(D,A,p) for D in Cs}
        check(len(orbit)==p-1)
        check(orbit <= images)
        check({x[0] for x in orbit}=={A[0]})
        check({(x[0]-x[3])%p for x in orbit}=={(2*A[0]-tr)%p})
        seen.update(orbit)
        all_us.add(A[0])
        # A has irreducible characteristic polynomial, so intersection is scalar.
        check(sum(1 for D in Cs if mm(D,A,p)==mm(A,D,p))==p-1)
    check(seen==images and all_us==set(range(p)))
    for u in range(p):
        block=[A for A in images if A[0]==u]
        check(len(block)==p-1)
        check(all(A[3]==(tr-u)%p and determinant(A,p)==det for A in block))
    return {'ell':p,'trace':tr,'det':det,'gl2_size':len(group),
            'split_centralizer':len(Cs),'nonsplit_centralizer':len(Ct),
            'scalar_intersection':p-1,'relative_orbits':len(all_us),
            'matrices_per_orbit':p-1,'conjugates':len(images),
            'invariant_u':sorted(all_us)}

for p in PRIMES:
    print(json.dumps(run(p),sort_keys=True,separators=(',',':')))
