"""Binary Semaev polynomials S_3, S_4 over F_2[a6, x1..x4] and verification.
Curve: y^2 + x y = x^3 + a2 x^2 + a6   (a2 does not occur in S_m).
A polynomial is a dict {exponent tuple: 1}, tuple = (e_a6, e_x1, ..., e_xk); coefficients in F_2."""
import itertools, sys
NV = 5  # a6, x1..x4

def padd(p, q):
    r = dict(p)
    for k in q:
        if k in r: del r[k]
        else: r[k] = 1
    return r
def pmul(p, q):
    r = {}
    for a in p:
        for b in q:
            k = tuple(i + j for i, j in zip(a, b))
            if k in r: del r[k]
            else: r[k] = 1
    return r
def mono(**e):
    v = [0] * NV
    for k, x in e.items():
        v[{'a': 0, 'x1': 1, 'x2': 2, 'x3': 3, 'x4': 4}[k]] = x
    return tuple(v)
def var(i):
    v = [0] * NV; v[i] = 1; return {tuple(v): 1}
ONE = {(0,) * NV: 1}
A6 = var(0)
def X(i): return var(i)
def psq(p): return pmul(p, p)

def S3_coeffs(u, v):
    """S_3(u, v, T) = A T^2 + B T + C  as coefficient polys in T (A, B, C in F_2[a6,u,v])"""
    A = padd(psq(u), psq(v))
    B = pmul(u, v)
    C = padd(psq(pmul(u, v)), A6)
    return A, B, C

def resultant_quadratics(f, g):
    """Res_T(aT^2+bT+c, a'T^2+b'T+c') = (ac'-a'c)^2 - (ab'-a'b)(bc'-b'c)   (char 2: signs irrelevant)"""
    a, b, c = f; a2, b2, c2 = g
    t1 = padd(pmul(a, c2), pmul(a2, c))
    t2 = padd(pmul(a, b2), pmul(a2, b))
    t3 = padd(pmul(b, c2), pmul(b2, c))
    return padd(psq(t1), pmul(t2, t3))

def sylvester_det_quadratics(f, g):
    """independent check: determinant of the 4x4 Sylvester matrix by Leibniz expansion"""
    a, b, c = f; a2, b2, c2 = g
    Z = {}
    M = [[a, b, c, Z], [Z, a, b, c], [a2, b2, c2, Z], [Z, a2, b2, c2]]
    det = {}
    for perm in itertools.permutations(range(4)):
        term = ONE
        for i in range(4):
            term = pmul(term, M[i][perm[i]])
            if not term: break
        det = padd(det, term)
    return det

def S3():
    x1, x2, x3 = X(1), X(2), X(3)
    return padd(padd(padd(pmul(psq(x1), psq(x2)), pmul(psq(x1), psq(x3))), padd(pmul(psq(x2), psq(x3)), pmul(pmul(x1, x2), x3))), A6)

def S4():
    """S_4(x1,x2,x3,x4) = Res_T(S_3(x1,x2,T), S_3(T,x3,x4))"""
    f = S3_coeffs(X(1), X(2))
    g = S3_coeffs(X(3), X(4))
    r = resultant_quadratics(f, g)
    r2 = sylvester_det_quadratics(f, g)
    assert r == r2, "Sylvester determinant and closed-form resultant disagree"
    return r

def permute_vars(p, perm):
    """perm acts on x-variable indices 1..4: new exponent vector"""
    r = {}
    for k in p:
        nk = [k[0]] + [0] * 4
        for i in range(4): nk[1 + perm[i]] = k[1 + i]
        r[tuple(nk)] = 1
    return r

def degrees(p):
    return [max(k[1 + i] for k in p) for i in range(4)], max(sum(k[1:]) for k in p)

if __name__ == "__main__":
    out = []
    P = lambda *a: (print(*a), out.append(" ".join(str(x) for x in a)))
    s3 = S3()
    s3b = padd(pmul(padd(psq(X(1)), psq(X(2))), psq(X(3))), padd(padd(pmul(pmul(X(1), X(2)), X(3)), pmul(psq(X(1)), psq(X(2)))), A6))
    assert s3 == s3b; P("S3 = (x1^2+x2^2)x3^2 + x1x2x3 + x1^2x2^2 + a6 : form agrees with KW eq. in Sec. 3")
    # S3 symmetric?
    for perm in itertools.permutations(range(3)):
        q = {}
        for k in s3:
            nk = [k[0], 0, 0, 0, 0]
            for i in range(3): nk[1 + perm[i]] = k[1 + i]
            q[tuple(nk)] = 1
        assert q == s3
    P("S3 symmetric under S_3: OK")
    s4 = S4()
    P("S4: %d monomials over F_2[a6,x1..x4]" % len(s4))
    dx, dt = degrees(s4)
    P("S4 degree in each x_i:", dx, " total degree:", dt)
    assert dx == [4, 4, 4, 4]
    for perm in itertools.permutations(range(4)):
        assert permute_vars(s4, perm) == s4
    P("S4 symmetric under all 24 permutations of (x1..x4): OK")
    # kappa(x4) = coefficient of x1^3 x2^3 x3^3
    kap = {}
    for k in s4:
        if k[1:4] == (3, 3, 3): kap[(k[0], k[4])] = 1
    P("kappa(x4, a6) = coefficient of x1^3 x2^3 x3^3 in S4 =", " + ".join("a6^%d*x4^%d" % t for t in sorted(kap)))
    # max Hamming weight of exponent pattern (e1,e2,e3), x4 specialised to constant
    wt = lambda e: bin(e).count("1")
    best = max(wt(k[1]) + wt(k[2]) + wt(k[3]) for k in s4)
    P("max over monomials of wt(e1)+wt(e2)+wt(e3) (Hamming weights) =", best, "(m^2-m = 6)")
    P("monomials attaining it:", sorted({k[1:4] for k in s4 if wt(k[1]) + wt(k[2]) + wt(k[3]) == best}))
    open("verify_s4_symbolic.txt", "w").write("\n".join(out) + "\n")
