"""Uncounted polynomial helpers over F_p (lowest degree first) and Cantor."""


def trim(a):
    a = list(a)
    while a and a[-1] == 0:
        a.pop()
    return a


def deg(a):
    return len(a) - 1


def padd(a, b, p):
    n = max(len(a), len(b))
    return trim([((a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)) % p for i in range(n)])


def psub(a, b, p):
    n = max(len(a), len(b))
    return trim([((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(n)])


def pneg(a, p):
    return trim([(-x) % p for x in a])


def pmul(a, b, p):
    if not a or not b:
        return []
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                r[i + j] += x * y
    return trim([v % p for v in r])


def pscale(a, k, p):
    return trim([x * k % p for x in a])


def divrem(a, b, p):
    a = trim(a)
    b = trim(b)
    assert b, "division by zero polynomial"
    if len(a) < len(b):
        return [], a
    ilc = pow(b[-1], p - 2, p)
    r = list(a)
    q = [0] * (len(a) - len(b) + 1)
    for i in range(len(a) - len(b), -1, -1):
        co = r[i + len(b) - 1] * ilc % p
        q[i] = co
        if co:
            for j, y in enumerate(b):
                r[i + j] = (r[i + j] - co * y) % p
    return trim(q), trim(r)


def monic(a, p):
    a = trim(a)
    if not a:
        return a
    return pscale(a, pow(a[-1], p - 2, p), p)


def xgcd(a, b, p):
    """Return (d, s, t) with d monic and d = s*a + t*b."""
    r0, r1 = trim(a), trim(b)
    s0, s1 = [1], []
    t0, t1 = [], [1]
    while r1:
        q, r = divrem(r0, r1, p)
        r0, r1 = r1, r
        s0, s1 = s1, psub(s0, pmul(q, s1, p), p)
        t0, t1 = t1, psub(t0, pmul(q, t1, p), p)
    il = pow(r0[-1], p - 2, p)
    return pscale(r0, il, p), pscale(s0, il, p), pscale(t0, il, p)


def ev(a, x, p):
    acc = 0
    for c in reversed(a):
        acc = (acc * x + c) % p
    return acc


def pderiv(a, p):
    return trim([(i * a[i]) % p for i in range(1, len(a))])


def pgcd(a, b, p):
    a, b = trim(a), trim(b)
    while b:
        a, b = b, divrem(a, b, p)[1]
    return monic(a, p)


# ---- Cantor on y^2 = h, h monic odd degree 2g+1 -----------------------------
IDENT = ([1], [])


def point(X, Y, p):
    return ([(-X) % p, 1], trim([Y % p]))


def negate(D, p):
    return (D[0], pneg(D[1], p))


def cantor(A, B, h, g, p):
    u1, v1 = A
    u2, v2 = B
    if u1 == [1]:
        return B
    if u2 == [1]:
        return A
    d1, e1, e2 = xgcd(u1, u2, p)
    d, c1, c2 = xgcd(d1, padd(v1, v2, p), p)
    s1 = pmul(c1, e1, p)
    s2 = pmul(c1, e2, p)
    u, r = divrem(pmul(u1, u2, p), pmul(d, d, p), p)
    assert not r
    num = padd(padd(pmul(pmul(s1, u1, p), v2, p), pmul(pmul(s2, u2, p), v1, p), p),
               pmul(c2, padd(pmul(v1, v2, p), h, p), p), p)
    num, r = divrem(num, d, p)
    assert not r
    v = divrem(num, u, p)[1]
    while len(u) - 1 > g:
        u2_, r = divrem(psub(h, pmul(v, v, p), p), u, p)
        assert not r
        u = monic(u2_, p)
        v = divrem(pneg(v, p), u, p)[1]
    return (monic(u, p), v)


def smul(D, k, h, g, p):
    R = IDENT
    for bit in bin(k)[2:] if k > 0 else "":
        R = cantor(R, R, h, g, p)
        if bit == "1":
            R = cantor(R, D, h, g, p)
    return R


def div_class(D, h, g, p):
    """D: list of ((X,Y), m)."""
    R = IDENT
    for (P, m) in D:
        pt = point(P[0], P[1], p)
        if m < 0:
            pt = negate(pt, p)
        for _ in range(abs(m)):
            R = cantor(R, pt, h, g, p)
    return R
