#!/usr/bin/env python3
"""Validator: independent curve facts for the L1 curves (does not import producer code).

Methods (different from the producer's numpy squares-table count and trial division):
  (a) #E = p + 1 + sum_x chi(x^3 + A x^2 + x), chi by Euler's criterion computed with a
      vectorised square-and-multiply in numpy (chunked);
  (b) group-theoretic proof: an x-only Montgomery-ladder point of order exactly 8r on E and a
      twist point whose order is divisible by r'; Hasse interval width 4 sqrt(p) < r, r' makes the
      multiple unique;
  (c) primality by deterministic Miller-Rabin (bases 2..37, valid far beyond 2^64).
"""
import json, math, sys, hashlib
import numpy as np

CURVES = {16: (65437, 1058), 18: (262133, 78), 20: (1048573, 926),
          22: (4194301, 43186), 24: (16777141, 7842)}


def mr_prime(n):
    if n < 2:
        return False
    small = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    for q in small:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
    for a in small:
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def euler_charsum(p, A, chunk=1 << 21):
    """sum_x chi(f(x)), number of x with f(x) = 0, and #x with f(x) a nonzero square."""
    e = (p - 1) // 2
    total = 0; zeros = 0; squares = 0
    for lo in range(0, p, chunk):
        x = np.arange(lo, min(p, lo + chunk), dtype=np.int64)
        f = (x * x % p * x % p + A * (x * x % p) % p + x) % p
        res = np.ones_like(f); b = f.copy(); ee = e
        while ee:
            if ee & 1:
                res = res * b % p
            b = b * b % p; ee >>= 1
        z = f == 0
        zeros += int(z.sum())
        sq = (res == 1) & ~z
        nsq = res == p - 1
        assert int((sq | nsq | z).sum()) == len(f)
        squares += int(sq.sum())
        total += int(sq.sum()) - int(nsq.sum())
    return total, zeros, squares


# ---- x-only Montgomery ladder over F_p (projective X:Z; Z = 0 is the point at infinity) ----
def ladder(k, x, p, a24):
    if k == 0:
        return (1, 0)
    X1 = x
    R0 = (1, 0); R1 = (x, 1)
    for bit in bin(k)[2:]:
        (X2, Z2), (X3, Z3) = R0, R1
        if bit == '1':
            (X2, Z2), (X3, Z3) = (X3, Z3), (X2, Z2)
        # differential add and double: (X2:Z2) doubled, (X2:Z2)+(X3:Z3) with difference X1
        A_ = (X2 + Z2) % p; AA = A_ * A_ % p; B_ = (X2 - Z2) % p; BB = B_ * B_ % p; E_ = (AA - BB) % p
        C_ = (X3 + Z3) % p; D_ = (X3 - Z3) % p; DA = D_ * A_ % p; CB = C_ * B_ % p
        X5 = (DA + CB) % p; X5 = X5 * X5 % p; Z5 = (DA - CB) % p; Z5 = X1 * (Z5 * Z5 % p) % p
        X4 = AA * BB % p; Z4 = E_ * (BB + a24 * E_ % p) % p
        if bit == '1':
            R0, R1 = (X5, Z5), (X4, Z4)
        else:
            R0, R1 = (X4, Z4), (X5, Z5)
    return R0


def is_inf(P, p):
    return P[1] % p == 0


def facts(k):
    p, A = CURVES[k]
    s, zeros, squares = euler_charsum(p, A)
    nE = p + 1 + s
    nT = p + 1 - s
    assert nE == 1 + zeros + 2 * squares
    a24 = (A + 2) * pow(4, -1, p) % p
    # order-8r point: need r from nE; checked to be prime below
    out = dict(k=k, p=p, A=A, order_euler=nE, twist_order_euler=nT, zeros_of_f=zeros)
    assert nE % 8 == 0
    r = nE // 8
    out['r'] = r
    out['r_prime_MR'] = mr_prime(r)
    out['r_gt_2^(k-3)'] = r > 2 ** (k - 3)
    out['twist_div4'] = nT % 4 == 0
    rt = nT // 4
    out['twist_r'] = rt
    out['twist_r_prime_MR'] = mr_prime(rt)
    out['p_mod_8'] = p % 8
    out['A2m4_nonsquare'] = pow((A * A - 4) % p, (p - 1) // 2, p) == p - 1
    out['hasse_width_lt_r'] = 4 * math.isqrt(p) + 4 < r and 4 * math.isqrt(p) + 4 < rt
    # x on E: f(x) nonzero square; on twist: f(x) non-square.  deterministic search from x = 2
    def chi(x):
        f = (x * x * x + A * x * x + x) % p
        return 0 if f == 0 else (1 if pow(f, (p - 1) // 2, p) == 1 else -1)
    found = None
    x = 2
    while found is None:
        if chi(x) == 1:
            o8r = is_inf(ladder(8 * r, x, p, a24), p)
            n4r = not is_inf(ladder(4 * r, x, p, a24), p)
            n8 = not is_inf(ladder(8, x, p, a24), p)
            if o8r and n4r and n8:
                found = x
        x += 1
    out['E_point_order_8r_x'] = found
    # cyclicity: a point of order 8r = #E exists -> E(F_p) cyclic
    xt = None; x = 2
    while xt is None:
        if chi(x) == -1:
            if is_inf(ladder(4 * rt, x, p, a24), p) and not is_inf(ladder(4, x, p, a24), p):
                xt = x
        x += 1
    out['twist_point_x_order_div_by_r_prime'] = xt
    # Hasse: the unique multiple of 8r in [p+1-2sqrt p, p+1+2sqrt p]
    lo, hi = p + 1 - 2 * math.isqrt(p) - 2, p + 1 + 2 * math.isqrt(p) + 2
    mults = [m for m in range((lo // (8 * r)) * 8 * r, hi + 1, 8 * r) if lo <= m <= hi]
    out['hasse_multiples_of_8r'] = mults
    multsT = [m for m in range((lo // rt) * rt, hi + 1, rt) if lo <= m <= hi and m % 4 == 0]
    out['hasse_multiples_of_4rprime'] = multsT
    # E[8] u-values (excluded from bases) from [r]G, [2r]G, [3r]G, 4r -> T0 (u=0)
    e8 = set()
    for t in (1, 2, 3, 4):
        P = ladder(t * r, found, p, a24)
        e8.add(P[0] * pow(P[1], -1, p) % p)
    out['E8_u_values'] = sorted(e8)
    out['valid_columns_expected'] = squares + (1 if zeros else 0) - len(e8)
    out['N'] = [max(2, round(c * math.sqrt(r))) for c in (0.2, 0.4)]
    return out


if __name__ == '__main__':
    res = [facts(k) for k in (sorted(CURVES) if len(sys.argv) < 2 else map(int, sys.argv[1:]))]
    print(json.dumps(res, indent=1))
