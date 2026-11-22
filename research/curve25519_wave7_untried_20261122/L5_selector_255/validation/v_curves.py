#!/usr/bin/env python3
"""Validator's independent curve-fact check (no import from the lead's scripts).
Miniatures: #E by counting squares (table of y^2), A^2-4 non-square (only one 2-torsion point -> cyclic 2-part),
r and twist cofactor primality (deterministic Miller-Rabin, bases = first 13 primes, valid < 3.3e24), and a
point of exact order 8r found with my own affine Montgomery arithmetic.
Curve25519: MR with 40 bases from my own seed, a point of exact order 8r by affine (u, v) arithmetic, twist 4r'."""
import json, random, sys
import numpy as np

def mr(n, bases):
    if n < 2: return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47):
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in bases:
        a %= n
        if a in (0, 1, n - 1): continue
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else:
            return False
    return True

DET = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]

def sqrt_p(a, p):
    """Tonelli-Shanks (general p)."""
    a %= p
    if a == 0: return 0
    if pow(a, (p - 1) // 2, p) != 1: return None
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, x = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1: tt = tt * tt % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, x = i, b * b % p, t * b * b % p, x * b % p
    return x

def mont_ops(p, A):
    """Affine Montgomery-form (B = 1) group law: v^2 = u^3 + A u^2 + u."""
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        u1, v1 = P; u2, v2 = Q
        if u1 == u2:
            if (v1 + v2) % p == 0: return None
            lam = (3 * u1 * u1 + 2 * A * u1 + 1) * pow(2 * v1 % p, -1, p) % p
        else:
            lam = (v2 - v1) * pow((u2 - u1) % p, -1, p) % p
        u3 = (lam * lam - A - u1 - u2) % p
        return (u3, (lam * (u1 - u3) - v1) % p)
    def mul(k, P):
        # left-to-right double-and-add
        R = None
        for bit in bin(k)[2:]:
            R = add(R, R)
            if bit == '1': R = add(R, P)
        return R
    return add, mul

def count_points_squares(p, A):
    x = np.arange(p, dtype=np.int64)
    sq = np.zeros(p, dtype=np.int8)
    sq[(x * x) % p] = 1          # sq[v] = 1 iff v is a square (incl. 0)
    f = ((x * x % p) * x + A * (x * x % p) + x) % p
    nz = f != 0
    # each x with f(x) a nonzero square gives 2 points, f(x) = 0 gives 1, plus infinity
    return int(1 + (f == 0).sum() + 2 * (sq[f] * nz).sum())

def mini(name, p, A):
    N = count_points_squares(p, A)
    r = N // 8
    tw = 2 * p + 2 - N
    out = dict(curve=name, p=p, A=A, p_prime=mr(p, DET), p_mod_8=p % 8, count_E=N, div8=N % 8 == 0, r=r,
               r_prime=mr(r, DET), twist=tw, twist_div4=tw % 4 == 0, twist_cof=tw // 4, twist_cof_prime=mr(tw // 4, DET),
               A2m4_nonsquare=pow((A * A - 4) % p, (p - 1) // 2, p) == p - 1,
               hasse=(p + 1 - N) ** 2 <= 4 * p)
    add, mul = mont_ops(p, A)
    rng = random.Random(987654321)
    while True:
        u = rng.randrange(1, p)
        v = sqrt_p((u * u * u + A * u * u + u) % p, p)
        if v is None or v == 0: continue
        P = (u, v)
        if mul(N, P) is not None: raise SystemExit('order check failed')
        if mul(N // 2, P) is not None and mul(N // r, P) is not None:
            out['generator_found'] = True; out['cyclic_Z_8r'] = True; out['gen'] = [u, v]
            break
    return out

def real():
    p = 2 ** 255 - 19; A = 486662
    r = 2 ** 252 + 27742317777372353535851937790883648493
    rng = random.Random(20261122)
    bases = [rng.randrange(2, p - 2) for _ in range(40)] + DET
    out = dict(curve='curve25519', p_prime=mr(p, bases), r_prime=mr(r, bases), p_mod_8=p % 8,
               r_bits=r.bit_length(), r_popcount=bin(r).count('1'),
               A2m4_nonsquare=pow((A * A - 4) % p, (p - 1) // 2, p) == p - 1)
    add, mul = mont_ops(p, A)
    while True:
        u = rng.randrange(2, p)
        v = sqrt_p((u * u * u + A * u * u + u) % p, p)
        if v: break
    P = (u, v)
    N = 8 * r
    out.update(NP_is_O=mul(N, P) is None, half_not_O=mul(4 * r, P) is not None, eightP_not_O=mul(8, P) is not None)
    out['order_is_8r'] = out['NP_is_O'] and out['half_not_O'] and out['eightP_not_O']
    out['hasse'] = (p + 1 - N) ** 2 <= 4 * p
    out['unique_multiple_in_hasse'] = 8 * r > 4 * int((p ** 0.5)) + 4
    tw = 2 * p + 2 - N
    out.update(twist_div4=tw % 4 == 0, twist_cof_prime=mr(tw // 4, bases), twist_cof=str(tw // 4))
    import math
    out['log2_r'] = math.log2(r)
    out['log2_rho_sqrt_pi_r_over_4'] = 0.5 * math.log2(math.pi * r / 4)
    return out

if __name__ == '__main__':
    res = [mini('k20', 1048573, 926), mini('k24f', 16777141, 7842), mini('k24r', 16777213, 318), real()]
    print(json.dumps(res, indent=1))
