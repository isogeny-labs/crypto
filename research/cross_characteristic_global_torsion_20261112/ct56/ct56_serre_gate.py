#!/usr/bin/env python3
"""CT56: exact binary Serre torsion/degree gates; necessary, not existence tests."""
from math import isqrt


def require(condition, reason):
    if not condition:
        raise AssertionError(reason)


def b(f):
    require(f >= 1, 'invalid extension degree')
    q = 1 << f
    return q + 1 + isqrt(4*q)


def g_min(n, f):
    require(n >= 2, 'invalid target order')
    bound = b(f)
    power, g = 1, 0
    while power < n:
        power *= bound
        g += 1
    return g


def f_min(n, g):
    require(g >= 1, 'invalid dimension')
    f = 1
    while b(f)**g < n:
        f += 1
    return f


def order_mod_two(ell):
    require(ell > 1 and ell % 2, 'ell must be odd')
    a = 2 % ell
    x = a
    for f in range(1, ell+1):
        if x == 1:
            return f
        x = (x*a) % ell
    raise AssertionError('order not found; ell should be prime')


def full_level_possible(ell, f):
    # Entire rank-2g torsion rational requires ell^2 <= Serre base,
    # independently of g, plus ell-th roots of unity via Weil pairing.
    return ell*ell <= b(f) and pow(2, f, ell) == 1


def count_curve_f2(a):
    return 1 + sum((y*y+y-x*x*x-a*x) % 2 == 0 for x in (0,1) for y in (0,1))


def run():
    require(count_curve_f2(0) == 3 and count_curve_f2(1) == 5,
            'elliptic F2 controls')
    require(b(1) == 5 and b(2) == 9 and b(3) == 14 and b(4) == 25,
            'Serre base regression')
    for bits, n in ((127,(1<<127)-1),(255,(1<<255)-19)):
        row = tuple(g_min(n, f) for f in (1,2,4,8))
        cols = tuple(f_min(n, g) for g in (1,2,3,4))
        for f,g in zip((1,2,4,8),row):
            require(b(f)**g >= n and (g==0 or b(f)**(g-1) < n),
                    'g not minimal')
        for g,f in zip((1,2,3,4),cols):
            require(b(f)**g >= n and (f==1 or b(f-1)**g < n),
                    'f not minimal')
        print('bits',bits,'minimum g for f=1,2,4,8',row,
              'minimum f for g=1,2,3,4',cols)
    require(3 <= b(1) and not full_level_possible(3,1),
            'full-vs-single control')
    require(full_level_possible(3,2), 'full 3-torsion F4 control')
    require(not full_level_possible(5,2), 'full 5-torsion F4 failure')
    require(full_level_possible(5,4), 'F16 necessary (not sufficient)')
    # CT55 binary E0:y²+y=x³ has Frobenius trace zero. Its F16
    # group has 9 points, so 5-torsion is absent despite the necessary gate.
    t0, t1 = 2, 0
    for _ in range(2, 5):
        t0, t1 = t1, -2*t0
    require((1<<4)+1-t1 == 9, 'necessary is not sufficient')
    for ell in (3,5,7,11,13,17,19,23,31,43):
        f = 1
        while not full_level_possible(ell, f):
            f += 1
        require(f % order_mod_two(ell) == 0 and b(f) >= ell*ell,
                'full-level necessary conditions')
        require(not any(full_level_possible(ell, j) for j in range(1,f)),
                'earlier full-level admission')
        print('ell',ell,'f_min_full_necessary',f,'ord_ell(2)',order_mod_two(ell))
    print('PASS: exact Serre/Weil pairing gates; no existence or runtime claim')


if __name__=='__main__':
    run()
