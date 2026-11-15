#!/usr/bin/env python3
"""Skeptic's own Weil descent + checks (VERIFY of closure-falls-crosscheck, 2026-11-12).
Written independently of fallclose.c and gen_system.py.
Usage: mydescent.py LINE_FILE SYSFILE_TO_COMPARE MYSYS_OUT -> prints JSON; writes my system to MYSYS_OUT
S3(X1,X2,z) = (X1 X2 + z(X1+X2))^2 + z X1 X2 + a6  (Semaev, char 2), X1 = sum a_i v_i, X2 = sum b_j v_j.
Symbolic ANF: a_i b_j -> v_i^2 v_j^2 + z v_i v_j ; a_i -> z^2 v_i^2 ; b_j -> z^2 v_j^2 ; 1 -> a6.
Also counts solutions (a,b) exactly: for each a (Gray code), solve the GF(2)-linear system in b."""
import sys, json, random

def clmul(a, b):
    r = 0
    while b:
        if b & 1: r ^= a
        a <<= 1; b >>= 1
    return r

def mkfield(n, poly):
    def red(x):
        while x.bit_length() > n:
            x ^= poly << (x.bit_length() - 1 - n)
        return x
    return (lambda a, b: red(clmul(a, b)))

def main():
    toks = list(map(int, open(sys.argv[1]).read().split()))
    n, k, poly, a6, z = toks[:5]; V = toks[5:5 + k]; assert len(V) == k
    M = mkfield(n, poly)
    sq = lambda x: M(x, x)
    z2 = sq(z)
    N = 2 * k
    coef = {}
    def add(m, c):
        coef[m] = coef.get(m, 0) ^ c
    add(0, a6)
    for i in range(k):
        add(1 << i, M(z2, sq(V[i])))
        add(1 << (k + i), M(z2, sq(V[i])))
        for j in range(k):
            add((1 << i) | (1 << (k + j)), sq(M(V[i], V[j])) ^ M(z, M(V[i], V[j])))
    coef = {m: c for m, c in coef.items() if c}
    # direct evaluation of S
    def S(x):
        X1 = X2 = 0
        for i in range(k):
            if x >> i & 1: X1 ^= V[i]
            if x >> (k + i) & 1: X2 ^= V[i]
        p = M(X1, X2)
        return sq(p ^ M(z, X1 ^ X2)) ^ M(z, p) ^ a6
    rng = random.Random(9001)
    for _ in range(300):
        x = rng.getrandbits(N); v = 0
        for m, c in coef.items():
            if m & x == m: v ^= c
        assert v == S(x), "ANF mismatch"
    out = {"n": n, "k": k, "N": N, "nmono": len(coef)}
    # my system file, same text format (N n; per bit: cnt masks) for my closure
    lines = [f"{N} {n}"]
    polys = []
    for b in range(n):
        ms = sorted(m for m, c in coef.items() if c >> b & 1)
        polys.append(frozenset(ms)); lines.append(" ".join(map(str, [len(ms)] + ms)))
    open(sys.argv[3] if len(sys.argv) > 3 else "/dev/null", "w").write("\n".join(lines) + "\n")
    if len(sys.argv) > 2:
        t = open(sys.argv[2]).read().split("\n")
        hdr = list(map(int, t[0].split()))
        other = []
        for l in t[1:]:
            if l.strip():
                w = list(map(int, l.split())); assert w[0] == len(w) - 1; other.append(frozenset(w[1:]))
        out["sysfile_header_ok"] = hdr == [N, n]
        out["sysfile_identical_polys"] = other == polys
    # exact solution count. L_j(a) = Q_j + sum_i a_i P_ij ; C(a) = a6 + z^2 * sum a_i v_i^2
    P = [[sq(M(V[i], V[j])) ^ M(z, M(V[i], V[j])) for j in range(k)] for i in range(k)]
    Q = [M(z2, sq(V[j])) for j in range(k)]
    Cv = [M(z2, sq(V[i])) for i in range(k)]
    L = Q[:]; C = a6; sols = 0; a = 0
    def nsol(cols, rhs):
        # solve sum b_j cols[j] = rhs over GF(2)^n ; return number of solutions b
        piv = {}  # leading bit -> (vec, mask over b)
        for j, c in enumerate(cols):
            m = 1 << j
            while c:
                hb = c.bit_length() - 1
                if hb in piv:
                    pc, pm = piv[hb]; c ^= pc; m ^= pm
                else:
                    piv[hb] = (c, m); break
        r = rhs
        while r:
            hb = r.bit_length() - 1
            if hb not in piv: return 0
            r ^= piv[hb][0]
        return 1 << (k - len(piv))
    for g in range(1 << k):
        if g:
            i = (g & -g).bit_length() - 1   # Gray code: flip bit i
            a ^= 1 << i
            for j in range(k): L[j] ^= P[i][j]
            C ^= Cv[i]
        sols += nsol(L, C)
    out["sols"] = sols
    print(json.dumps(out))

main()
