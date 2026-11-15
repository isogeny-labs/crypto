#!/usr/bin/env python3
"""CT53: independently check constructive symplectic transvection orbits.

Pure group-theoretic control. No elliptic curve group law or ECDLP involved.
"""
from itertools import product
from math import gcd


def require(condition, reason):
    if not condition:
        raise RuntimeError(reason)


def omega(v, w, ell):
    g = len(v) // 2
    return sum(v[i] * w[i+g] - v[i+g] * w[i] for i in range(g)) % ell


def transvection(v, u, a, ell):
    c = a * omega(v, u, ell) % ell
    return tuple((x + c*y) % ell for x,y in zip(v,u))


def candidates(ell, g):
    return (v for v in product(range(ell), repeat=2*g) if any(v))


def construct_path(v, w, ell):
    """At most two explicitly checkable Sp transvections take v to w."""
    require(any(v) and any(w) and len(v)==len(w), 'nonzero same-dimension required')
    if v == w:
        return []
    b = omega(v,w,ell)
    if b:
        u = tuple((y-x) % ell for x,y in zip(v,w))
        return [(u,pow(b,-1,ell))]
    for z in candidates(ell,len(v)//2):
        b1 = omega(v,z,ell)
        b2 = omega(z,w,ell)
        if b1 and b2:
            u1 = tuple((y-x)%ell for x,y in zip(v,z))
            u2 = tuple((y-x)%ell for x,y in zip(z,w))
            return [(u1,pow(b1,-1,ell)),(u2,pow(b2,-1,ell))]
    raise RuntimeError('no intermediate vector')


def verify_path(v,w,steps,ell):
    out=v
    for u,a in steps:
        require(len(u)==len(v) and a%ell and any(u),'bad transvection')
        out = transvection(out,u,a,ell)
    require(out==w, ('bad path',v,w,steps,out))


def check(ell,g,exhaustive):
    nonzero=list(candidates(ell,g))
    require(len(nonzero)==ell**(2*g)-1,'size')
    v=tuple([1]+[0]*(2*g-1))
    for w in nonzero:
        verify_path(v,w,construct_path(v,w,ell),ell)
    if exhaustive:
        for a in nonzero:
            for b in nonzero:
                verify_path(a,b,construct_path(a,b,ell),ell)
    # Test alternating bilinear form preservation against a deterministically chosen set of probes.
    seed=nonzero[:min(len(nonzero),40)]
    for u in seed:
        for a in (1,ell-1):
            for v in seed:
                for w in seed:
                    require(omega(transvection(v,u,a,ell),transvection(w,u,a,ell),ell)==omega(v,w,ell),'not symplectic')
    return {'ell':ell,'g':g,'orbit':len(nonzero),'all_source_pairs':exhaustive}


def run():
    for ell,g,exh in [(2,1,True),(2,2,True),(3,1,True),(3,2,True),(3,3,False),(5,1,True),(5,2,False),(7,1,True)]:
        print(check(ell,g,exh))
    for ell in (3,5,7,11):
        print('degree_floor',ell,'g1',ell**2-1,'g2',ell**4-1,'g3',ell**6-1,'split_factor',ell**2-1)
    # Deliberate mutant must fail even though all coordinates remain valid.
    v=(1,0,0,0); w=(0,0,1,0)
    path=construct_path(v,w,3)
    require(path,'bad mutant fixture')
    mutated=path[:-1] + [(path[-1][0],0)]
    try:
        verify_path(v,w,mutated,3)
    except RuntimeError:
        print('mutant rejected')
    else:
        raise RuntimeError('mutant escaped')
    print('PASS constructive transvection proof controls')

if __name__=='__main__': run()
