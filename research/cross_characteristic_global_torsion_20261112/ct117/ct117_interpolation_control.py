#!/usr/bin/env python3
"""CT117 negative control: a scalar-blind online interpolant compiled from a DLP table.

This does NOT evaluate an algebraic transfer at another prime. Integer bit masks
are a non-homomorphic encoding convention for F_(2^20) elements.
"""
import json
from fractions import Fraction
from math import isqrt, lcm

P = 59
G = (19, 22)
L = 11
BINARY_MOD = 0x130027
BIT_MASK = (1 << 20) - 1

def on_curve(a):
    if a is None:
        return True
    x, y = a
    return (y*y + y - x*x*x + x*x + 10*x + 20) % P == 0

def neg(a):
    return None if a is None else (a[0], (-a[1]-1) % P)

def plus(a, b):
    if a is None:
        return b
    if b is None:
        return a
    x, y = a
    z, w = b
    if x == z:
        if (y+w+1) % P == 0:
            return None
        if y != w:
            raise ArithmeticError('duplicate x and neither opposite nor equal')
        slope = (3*x*x-2*x-10) * pow(2*y+1, -1, P) % P
    else:
        slope = (w-y) * pow(z-x, -1, P) % P
    xx = (slope*slope + 1-x-z) % P
    yy = (-1-y+slope*(x-xx)) % P
    return xx, yy

def times(n, a):
    out = None
    while n > 0:
        if n & 1:
            out = plus(out, a)
        a = plus(a, a)
        n >>= 1
    return out

def fmul(a, b):
    out = 0
    for _ in range(20):
        if b & 1:
            out ^= a
        b >>= 1
        a <<= 1
        if a & (1 << 20):
            a ^= BINARY_MOD
    if b:
        raise ArithmeticError('oversized field factor')
    return out & BIT_MASK

def frob4(a):
    for _ in range(4):
        a = fmul(a, a)
    return a

def interpolate_lagrange(pairs):
    coeff = [Fraction(0)] * len(pairs)
    for i, (x, value) in enumerate(pairs):
        basis = [Fraction(value)]
        denom = 1
        for j, (other, _) in enumerate(pairs):
            if i == j:
                continue
            grown = [Fraction(0)] * (len(basis)+1)
            for k, c in enumerate(basis):
                grown[k] -= other*c
                grown[k+1] += c
            basis = grown
            denom *= x-other
        for k, c in enumerate(basis):
            coeff[k] += c/denom
    return coeff

def eval_horner(coeff, x):
    value = Fraction(0)
    for c in reversed(coeff):
        value = value*x+c
    return value

def bsgs(q):
    n = isqrt(L)+1
    babies, r = {}, None
    cost = 0
    for j in range(n):
        babies[r] = j
        r = plus(r, G)
        cost += 1
    shift = neg(r)
    t = q
    for i in range(n+1):
        if t in babies:
            return (i*n+babies[t]) % L, cost
        t = plus(t, shift)
        cost += 1
    raise ArithmeticError('BSGS failed')

def main():
    if not on_curve(G) or times(L, G) is not None:
        raise ArithmeticError('invalid base point')
    orbit = [2]
    for _ in range(4):
        orbit.append(frob4(orbit[-1]))
    if len(set(orbit)) != 5 or frob4(orbit[-1]) != orbit[0]:
        raise ArithmeticError('incorrect binary Frobenius orbit')
    exponents = [pow(7, j, L) for j in range(5)]
    if exponents != [1, 7, 5, 2, 3]:
        raise ArithmeticError('wrong Frobenius exponent order')
    # OFFLINE: enumerate five projective DLP classes by elliptic addition.
    pairs, r, source_adds = [], None, 0
    for k in range(1, 6):
        if k == 1:
            r = G
        else:
            r = plus(r, G)
            source_adds += 1
        j = next(i for i, e in enumerate(exponents) if k in (e, (-e) % L))
        pairs.append((r[0], orbit[j]))
    if len({x for x, _ in pairs}) != 5:
        raise ArithmeticError('duplicated interpolation abscissa')
    polynomial = interpolate_lagrange(pairs)
    denominator = lcm(*(c.denominator for c in polynomial))
    numerator = [int(c*denominator) for c in polynomial]
    if denominator != 1162512 or numerator != [1822729272584,-280573814794,15582488805,-368168372,3131989]:
        raise ArithmeticError('interpolation certificate changed')
    for x, target in pairs:
        if eval_horner(polynomial, x) != target:
            raise ArithmeticError('interpolation check')
    if (denominator & -denominator).bit_length()-1 != 4:
        raise ArithmeticError('unexpected binary denominator valuation')

    # ONLINE: the compiled polynomial is the only source-x-dependent mapping.
    def online(q):
        if q is None or not isinstance(q, tuple) or len(q) != 2 or not all(isinstance(v, int) and 0 <= v < P for v in q):
            raise ValueError('bad encoded input')
        if not on_curve(q) or times(L, q) is not None:
            raise ValueError('point not in source order-11 subgroup')
        y = eval_horner(polynomial, q[0])
        if y.denominator != 1 or not (0 <= y.numerator <= BIT_MASK):
            raise ValueError('interpolant not an admitted bit mask')
        hits = [i for i, z in enumerate(orbit) if z == y.numerator]
        if len(hits) != 1:
            raise ValueError('unrecognized binary orbit')
        a = exponents[hits[0]]
        k = a if times(a, G) == q else (-a) % L
        if times(k, G) != q:
            raise ArithmeticError('invalid source sign witness')
        return y.numerator, hits[0], k

    public = []
    bsgs_cost = []
    for hidden in range(1, L):
        q = times(hidden, G)
        b, j, recovered = online(q)
        if recovered != hidden:
            raise ArithmeticError('toy recovery mismatch')
        known, cost = bsgs(q)
        if known != hidden:
            raise ArithmeticError('source BSGS mismatch')
        bsgs_cost.append(cost)
        public.append([q[0], q[1], b, j, recovered])
    rejected = []
    for name, bad in [('off_curve', (19,23)), ('outside_subgroup',(4,18)), ('identity',None), ('wrong_type',(19,22,0))]:
        try:
            online(bad)
        except ValueError:
            rejected.append(name)
        else:
            raise ArithmeticError('invalid target accepted: '+name)
    altered = polynomial.copy()
    altered[0] += 1
    if eval_horner(altered, G[0]) == orbit[0]:
        raise ArithmeticError('mutation not detected')
    if polynomial == [Fraction(0), Fraction(1), *[Fraction(0)]*3]:
        raise ArithmeticError('unexpected globally matching coordinate polynomial')
    # Since degree(psi_11)=60 and degree(f(X)-X)<=4, f(alpha) != alpha.
    print(json.dumps({
        'case':'ct117-scalar-blind-interpolation-negative-control',
        'pcc':'unverified',
        'source_x_target_bitmask_pairs':[[x,y] for x,y in pairs],
        'binary_orbit':orbit,
        'rational_polynomial_common_denominator':denominator,
        'rational_polynomial_numerator_ascending':numerator,
        'denominator_v2':4,
        'offline_source_class_additions':source_adds,
        'online_source_class_lookup':False,
        'online_cold_interpolation':False,
        'online_public_cases':public,
        'all_public_logs_recovered':len(public),
        'rejected_controls':rejected,
        'mutation_detected':True,
        'source_bsgs_costs':bsgs_cost,
        'global_x_lift_certificate':False,
        'binary_integer_reduction_is_field_bitmask':False,
        'target_dlp_shortcut':False,
        'complete_cold_cost_available':False
    },sort_keys=True,separators=(',',':')))

if __name__ == '__main__':
    main()
