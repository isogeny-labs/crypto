#!/usr/bin/env python3
"""CT121 exact Frobenius-field degrees for E/F2; direct vector-orbit checks."""
import json
import math


def primes_upto(limit):
    return [p for p in range(3, limit + 1, 2)
            if all(p % j for j in range(3, int(p ** .5) + 1, 2))]


def order(a, ell):
    if a % ell == 0:
        raise ValueError('zero not in multiplicative group')
    k, z = 1, a % ell
    while z != 1:
        k, z = k + 1, z * a % ell
    return k


def predicted(ell, v):
    x, y = (z % ell for z in v)
    if x == y == 0:
        raise ValueError('zero input')
    if ell % 4 == 3:
        h = order(-4, ell)
        return 4 * h, 2 * h if h % 2 == 0 else 4 * h, 'inert'
    i = next(z for z in range(1, ell) if z*z % ell == ell-1)
    a, b = (-1+i) % ell, (-1-i) % ell
    # A=[0,-2;1,-2]. Identify eigenlines by the two eigen equations.
    def eigen(z):
        return (-2*y-z*x) % ell == 0 and (x-2*y-z*y) % ell == 0
    on_a, on_b = eigen(a), eigen(b)
    da, db = order(a, ell), order(b, ell)
    if on_a:
        d = da
        dx = d // 2 if d % 2 == 0 else d
        kind = 'eigen'
    elif on_b:
        d = db
        dx = d // 2 if d % 2 == 0 else d
        kind = 'eigen'
    else:
        d = da * db // math.gcd(da, db)
        dx = d // 2 if (d % 2 == 0
                       and pow(a, d//2, ell) == ell-1
                       and pow(b, d//2, ell) == ell-1) else d
        kind = 'mixed'
    return d, dx, kind


def measured(ell, v):
    x, y = (z % ell for z in v)
    if x == y == 0:
        raise ValueError('zero')
    a, b = x, y
    for n in range(1, ell * ell):
        a, b = -2*b % ell, (a-2*b) % ell
        if (a, b) == (x, y):
            return n, projective_period(ell, (x, y))
    raise ArithmeticError('orbit exceeded ell^2-1')


def projective_period(ell, v):
    x, y = v
    a, b = x, y
    for n in range(1, ell * ell):
        a, b = -2*b % ell, (a-2*b) % ell
        if (a, b) in ((x, y), (-x % ell, -y % ell)):
            return n
    raise ArithmeticError('projective orbit exceeded ell^2-1')


def eigenvectors(ell):
    if ell % 4 == 3:
        return []
    i = next(z for z in range(1, ell) if z*z % ell == ell-1)
    return [(-2 * pow((-1+s*i) % ell, -1, ell) % ell, 1)
            for s in (1, -1)]


def record(ell):
    vectors = [(1, 0), (0, 1), (1, 1)] + eigenvectors(ell)
    cases = []
    for v in vectors:
        p, m = predicted(ell, v), measured(ell, v)
        if p[:2] != m:
            raise ArithmeticError(f'mismatch {ell} {v}: {p} vs {m}')
        cases.append({'vector': list(v), 'point_degree': p[0],
                      'x_degree': p[1], 'class': p[2]})
    counts = {}
    if ell in (5, 13, 17, 29):
        for x in range(ell):
            for y in range(ell):
                if x or y:
                    p = predicted(ell, (x, y))
                    key = f'{p[2]}:{p[0]}:{p[1]}'
                    counts[key] = counts.get(key, 0) + 1
                    if p[:2] != measured(ell, (x, y)):
                        raise ArithmeticError(f'exhaustive mismatch {ell}: {(x, y)}')
        if sum(counts.values()) != ell*ell-1:
            raise ArithmeticError('coverage')
        if sum(c for k, c in counts.items() if k.startswith('eigen:')) != 2*(ell-1):
            raise ArithmeticError('eigenline count')
    return {'ell': ell, 'split': ell % 4 == 1,
            'cases': cases, 'full_counts': dict(sorted(counts.items()))}


def main():
    rows = [record(e) for e in primes_upto(211)]
    if len(rows) != 46:
        raise ArithmeticError(f'prime count: {len(rows)}')
    samples = [(3, 8, 4), (5, 4, 4), (11, 40, 20), (13, 12, 12),
               (17, 16, 8), (31, 40, 20), (47, 184, 92), (211, 840, 420)]
    actual = [(x['ell'], x['cases'][0]['point_degree'], x['cases'][0]['x_degree'])
              for x in rows if x['ell'] in (3, 5, 11, 13, 17, 31, 47, 211)]
    if actual != samples:
        raise ArithmeticError(f'fixture drift: {actual}')
    if predicted(5, eigenvectors(5)[0])[:2] != (1, 1):
        raise ArithmeticError('split counterexample missing')
    try:
        predicted(5, (0, 0))
    except ValueError:
        pass
    else:
        raise ArithmeticError('zero accepted')
    print(json.dumps({'curve': 'y^2+y=x^3+x^2 over F2',
                      'tested_primes': len(rows), 'rows': rows},
                     separators=(',', ':')))


if __name__ == '__main__':
    main()
