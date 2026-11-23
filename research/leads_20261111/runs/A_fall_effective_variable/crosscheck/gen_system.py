#!/usr/bin/env python3
"""Independent Weil descent for the closure cross-check (closure-falls-crosscheck, 2026-11-12).

Reads one instance line `n k poly a6 z v_1..v_k` (format of emit_instance*.py) on stdin and writes the
Boolean polynomial system to stdout.  It does NOT reuse fallclose.c's symbolic coefficient formulas nor
lfd_small: it evaluates S(X1,X2) = X1^2 X2^2 + z^2 (X1^2 + X2^2) + z X1 X2 + a6 with its own GF(2^n)
arithmetic at the points 0, e_i, e_i+e_j and recovers the ANF coefficients of degree <= 2 by
inclusion-exclusion.  Variables: x_0..x_{k-1} = a (X1 = sum a_i v_i), x_k..x_{2k-1} = b (X2 = sum b_j v_j).
Checks (abort on failure): ANF vs direct evaluation at 400 random points (so the system has degree <= 2).
Output: `N n` then n lines `cnt m_1 .. m_cnt` (monomial bitmasks, 0 = constant), one per coordinate bit.
"""
import random, sys

def gf_mul(a, b, n, poly):
    r = 0
    for i in range(n):           # schoolbook, reduce at the end
        if (b >> i) & 1:
            r ^= a << i
    for d in range(2 * n - 2, n - 1, -1):
        if (r >> d) & 1:
            r ^= poly << (d - n)
    return r

def main():
    f = list(map(int, sys.stdin.read().split()))
    n, k, poly, a6, z, V = f[0], f[1], f[2], f[3], f[4], f[5:5 + f[1]]
    assert len(V) == k and (poly >> n) == 1
    N = 2 * k
    mul = lambda a, b: gf_mul(a, b, n, poly)

    def S(x):  # x: N-bit assignment
        X1 = X2 = 0
        for i in range(k):
            if (x >> i) & 1: X1 ^= V[i]
            if (x >> (k + i)) & 1: X2 ^= V[i]
        s1, s2 = mul(X1, X1), mul(X2, X2)
        return mul(s1, s2) ^ mul(mul(z, z), s1 ^ s2) ^ mul(z, mul(X1, X2)) ^ a6

    s0 = S(0)
    s1 = [S(1 << i) for i in range(N)]
    coef = {0: s0}
    for i in range(N):
        coef[1 << i] = s1[i] ^ s0
    for i in range(N):
        for j in range(i + 1, N):
            coef[(1 << i) | (1 << j)] = S((1 << i) | (1 << j)) ^ s1[i] ^ s1[j] ^ s0
    coef = {m: c for m, c in coef.items() if c}
    rng = random.Random(20261112)
    for _ in range(400):
        x = rng.getrandbits(N)
        v = 0
        for m, c in coef.items():
            if m & x == m: v ^= c
        if v != S(x):
            sys.exit("degree check failed: S is not quadratic in the descent variables")
    out = [f"{N} {n}"]
    for b in range(n):
        ms = sorted(m for m, c in coef.items() if (c >> b) & 1)
        out.append(" ".join(map(str, [len(ms)] + ms)))
    print("\n".join(out))

main()
