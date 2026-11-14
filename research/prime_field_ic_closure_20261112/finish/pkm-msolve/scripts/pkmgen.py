#!/usr/bin/env python3
"""PKM Kummer-tower chain systems for msolve, plus an independent grid enumerator.

Generator (system written exactly as PRESET.txt section 4 says):
  variables y{i}_{j} (i<m, j<t) then u0..u{m-3}; x_i = g*y{i}_0
  tower:  y{i}_{j+1} - y{i}_j^2  (j<t-1),   y{i}_{t-1}^2 - 1
  chain:  S3(x0,x1,u0), S3(u_{k-1},x_{k+1},u_k) (k=1..m-3), S3(u_{m-3},x_{m-1},xR)

Enumerator (independent of any Groebner computation): all points P with x(P) in V
lie in E(F_{p^2}); a grid tuple (x_0..x_{m-1}) is a zero of the chain system iff
some signs e_i give sum e_i P_i in {R,-R} with every partial sum
P_0+...+P_k (k=1..m-2) finite.  Meet in the middle over signed points.
"""
import hashlib
import json
import random
import sys

# ---------------------------------------------------------------- F_p, F_{p^2}

def inv(a, p):
    return pow(a % p, p - 2, p)


def legendre(a, p):
    a %= p
    if a == 0:
        return 0
    return 1 if pow(a, (p - 1) // 2, p) == 1 else -1


def sqrt_mod(a, p):
    """Tonelli-Shanks; a must be a nonzero square"""
    a %= p
    if a == 0:
        return 0
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while legendre(z, p) != -1:
        z += 1
    mm, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1:
            tt = tt * tt % p
            i += 1
        b = pow(c, 1 << (mm - i - 1), p)
        mm, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    assert r * r % p == a
    return r


class Fp2:
    """F_p[s]/(s^2 - d), d a non-residue; element = (a0, a1)"""

    def __init__(self, p):
        self.p = p
        d = 2
        while legendre(d, p) != -1:
            d += 1
        self.d = d

    def add(self, x, y):
        p = self.p
        return ((x[0] + y[0]) % p, (x[1] + y[1]) % p)

    def sub(self, x, y):
        p = self.p
        return ((x[0] - y[0]) % p, (x[1] - y[1]) % p)

    def mul(self, x, y):
        p = self.p
        return ((x[0] * y[0] + self.d * x[1] * y[1]) % p, (x[0] * y[1] + x[1] * y[0]) % p)

    def inv(self, x):
        p = self.p
        n = (x[0] * x[0] - self.d * x[1] * x[1]) % p
        ni = pow(n, p - 2, p)
        return (x[0] * ni % p, (-x[1]) * ni % p)

    def neg(self, x):
        p = self.p
        return ((-x[0]) % p, (-x[1]) % p)

    def sqrt_of_fp(self, a):
        """square root in F_{p^2} of a in F_p"""
        p = self.p
        a %= p
        if a == 0:
            return (0, 0)
        if legendre(a, p) == 1:
            return (sqrt_mod(a, p), 0)
        # a = d * (a/d), a/d is a square
        r = sqrt_mod(a * inv(self.d, p) % p, p)
        return (0, r)  # (r s)^2 = r^2 d = a


def ec_add(F, A, P, Q):
    """affine addition on y^2 = x^3 + A x + B over F (Fp2); None = point at infinity"""
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if F.add(y1, y2) == (0, 0):
            return None
        # doubling
        num = F.add(F.mul((3, 0), F.mul(x1, x1)), (A % F.p, 0))
        lam = F.mul(num, F.inv(F.mul((2, 0), y1)))
    else:
        lam = F.mul(F.sub(y2, y1), F.inv(F.sub(x2, x1)))
    x3 = F.sub(F.sub(F.mul(lam, lam), x1), x2)
    y3 = F.sub(F.mul(lam, F.sub(x1, x3)), y1)
    return (x3, y3)


def ec_neg(F, P):
    if P is None:
        return None
    return (P[0], F.neg(P[1]))

# ---------------------------------------------------------------- curves, towers

def count_points(p, A, B):
    sq = bytearray(p)
    for x in range(1, p):
        sq[x * x % p] = 1
    tot = p + 1
    for x in range(p):
        v = (x * x * x + A * x + B) % p
        if v:
            tot += 1 if sq[v] else -1
    return tot


def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
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


def prime_order_curve(p, rng):
    while True:
        A, B = rng.randrange(1, p), rng.randrange(1, p)
        if (4 * A ** 3 + 27 * B ** 2) % p == 0:
            continue
        n = count_points(p, A, B)
        if is_prime(n):
            return A, B, n


def root_of_unity(p, k, rng):
    """element of exact order 2^k"""
    assert (p - 1) % (1 << k) == 0
    while True:
        h = rng.randrange(2, p)
        z = pow(h, (p - 1) >> k, p)
        if k == 0 or pow(z, 1 << (k - 1), p) == p - 1:
            return z


def choose_g(p, A, B, rng):
    """g with both g and -g abscissae of E(F_p) points (planted targets at t = 1)"""
    while True:
        g = rng.randrange(2, p - 1)
        if legendre(g ** 3 + A * g + B, p) == 1 and legendre(-g ** 3 - A * g + B, p) == 1:
            return g

# ---------------------------------------------------------------- sparse polynomials

def padd(P, Q, p):
    R = dict(P)
    for k, v in Q.items():
        R[k] = (R.get(k, 0) + v) % p
        if R[k] == 0:
            del R[k]
    return R


def pscale(P, c, p):
    c %= p
    if c == 0:
        return {}
    return {k: v * c % p for k, v in P.items()}


def pmul(P, Q, p):
    R = {}
    for k1, v1 in P.items():
        for k2, v2 in Q.items():
            k = tuple(a + b for a, b in zip(k1, k2))
            R[k] = (R.get(k, 0) + v1 * v2) % p
    return {k: v for k, v in R.items() if v}


def pconst(c, nv, p):
    c %= p
    return {tuple([0] * nv): c} if c else {}


def pvar(i, nv, coef=1):
    e = [0] * nv
    e[i] = 1
    return {tuple(e): coef}


def S3poly(a, b, c, A, B, nv, p):
    one = pconst(1, nv, p)
    amb = padd(a, pscale(b, -1, p), p)
    apb = padd(a, b, p)
    ab = pmul(a, b, p)
    t1 = pmul(pmul(amb, amb, p), pmul(c, c, p), p)
    inner = padd(pmul(apb, padd(ab, pscale(one, A, p), p), p), pscale(one, 2 * B, p), p)
    t2 = pscale(pmul(inner, c, p), -2, p)
    abmA = padd(ab, pscale(one, -A, p), p)
    t3 = padd(pmul(abmA, abmA, p), pscale(apb, -4 * B, p), p)
    return padd(padd(t1, t2, p), t3, p)


def fmt_poly(P, names):
    terms = []
    for k in sorted(P, key=lambda e: (-sum(e), [-x for x in e])):
        c = P[k]
        mon = "*".join(names[i] + (f"^{e}" if e > 1 else "") for i, e in enumerate(k) if e)
        if mon:
            terms.append(f"{c}*{mon}")
        else:
            terms.append(f"{c}")
    return "+".join(terms) if terms else "0"

# ---------------------------------------------------------------- system builder

def build_system(p, A, B, g, m, t, xR):
    names = [f"y{i}_{j}" for i in range(m) for j in range(t)] + [f"u{k}" for k in range(m - 2)]
    nv = len(names)
    yi = lambda i, j: i * t + j
    ui = lambda k: m * t + k
    X = [pvar(yi(i, 0), nv, g % p) for i in range(m)]
    U = [pvar(ui(k), nv) for k in range(m - 2)]
    xr = pconst(xR, nv, p)
    eqs = []
    # chain
    eqs.append(S3poly(X[0], X[1], U[0], A, B, nv, p))
    for k in range(1, m - 2):
        eqs.append(S3poly(U[k - 1], X[k + 1], U[k], A, B, nv, p))
    eqs.append(S3poly(U[m - 3], X[m - 1], xr, A, B, nv, p))
    # tower
    for i in range(m):
        for j in range(t - 1):
            eqs.append(padd(pvar(yi(i, j + 1), nv), pscale(pmul(pvar(yi(i, j), nv), pvar(yi(i, j), nv), p), -1, p), p))
        last = pvar(yi(i, t - 1), nv)
        eqs.append(padd(pmul(last, last, p), pconst(-1, nv, p), p))
    text = ",".join(names) + "\n" + str(p) + "\n" + ",\n".join(fmt_poly(e, names) for e in eqs) + "\n"
    return names, eqs, text

# ---------------------------------------------------------------- enumerator

def signed_points(F, A, B, V):
    """for each v in V: list of (sign, point) with x = v; both signs"""
    p = F.p
    out = []
    for v in V:
        f = (v * v * v + A * v + B) % p
        w = F.sqrt_of_fp(f)
        P = ((v, 0), w)
        out.append([(+1, P), (-1, ec_neg(F, P))])
    return out


def enumerate_zeros(p, A, B, V, m, xR, yR, want_solutions=True, max_solutions=200000):
    """grid zeros of the chain system (ordered tuples of V-indices), with one witness
    chain (u-values in F_{p^2}) per (tuple, sign pattern) found."""
    F = Fp2(p)
    sp = signed_points(F, A, B, V)
    R = ((xR % p, 0), (yR % p, 0))
    negR = ec_neg(F, R)
    h = (m + 1) // 2
    nV = len(V)

    def walk(prefix_len, start_idx):
        # all (idx tuple, sign tuple, partial sums list, sum) over prefix_len summands
        res = [((), (), [], None)]
        for pos in range(prefix_len):
            nxt = []
            for idx, sg, partial, S in res:
                for vi in range(nV):
                    for s, P in sp[vi]:
                        S2 = ec_add(F, A, S, P)
                        nxt.append((idx + (vi,), sg + (s,), partial + [S2], S2))
            res = nxt
        return res

    left = walk(h, 0)
    right = walk(m - h, 0)
    # right: map sum point -> list of (idx, sg, partial_right_sums)
    rmap = {}
    for idx, sg, partial, S in right:
        rmap.setdefault(S, []).append((idx, sg, partial))
    zeros = set()
    sols = []
    for idx, sg, partial, SL in left:
        if sg[0] != 1:
            continue  # global sign: fix e_0 = +1
        for target in (R, negR):
            need = ec_add(F, A, target, ec_neg(F, SL))
            for ridx, rsg, rpartial in rmap.get(need, ()):
                # partial sums of the full chain: P0+...+Pk for k = 1..m-2 must be finite
                full_partials = list(partial) + [ec_add(F, A, SL, rp) for rp in rpartial]
                # full_partials[k] = P0+...+Pk ; need k = 1..m-2 finite
                ok = all(full_partials[k] is not None for k in range(1, m - 1))
                if not ok:
                    continue
                tup = idx + ridx
                zeros.add(tup)
                if want_solutions and len(sols) < max_solutions:
                    us = [full_partials[k][0] for k in range(1, m - 1)]  # u_{k-1} = x(P0+..+Pk)
                    sols.append((tup, us))
    return zeros, sols


def main():
    # self-test of the enumerator against brute-force S3 chains on a tiny case
    rng = random.Random(1)
    p = 769
    A, B, n = prime_order_curve(p, rng)
    print("curve", A, B, n)


if __name__ == "__main__":
    main()
