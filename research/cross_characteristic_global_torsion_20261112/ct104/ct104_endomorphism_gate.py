#!/usr/bin/env python3
"""CT104: non-CM endomorphism rigidity and degree leakage for 11a1.
No global arbitrary-Q reconstruction and no timing claim.
"""
import json
from math import gcd


def check(cond, message):
    if not cond:
        raise ArithmeticError(message)


def norm(f):
    while len(f) > 1 and f[-1] == 0:
        f.pop()
    return f


def add(a, b):
    return norm([(a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)
                 for i in range(max(len(a), len(b)))])


def neg(a):
    return [-x for x in a]


def sub(a, b):
    return add(a, neg(b))


def mul(a, b):
    c = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            c[i + j] += x * y
    return norm(c)


def power(a, n):
    out = [1]
    while n:
        if n & 1:
            out = mul(out, a)
        a = mul(a, a)
        n >>= 1
    return out


def modpoly(a, p):
    return norm([x % p for x in a])


def poly_rem(a, b, p):
    a, b = modpoly(a, p), modpoly(b, p)
    check(b != [0], "polynomial remainder by zero")
    while a != [0] and len(a) >= len(b):
        shift = len(a) - len(b)
        z = a[-1] * pow(b[-1], -1, p) % p
        for i, v in enumerate(b):
            a[shift + i] = (a[shift + i] - z * v) % p
        norm(a)
    return a


def gcd_degree_mod(a, b, p):
    a, b = modpoly(a, p), modpoly(b, p)
    while b != [0]:
        a, b = b, poly_rem(a, b, p)
    return len(a) - 1


def poly_eval_mod(a, x, p):
    z = 0
    for v in reversed(a):
        z = (z * x + v) % p
    return z


# General Weierstrass data for y^2+y=x^3-x^2-10x-20.
a1, a2, a3, a4, a6 = 0, -1, 1, -10, -20
b2 = a1*a1 + 4*a2
b4 = a1*a3 + 2*a4
b6 = a3*a3 + 4*a6
b8 = a1*a1*a6 + 4*a2*a6 - a1*a3*a4 + a2*a3*a3 - a4*a4
c4 = b2*b2 - 24*b4
discriminant = -b2*b2*b8 - 8*b4*b4*b4 - 27*b6*b6 + 9*b2*b4*b6
check((b2, b4, b6, b8, c4, discriminant) == (-4, -20, -79, -21, 496, -161051), "Weierstrass invariants")
j_num, j_den = c4**3, discriminant
check(j_num == 122023936 and j_den == -161051 and gcd(j_num, -j_den) == 1, "j is nonintegral rational")

# Odd/even division-polynomial recurrences (same 11a1 fixture, computed freshly).
D = [b6, 2*b4, b2, 4]
check(D == [-79, -40, -4, 4], "psi2 squared")
P3 = [b8, 3*b6, 3*b4, b2, 3]
check(P3 == [-21, -237, -60, -4, 3], "psi3")
F4 = [b4*b8 - b6*b6, b2*b8 - b4*b6, 10*b8, 10*b6, 5*b4, b2, 2]
check(F4 == [-5821, -1496, -210, -790, -100, -4, 2], "fourth division polynomial")
F5 = sub(mul(power(D, 2), F4), power(P3, 3))
F6 = mul(P3, sub(F5, power(F4, 2)))  # psi6/psi2
psq = {1: [1], 2: D, 3: power(P3, 2), 4: mul(D, power(F4, 2)), 5: power(F5, 2)}
phi = {
    1: [0, 1],
    2: sub([0]+psq[2], P3),
    3: sub([0]+psq[3], mul(D, F4)),
    4: sub([0]+psq[4], mul(F5, P3)),
    5: sub([0]+psq[5], mul(D, mul(F6, F4)))
}
map_degrees = [[n, len(phi[n])-1, len(psq[n])-1] for n in range(1, 6)]
check(map_degrees == [[n, n*n, n*n-1] for n in range(1, 6)], "multiplication-map degrees")
for n in range(1, 6):
    check(phi[n][-1] == 1 and psq[n][-1] == n*n, "leading coefficients")
    for p in (101, 103):
        check(gcd_degree_mod(phi[n], psq[n], p) == 0, "division rational map not coprime")

# Verify that no two canonical multiplication rational functions become equal
# merely after evaluation at the degree-60 global point-field generator.
cross_max = 0
for m in range(1, 6):
    for n in range(m+1, 6):
        lhs = mul(phi[m], psq[n])
        rhs = mul(phi[n], psq[m])
        difference = sub(lhs, rhs)
        check(difference != [0], "distinct scalar maps unexpectedly equal")
        cross_max = max(cross_max, len(difference)-1)
check(cross_max == 40 and cross_max < 60, "global x-field noncollision gate")

p = 59
P = (19, 22)


def curve_check(X):
    if X is None:
        return True
    x, y = X
    return (y*y + y - (x*x*x-x*x-10*x-20)) % p == 0


def point_add(P, Q):
    if P is None:
        return Q
    if Q is None:
        return P
    x, y = P
    xx, yy = Q
    if x == xx and (y+yy+1) % p == 0:
        return None
    if P == Q:
        lam = (3*x*x - 2*x - 10) * pow(2*y+1, -1, p) % p
    else:
        lam = (yy-y) * pow(xx-x, -1, p) % p
    x3 = (lam*lam + 1 - x - xx) % p
    y3 = (-lam*(x3-x)-y-1) % p
    check(curve_check((x3, y3)), "group-law output off curve")
    return x3, y3


check(curve_check(P), "generator off curve")
points = [None]
for i in range(11):
    points.append(point_add(points[-1], P))
check(points[-1] is None, "generator order not 11")
points = points[:-1]
check(len(set(points)) == 11, "11 distinct subgroup elements")
for i in range(11):
    for j in range(11):
        check(point_add(points[i], points[j]) == points[(i+j)%11], "wrong public group law")
source_x = [points[n][0] for n in range(1, 6)]
check(source_x == [19,20,43,46,22], "independent source scalar x")
for n in range(1, 6):
    x = poly_eval_mod(phi[n], 19, p) * pow(poly_eval_mod(psq[n],19,p),-1,p) % p
    check(x == source_x[n-1], "division rational function / source group mismatch")

print(json.dumps({
    "case": "ct104-noncm-endomorphism-gate",
    "curve": "11a1", "source_modulus": p,
    "curve_discriminant": discriminant,
    "j_num": -j_num, "j_den": -j_den, "j_integral": False,
    "global_x_degree": 60, "global_oriented_degree": 120,
    "source_subgroup_order": 11,
    "source_x_multiples_1_to_5": source_x,
    "source_group_addition_pairs": 121,
    "scalar_map_degrees": map_degrees,
    "scalar_maps_coprime_mod_primes": [101,103],
    "max_cross_difference_degree": cross_max,
    "fixed_target_binary_degree": 40,
    "online_global_lift_evaluated": False,
    "complete_cost_measured": False,
}, sort_keys=True,separators=(',',':')))
