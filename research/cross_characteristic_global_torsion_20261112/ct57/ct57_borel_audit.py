#!/usr/bin/env python3
"""Finite GL2 audit: local Frobenius lines are not global invariant lines.
This is a toy arithmetic check of elementary algebra, NOT a proof of Mazur.
"""
from itertools import product

PRIMES = (3, 5, 7, 11)
ALLOWED_Q_ISOGENY_PRIMES = frozenset((2, 3, 5, 7, 11, 13, 17, 19, 37, 43, 67, 163))

def require(test, reason):
    if not test:
        raise AssertionError(reason)

def mul(m, v, ell):
    a, b, c, d = m
    x, y = v
    return ((a*x+b*y) % ell, (c*x+d*y) % ell)

def fixed_line(m, v, ell):
    w = mul(m, v, ell)
    return (v[0]*w[1]-v[1]*w[0]) % ell == 0

def lines(ell):
    return [(1, t) for t in range(ell)] + [(0, 1)]

def run():
    for ell in PRIMES:
        total, fixed_point, fixed_line_count = 0, 0, 0
        for a, b, c, d in product(range(ell), repeat=4):
            if (a*d-b*c) % ell == 0:
                continue
            total += 1
            if (a, c) == (1, 0):
                fixed_point += 1
            if c == 0:
                fixed_line_count += 1
        require(total == (ell**2-1)*(ell**2-ell), 'bad GL2 size')
        require(fixed_point == ell*(ell-1), 'bad oriented point stabilizer')
        require(fixed_line_count == ell*(ell-1)**2, 'bad line stabilizer')
        require(total // fixed_point == ell**2-1, 'bad oriented orbit')
        require(total // fixed_line_count == ell+1, 'bad projective orbit')
        frob = (1, 0, 0, 2)         # locally fixes (1,0); det=2 != 0
        swap = (0, ell-1, 1, 0)    # exchanges the two coordinate lines
        require(fixed_line(frob, (1, 0), ell), 'local fixed line absent')
        require(any(fixed_line(frob, v, ell) for v in lines(ell)), 'no local line')
        require(not any(fixed_line(frob, v, ell) and fixed_line(swap, v, ell)
                        for v in lines(ell)), 'local fixed line wrongly global')
        print(f'ell={ell} GL2={total} oriented_stabilizer={fixed_point}'
              f' line_stabilizer={fixed_line_count} oriented_orbit={ell**2-1}'
              f' line_orbit={ell+1} local_not_global=PASS')
    require(all(n not in ALLOWED_Q_ISOGENY_PRIMES for n in (23, 29, 31, 127, 163 + 2)),
            'disallowed rational-isogeny cases')
    require(all(n in ALLOWED_Q_ISOGENY_PRIMES for n in (3, 5, 7, 11, 163)),
            'allowlist control')
    print('PASS: finite group audits; Mazur input is external theorem, not proved here')

if __name__ == '__main__':
    run()
