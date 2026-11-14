"""Validator's own arithmetic: F_p, elliptic curves y^2 = x^3 + a2 x^2 + a4 x + a6
(affine, None = infinity), genus-2 Cantor arithmetic on y^2 = f(x), deg f = 5
(Mumford pairs, lowest degree first, u monic), and Velu isogenies.
Written by the validator; shares no code with src/cryptanalysis/chained_transfer."""

def is_prime(n):
    if n < 2: return False
    small = [2,3,5,7,11,13,17,19,23,29,31,37]
    for q in small:
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in small:
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else: return False
    return True

# ---------------- elliptic curves ----------------
class EC:
    def __init__(s, p, a2, a4, a6):
        s.p, s.a2, s.a4, s.a6 = p, a2 % p, a4 % p, a6 % p
    def rhs(s, x):
        p = s.p
        return (x * x * x + s.a2 * x * x + s.a4 * x + s.a6) % p
    def on(s, P):
        if P is None: return True
        x, y = P
        return 0 <= x < s.p and 0 <= y < s.p and (y * y - s.rhs(x)) % s.p == 0
    def neg(s, P):
        return None if P is None else (P[0], (-P[1]) % s.p)
    def add(s, P, Q):
        p = s.p
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0: return None
            lam = (3 * x1 * x1 + 2 * s.a2 * x1 + s.a4) * pow(2 * y1, -1, p) % p
        else:
            lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (lam * lam - s.a2 - x1 - x2) % p
        y3 = (lam * (x1 - x3) - y1) % p
        return (x3, y3)
    def mul(s, k, P):
        if k < 0: return s.mul(-k, s.neg(P))
        R = None; A = P
        while k:
            if k & 1: R = s.add(R, A)
            A = s.add(A, A); k >>= 1
        return R
    def count(s):
        """#E(F_p) by Euler's criterion (small p only)."""
        p = s.p; n = 1
        for x in range(p):
            r = s.rhs(x)
            if r == 0: n += 1
            elif pow(r, (p - 1) // 2, p) == 1: n += 2
        return n

def velu(E, K, ell):
    """Velu isogeny with kernel <K> of prime order ell on y^2=x^3+a2x^2+a4x+a6
    (a1=a3=0). Returns (codomain EC, map function)."""
    p = E.p
    pts = []
    Q = K
    for _ in range(ell - 1):
        pts.append(Q); Q = E.add(Q, K)
    assert Q is None, "kernel order"
    # S: 2-torsion points plus one of each {Q,-Q}
    S, seen = [], set()
    for Q in pts:
        if Q in seen: continue
        seen.add(Q); seen.add(E.neg(Q)); S.append(Q)
    t = w = 0; data = []
    for (xq, yq) in S:
        gx = (3 * xq * xq + 2 * E.a2 * xq + E.a4) % p
        gy = (-2 * yq) % p
        if yq % p == 0:  # 2-torsion
            tq = gx; uq = 0
        else:
            tq = 2 * gx % p; uq = gy * gy % p
        t = (t + tq) % p; w = (w + uq + xq * tq) % p
        data.append((xq, yq, tq, uq, gx, gy))
    A4 = (E.a4 - 5 * t) % p
    A6 = (E.a6 - 4 * E.a2 * t - 7 * w) % p
    E2 = EC(p, E.a2, A4, A6)
    def phi(P):
        if P is None: return None
        if P in seen or P == K: return None
        x, y = P
        X, Y = x, y
        for (xq, yq, tq, uq, gx, gy) in data:
            d = (x - xq) % p; di = pow(d, -1, p)
            X = (X + tq * di + uq * di * di) % p
            Y = (Y - uq * 2 * y * pow(di, 3, p) - tq * (y - yq) * di * di + gx * gy * di * di) % p
        return (X, Y)
    return E2, phi

# ---------------- polynomials over F_p (lists, lowest degree first) ----------------
def ptrim(a):
    a = list(a)
    while a and a[-1] == 0: a.pop()
    return a
def padd(a, b, p):
    n = max(len(a), len(b))
    return ptrim([((a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)) % p for i in range(n)])
def psub(a, b, p):
    n = max(len(a), len(b))
    return ptrim([((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(n)])
def pmul(a, b, p):
    if not a or not b: return []
    c = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                c[i + j] = (c[i + j] + x * y) % p
    return ptrim(c)
def pscale(a, c, p): return ptrim([x * c % p for x in a])
def pdivmod(a, b, p):
    a = ptrim(a); b = ptrim(b)
    assert b, "division by zero poly"
    q = [0] * max(0, len(a) - len(b) + 1); r = list(a)
    inv = pow(b[-1], -1, p)
    while len(r) >= len(b) and r:
        c = r[-1] * inv % p; d = len(r) - len(b)
        q[d] = c
        for i, y in enumerate(b): r[i + d] = (r[i + d] - c * y) % p
        r = ptrim(r)
    return ptrim(q), r
def pmod(a, b, p): return pdivmod(a, b, p)[1]
def pmonic(a, p):
    a = ptrim(a); return pscale(a, pow(a[-1], -1, p), p)
def pxgcd(a, b, p):
    """returns (d, s, t) with s*a + t*b = d, d monic (or [] if both zero)."""
    r0, r1 = ptrim(a), ptrim(b); s0, s1 = [1], []; t0, t1 = [], [1]
    while r1:
        q, r = pdivmod(r0, r1, p)
        r0, r1 = r1, r
        s0, s1 = s1, psub(s0, pmul(q, s1, p), p)
        t0, t1 = t1, psub(t0, pmul(q, t1, p), p)
    if not r0: return [], [], []
    c = pow(r0[-1], -1, p)
    return pscale(r0, c, p), pscale(s0, c, p), pscale(t0, c, p)
def peval(a, x, p):
    r = 0
    for c in reversed(a): r = (r * x + c) % p
    return r

# ---------------- genus 2, imaginary model, Cantor ----------------
class Jac2:
    def __init__(s, p, f):
        s.p = p; s.f = ptrim([c % p for c in f])
        assert len(s.f) == 6, "deg f = 5"
    ID = ([1], [])
    def valid(s, D):
        u, v = ptrim(D[0]), ptrim(D[1])
        p = s.p
        if not u or u[-1] != 1 or len(u) - 1 > 2: return False
        if len(v) >= len(u): return False
        return not pmod(psub(s.f, pmul(v, v, p), p), u, p)
    def norm(s, D):
        return (ptrim(D[0]), ptrim(D[1]))
    def compose(s, D1, D2):
        p = s.p
        u1, v1 = D1; u2, v2 = D2
        d1, e1, e2 = pxgcd(u1, u2, p)
        d, c1, c2 = pxgcd(d1, padd(v1, v2, p), p)
        s1 = pmul(c1, e1, p); s2 = pmul(c1, e2, p); s3 = c2
        u = pdivmod(pmul(u1, u2, p), pmul(d, d, p), p)[0]
        num = padd(padd(pmul(pmul(s1, u1, p), v2, p), pmul(pmul(s2, u2, p), v1, p), p),
                   pmul(s3, padd(pmul(v1, v2, p), s.f, p), p), p)
        v = pdivmod(num, d, p)[0]
        v = pmod(v, u, p)
        # reduce
        while len(u) - 1 > 2:
            u = pdivmod(psub(s.f, pmul(v, v, p), p), u, p)[0]
            v = pmod(pscale(v, p - 1, p), u, p)
        u = pmonic(u, p)
        v = pmod(v, u, p)
        return (u, v)
    def add(s, D1, D2): return s.compose(s.norm(D1), s.norm(D2))
    def neg(s, D): return (D[0], pscale(D[1], s.p - 1, s.p))
    def mul(s, k, D):
        if k < 0: return s.mul(-k, s.neg(D))
        R = Jac2.ID; A = s.norm(D)
        while k:
            if k & 1: R = s.add(R, A)
            A = s.add(A, A); k >>= 1
        return R
    def is_id(s, D): return ptrim(D[0]) == [1] and not ptrim(D[1])

def bielliptic(p, c):
    """C: y^2 = F(x^2), F = c0 + c1 X + c2 X^2 + c3 X^3. Returns (x0, quintic g,
    E1 monic model, E2 monic model). x0 = smallest rational root of F(x^2) in 1..p-1."""
    c = [v % p for v in c]
    f = [c[0], 0, c[1], 0, c[2], 0, c[3]]
    x0 = next(x for x in range(1, p) if peval(f, x, p) == 0)
    # g(t) = t^6 f(x0 + 1/t) = sum f_j (x0 t + 1)^j t^(6-j)
    g = [0] * 7; powj = [1]
    for j, fj in enumerate(f):
        if j: powj = pmul(powj, [1, x0], p) or [0]
        for i, a in enumerate(powj):
            g[i + 6 - j] = (g[i + 6 - j] + fj * a) % p
    assert g[6] == 0
    E1 = EC(p, c[2], c[1] * c[3], c[0] * c[3] * c[3])
    E2 = EC(p, c[1], c[2] * c[0], c[3] * c[0] * c[0])
    return x0, g[:6], E1, E2

def pullback_e1(p, c, x0, Q):
    """phi1^*([Q - O]) on the quintic model, for an affine Q=(X',Y') on the monic E1
    with X = X'/c3 != x0^2 (generic fibre). Closed form over F_p (no square roots)."""
    if Q is None: return Jac2.ID
    c3i = pow(c[3] % p, -1, p)
    X = Q[0] * c3i % p; Y = Q[1] * c3i % p
    den = (x0 * x0 - X) % p
    assert den != 0 and X != 0, "non-generic fibre"
    di = pow(den, -1, p)
    a1 = 2 * x0 * di % p   # u = t^2 + a1 t + a0
    a0 = di
    v1 = Y * ((a1 * a1 - a0) % p) % p
    v0 = Y * (a1 * a0 % p) % p
    return (ptrim([a0, a1, 1]), ptrim([v0, v1]))

class JacG(Jac2):
    """Genus-g imaginary model y^2 = f(x), deg f = 2g+1 (validator's Cantor)."""
    def __init__(s, p, f, g):
        s.p = p; s.f = ptrim([c % p for c in f]); s.g = g
        assert len(s.f) == 2 * g + 2, "deg f = 2g+1"
    def valid(s, D):
        u, v = ptrim(D[0]), ptrim(D[1]); p = s.p
        if not u or u[-1] != 1 or len(u) - 1 > s.g: return False
        if len(v) >= len(u): return False
        return not pmod(psub(s.f, pmul(v, v, p), p), u, p)
    def compose(s, D1, D2):
        p = s.p
        u1, v1 = D1; u2, v2 = D2
        d1, e1, e2 = pxgcd(u1, u2, p)
        d, c1, c2 = pxgcd(d1, padd(v1, v2, p), p)
        s1 = pmul(c1, e1, p); s2 = pmul(c1, e2, p); s3 = c2
        u = pdivmod(pmul(u1, u2, p), pmul(d, d, p), p)[0]
        num = padd(padd(pmul(pmul(s1, u1, p), v2, p), pmul(pmul(s2, u2, p), v1, p), p),
                   pmul(s3, padd(pmul(v1, v2, p), s.f, p), p), p)
        v = pmod(pdivmod(num, d, p)[0], u, p)
        while len(u) - 1 > s.g:
            u = pdivmod(psub(s.f, pmul(v, v, p), p), u, p)[0]
            v = pmod(pscale(v, p - 1, p), u, p)
        u = pmonic(u, p); v = pmod(v, u, p)
        return (u, v)

def tonelli(a, p):
    a %= p
    if a == 0: return 0
    if p % 4 == 3: return pow(a, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, R = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2 * t2 % p; i += 1
        b = pow(c, 1 << (m - i - 1), p); m, c, t, R = i, b * b % p, t * b * b % p, R * b % p
    assert R * R % p == a
    return R

def pullback_e1_any(p, c, x0, Q, E1, J, aux=None):
    """phi1^*(Q) for any Q on the monic E1, using the homomorphism property for a
    non-generic fibre (X(Q) = x0^2 or X(Q) = 0): phi1^*(Q) = phi1^*(Q + P') - phi1^*(P')
    with P' a multiple of `aux` such that both fibres are generic (added restart 3)."""
    try:
        return pullback_e1(p, c, x0, Q)
    except AssertionError:
        pass
    assert aux is not None
    for m in range(1, 200):
        P2 = E1.mul(m, aux)
        if P2 is None: continue
        S = E1.add(Q, P2)
        try:
            a = pullback_e1(p, c, x0, S); b = pullback_e1(p, c, x0, P2)
        except AssertionError:
            continue
        return J.add(a, J.neg(b))
    raise AssertionError("no generic auxiliary point")
