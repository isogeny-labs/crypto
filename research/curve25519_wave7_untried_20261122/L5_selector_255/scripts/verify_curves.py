#!/usr/bin/env python3
"""Verify every curve fact L5 uses, by computation. Writes JSON to stdout.
Miniatures: exact #E by a vectorised Legendre sum over all x (numpy), twist order 2p+2-#E,
primality of p, r, r' (deterministic Miller-Rabin for n < 3.3e24), A^2-4 non-square, p mod 8.
Real Curve25519: p, r, r' probable primes (Miller-Rabin, 64 derived bases), #E = 8r from a point of
order exactly 8r (> 4 sqrt p, so unique in the Hasse interval), twist order 2p+2-8r = 4r'."""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from l5common import CURVES, derived_seed
import random

def is_prime(n, rounds=None, seed=0):
    if n < 2: return False
    small = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]
    for q in small:
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    bases = small if rounds is None else [random.Random(seed).randrange(2, n - 1) for _ in range(rounds)] + small
    for a in bases:
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else:
            return False
    return True

def vec_legendre(v, p):
    e = (p - 1) // 2; res = np.ones_like(v); b = v.copy()
    while e:
        if e & 1: res = res * b % p
        b = b * b % p; e >>= 1
    res[v == 0] = 0
    res[res == p - 1] = -1
    return res

def mini(name):
    k, p, A, n = CURVES[name]
    x = np.arange(p, dtype=np.int64)
    f = (x * x % p * x % p + A * (x * x % p) % p + x) % p
    chi = vec_legendre(f, p)
    N = p + 1 + int(chi.sum())
    tw = 2 * p + 2 - N
    out = dict(curve=name, k=k, p=p, A=A, p_prime=is_prime(p), p_mod_8=p % 8,
               A2m4_nonsquare=pow((A * A - 4) % p, (p - 1) // 2, p) == p - 1,
               count_E=N, count_E_matches_table=(N == n), cofactor_8=(N % 8 == 0), r=N // 8,
               r_prime=is_prime(N // 8), twist=tw, twist_div4=(tw % 4 == 0), twist_r=tw // 4,
               twist_r_prime=is_prime(tw // 4), onCurve_fraction=float(((chi == 1).sum()) / p))
    return out

def real25519():
    p = 2 ** 255 - 19; A = 486662
    r = 2 ** 252 + 27742317777372353535851937790883648493
    out = dict(curve='curve25519', p_prime=is_prime(p, 64, derived_seed('mr', 'p')), p_mod_8=p % 8,
               r_prime=is_prime(r, 64, derived_seed('mr', 'r')),
               A2m4_nonsquare=pow((A * A - 4) % p, (p - 1) // 2, p) == p - 1)
    # x-only Montgomery ladder; order check by projective (X:Z)
    a24 = (A - 2) // 4   # RFC 7748: z2 = E (AA + a24 E), a24 = (A - 2)/4
    def ladder(k, u):
        X1 = u; X2, Z2, X3, Z3 = 1, 0, u, 1
        for i in reversed(range(k.bit_length())):
            b = (k >> i) & 1
            if b: X2, X3, Z2, Z3 = X3, X2, Z3, Z2
            A_ = X2 + Z2; AA = A_ * A_ % p; B_ = X2 - Z2; BB = B_ * B_ % p; E = AA - BB
            C = X3 + Z3; D = X3 - Z3; DA = D * A_ % p; CB = C * B_ % p
            X3 = (DA + CB) ** 2 % p; Z3 = X1 * (DA - CB) ** 2 % p
            X2 = AA * BB % p; Z2 = E * (AA + a24 * E) % p
            if b: X2, X3, Z2, Z3 = X3, X2, Z3, Z2
        return X2, Z2
    rng = random.Random(derived_seed('real', 'ordercheck'))
    while True:
        u = rng.randrange(2, p)
        if pow((u * u * u + A * u * u + u) % p, (p - 1) // 2, p) == 1: break
    _, z8r = ladder(8 * r, u); _, z4r = ladder(4 * r, u); _, z8 = ladder(8, u)
    out.update(point_u=u, eight_r_P_is_O=(z8r == 0), four_r_P_not_O=(z4r != 0), eight_P_not_O=(z8 != 0))
    N = 8 * r; hasse = (p + 1 - N) ** 2 <= 4 * p
    tw = 2 * p + 2 - N
    out.update(count_E=N, in_hasse=hasse, order_8r_exceeds_4sqrtp=(N * N > 16 * p), twist=tw,
               twist_div4=(tw % 4 == 0), twist_r_prime=is_prime(tw // 4, 64, derived_seed('mr', 'rt')))
    return out

if __name__ == '__main__':
    names = sys.argv[1:] or list(CURVES)
    res = [mini(nm) for nm in names if nm in CURVES]
    if 'nocurve25519' not in sys.argv: res.append(real25519())
    print(json.dumps(res, indent=1))
