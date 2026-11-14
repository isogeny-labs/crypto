#!/usr/bin/env python3
"""CT83: exact local-to-global precision obstruction and conservative norm certificate.

Independent of CT80/82 code; no scalar enumeration, no cloud dependencies.
"""
import json
from math import comb
p, d, n = 37, 24, 5

def mul(a, b):
    z = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            z[i + j] += x * y
    return z

def power(a, k):
    z = [1]
    for _ in range(k):
        z = mul(z, a)
    return z

def minus(a, b):
    return [(a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)
            for i in range(max(len(a), len(b)))]

def eval_poly(a, x, modulus=None):
    z = 0
    for c in reversed(a):
        z = z * x + c
        if modulus is not None:
            z %= modulus
    return z

D = [-79, -40, -4, 4]
P3 = [-21, -237, -60, -4, 3]
F4 = [-5821, -1496, -210, -790, -100, -4, 2]
F = minus(mul(minus(mul(power(D, 2), F4), power(P3, 3)), power(P3, 3)),
          mul(power(D, 2), power(F4, 3)))
if len(F) != d + 1 or F[d] != 7:
    raise ArithmeticError('7-division polynomial invalid')
C = max(map(abs, F))
if C != 2207441511783053:
    raise ArithmeticError('height mismatch')
F9 = eval_poly(F, 9)
if F9 % p != 0 or F9 % p**2 == 0:
    raise ArithmeticError('source root valuation mismatch')

# Distinct global algebraic elements zero and (alpha-9)^5 agree modulo 37^5
# at alpha -> 9 mod 37. The latter is NOT a torsion-point coordinate.
g = power([-9, 1], n)
H = max(map(abs, g))
if H != 59049 or len(g) != 6:
    raise ArithmeticError('kernel height')
for level in range(1, n + 1):
    M = p**level
    # For any alpha ≡ 9 mod 37, g(alpha) is divisible by 37^5.
    if eval_poly(g, 9 + p * 1234567, M) != 0:
        raise ArithmeticError('local ambiguity falsified')

# GF(2^12), CT80 alpha=bit 1 and first irreducible f12.
f12 = (1 << 12) | (1 << 9) | (1 << 5) | (1 << 2) | 1

def mul_gf(a, b):
    z = 0
    for _ in range(12):
        if b & 1:
            z ^= a
        b >>= 1
        a <<= 1
        if a & (1 << 12):
            a ^= f12
    if b:
        raise ArithmeticError('GF multiplier unexpectedly long')
    return z

def pow_gf(a, exponent):
    z = 1
    while exponent:
        if exponent & 1:
            z = mul_gf(z, a)
        a = mul_gf(a, a)
        exponent >>= 1
    return z

# x(T) = alpha => alpha mod 2 encoded as 2, and -9 = +1 mod 2.
binary = pow_gf(2 ^ 1, n)
if binary == 0:
    raise ArithmeticError('bad binary local-difference witness')

# Necessary pigeonhole bound for injectivity on ALL 24-coefficient integer boxes
# [-H,H]^24, independent of elliptic-curve torsion constraints.
box = (2 * H + 1)**d
necessary = 0
while p**necessary < box:
    necessary += 1
if p**(necessary - 1) >= box or p**necessary < box:
    raise ArithmeticError('pigeonhole precision')

# Sufficient via resultant and integral Sylvester/Hadamard row 1-norm:
# For a difference g of coefficients bounded by 2H and degree <=23,
# |Res(F,g)| <= ((d+1)C)^(d-1) * (2 d H)^d. If 37^N > bound and
# g(alpha) == 0 mod 37^N at the chosen simple degree-one place,
# then Res(F,g)=0, hence g=0 by irreducibility of F over Q (CT80).
resultant_bound = ((d + 1) * C)**(d - 1) * (2 * d * H)**d
sufficient = 0
while p**sufficient <= resultant_bound:
    sufficient += 1
if p**sufficient <= resultant_bound:
    raise ArithmeticError('norm bound failed')
if not (n < necessary <= sufficient):
    raise ArithmeticError('precision inequalities')
print(json.dumps({
    'binary_witness': binary,
    'coefficient_height': H,
    'degree': d,
    'experiment': 'ct83-fixed-prime-precision-obstruction',
    'f9_valuation': 1,
    'information_required_digits': necessary,
    'local_collision_digits': n,
    'local_modulus': p**n,
    'p': p,
    'poly_height': C,
    'resultant_sufficient_digits': sufficient,
}, sort_keys=True, separators=(',', ':')))
