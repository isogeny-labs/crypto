#!/usr/bin/env python3
"""CT76 actual 11a1 binary reduction: direct binary-field counts and Frobenius gates.

No external modules. Every assertion is an explicit check, including under -O.
The polynomial quotient is accepted only if every nonzero element is invertible.
"""
import json

ELLS = (3, 5, 7, 11, 13)
MAX_F = 128

def check(test, why):
    if not test:
        raise ArithmeticError(why)

def product(a, b, modulus, f):
    mask = (1 << f) - 1
    result = 0
    while b:
        if b & 1:
            result ^= a
        b >>= 1
        a <<= 1
        if a & (1 << f):
            a ^= modulus
        a &= mask
    return result

def power(a, k, modulus, f):
    ans = 1
    while k:
        if k & 1:
            ans = product(ans, a, modulus, f)
        a = product(a, a, modulus, f)
        k >>= 1
    return ans

def find_field_polynomial(f):
    # All nonzero elements of the finite quotient are invertible iff
    # the polynomial modulus is irreducible.
    for m in range((1 << f) | 1, 1 << (f + 1), 2):
        if all(power(a, (1 << f) - 1, m, f) == 1
               for a in range(1, 1 << f)):
            return m
    raise ArithmeticError('no irreducible polynomial for f=' + str(f))

def count_points(f):
    modulus = find_field_polynomial(f)
    size = 1 << f
    ordinates = [0] * size
    for y in range(size):
        ordinates[product(y, y, modulus, f) ^ y] += 1
    total = 1  # identity at infinity
    for x in range(size):
        x2 = product(x, x, modulus, f)
        rhs = product(x2, x, modulus, f) ^ x2
        total += ordinates[rhs]
    return total

def count_trace(f):
    # E/F2 : y^2+y=x^3+x^2, #E(F2)=5, t=-2, q=2.
    t0, t1 = 2, -2
    for _ in range(1, f):
        t0, t1 = t1, -2*t1-2*t0
    return (1 << f) + 1 - t1

def mul(A, B, l):
    return ((A[0]*B[0]+A[1]*B[2]) % l,
            (A[0]*B[1]+A[1]*B[3]) % l,
            (A[2]*B[0]+A[3]*B[2]) % l,
            (A[2]*B[1]+A[3]*B[3]) % l)

def gate(l):
    # Frobenius companion of X^2+2X+2, valid for odd l (disc=-4).
    F = (0, (-2) % l, 1, (-2) % l)
    M = (1, 0, 0, 1)
    some = full = None
    for f in range(1, MAX_F + 1):
        M = mul(M, F, l)
        det = ((M[0]-1)*(M[3]-1)-M[1]*M[2]) % l
        check(det == count_trace(f) % l, 'matrix/trace mismatch')
        if det == 0 and some is None:
            some = f
        if M == (1, 0, 0, 1):
            full = f
            break
    check(some is not None and full is not None, 'unresolved order')
    check(some <= full and full % some == 0, 'extension incompatibility')
    return some, full

def main():
    for f in range(1, 9):
        counted = count_points(f)  # complete enumeration, not trace-only
        check(counted == count_trace(f), 'direct binary count mismatch')
        print(json.dumps({'f': f, 'points': counted,
                          'type': 'point_count'}, sort_keys=True, separators=(',', ':')))
    for ell in ELLS:
        some, full = gate(ell)
        check(count_trace(some) % ell == 0, 'missing torsion')
        check(all(count_trace(f) % ell != 0 for f in range(1, some)),
              'nonminimal field')
        print(json.dumps({'ell': ell, 'first_some': some,
                          'first_full': full, 'type': 'frobenius'},
                         sort_keys=True, separators=(',', ':')))
    check([gate(e) for e in ELLS] ==
          [(8, 8), (1, 4), (24, 24), (40, 40), (6, 12)],
          'unexpected fixed-model field gates')

if __name__ == '__main__':
    main()
