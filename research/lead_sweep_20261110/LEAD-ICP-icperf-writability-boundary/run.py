#!/usr/bin/env python3
"""Writability of the Weil-descended Semaev system over GF(2^n): toy ANF size scaling. stdlib only."""
import argparse, json, math, random, sys

def pmul(a, b, n, mod):
    r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if a >> n & 1: a ^= mod
    return r

def irreducible(poly, n):
    # x^(2^k) mod poly; irreducible iff x^(2^n)=x and gcd conditions (brute: no factor of degree <= n/2)
    for d in range(1, n // 2 + 1):
        for f in range(1 << d, 1 << (d + 1)):
            a, b = poly, f
            # polynomial remainder
            while a.bit_length() >= b.bit_length():
                a ^= b << (a.bit_length() - b.bit_length())
            if a == 0: return False
    return True

def find_mod(n):
    for c in range(1, 1 << n, 2):
        p = (1 << n) | c
        if irreducible(p, n): return p

# Boolean-ring polynomials: dict mask -> GF element (x^2 = x on 0/1 variables)
def padd(P, Q):
    R = dict(P)
    for m, c in Q.items():
        v = R.get(m, 0) ^ c
        if v: R[m] = v
        else: R.pop(m, None)
    return R

def pmulp(P, Q, n, mod):
    R = {}
    for m1, c1 in P.items():
        for m2, c2 in Q.items():
            m = m1 | m2
            v = R.get(m, 0) ^ pmul(c1, c2, n, mod)
            if v: R[m] = v
            else: R.pop(m, None)
    return R

def lin(vars_, basis, l, n):
    """x = sum_j y_{v,j} basis_j for variable block v"""
    return {1 << (vars_ * l + j): basis[vars_][j] for j in range(l)}

def const(c): return {0: c} if c else {}

def s3_coeffs(x1, x2, b, n, mod):
    A = pmulp(padd(x1, x2), padd(x1, x2), n, mod)
    B = pmulp(x1, x2, n, mod)
    C = padd(pmulp(B, B, n, mod), const(b))
    return A, B, C

def system(n, m, seed, null=False):
    rnd = random.Random(seed)
    mod = find_mod(n)
    b = rnd.randrange(1, 1 << n)
    l = -(-n // m)  # per-variable subspace dimension, m*l >= n
    basis = [[rnd.randrange(1, 1 << n) for _ in range(l)] for _ in range(m)]
    R = rnd.randrange(1, 1 << n)
    X = [lin(v, basis, l, n) for v in range(m)]
    if m == 2:
        A, B, C = s3_coeffs(X[0], X[1], b, n, mod)
        S = padd(padd(pmulp(A, pmulp(const(R), const(R), n, mod), n, mod), pmulp(B, const(R), n, mod)), C)
    else:  # m == 3: S4(x1,x2,x3,R) = Res_t(S3(x1,x2,t), S3(x3,R,t))
        A, B, C = s3_coeffs(X[0], X[1], b, n, mod)
        A2, B2, C2 = s3_coeffs(X[2], const(R), b, n, mod)
        mm = lambda P, Q: pmulp(P, Q, n, mod)
        u = padd(mm(A, C2), mm(A2, C))
        v1 = padd(mm(A, B2), mm(A2, B))
        v2 = padd(mm(B, C2), mm(B2, C))
        S = padd(mm(u, u), mm(v1, v2))
    nv = m * l
    if null:  # same support, uniformly random GF coefficients
        S = {k: rnd.randrange(1, 1 << n) for k in S}
    monos = len(S)
    terms = sum(bin(c).count("1") for c in S.values())  # ANF terms over the n Boolean component equations
    return dict(n=n, m=m, nvars=nv, monomials=monos, anf_terms=terms,
                log2_terms=math.log2(terms) if terms else 0.0,
                dense_cap_log2=nv + math.log2(n))

def slope(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--smoke", action="store_true")
    a = ap.parse_args()
    seeds = [1] if a.smoke else [11, 22, 33]
    plan = {2: [6, 7]} if a.smoke else {2: list(range(6, 15)), 3: list(range(6, 12))}
    out = dict(lead="LEAD-ICP-icperf-writability-boundary", seeds=seeds, smoke=a.smoke,
               rho_budget_slope_per_n=0.5, rows=[], fits={})
    for m, ns in plan.items():
        for n in ns:
            for s in seeds:
                out["rows"].append(system(n, m, s * 1000 + n))
    # null (random coeffs on same support) at one size per m, A/A = same size different seed
    out["null_control"] = system(7 if a.smoke else 10, 2, 5, null=True)
    out["aa_arm"] = [system(7 if a.smoke else 10, 2, s) for s in ([2, 3] if a.smoke else [101, 102, 103])]
    verdict = "inconclusive"
    for m in plan:
        rs = [r for r in out["rows"] if r["m"] == m]
        if len(rs) >= 4 and len({r["n"] for r in rs}) >= 3:
            out["fits"][str(m)] = slope([r["n"] for r in rs], [r["log2_terms"] for r in rs])
    if not a.smoke and out["fits"]:
        sl = list(out["fits"].values())
        if min(sl) >= 0.5 + 0.1: verdict = "consistent_with_prediction"
        elif max(sl) < 0.5: verdict = "falsified"
    out["verdict"] = verdict
    print(json.dumps(out))
main()
