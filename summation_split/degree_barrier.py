"""Exact signed-fiber obstruction for Semaev-only lex elimination.

This is NOT a Frobenius-sector Groebner solver, an F4 measurement or a
first-fall-degree estimate. It uses E(Fp) rational points only, deliberately
selects full-degree fibers, and checks the specialized resultant independently.

Reproduce: PYTHONPATH=. python -m summation_split.degree_barrier
"""
from __future__ import annotations

import argparse
import csv
import os
import random

from .core import admissible, eval_poly, specialized_summation
from .sector import add, point_neg, sqrt_mod

P31 = 2**31 - 1
CELLS = ((P31, 1, 1, 3), (P31, 1, 1, 4),
         (P31, 1, 1, 5), (P31, 1, 1, 6),
         (P31, 1, 4, 3), (P31, 1, 4, 4),
         (P31, 1, 4, 5), (P31, 1, 4, 6))
SEED = 20261123


def point_with_nonzero_ordinate(p: int, a: int, b: int, x: int):
    rhs = (x*x*x + a*x + b) % p
    if rhs == 0:
        return None
    if pow(rhs, (p-1)//2, p) != 1:
        return None
    y = sqrt_mod(rhs, p)
    if y is None or y == 0 or y*y % p != rhs:
        raise ArithmeticError('bad square-root lift')
    return x, y


def signed_states(p: int, a: int, points):
    states = {None}
    for P in points:
        states = {add(Q, R, p, a, 1)
                  for Q in states for R in (P, point_neg(P, p))}
    return states


def product_linear(roots, p):
    coeffs = [1]
    for root in sorted(roots):
        out = [0] * (len(coeffs)+1)
        for i, c in enumerate(coeffs):
            out[i] = (out[i] - root*c) % p
            out[i+1] = (out[i+1] + c) % p
        coeffs = out
    return coeffs


def check_case(p, a, b, xs):
    """Check the exact polynomial identity on a maximal sign-orbit fiber."""
    if not admissible(p,a,b):
        raise ValueError('nonsingular odd-prime curve required')
    points = [point_with_nonzero_ordinate(p,a,b,x) for x in xs]
    if None in points:
        raise ValueError('all input x must lift to non-2-torsion E(Fp)')
    states = signed_states(p,a,points)
    if None in states:
        raise ValueError('fiber sums contain identity; choose another fixture')
    roots = {P[0] for P in states}
    degree = 1 << (len(xs)-1)
    if len(roots) != degree:
        raise ValueError('nonmaximal sign orbit: select collision-free fixture')
    coeffs = specialized_summation(p,a,b,xs)
    if len(coeffs) != degree+1 or not coeffs[-1]:
        raise AssertionError('unexpected resultant degree drop')
    for root in roots:
        if eval_poly(coeffs,root,p) != 0:
            raise AssertionError('signed point sum is not a Semaev root')
    prod = product_linear(roots,p)
    if any((u - coeffs[-1]*v)%p for u,v in zip(coeffs,prod)):
        raise AssertionError('Semaev resultant does not factor as signed fiber')
    return {'degree':degree,'root_count':len(roots),
            'distinct_signed_states':len(states),
            'leading_coefficient':coeffs[-1],
            'coefficients_equal_product':True}


def pick_case(p,a,b,m,rng):
    # Distinct x avoids trivial repeated-leaf collisions.
    for _ in range(200):
        xs=[]
        while len(xs)<m:
            x=rng.randrange(p)
            if x not in xs and point_with_nonzero_ordinate(p,a,b,x):
                xs.append(x)
        states=signed_states(p,a,[point_with_nonzero_ordinate(p,a,b,x) for x in xs])
        if None not in states and len({Q[0] for Q in states}) == 1 << (m-1):
            return tuple(xs)
    raise AssertionError('no full-degree fixture after 200 tries')


def run(out_dir='summation_split',samples=50,seed=SEED):
    if samples<50:
        raise ValueError('minimum 50 independent seeded cases per cell')
    os.makedirs(out_dir,exist_ok=True)
    records=[]
    for p,a,b,m in CELLS:
        rng=random.Random(seed + p*10007 + a*101 + b*104729 + m*733)
        for i in range(samples):
            xs=pick_case(p,a,b,m,rng)
            result=check_case(p,a,b,xs)
            records.append(dict(p=p,a=a,b=b,m=m,case_id=i,
                xs=':'.join(map(str,xs)),generator_max_degree=4,
                lex_elimination_degree_lower_bound=result['root_count'],
                full_resultant_X_degree=result['degree'],
                signed_states=result['distinct_signed_states'],
                leading_coefficient=result['leading_coefficient'],
                exact_polynomial_factorization=result['coefficients_equal_product'],
                first_fall_degree='',d_F4='',backend='exact Fp signed point sums and resultant; NOT GB',
                pcc_standing='unverified'))
    path=os.path.join(out_dir,'degree_barrier_gate.csv')
    with open(path,'w',newline='') as fh:
        writer=csv.DictWriter(fh,fieldnames=records[0].keys())
        writer.writeheader()
        writer.writerows(records)
    tiny=check_case(5,1,1,(0,3,4))
    if tiny['root_count']!=4:
        raise AssertionError('tiny sharp witness')
    print('PASS',len(records),'prime31 exact full-fiber resultant certificates;',
          'smallest-p witness p=5 m=3 roots=4; d_F4 unmeasured')
    return records


if __name__=='__main__':
    ap=argparse.ArgumentParser()
    ap.add_argument('--out',default='summation_split')
    ap.add_argument('--samples',type=int,default=50)
    ap.add_argument('--seed',type=int,default=SEED)
    opts=ap.parse_args()
    run(opts.out,opts.samples,opts.seed)
