"""Smith's Proposition 4 for a pair-type kernel on a split even model over F_p
(ground truth for the positive control). Rational trigonal maps g = N/D with
g(a_k) = g(b_k) for the four pairs; Prop. 4 passes iff prod_{g(x)=t0} F(x) is a
square (Smith 2009, Lemma 4 / Prop. 4; lab: smith_prop24::eval_alpha_chi)."""


def solve_kernel(rows, p, ncols):
    """Null space of rows (mod p)."""
    m = [r[:] for r in rows]
    piv = []
    r = 0
    for c in range(ncols):
        pr = next((i for i in range(r, len(m)) if m[i][c] % p), None)
        if pr is None:
            continue
        m[r], m[pr] = m[pr], m[r]
        inv = pow(m[r][c], p - 2, p)
        m[r] = [x * inv % p for x in m[r]]
        for i in range(len(m)):
            if i != r and m[i][c] % p:
                f = m[i][c]
                m[i] = [(x - f * y) % p for x, y in zip(m[i], m[r])]
        piv.append(c)
        r += 1
    free = [c for c in range(ncols) if c not in piv]
    basis = []
    for fc in free:
        v = [0] * ncols
        v[fc] = 1
        for i, pc in enumerate(piv):
            v[pc] = (-m[i][fc]) % p
        basis.append(v)
    return basis


def polmul(a, b, p):
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            r[i + j] = (r[i + j] + x * y) % p
    return r


def peval(a, x, p):
    v = 0
    for c in reversed(a):
        v = (v * x + c) % p
    return v


def trim(a):
    a = list(a)
    while a and a[-1] == 0:
        a.pop()
    return a


def pgcd(a, b, p):
    a, b = trim(a), trim(b)
    while b:
        while len(a) >= len(b):
            f = a[-1] * pow(b[-1], p - 2, p) % p
            sh = len(a) - len(b)
            a = trim([(x - f * (b[i - sh] if i >= sh else 0)) % p for i, x in enumerate(a)])
            if not a:
                break
        a, b = b, a
    return a


def maps_for_pairing(pairs, p):
    (a1, b1), (a2, b2), (a3, b3), (a4, b4) = pairs
    # N = (x-a2)(x-b2)(g1 x - g0), D = (x-a1)(x-b1)(d1 x - d0); unknown (g1,g0,d1,d0)
    def NDvec(x):
        qn = (x - a2) * (x - b2) % p
        qd = (x - a1) * (x - b1) % p
        return [qn * x % p, (-qn) % p], [qd * x % p, (-qd) % p]
    rows = []
    for x in (a3, b3):
        n, d = NDvec(x)
        rows.append(n + [(-v) % p for v in d])
    ker = solve_kernel(rows, p, 4)
    if len(ker) != 2:
        return None
    v1, v2 = ker

    def ND_at(v, x):
        n, d = NDvec(x)
        return (n[0] * v[0] + n[1] * v[1]) % p, (d[0] * v[2] + d[1] * v[3]) % p

    # q(l, m) = N(a4)D(b4) - N(b4)D(a4), bilinear expansion
    def q(v, w):
        Na, Da = ND_at(v, a4)
        Nb, Db = ND_at(w, b4)
        return (Na * Db - ND_at(w, b4)[0] * 0) % p  # placeholder, replaced below

    def Q(v):
        Na, Da = ND_at(v, a4)
        Nb, Db = ND_at(v, b4)
        return (Na * Db - Nb * Da) % p
    # coefficients of Q(l v1 + m v2) = A l^2 + B l m + C m^2
    A = Q(v1)
    C = Q(v2)
    B = (Q([(x + y) % p for x, y in zip(v1, v2)]) - A - C) % p
    sols = []
    if A == 0 and B == 0 and C == 0:
        return "degenerate"
    # projective roots
    if A == 0:
        sols.append((1, 0))
        if B % p:
            sols.append(((-C) * pow(B, p - 2, p) % p, 1))
    else:
        disc = (B * B - 4 * A * C) % p
        if disc == 0:
            sols.append(((-B) * pow(2 * A, p - 2, p) % p, 1))
        elif pow(disc, (p - 1) // 2, p) == 1:
            r = next(x for x in range(p) if x * x % p == disc)
            for s in (r, (-r) % p):
                sols.append(((-B + s) * pow(2 * A, p - 2, p) % p, 1))
    out = []
    for (l, m) in sols:
        v = [(l * x + m * y) % p for x, y in zip(v1, v2)]
        N = polmul(polmul([(-a2) % p, 1], [(-b2) % p, 1], p), [(-v[1]) % p, v[0]], p)
        D = polmul(polmul([(-a1) % p, 1], [(-b1) % p, 1], p), [(-v[3]) % p, v[2]], p)
        N, D = trim(N), trim(D)
        if not N or not D or max(len(N), len(D)) != 4:
            continue
        if len(pgcd(N, D, p)) != 1:
            continue
        ok = True
        for (a, b) in pairs:
            if (peval(N, a, p) * peval(D, b, p) - peval(N, b, p) * peval(D, a, p)) % p:
                ok = False
        if ok:
            out.append((N, D))
    return out


def norm_mod(G, F, p):
    """det of multiplication by F in F_p[x]/(G), G monic cubic."""
    d = len(G) - 1

    def red(a):
        a = list(a)
        for k in range(len(a) - 1, d - 1, -1):
            t = a[k]
            if t:
                for j in range(d + 1):
                    a[k - d + j] = (a[k - d + j] - t * G[j]) % p
        return (a + [0] * d)[:d]
    Fr = red(F)
    cols = []
    xk = [1] + [0] * (d - 1)
    for k in range(d):
        cols.append(red(polmul(Fr, xk, p)))
        xk = [0] + xk[:-1] if False else red(polmul(xk, [0, 1], p))
    M = [[cols[j][i] for j in range(d)] for i in range(d)]
    # det mod p
    det = 1
    M = [r[:] for r in M]
    for c in range(d):
        pr = next((i for i in range(c, d) if M[i][c] % p), None)
        if pr is None:
            return 0
        if pr != c:
            M[c], M[pr] = M[pr], M[c]
            det = -det
        det = det * M[c][c] % p
        inv = pow(M[c][c], p - 2, p)
        for i in range(c + 1, d):
            f = M[i][c] * inv % p
            M[i] = [(x - f * y) % p for x, y in zip(M[i], M[c])]
    return det % p


def prop4_symbols(N, D, F, p, maxt=None):
    """Legendre symbols of prod_{g(x)=t0} F(x) over usable t0 in F_p."""
    syms = []
    for t0 in range(p):
        G = trim([(x - t0 * y) % p for x, y in zip(N + [0] * 4, D + [0] * 4)])
        if len(G) != 4:
            continue
        inv = pow(G[-1], p - 2, p)
        G = [x * inv % p for x in G]
        dG = [(i * G[i]) % p for i in range(1, 4)]
        if len(pgcd(G, dG, p)) != 1:
            continue
        v = norm_mod(G, F, p)
        if v == 0:
            continue
        syms.append(1 if pow(v, (p - 1) // 2, p) == 1 else -1)
        if maxt and len(syms) >= maxt:
            break
    return syms
