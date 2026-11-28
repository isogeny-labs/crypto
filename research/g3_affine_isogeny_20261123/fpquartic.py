#!/usr/bin/env python3
"""Explicit plane quartic X over F_p with Jac(X) = B = J/L (or its quadratic twist) for an
affine-type kernel L of a genus-3 hyperelliptic J: y^2 = f(x), f monic irreducible septic.

Pipeline (all exact, in K = F_{p^56} via python-flint):
 1. Even model as in the twist study: u = 1/(x - t); branch points u_i = 1/(z^{p^i} - t)
    (i < 7) and u_8 = 0, labelled onto the formula's labels by the same permutation sig.
 2. Theta^2 of B (up to one common factor): the 36 squares and 70 same-top-row products
    from formulas/prod_001000.json (derive2.py), with rho_ij = an arbitrary fourth root of
    e_j - e_i and zeta8 a primitive 8th root of unity in K.
 3. Individual theta constants: one square root per top row a, the others by division by
    the same-row products; the 8 row signs are fixed by Riemann's quartic relations
    (formulas/riemann_rel.json).
 4. Frobenius label action g: (sigma Theta)(c) = lambda * mu4 * Theta(g c), found by
    matching fourth powers of ratios; g is an affine symplectic map of F_2^6.
 5. A g-stable Aronhold set A = (m1..m7); its Riemann matrix a_ij by Jacobi's derivative
    formula (formulas/jacobi.json):  a_{k,1} = D(k,2,3)/D(4,2,3), etc.
 6. Riemann's quartic F (Riemann 1876; Dolgachev CAG Thm 6.1.9):
    (X1 xi1 + X2 xi2 - X3 xi3)^2 - 4 X1 X2 xi1 xi2 = 0.
 7. Descent: sigma(F)(H X) = lambda F(X) with H the frame change of the permuted
    Aronhold system; Hilbert 90 (P = sum_i H_i sigma^i(R)) gives G(Y) = F(P Y) over F_p.
 8. Point counts of G over F_p, F_{p^2}, F_{p^3} -> L-polynomial.
"""
import itertools
import json
import os
import random

import flint

HERE = os.path.dirname(os.path.abspath(__file__))
BITS3 = list(itertools.product((0, 1), repeat=3))
CHARS = [(a, b) for a in BITS3 for b in BITS3]
ZC = ((0, 0, 0), (0, 0, 0))


def q(c):
    return sum(x * y for x, y in zip(c[0], c[1])) % 2


EVEN = [c for c in CHARS if q(c) == 0]
ODD = [c for c in CHARS if q(c) == 1]


def cadd(*cs):
    return (tuple(sum(c[0][i] for c in cs) % 2 for i in range(3)),
            tuple(sum(c[1][i] for c in cs) % 2 for i in range(3)))


def tc(x):
    return (tuple(x[0]), tuple(x[1]))


def vec(c):
    return list(c[0]) + list(c[1])


def unvec(v):
    return (tuple(v[:3]), tuple(v[3:]))


def load_tables():
    P = json.load(open(os.path.join(HERE, "formulas", "prod_001000.json")))
    J = json.load(open(os.path.join(HERE, "formulas", "jacobi.json")))
    R = json.load(open(os.path.join(HERE, "formulas", "riemann_rel.json")))
    jac = {tuple(tc(n) for n in tr): ([tc(e) for e in es5], s) for tr, es5, s in J}
    rels = [([[tc(c) for c in k] for k in ks], ss) for ks, ss in R]
    prods = {(tuple(d["a"]), tuple(d["b1"]), tuple(d["b2"])): d["terms"] for d in P["products"]}
    return prods, [tuple(x) for x in P["pairs"]], jac, rels


def all_aronhold_sets():
    out = []

    def bt(cur, start):
        if len(cur) == 7:
            out.append(tuple(cur))
            return
        for k in range(start, len(ODD)):
            m = ODD[k]
            if all(q(cadd(x, y, m)) == 0 for x, y in itertools.combinations(cur, 2)):
                bt(cur + [m], k + 1)
    bt([], 0)
    return out


# ------------------------------------------------------------------ linear algebra over K

def solve(M, B, K):
    """Solve M X = B (M n x n, B n x r lists), Gaussian elimination. None if singular."""
    n = len(M)
    r = len(B[0])
    A = [list(M[i]) + list(B[i]) for i in range(n)]
    for col in range(n):
        piv = next((i for i in range(col, n) if not A[i][col].is_zero()), None)
        if piv is None:
            return None
        A[col], A[piv] = A[piv], A[col]
        inv = A[col][col].inverse()
        A[col] = [x * inv for x in A[col]]
        for i in range(n):
            if i != col and not A[i][col].is_zero():
                f = A[i][col]
                A[i] = [x - f * y for x, y in zip(A[i], A[col])]
    return [row[n:] for row in A]


def matmul(A, B):
    return [[sum((A[i][k] * B[k][j] for k in range(len(B))), A[0][0] * 0) for j in range(len(B[0]))]
            for i in range(len(A))]


def ident(K):
    return [[K.one() if i == j else K.zero() for j in range(3)] for i in range(3)]


def inv3(M, K):
    return solve(M, ident(K), K)


def frob_mat(M, p):
    return [[x ** p for x in row] for row in M]


# ------------------------------------------------------------------ polynomials as dicts

def padd(P, Q, K, s=1):
    R = dict(P)
    for e, c in Q.items():
        R[e] = R.get(e, K.zero()) + (c if s == 1 else -c)
    return {e: c for e, c in R.items() if not c.is_zero()}


def pmul(P, Q, K):
    R = {}
    for e1, c1 in P.items():
        for e2, c2 in Q.items():
            e = (e1[0] + e2[0], e1[1] + e2[1], e1[2] + e2[2])
            R[e] = R.get(e, K.zero()) + c1 * c2
    return {e: c for e, c in R.items() if not c.is_zero()}


def lin(v, K):
    return {tuple(1 if t == s else 0 for t in range(3)): v[s] for s in range(3) if not v[s].is_zero()}


def riemann_quartic(A, K):
    """Riemann's quartic for the Aronhold frame u1..u4 = X1, X2, X3, X1+X2+X3,
    u_{5+j} = A[j] . X. Returns (F as dict, residual-ok flag)."""
    one = K.one()
    M = [[A[j][i].inverse() for j in range(3)] for i in range(3)]
    U = solve(M, [[-one]] * 3, K)
    U = [u[0] for u in U]
    At = [[A[j][i] * U[j] for j in range(3)] for i in range(3)]  # (A^T diag U)[i][j] = A[j][i] U[j]
    kk = solve(At, [[-one]] * 3, K)
    kk = [x[0] for x in kk]
    Left = [[one] * 3] + [[A[j][i].inverse() for i in range(3)] for j in range(3)]
    Right = [[-one] * 3] + [[-kk[j] * A[j][i] for i in range(3)] for j in range(3)]
    Xi = solve(Left[:3], Right[:3], K)
    ok = all(sum((Left[3][t] * Xi[t][s] for t in range(3)), K.zero()) == Right[3][s] for s in range(3))
    X = [lin([one if t == s else K.zero() for t in range(3)], K) for s in range(3)]
    xi = [lin(Xi[r], K) for r in range(3)]
    Gp = padd(padd(pmul(X[0], xi[0], K), pmul(X[1], xi[1], K), K), pmul(X[2], xi[2], K), K, -1)
    T4 = pmul(pmul(pmul(X[0], X[1], K), xi[0], K), xi[1], K)
    four = K(4)
    F = padd(pmul(Gp, Gp, K), {e: four * c for e, c in T4.items()}, K, -1)
    return F, ok


def subst(F, P, K):
    """F(P Y): X_s = sum_t P[s][t] Y_t."""
    forms = [lin(P[s], K) for s in range(3)]
    out = {}
    for e, c in F.items():
        term = {(0, 0, 0): c}
        for s in range(3):
            for _ in range(e[s]):
                term = pmul(term, forms[s], K)
        out = padd(out, term, K)
    return out


def proportional(F, G):
    keys = set(F) | set(G)
    if set(F) != set(G):
        return None
    k0 = next(iter(keys))
    lam = G[k0] * F[k0].inverse()
    return lam if all(G[k] == lam * F[k] for k in keys) else None


# ------------------------------------------------------------------ the construction

class Ctx:
    def __init__(self, p, n=56, seed=0):
        self.p, self.n = p, n
        self.K = flint.fq_default_ctx(p, n)
        self.R = flint.fq_default_poly_ctx(self.K)
        self.rng = random.Random(seed)
        K = self.K
        q = p ** n
        while True:
            x = K.random_element()
            if x.is_zero():
                continue
            z = x ** ((q - 1) // 8)
            if not (z ** 4 + K.one()).is_zero():
                continue
            self.z8 = z
            break
        self.i4 = self.z8 ** 2

    def c(self, x):
        return self.K(x % self.p)

    def fourth_root(self, d, randomize=True):
        r = d.sqrt().sqrt()
        assert r ** 4 == d
        if randomize:
            r = r * self.i4 ** self.rng.randrange(4)
        return r


def theta_squares(cx, prods, PAIRS, es):
    K = cx.K
    rho = [cx.fourth_root(es[j] - es[i]) for (i, j) in PAIRS]
    zp = [K.one()]
    for _ in range(7):
        zp.append(zp[-1] * cx.z8)
    out = {}
    for key, terms in prods.items():
        acc = K.zero()
        for k, v in terms:
            t = zp[k]
            for idx, e in enumerate(v):
                if e:
                    t = t * rho[idx] ** e
            acc += t
        out[key] = acc
    return out


def individual_thetas(cx, P, jac_rels):
    """theta'(c) with theta'^2 = Theta(c), theta'(a,b1)theta'(a,b2) = P; returns list of
    candidate vectors (one per row-sign pattern satisfying all Riemann relations)."""
    K = cx.K
    groups = {}
    for (a, b) in EVEN:
        groups.setdefault(a, []).append(b)
    base = {}
    for a, bs in groups.items():
        bs = sorted(bs)
        b0 = next((b for b in bs if not P[(a, b, b)].is_zero()), None)
        if b0 is None:
            return None, "zero_row"
        t0 = P[(a, b0, b0)].sqrt()
        for b in bs:
            key = (a, min(b0, b), max(b0, b))
            base[(a, b)] = P[key] * t0.inverse() if b != b0 else t0
        for b1, b2 in itertools.combinations_with_replacement(bs, 2):
            assert base[(a, b1)] * base[(a, b2)] == P[(a, b1, b2)], "same-row product mismatch"
    if any(base[c].is_zero() for c in EVEN):
        return None, "vanishing_even_theta"
    cands = []
    for bits in itertools.product((1, -1), repeat=7):
        s = dict(zip(BITS3, (1,) + bits))
        th = {c: base[c] if s[c[0]] == 1 else -base[c] for c in EVEN}
        ok = True
        for ks, (s2, s3) in jac_rels:
            m = []
            for k in ks:
                v = K.one()
                for c in k:
                    v = v * th[c]
                m.append(v)
            tot = m[0] + (m[1] if s2 == 1 else -m[1]) + (m[2] if s3 == 1 else -m[2])
            if not tot.is_zero():
                ok = False
                break
        if ok:
            cands.append(th)
    return cands, "ok"


def frobenius_label_action(cx, Theta):
    """g with (Theta^p)(c) = lambda * mu4 * Theta(g c). Returns (G 6x6 over F2, h) or None."""
    p = cx.p
    sT = {c: Theta[c] ** p for c in EVEN}
    four = {}
    for c in EVEN:
        four.setdefault(Theta[c] ** 4, []).append(c)
    cref = ZC
    found = []
    for d in EVEN:
        base = Theta[d] ** 4 * (sT[cref] ** 4).inverse()
        g = {}
        ok = True
        for c in EVEN:
            t = sT[c] ** 4 * base
            hits = four.get(t)
            if not hits or len(hits) != 1:
                ok = False
                break
            g[c] = hits[0]
        if ok and len(set(g.values())) == 36:
            found.append(g)
    if len(found) != 1:
        return None, len(found)
    g = found[0]
    h = vec(g[ZC])
    cols = []
    for k in range(6):
        e = [1 if t == k else 0 for t in range(6)]
        img = vec(g[unvec(e)])
        cols.append([(x + y) % 2 for x, y in zip(img, h)])

    def apply(c):
        v = vec(c)
        out = list(h)
        for k in range(6):
            if v[k]:
                out = [(x + y) % 2 for x, y in zip(out, cols[k])]
        return unvec(out)
    for c in EVEN:
        assert apply(c) == g[c], "label action not affine"
    return apply, 1


def dval(jac, th, trip, K):
    srt = tuple(sorted(trip, key=ODD.index))
    perm = [srt.index(x) for x in trip]
    sg = 1
    for i in range(3):
        for j in range(i + 1, 3):
            if perm[i] > perm[j]:
                sg = -sg
    es5, s = jac[srt]
    v = K.one()
    for e in es5:
        v = v * th[e]
    return v if sg * s == 1 else -v


def aronhold_matrix(jac, th, m, K):
    A = []
    for k in (4, 5, 6):
        r = [dval(jac, th, (m[k], m[1], m[2]), K) * dval(jac, th, (m[3], m[1], m[2]), K).inverse(),
             dval(jac, th, (m[0], m[k], m[2]), K) * dval(jac, th, (m[0], m[3], m[2]), K).inverse(),
             dval(jac, th, (m[0], m[1], m[k]), K) * dval(jac, th, (m[0], m[1], m[3]), K).inverse()]
        r0 = r[0].inverse()
        A.append([x * r0 for x in r])
    return A


def frame(lines, K):
    """lines: 7 covectors; returns (H, Anorm) with X' = H X the frame of the first four."""
    first = lines[:3]
    firstT = [[first[j][i] for j in range(3)] for i in range(3)]
    co = solve(firstT, [[x] for x in lines[3]], K)
    if co is None:
        return None, None
    co = [x[0] for x in co]
    H = [[co[i] * first[i][j] for j in range(3)] for i in range(3)]
    Hi = inv3(H, K)
    if Hi is None:
        return None, None
    A = []
    for l in lines[4:]:
        r = [sum((l[t] * Hi[t][s] for t in range(3)), K.zero()) for s in range(3)]
        if r[0].is_zero():
            return None, None
        r0 = r[0].inverse()
        A.append([x * r0 for x in r])
    return H, A


def descend(cx, F, H):
    """Find P with F(P Y) proportional to an F_p-rational form, given
    sigma(F)(H X) = lambda F(X). Returns dict over F_p (ints)."""
    K, p, n = cx.K, cx.p, cx.n
    Hinv = inv3(H, K)
    Hs = [ident(K)]
    for _ in range(n):
        Hs.append(matmul(Hinv, frob_mat(Hs[-1], p)))
    N = Hs[n]
    c = N[0][0]
    assert all(N[i][j] == (c if i == j else K.zero()) for i in range(3) for j in range(3)), "norm not scalar"
    assert c ** p == c, "norm scalar not in F_p"
    # rescale H -> mu H so that the cocycle has norm I: N -> N / Norm(mu)
    while True:
        mu = K.random_element()
        if mu.is_zero():
            continue
        nm = K.one()
        x = mu
        for _ in range(n):
            nm = nm * x
            x = x ** p
        if nm == c:
            break
    Hm = [[mu * x for x in row] for row in H]
    Hinv = inv3(Hm, K)
    Hs = [ident(K)]
    for _ in range(n - 1):
        Hs.append(matmul(Hinv, frob_mat(Hs[-1], p)))
    for _ in range(20):
        Rm = [[K.random_element() for _ in range(3)] for _ in range(3)]
        Pm = [[K.zero()] * 3 for _ in range(3)]
        Ri = Rm
        for i in range(n):
            T = matmul(Hs[i], Ri)
            Pm = [[Pm[a][b] + T[a][b] for b in range(3)] for a in range(3)]
            Ri = frob_mat(Ri, p)
        if inv3(Pm, K) is not None:
            break
    G = subst(F, Pm, K)
    k0 = sorted(G)[0]
    s = G[k0].inverse()
    G = {e: c * s for e, c in G.items()}
    out = {}
    for e, c in G.items():
        assert c ** p == c, "descended coefficient not in F_p"
        lst = c.to_list()
        assert all(int(x) == 0 for x in lst[1:])
        out[e] = int(lst[0])
    return out


# ------------------------------------------------------------------ point counting

def count_points(G, p, k):
    """Projective points of G(X,Y,Z) = 0 over F_{p^k}."""
    if k == 1:
        n = 0

        def ev(x, y, z):
            return sum(c * pow(x, e[0], p) * pow(y, e[1], p) * pow(z, e[2], p) for e, c in G.items()) % p
        for x in range(p):
            for y in range(p):
                n += ev(x, y, 1) == 0
        for x in range(p):
            n += ev(x, 1, 0) == 0
        n += ev(1, 0, 0) == 0
        return n
    L = flint.fq_default_ctx(p, k)
    R = flint.fq_default_poly_ctx(L)
    # G(x, y, 1) = sum_j c_j(x) y^j ; c_j has F_p coefficients
    cy = {}
    for e, c in G.items():
        cy.setdefault(e[1], {})
        cy[e[1]][e[0]] = cy[e[1]].get(e[0], 0) + c
    n = 0
    q = p ** k
    elems = _elements(L, p, k)
    for x in elems:
        pw = [L.one()]
        for _ in range(4):
            pw.append(pw[-1] * x)
        coeffs = [L.zero()] * 5
        for j, d in cy.items():
            acc = L.zero()
            for i, c in d.items():
                acc += c * pw[i]
            coeffs[j] = acc
        f = R(coeffs)
        if f.is_zero():
            n += q
        else:
            n += len(f.roots())
    # Z = 0: points [x:1:0] and [1:0:0]
    c0 = [0] * 5
    for e, c in G.items():
        if e[2] == 0:
            c0[e[0]] = (c0[e[0]] + c) % p
    f0 = R([L(c) for c in c0])
    n += len(f0.roots()) if not f0.is_zero() else q
    n += 1 if c0[4] % p == 0 else 0
    return n


def _elements(L, p, k):
    for coeffs in itertools.product(range(p), repeat=k):
        yield L(list(coeffs))


def charpoly_from_counts(p, N):
    S1 = p + 1 - N[0]
    S2 = p ** 2 + 1 - N[1]
    S3 = p ** 3 + 1 - N[2]
    e1 = S1
    e2n = S1 * S1 - S2
    e3n = S1 ** 3 - 3 * S1 * S2 + 2 * S3
    integral = (e2n % 2 == 0) and (e3n % 6 == 0)
    e2, e3 = e2n // 2, e3n // 6
    P = [p ** 3, -p * p * e1, p * e2, -e3, e2, -e1, 1]
    return P, integral
