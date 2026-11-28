#!/usr/bin/env python3
"""Independent big-integer verification of the Prime31 S3 projective identity.

This is a group-law / polynomial identity control, not root extraction.
Uses Python arithmetic separate from original Rust implementation.
"""
import random

P = 2_147_483_647
A = 847_514_846
B = 2_087_444_724
G = (625_037_715, 2_100_627_251)
N = 2_147_446_831

def ec_add(a, b):
    if a is None: return b
    if b is None: return a
    x, y = a
    u, v = b
    if x == u and (y + v) % P == 0: return None
    if x == u:
        slope = ((3*x*x + A) * pow(2*y, -1, P)) % P
    else:
        slope = ((v-y)*pow((u-x) % P, -1, P)) % P
    z = (slope*slope-x-u) % P
    return (z, (slope*(x-z)-y) % P)

def coeff(x, y):
    return ((x-y)**2 % P,
            (-2*((x+y)*(x*y + A) + 2*B)) % P,
            ((x*y-A)**2 - 4*B*(x+y)) % P)

def affine(c, x):
    return (c[0]*x*x + c[1]*x + c[2]) % P

def homogeneous(c, X, Z):
    return (c[0]*X*X + c[1]*X*Z*Z + c[2]*pow(Z, 4, P)) % P

def main():
    assert (G[1]*G[1] - (G[0]**3+A*G[0]+B)) % P == 0
    points = []
    u = None
    for _ in range(75):
        u = ec_add(u, G)
        assert u is not None
        points.append(u)
    checks = 0
    for a in points:
        for b in points:
            c = coeff(a[0], b[0])
            v = ec_add(a, b)
            if v:
                assert affine(c, v[0]) == 0
                checks += 1
    rng = random.Random(0x6420261122)
    pchecks = 0
    for _ in range(4096):
        a = points[rng.randrange(len(points))]
        b = points[rng.randrange(len(points))]
        c = coeff(a[0], b[0])
        q = points[rng.randrange(len(points))]
        z = rng.randrange(1, P)
        X = q[0] * z*z % P
        assert homogeneous(c, X, z) == affine(c, q[0])*pow(z, 4, P) % P
        pchecks += 1
    assert ec_add(G, (G[0], (-G[1]) % P)) is None
    assert ec_add(None, G) == G
    print(f"run64 independent_python S3_sum=PASS {checks}/5625 projective=PASS {pchecks}/4096 exceptional=PASS")

if __name__ == "__main__":
    main()
