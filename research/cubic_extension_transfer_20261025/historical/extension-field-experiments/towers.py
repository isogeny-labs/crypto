"""Intermediate-field controls: trace transitivity and loss of extension torsion."""
import random
from time import perf_counter
from sage.all import GF, EllipticCurve, factor
from common import (frobenius, trace_point, min_field_degree, prime_generator,
                    point_data, field_data, bsgs_ec, project_prime_order)


def tower_controls(p,n,d):
    assert n%d==0 and d>1
    start=perf_counter()
    k=GF(p); K=GF(p**n,name='z')
    for a in range(p):
        for b in range(1,p):
            if (4*a**3+27*b*b)%p==0:
                continue
            E0=EllipticCurve(k,[a,b])
            if E0.is_supersingular() or E0.cardinality()==p:
                continue
            E=E0.change_ring(K)
            # Count over F_(p^d) without relying on a particular subfield embedding.
            Ed=E0.change_ring(GF(p**d,name='v'))
            N=int(E.cardinality()); Nd=int(Ed.cardinality())
            new_primes=[int(r) for r,e in factor(N) if r>3 and Nd%r]
            base_primes=[int(r) for r,e in factor(E0.cardinality()) if r>3 and n%r]
            if new_primes and base_primes:
                break
        else:
            continue
        break
    else:
        raise ValueError('No curve found for both tower controls')
    r=max(new_primes)
    for _ in range(100):
        P=project_prime_order(E.random_point(),N,r)
        if not P.is_zero() and (r*P).is_zero():
            break
    assert not P.is_zero()
    secret=random.randrange(1,r); Q=secret*P
    T=trace_point(P,p**d,n//d)
    assert T.is_zero()  # E(F_(p^d)) has no r-torsion at all.
    assert trace_point(Q,p**d,n//d).is_zero()
    R=E.random_point(); S=E.random_point()
    assert trace_point(R+S,p**d,n//d)==trace_point(R,p**d,n//d)+trace_point(S,p**d,n//d)
    stage=trace_point(R,p**d,n//d)
    assert frobenius(stage,p**d)==stage
    assert trace_point(stage,p,d)==trace_point(R,p,n)
    rb=max(base_primes)
    P0,_=prime_generator(E0,exclude=tuple(int(t) for t,e in factor(E0.cardinality()) if t!=rb))
    B=E(P0)
    Bimage=trace_point(B,p**d,n//d)
    assert Bimage==(n//d)*B and not Bimage.is_zero()
    mb=random.randrange(1,rb)
    recovered=bsgs_ec(Bimage,trace_point(mb*B,p**d,n//d),rb)
    assert recovered['scalar']==mb
    return {'family':'intermediate_field_controls','p':p,'n':n,'d':d,'status':'passed',
            'field':field_data(K),'curve':str(E),'ambient_curve_order':N,
            'intermediate_curve_order':Nd,'base_curve_order':int(E0.cardinality()),
            'extension_subgroup_order':r,'extension_point':point_data(P),
            'extension_point_minimum_degree':min_field_degree(P,p,n),
            'extension_trace_is_zero':True,'extension_scalar_information_retained':False,
            'base_subgroup_order':rb,'base_trace_multiplier':n//d,
            'base_subgroup_survives':True,'trace_transitivity_verified':True,
            'base_bsgs':recovered,'seconds':perf_counter()-start,
            'claim':'field trace is homomorphic but annihilates the selected new subgroup'}
