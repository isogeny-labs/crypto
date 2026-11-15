#!/usr/bin/env python3
"""Seven-evaluation cubic-square quotient selector for the norm-rank locus.

Proved structural factor: det K(z) is divisible in Z[x_0,...,x_7]
by product_{j=4}^6 (z-x_j)^4. Each of the three variable-dependent
columns has degree <=6 in z. Hence quotient has degree <=6, so seven
noncollision evaluations determine it over F_p (p prime >50).

This result does not prove the remaining *square-of-cubic* hypothesis.
The independent out-of-sample determinants detect interpolation mistakes.
"""
from locus_selector import add, mul, evaluate, det_at


def quotient_value(fixed,z,p):
    den=1
    for x in fixed[4:7]:
        d=(z-x)%p
        if not d: raise ValueError('collision with right-hand fixed abscissa')
        den=den*pow(d,4,p)%p
    return det_at(fixed,z,p)*pow(den,-1,p)%p


def interpolate_quotient(fixed,p,heldout_checks=32):
    if p<=50 or p%2==0 or len(fixed)!=7 or len({x%p for x in fixed})!=7:
        raise ValueError('odd prime p>50 and seven distinct residues required')
    if not 0 <= heldout_checks <= p-14:
        raise ValueError('insufficient distinct field samples')
    fixed=[x%p for x in fixed]
    points=[]; z=0
    while len(points)<7+heldout_checks:
        if z not in fixed: points.append(z)
        z+=1
        if z>=p and len(points)<7+heldout_checks:
            raise ValueError('not enough noncollision samples')
    q=[0];basis=[1]
    for z in points[:7]:
        coeff=(quotient_value(fixed,z,p)-evaluate(q,z,p))*pow(evaluate(basis,z,p),-1,p)%p
        q=add(q,[coeff*c%p for c in basis],p)
        basis=mul(basis,[-z%p,1],p)
    for z in points[7:]:
        if quotient_value(fixed,z,p)!=evaluate(q,z,p):
            raise AssertionError('out-of-sample degree-six quotient check failed')
    assert len(q)<=7
    return q


if __name__=='__main__':
    import json
    from locus_selector import divide, interpolate
    cases=[(1019,[944,243,432,225,627,387,914]),(10007,[0,1,2,3,4,5,6]),
           (2**61-1,[944,243,432,225,627,387,914]),(2**127-1,[944,243,432,225,627,387,914])]
    out=[]
    for p, fixed in cases:
        q=interpolate_quotient(fixed,p)
        old=interpolate(fixed,p)
        collision=[1]
        for x in fixed[4:]:
            for _ in range(4):collision=mul(collision,[-x%p,1],p)
        old_q,rest=divide(old,collision,p)
        assert rest==[0] and old_q==q
        out.append({'prime_bits':p.bit_length(),'degree':len(q)-1,'old_interpolation_evals':19,
                    'new_interpolation_evals':7,'heldout_checks':32})
    print(json.dumps(out,sort_keys=True,indent=2))
