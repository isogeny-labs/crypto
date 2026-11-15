#!/usr/bin/env python3
"""Exact unit-ideal test for the two-summand mu_N chain over any prime field F_l (no roots of unity needed).

The ideal (g, x1^N - 1, x2^N - 1) in F_l[x1, x2], g = S_3(x1, x2, t) = A(x2) x1^2 + B(x2) x1 + C(x2), is the unit
ideal iff g has no zero on mu_N^2 over the algebraic closure iff
    R(x2) := Res_{x1}(g, x1^N - 1) = prod_{zeta^N = 1} g(zeta, x2)
is coprime to x2^N - 1.  With r1, r2 the roots of g in x1:  R = A^N (r1^N - 1)(r2^N - 1)
  = C^N - a_N + A^N,   a_n := A^n (r1^n + r2^n)  (a polynomial),  a_1 = -B,  a_{2n} = a_n^2 - 2 (A C)^n.
Everything is computed in F_l[x2]/(x2^N - 1) (a ring homomorphism; gcd(R, x2^N - 1) is unchanged), so N = 2^k
doublings of cyclic convolutions of length N, then one polynomial gcd.  deg gcd = number of zeta2 in mu_N (over the
closure) that extend to a zero of g on mu_N^2 (counted without multiplicity): 0 iff unit.
Also: a brute-force cross-check when mu_N is rational (2^k | l - 1) counting all zeros on mu_N^2.
"""
import sys, json
import numpy as np

def s3_coeffs(p, a, b, t):
    c = {}
    def add(e, v): c[e] = (c.get(e, 0) + v) % p
    add((2,0), t*t); add((1,1), -2*t*t); add((0,2), t*t); add((2,1), -2*t); add((1,2), -2*t)
    add((1,0), -2*t*a); add((0,1), -2*t*a); add((0,0), -4*t*b); add((2,2), 1); add((1,1), -2*a); add((0,0), a*a)
    add((1,0), -4*b); add((0,1), -4*b)
    return {e: v for e, v in c.items() if v}

def cyc_mul(u, v, p, N):
    """cyclic convolution mod (x^N - 1) over F_p, exact (int64 with p < 2^20 and N <= 2^12 -> sums < 2^52)."""
    w = np.convolve(u, v) % p            # length 2N-1
    out = w[:N].copy(); out[:len(w)-N] = (out[:len(w)-N] + w[N:]) % p
    return out

def cyc_pow(u, e, p, N):
    r = np.zeros(N, dtype=np.int64); r[0] = 1
    base = u.copy()
    while e:
        if e & 1: r = cyc_mul(r, base, p, N)
        e >>= 1
        if e: base = cyc_mul(base, base, p, N)
    return r

def poly_gcd_deg(f, p, N):
    """degree of gcd(f(x), x^N - 1) over F_p; f given as length-N coefficient vector (low to high)."""
    def trim(a):
        a = [int(v) % p for v in a]
        while a and a[-1] == 0: a.pop()
        return a
    def rem(a, b):
        a = a[:]; inv = pow(b[-1], p - 2, p); db = len(b) - 1
        while len(a) - 1 >= db and a:
            c = a[-1] * inv % p; da = len(a) - 1
            for i, bv in enumerate(b): a[da - db + i] = (a[da - db + i] - c * bv) % p
            a = trim(a)
        return a
    A = [p - 1] + [0] * (N - 1) + [1]      # x^N - 1
    B = trim(f)
    while B:
        A, B = B, rem(A, B)
    return len(A) - 1

def resultant_unit(p, a, b, t, k):
    N = 1 << k
    co = s3_coeffs(p, a, b, t)
    A = np.zeros(N, dtype=np.int64); B = np.zeros(N, dtype=np.int64); C = np.zeros(N, dtype=np.int64)
    for (e1, e2), v in co.items():
        tgt = {2: A, 1: B, 0: C}[e1]; tgt[e2 % N] = (tgt[e2 % N] + v) % p
    AC = cyc_mul(A, C, p, N)
    a_n = (-B) % p; acn = AC.copy()        # a_1 = -B ; (AC)^1
    n = 1
    while n < N:                            # a_{2n} = a_n^2 - 2 (AC)^n
        a_n = (cyc_mul(a_n, a_n, p, N) - 2 * acn) % p
        acn = cyc_mul(acn, acn, p, N); n *= 2
    R = (cyc_pow(C, N, p, N) - a_n + cyc_pow(A, N, p, N)) % p
    d = poly_gcd_deg(R, p, N)
    return dict(p=p, a=a, b=b, t=t, k=k, gcd_degree=d, unit=(d == 0))

def brute_zeros(p, a, b, t, k):
    """zeros of g on mu_N^2 when mu_N is rational over F_p (2^k | p - 1); None otherwise."""
    N = 1 << k
    if (p - 1) % N: return None
    h = next(pow(x, (p - 1) // N, p) for x in range(2, p) if pow(pow(x, (p - 1) // N, p), N // 2, p) != 1)
    H = np.array([pow(h, e, p) for e in range(N)], dtype=np.int64)
    co = s3_coeffs(p, a, b, t)
    X1 = H[:, None]; X2 = H[None, :]
    v = np.zeros((N, N), dtype=np.int64)
    for (e1, e2), c in co.items():
        v = (v + c * (pow_arr(X1, e1, p) * pow_arr(X2, e2, p) % p)) % p
    zs = np.argwhere(v == 0)
    return dict(zeros=int(len(zs)), distinct_x2=int(len(set(zs[:, 1].tolist()))))

def pow_arr(x, e, p):
    r = np.ones_like(x)
    for _ in range(e): r = r * x % p
    return r

if __name__ == '__main__':
    p, a, b, t, k = map(int, sys.argv[1:6])
    out = resultant_unit(p, a, b, t, k)
    bz = brute_zeros(p, a, b, t, k)
    if bz is not None: out.update(bz)
    print(json.dumps(out))
