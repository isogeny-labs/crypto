"""Polynomials over F_p (SPEC section 4). Lists low degree first, trimmed."""
import ff

def trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a

def padd(a, b):
    p = ff.p
    n = max(len(a), len(b))
    r = [((a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)) % p for i in range(n)]
    return trim(r)

def psub(a, b):
    p = ff.p
    n = max(len(a), len(b))
    r = [((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(n)]
    return trim(r)

def pneg(a):
    p = ff.p
    return [(-c) % p for c in a]

def pmul(a, b):
    if not a or not b:
        return []
    p = ff.p
    ff.cnt_mul(len(a) * len(b))
    res = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                res[i + j] += x * y
    return trim([c % p for c in res])

def pdivmod(a, b):
    p = ff.p
    db = len(b) - 1
    da = len(a) - 1
    if da < db:
        return [], list(a)
    nq = da - db + 1
    lead = b[-1]
    inv = None
    if lead != 1:
        inv = ff.finv(lead)
        ff.cnt_mul(nq)
    ff.cnt_mul(nq * db)
    r = list(a)
    q = [0] * nq
    for i in range(nq - 1, -1, -1):
        c = r[i + db]
        if c:
            if inv is not None:
                c = c * inv % p
            q[i] = c
            for j in range(db):
                bj = b[j]
                if bj:
                    r[i + j] = (r[i + j] - c * bj) % p
    rem = trim(r[:db])
    return trim(q), rem

def pmonic(a):
    if not a or a[-1] == 1:
        return list(a)
    p = ff.p
    inv = ff.finv(a[-1])
    ff.cnt_mul(len(a) - 1)
    r = [c * inv % p for c in a[:-1]] + [1]
    return r

def xgcd(a, b):
    p = ff.p
    r0, r1 = a, b
    s0, s1 = [1], []
    t0, t1 = [], [1]
    while r1:
        q, rem = pdivmod(r0, r1)
        r0, r1 = r1, rem
        s0, s1 = s1, psub(s0, pmul(q, s1))
        t0, t1 = t1, psub(t0, pmul(q, t1))
    d, s, t = r0, s0, t0
    if d and d[-1] != 1:
        inv = ff.finv(d[-1])
        ff.cnt_mul(len(d) - 1 + len(s) + len(t))
        d = [c * inv % p for c in d[:-1]] + [1]
        s = [c * inv % p for c in s]
        t = [c * inv % p for c in t]
    return d, s, t

def peval(a, x):
    p = ff.p
    if not a:
        return 0
    ff.cnt_mul(len(a) - 1)
    r = 0
    for c in reversed(a):
        r = (r * x + c) % p
    return r
