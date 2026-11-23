#!/usr/bin/env python3
"""L08: Diem line relations on nodal plane models (genus 4 quintic; d=4 control; d=6 optional).
Implements SPEC.md exactly. Python 3 standard library only. Single thread.
Status of all outputs: [pcc:unverified]."""
import sys, math, json, random, argparse, glob, itertools

P = 2  # current prime (module level, set by set_p)


def set_p(p):
    global P
    P = p


class Ctr:
    __slots__ = ("mul", "inv")

    def __init__(self):
        self.mul = 0
        self.inv = 0


def fpm(C):
    return C.mul + 10 * C.inv


def M(a, b, C):
    C.mul += 1
    return a * b % P


def INV(a, C):
    C.inv += 1
    return pow(a, P - 2, P)


def powmod(a, e, C):
    if e == 0:
        return 1
    r = a
    for bit in bin(e)[3:]:
        r = M(r, r, C)
        if bit == "1":
            r = M(r, a, C)
    return r


def legendre(a, C):
    r = powmod(a % P, (P - 1) // 2, C)
    if r == 1:
        return 1
    if r == 0:
        return 0
    return -1


def sqrt_ts(a, C):
    q = P - 1
    s = 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    T = Ctr()
    while legendre(z, T) != -1:
        z += 1
    c = powmod(z, q, C)
    x = powmod(a, (q + 1) // 2, C)
    t = powmod(a, q, C)
    mloop = s
    while t != 1:
        t2 = t
        i = 0
        while t2 != 1:
            t2 = M(t2, t2, C)
            i += 1
        b = c
        for _ in range(mloop - i - 1):
            b = M(b, b, C)
        x = M(x, b, C)
        c = M(b, b, C)
        t = M(t, c, C)
        mloop = i
    return x


# ---------------- polynomials (low -> high, trimmed) ----------------
def trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def pmul(a, b, C):
    if not a or not b:
        return []
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            r[i + j] += M(x, y, C)
    return trim([v % P for v in r])


def pmonic(a, C):
    if not a:
        return []
    if a[-1] == 1:
        return list(a)
    il = INV(a[-1], C)
    r = [M(c, il, C) for c in a[:-1]]
    r.append(1)
    return r


def prem(a, h, C):
    a = list(a)
    k = len(h) - 1
    for i in range(len(a) - 1, k - 1, -1):
        q = a[i]
        if q != 0:
            for j in range(k):
                a[i - k + j] = (a[i - k + j] - M(q, h[j], C)) % P
            a[i] = 0
    return trim(a[:k] if len(a) > k else a)


def pdivexact(a, h, C):
    a = list(a)
    k = len(h) - 1
    quo = [0] * max(0, len(a) - k)
    for i in range(len(a) - 1, k - 1, -1):
        q = a[i]
        quo[i - k] = q
        if q != 0:
            for j in range(k):
                a[i - k + j] = (a[i - k + j] - M(q, h[j], C)) % P
            a[i] = 0
    assert not trim(a), "pdivexact: nonzero remainder"
    return trim(quo)


def pmulmod(a, b, h, C):
    return prem(pmul(a, b, C), h, C)


def ppowmod(base, e, h, C):
    br = prem(base, h, C)
    r = br
    for bit in bin(e)[3:]:
        r = pmulmod(r, r, h, C)
        if bit == "1":
            r = pmulmod(r, br, h, C)
    return r


def pgcd(a, b, C):
    while b:
        b = pmonic(b, C)
        a, b = b, prem(a, b, C)
    return pmonic(a, C) if a else []


def roots_edf(h, rng, C):
    n = len(h) - 1
    if n <= 0:
        return []
    if n == 1:
        return [(-h[0]) % P]
    while True:
        delta = rng.randrange(P)
        w = ppowmod([delta, 1], (P - 1) // 2, h, C)
        w = list(w) if w else [0]
        w[0] = (w[0] - 1) % P
        w = trim(w)
        gg = pgcd(h, w, C)
        dg = len(gg) - 1
        if 0 < dg < n:
            return roots_edf(gg, rng, C) + roots_edf(pdivexact(h, gg, C), rng, C)


def rational_roots(poly, rng, C):
    poly = trim(list(poly))
    if len(poly) <= 1:
        return []
    f = pmonic(poly, C)
    w = ppowmod([0, 1], P, f, C)
    w = list(w)
    while len(w) < 2:
        w.append(0)
    w[1] = (w[1] - 1) % P
    w = trim(w)
    h = pgcd(f, w, C) if w else list(f)
    return roots_edf(h, rng, C)


def sylvester_det(A, B, C):
    nA = len(A) - 1
    nB = len(B) - 1
    n = nA + nB
    mat = [[0] * n for _ in range(n)]
    for r in range(nB):
        for k, c in enumerate(reversed(A)):
            mat[r][r + k] = c
    for s in range(nA):
        for k, c in enumerate(reversed(B)):
            mat[nB + s][s + k] = c
    sign = 1
    res = 1
    for col in range(n):
        pr = None
        for r in range(col, n):
            if mat[r][col] % P != 0:
                pr = r
                break
        if pr is None:
            return 0
        if pr != col:
            mat[pr], mat[col] = mat[col], mat[pr]
            sign = -sign
        piv = mat[col][col]
        ip = INV(piv, C)
        res = M(res, piv, C)
        for rr in range(col + 1, n):
            if mat[rr][col] != 0:
                f = M(mat[rr][col], ip, C)
                for c in range(col, n):
                    mat[rr][c] = (mat[rr][c] - M(f, mat[col][c], C)) % P
    return (sign * res) % P


# ---------------- curves ----------------
def mons(d):
    return [(t - j, j) for t in range(d + 1) for j in range(t + 1)]


def genus_of(d, m):
    return (d - 1) * (d - 2) // 2 - m


def gen_coeffs(d, m, p, s):
    """returns dict a or reason string. Uncounted."""
    set_p(p)
    ms = mons(d)
    rng = random.Random(f"L08-curve-d{d}-m{m}-p{p}-s{s}")
    a = {}
    for mo in ms:
        a[mo] = rng.randrange(1, p) if mo == (0, d) else rng.randrange(p)
    if m == 0:
        return a
    rows = []
    for k in range(m):
        x = k
        y = k * k % p
        fr, fxr, fyr = [], [], []
        for (i, j) in ms:
            fr.append(pow(x, i, p) * pow(y, j, p) % p)
            fxr.append(i * pow(x, i - 1, p) * pow(y, j, p) % p if i > 0 else 0)
            fyr.append(j * pow(x, i, p) * pow(y, j - 1, p) % p if j > 0 else 0)
        rows += [fr, fxr, fyr]
    ncols = len(ms)
    r = 0
    pivcols = []
    for col in range(ncols - 1):
        pr = None
        for rr in range(r, len(rows)):
            if rows[rr][col] % p:
                pr = rr
                break
        if pr is None:
            continue
        rows[r], rows[pr] = rows[pr], rows[r]
        iv = pow(rows[r][col], p - 2, p)
        rows[r] = [v * iv % p for v in rows[r]]
        for rr in range(len(rows)):
            if rr != r and rows[rr][col]:
                f = rows[rr][col]
                rows[rr] = [(v - f * w) % p for v, w in zip(rows[rr], rows[r])]
        pivcols.append(col)
        r += 1
        if r == len(rows):
            break
    if len(pivcols) < 3 * m:
        return "rank"
    pset = set(pivcols)
    # use stored a values of the non-pivot columns (drawn before overwriting)
    orig = [a[mo] for mo in ms]
    for ri, c in enumerate(pivcols):
        sacc = 0
        for c2 in range(ncols):
            if c2 not in pset:
                sacc += rows[ri][c2] * orig[c2]
        a[ms[c]] = (-sacc) % p
    return a


def solve_mod(Mat, rhs, p):
    n = len(Mat)
    A = [list(Mat[i]) + [rhs[i]] for i in range(n)]
    for col in range(n):
        pr = None
        for r in range(col, n):
            if A[r][col] % p:
                pr = r
                break
        if pr is None:
            raise ValueError("singular")
        A[col], A[pr] = A[pr], A[col]
        iv = pow(A[col][col], p - 2, p)
        A[col] = [v * iv % p for v in A[col]]
        for r in range(n):
            if r != col and A[r][col]:
                f = A[r][col]
                A[r] = [(v - f * w) % p for v, w in zip(A[r], A[col])]
    return [A[i][n] for i in range(n)]


def synth_div_xk(S, k, p):
    """divide poly S (low->high) by (x-k); return (quotient, remainder)."""
    n = len(S) - 1
    if n < 0:
        return [], 0
    q = [0] * n
    carry = 0
    for i in range(n, 0, -1):
        carry = (S[i] + k * carry) % p
        q[i - 1] = carry
    rem = (S[0] + k * carry) % p
    return q, rem


def genus_test(a, d, m, p):
    """uncounted. returns None if admitted else reason."""
    set_p(p)
    D = d * (d - 1)
    T0 = Ctr()
    vals = []
    for x0 in range(D + 1):
        A = [0] * (d + 1)
        for (i, j), c in a.items():
            A[j] = (A[j] + c * pow(x0, i, p)) % p
        B = [(j * A[j]) % p for j in range(1, d + 1)]
        vals.append(sylvester_det(A, B, T0))
    V = [[pow(x0, j, p) for j in range(D + 1)] for x0 in range(D + 1)]
    S = trim(solve_mod(V, vals, p))
    if len(S) - 1 != D:
        return "degS"
    for k in range(m):
        S, r1 = synth_div_xk(S, k, p)
        if r1 != 0:
            return "node_mult"
        S, r2 = synth_div_xk(S, k, p)
        if r2 != 0:
            return "node_mult"
    T = S
    for k in range(m):
        if sum(T[i] * pow(k, i, p) for i in range(len(T))) % p == 0:
            return "node_extra"
    Tp = trim([(i * T[i]) % p for i in range(1, len(T))])
    g = pgcd(T, Tp, T0)
    if len(g) - 1 >= 1:
        return "T_sqfree"
    g_ = genus_of(d, m)
    if (len(T) - 1 - 2 * d + 2) != 2 * g_:
        return "genus_mismatch"
    return None


_adm_cache = {}


def admitted(d, m, p, upto):
    """list of (seed, a, seeds_tried, reject_counts) with at least upto+1 entries (or all in 2000 seeds)."""
    key = (d, m, p)
    st = _adm_cache.setdefault(key, {"list": [], "next": 1, "rej": {}})
    while len(st["list"]) <= upto and st["next"] <= 2000:
        s = st["next"]
        st["next"] += 1
        a = gen_coeffs(d, m, p, s)
        if isinstance(a, str):
            reason = a
        else:
            reason = genus_test(a, d, m, p)
        if reason is None:
            st["list"].append((s, a, s, dict(st["rej"])))
        else:
            st["rej"][reason] = st["rej"].get(reason, 0) + 1
    return st["list"]


class Curve:
    pass


def build_curve(d, m, p, s, a, enum_ctr=None):
    set_p(p)
    cv = Curve()
    cv.d, cv.m, cv.p, cv.s, cv.a = d, m, p, s, a
    cv.g = genus_of(d, m)
    cv.mons = mons(d)
    cv.ml = [(i, j, d - i - j, a[(i, j)]) for (i, j) in cv.mons]
    cv.nodes = [(k, k * k % p) for k in range(m)]
    E = enum_ctr if enum_ctr is not None else Ctr()
    rng = random.Random(f"L08-enum-d{d}-m{m}-p{p}-s{s}")
    pts = []
    nodeset = set(cv.nodes)
    byj = {}
    for (i, j) in cv.mons:
        byj.setdefault(j, []).append(i)
    for x0 in range(p):
        xp = [1, x0] + [0] * (d - 1)
        for i in range(2, d + 1):
            xp[i] = M(xp[i - 1], x0, E)
        cj = []
        for j in range(d + 1):
            sacc = 0
            for i in byj.get(j, []):
                sacc += a[(i, j)] if i == 0 else M(a[(i, j)], xp[i], E)
            cj.append(sacc % p)
        for y0 in rational_roots(cj, rng, E):
            if (x0, y0) in nodeset:
                continue
            pts.append((x0, y0, 1))
    inf = [a[(d - j, j)] for j in range(d + 1)]
    for mm in rational_roots(inf, rng, E):
        pts.append((1, mm, 0))
    pts.sort()
    cv.pts = pts
    cv.index = {pt: i for i, pt in enumerate(pts)}
    cv.N = len(pts)
    cv.enum = E
    return cv


def evalF(cv, Q, C):
    d = cv.d
    X, Y, Z = Q
    Xp = [1, X] + [0] * (d - 1)
    Yp = [1, Y] + [0] * (d - 1)
    Zp = [1, Z] + [0] * (d - 1)
    for k in range(2, d + 1):
        Xp[k] = M(Xp[k - 1], X, C)
        Yp[k] = M(Yp[k - 1], Y, C)
        Zp[k] = M(Zp[k - 1], Z, C)
    s = 0
    for (i, j, k, av) in cv.ml:
        t = M(av, Xp[i], C)
        t = M(t, Yp[j], C)
        t = M(t, Zp[k], C)
        s += t
    return s % P


def plain_F_grad(cv, Pt):
    p = cv.p
    X, Y, Z = Pt
    f = fx = fy = fz = 0
    for (i, j, k, av) in cv.ml:
        f += av * pow(X, i, p) * pow(Y, j, p) * pow(Z, k, p)
        if i:
            fx += av * i * pow(X, i - 1, p) * pow(Y, j, p) * pow(Z, k, p)
        if j:
            fy += av * j * pow(X, i, p) * pow(Y, j - 1, p) * pow(Z, k, p)
        if k:
            fz += av * k * pow(X, i, p) * pow(Y, j, p) * pow(Z, k - 1, p)
    return f % p, fx % p, fy % p, fz % p


def verify_rel(cv, L, idxs):
    """returns (g1_ok, g2_ok). uncounted."""
    p = cv.p
    g2 = len(set(idxs)) == len(idxs)
    g1 = True
    for i in idxs:
        Pt = cv.pts[i]
        f, fx, fy, fz = plain_F_grad(cv, Pt)
        if f != 0:
            g1 = False
        if (L[0] * Pt[0] + L[1] * Pt[1] + L[2] * Pt[2]) % p != 0:
            g1 = False
        if fx == 0 and fy == 0 and fz == 0:
            g1 = False
        if Pt[2] == 1 and (Pt[0], Pt[1]) in cv.nodes:
            g1 = False
    return g1, g2


# ---------------- union-find ----------------
GROUND = -1


class UF:
    def __init__(self):
        self.parent = {}
        self.parity = {}
        self.size = {}
        self.E = {}
        self.V = {}
        self.grounded = {}
        self.bip = {}
        self.total = 0

    def add_vertex(self, x):
        if x in self.parent:
            return
        self.parent[x] = x
        self.parity[x] = 0
        self.size[x] = 1
        self.E[x] = 0
        self.V[x] = 0 if x == GROUND else 1
        self.grounded[x] = (x == GROUND)
        self.bip[x] = True

    def find(self, x):
        path = []
        orig = x
        while self.parent[x] != x:
            path.append(x)
            x = self.parent[x]
        root = x
        acc = 0
        for node in reversed(path):
            acc ^= self.parity[node]
            self.parity[node] = acc
            self.parent[node] = root
        return root, (self.parity[orig] if orig != root else 0)

    def contrib(self, r):
        return self.E[r] - self.V[r] + (1 if (self.bip[r] and not self.grounded[r]) else 0)

    def add_edge(self, x, y):
        self.add_vertex(x)
        self.add_vertex(y)
        rx, px = self.find(x)
        ry, py = self.find(y)
        if rx == ry:
            self.total -= self.contrib(rx)
            self.E[rx] += 1
            if px == py:
                self.bip[rx] = False
            self.total += self.contrib(rx)
        else:
            self.total -= self.contrib(rx) + self.contrib(ry)
            if self.size[rx] < self.size[ry]:
                child, big = rx, ry
            else:
                child, big = ry, rx
            self.parent[child] = big
            self.parity[child] = px ^ py ^ 1
            self.E[big] = self.E[rx] + self.E[ry] + 1
            self.V[big] = self.V[rx] + self.V[ry]
            self.grounded[big] = self.grounded[rx] or self.grounded[ry]
            self.bip[big] = self.bip[rx] and self.bip[ry]
            self.size[big] = self.size[rx] + self.size[ry]
            self.total += self.contrib(big)


# ---------------- pair order ----------------
def is_prime(n):
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True


def next_prime(n):
    while not is_prime(n):
        n += 1
    return n


def unrank(k):
    j = (1 + math.isqrt(1 + 8 * k)) // 2
    while j * (j - 1) // 2 > k:
        j -= 1
    while (j + 1) * j // 2 <= k:
        j += 1
    return k - j * (j - 1) // 2, j


def pair_ks(Pm, rng):
    Mp = next_prime(max(Pm, 2))
    A = rng.randrange(1, Mp)
    B = rng.randrange(0, Mp)
    for t in range(Mp):
        k = (A * t + B) % Mp
        if k >= Pm:
            continue
        yield k


_vinv_cache = {}


def vinv(d, p):
    key = (d, p)
    if key not in _vinv_cache:
        n = d - 1
        V = [[pow(u, r, p) for r in range(n)] for u in range(1, d)]
        # inverse via Gauss-Jordan
        A = [V[i] + [1 if i == j else 0 for j in range(n)] for i in range(n)]
        for col in range(n):
            pr = next(r for r in range(col, n) if A[r][col] % p)
            A[col], A[pr] = A[pr], A[col]
            iv = pow(A[col][col], p - 2, p)
            A[col] = [v * iv % p for v in A[col]]
            for r in range(n):
                if r != col and A[r][col]:
                    f = A[r][col]
                    A[r] = [(v - f * w) % p for v, w in zip(A[r], A[col])]
        _vinv_cache[key] = [row[n:] for row in A]
    return _vinv_cache[key]


# ---------------- relation collection ----------------
def collect(cv, arm, cF, rep, needed_override=None):
    set_p(cv.p)
    d, m, p, s, N = cv.d, cv.m, cv.p, cv.s, cv.N
    n = d - 2
    C = Ctr()
    invu = {u: INV(u, C) for u in range(1, d)}
    Vinv = vinv(d, p)
    fb = random.Random(f"L08-fb-d{d}-m{m}-p{p}-s{s}")
    order = list(range(N))
    fb.shuffle(order)
    if arm == "FB":
        F = order
        nb = N
    else:
        nb = min(N, round(cF * p ** (1 - 1.0 / (d - 2))))
        F = order[:nb]
    P0 = F[0]
    inF = set(F)
    needed = nb + 8 + nb // 100
    if needed_override is not None:
        needed = needed_override
    rng = random.Random(f"L08-run-d{d}-m{m}-p{p}-s{s}-{arm}-cF{cF}-rep{rep}")
    Pm = nb * (nb - 1) // 2
    st = dict(lines_tried=0, node_lines=0, tangent_lines=0, eligible=0, prefilter_pass=0,
              prefilter_reject=0, nosplit=0, split=0, rel_full=0, rel_1lp=0, rel_2lp=0,
              rel_discard3=0, g1_fail=0, g2_fail=0, rel_dup_lines=0)
    uf = UF()
    fulls = []
    edges = []  # (u, v, ptsidx, L)
    relations = []  # (L, idxs)
    seen = set()
    pts = cv.pts
    nodes = cv.nodes
    exhausted = True
    log = []
    for k in pair_ks(Pm, rng):
        ia, ib = unrank(k)
        a_, b_ = F[ia], F[ib]
        st["lines_tried"] += 1
        X1, Y1, Z1 = pts[a_]
        X2, Y2, Z2 = pts[b_]
        L0 = (M(Y1, Z2, C) - M(Z1, Y2, C)) % p
        L1 = (M(Z1, X2, C) - M(X1, Z2, C)) % p
        L2 = (M(X1, Y2, C) - M(Y1, X2, C)) % p
        bad = False
        for (xk, yk) in nodes:
            v = (M(L0, xk, C) + M(L1, yk, C) + L2) % p
            if v == 0:
                bad = True
                break
        if bad:
            st["node_lines"] += 1
            continue
        cval = {}
        for u in range(1, d):
            Q = ((M(u, X1, C) + X2) % p, (M(u, Y1, C) + Y2) % p, (M(u, Z1, C) + Z2) % p)
            cval[u] = M(evalF(cv, Q, C), invu[u], C)
        coef = []
        for r in range(d - 1):
            sacc = 0
            for u in range(1, d):
                sacc += M(Vinv[r][u - 1], cval[u], C)
            coef.append(sacc % p)
        if coef[n] == 0 or coef[0] == 0:
            st["tangent_lines"] += 1
            continue
        if n == 2:
            aa, bb, cc = coef[2], coef[1], coef[0]
            Delta = (M(bb, bb, C) - M(4, M(aa, cc, C), C)) % p
        elif n == 3:
            a3, b3, c3, e3 = coef[3], coef[2], coef[1], coef[0]
            ab = M(a3, b3, C)
            ce = M(c3, e3, C)
            bbb = M(b3, b3, C)
            ccc = M(c3, c3, C)
            aaa = M(a3, a3, C)
            eee = M(e3, e3, C)
            Delta = (M(18, M(ab, ce, C), C) - M(4, M(M(bbb, b3, C), e3, C), C) + M(bbb, ccc, C)
                     - M(4, M(a3, M(ccc, c3, C), C), C) - M(27, M(aaa, eee, C), C)) % p
        else:
            cp = [M(i, coef[i], C) for i in range(1, n + 1)]
            R = sylvester_det(coef, cp, C)
            Delta = M(R, coef[n], C)
        if Delta == 0:
            st["tangent_lines"] += 1
            continue
        st["eligible"] += 1
        if n == 2:
            if legendre(Delta, C) != 1:
                st["nosplit"] += 1
                continue
            sq = sqrt_ts(Delta, C)
            i2a = INV(M(2, aa, C), C)
            roots = [M((-bb + sq) % p, i2a, C), M((-bb - sq) % p, i2a, C)]
            st["split"] += 1
        else:
            if legendre(Delta, C) != 1:
                st["prefilter_reject"] += 1
                continue
            st["prefilter_pass"] += 1
            f = pmonic(coef, C)
            w = list(ppowmod([0, 1], p, f, C))
            while len(w) < 2:
                w.append(0)
            w[1] = (w[1] - 1) % p
            w = trim(w)
            h = pgcd(f, w, C) if w else list(f)
            if len(h) - 1 < n:
                st["nosplit"] += 1
                continue
            roots = roots_edf(h, rng, C)
            st["split"] += 1
        resid = []
        ok = True
        for u in roots:
            Rx = ((M(u, X1, C) + X2) % p, (M(u, Y1, C) + Y2) % p, (M(u, Z1, C) + Z2) % p)
            if Rx[2] != 0:
                iz = INV(Rx[2], C)
                Rn = (M(Rx[0], iz, C), M(Rx[1], iz, C), 1)
            elif Rx[0] != 0:
                ix = INV(Rx[0], C)
                Rn = (1, M(Rx[1], ix, C), 0)
            else:
                ok = False
                break
            idx = cv.index.get(Rn)
            if idx is None:
                ok = False
                break
            resid.append(idx)
        if not ok:
            st["g1_fail"] += 1
            log.append(("g1_residual_not_found", a_, b_))
            continue
        idxs = [a_, b_] + resid
        L = (L0, L1, L2)
        g1, g2 = verify_rel(cv, L, idxs)
        if not g1:
            st["g1_fail"] += 1
        if not g2:
            st["g2_fail"] += 1
        if not (g1 and g2):
            continue
        key = tuple(sorted(idxs))
        if key in seen:
            st["rel_dup_lines"] += 1
        seen.add(key)
        relations.append((L, idxs))
        out = [i for i in resid if i not in inF]
        if len(out) == 0:
            st["rel_full"] += 1
            fulls.append(idxs)
        elif len(out) == 1:
            st["rel_1lp"] += 1
            uf.add_edge(out[0], GROUND)
            edges.append((out[0], GROUND, idxs, L))
        elif len(out) == 2:
            st["rel_2lp"] += 1
            uf.add_edge(out[0], out[1])
            edges.append((out[0], out[1], idxs, L))
        else:
            st["rel_discard3"] += 1
        if st["rel_full"] + uf.total >= needed:
            exhausted = False
            break
    useful = st["rel_full"] + uf.total
    # matrix
    contrib_roots = set()
    for v in list(uf.parent):
        r, _ = uf.find(v)
        if uf.contrib(r) > 0:
            contrib_roots.add(r)
    rows = list(fulls)
    for (u, v, idxs, L) in edges:
        r, _ = uf.find(u)
        if r in contrib_roots:
            rows.append(idxs)
    cols = nb + sum(uf.V[r] for r in contrib_roots)
    W = sum(sum(1 for i in idxs if i != P0) + 1 for idxs in rows)
    la = 3 * cols * W
    res = dict(st)
    res.update(nb=nb, needed=needed, useful=useful, exhausted=exhausted,
               components_contributing=len(contrib_roots), mat_rows=len(rows), mat_cols=cols,
               mat_weight=W, la_fpm=la, rel_mul=C.mul, rel_inv=C.inv, fpm_rel=fpm(C),
               uf=uf, edges=edges, relations=relations, fulls=fulls, log=log, inF=inF, F=F)
    return res


# ---------------- rows ----------------
def rho_info(p, g):
    base = math.sqrt(math.pi * p ** g / 2)
    jlo = (math.sqrt(p) - 1) ** (2 * g) / p ** g
    jhi = (math.sqrt(p) + 1) ** (2 * g) / p ** g
    return base * 393, base * 8900, jlo, jhi


def g4_status(res, p, d):
    n = d - 2
    if res["eligible"] <= 0:
        return "n/a", None, None
    frac = res["split"] / res["eligible"]
    expct = math.comb(p, n) / p ** n
    diff = abs(frac - expct)
    if p >= 1031 and res["eligible"] >= 20000:
        return ("pass" if diff <= 0.01 else "fail"), frac, diff
    return "n/a", frac, diff


def make_row(cv, c, seeds_tried, rej, arm, cF, rep, res):
    p, g = cv.p, cv.g
    E = cv.enum
    r393, r8900, jlo, jhi = rho_info(p, g)
    la = res["la_fpm"]
    fe = fpm(E)
    ftot = fe + res["fpm_rel"] + la
    g4, frac, diff = g4_status(res, p, cv.d)
    row = dict(d=cv.d, m=cv.m, g=g, p=p, curve_index=c, curve_seed=cv.s, seeds_tried=seeds_tried,
               reject_counts=rej, arm=arm, cF=cF, rep=rep, nb=res["nb"], N=cv.N,
               needed=res["needed"], lines_tried=res["lines_tried"], node_lines=res["node_lines"],
               tangent_lines=res["tangent_lines"], eligible=res["eligible"],
               prefilter_pass=res["prefilter_pass"], prefilter_reject=res["prefilter_reject"],
               nosplit=res["nosplit"], split=res["split"], split_frac=frac,
               rel_full=res["rel_full"], rel_1lp=res["rel_1lp"], rel_2lp=res["rel_2lp"],
               rel_discard3=res["rel_discard3"], useful=res["useful"], exhausted=res["exhausted"],
               components_contributing=res["components_contributing"], mat_rows=res["mat_rows"],
               mat_cols=res["mat_cols"], mat_weight=res["mat_weight"], la_fpm=la,
               enum_mul=E.mul, enum_inv=E.inv, rel_mul=res["rel_mul"], rel_inv=res["rel_inv"],
               fpm_enum=fe, fpm_rel=res["fpm_rel"], fpm_total=ftot,
               fpm_total_kappa=fe + res["fpm_rel"] + g * g * la,
               rho_fpm_393=r393, rho_fpm_8900=r8900, jlo=jlo, jhi=jhi,
               g1_fail=res["g1_fail"], g2_fail=res["g2_fail"], g4=g4, g4_absdiff=diff,
               g4_expected=math.comb(p, cv.d - 2) / p ** (cv.d - 2), rel_dup_lines=res["rel_dup_lines"],
               python_version=sys.version.split()[0])
    return row


def get_curve(d, m, p, c):
    lst = admitted(d, m, p, c)
    if len(lst) <= c:
        raise RuntimeError(f"fewer than {c+1} admitted curves for d={d} m={m} p={p} in 2000 seeds")
    s, a, tried, rej = lst[c]
    cv = build_curve(d, m, p, s, a)
    return cv, tried, rej


def cmd_run(args):
    d, m, p = args.d, args.m, args.p
    curves = [int(x) for x in args.curves.split(",")]
    with open(args.out, "w") as fo:
        for c in curves:
            cv, tried, rej = get_curve(d, m, p, c)
            arms = [("FB", 0, 1), ("2LP", 1, 1), ("2LP", 2, 1), ("2LP", 4, 1), ("2LP", 2, 2)]
            for (arm, cF, rep) in arms:
                res = collect(cv, arm, cF, rep)
                row = make_row(cv, c, tried, rej, arm, cF, rep, res)
                fo.write(json.dumps(row, sort_keys=True) + "\n")
                fo.flush()
                print(f"d={d} p={p} c={c} {arm} cF={cF} rep={rep} total={row['fpm_total']} "
                      f"exh={row['exhausted']} g1={row['g1_fail']} g2={row['g2_fail']}", file=sys.stderr)


# ---------------- gate ----------------
def lam(L, Pt, p):
    return (L[0] * Pt[0] + L[1] * Pt[1] + L[2] * Pt[2]) % p


def g5_rank(edges, ell=2147483647):
    cols = {}
    rows = []
    for (u, v, _, _) in edges:
        r = {}
        for x in (u, v):
            if x != GROUND:
                ci = cols.setdefault(x, len(cols))
                r[ci] = (r.get(ci, 0) + 1) % ell
        rows.append(r)
    piv = {}
    rank = 0
    for r in rows:
        r = {k: v for k, v in r.items() if v}
        while r:
            c = min(r)
            if c in piv:
                pr = piv[c]
                f = r[c]
                for k, v in pr.items():
                    nv = (r.get(k, 0) - f * v) % ell
                    if nv:
                        r[k] = nv
                    else:
                        r.pop(k, None)
            else:
                iv = pow(r[c], ell - 2, ell)
                r = {k: v * iv % ell for k, v in r.items()}
                piv[c] = r
                rank += 1
                break
    return rank


def gate_curve(d, m, p, c):
    cv, tried, rej = get_curve(d, m, p, c)
    set_p(p)
    needed = cv.N + 8 + cv.N // 100
    res = collect(cv, "FB", 0, 1, needed_override=max(400, needed))
    rels = res["relations"]
    rng = random.Random(f"L08-gate-d{d}-m{m}-p{p}-c{c}")
    quads = []
    draws = 0
    while len(quads) < 200 and draws < 100000 and len(rels) >= 4:
        draws += 1
        q = rng.sample(range(len(rels)), 4)
        sets = [set(rels[i][1]) for i in q]
        if all(not (sets[i] & sets[j]) for i in range(4) for j in range(i + 1, 4)):
            quads.append(q)

    def prodratio(Lnum, Lden, S):
        num = 1
        den = 1
        for i in S:
            Pt = cv.pts[i]
            num = num * lam(Lnum, Pt, p) % p
            den = den * lam(Lden, Pt, p) % p
        return num, den

    def side(La, Lb, Sa, Sb, Lc, Ld):
        # prod_{P in Sc? } handled by caller; unused
        pass

    def recip(L1, L2, L3, L4, S1, S2, S3, S4):
        # lhs = prod_{S3} L1/L2 * prod_{S4} L2/L1 ; rhs = prod_{S1} L3/L4 * prod_{S2} L4/L3
        def term(Lnum, Lden, Spos, Sneg):
            num = 1
            den = 1
            for i in Spos:
                Pt = cv.pts[i]
                num = num * lam(Lnum, Pt, p) % p
                den = den * lam(Lden, Pt, p) % p
            for i in Sneg:
                Pt = cv.pts[i]
                num = num * lam(Lden, Pt, p) % p
                den = den * lam(Lnum, Pt, p) % p
            return num, den
        ln, ld = term(L1, L2, S3, S4)
        rn, rd = term(L3, L4, S1, S2)
        if ld == 0 or rd == 0:
            return None
        return (ln * rd - rn * ld) % p == 0
    g3_pass = 0
    for q in quads:
        (La, Sa), (Lb, Sb), (Lc, Sc), (Ld, Sd) = [rels[i] for i in q]
        r = recip(La, Lb, Lc, Ld, Sa, Sb, Sc, Sd)
        if r:
            g3_pass += 1
    g3pc_fail = 0
    for q in quads:
        (La, Sa), (Lb, Sb), (Lc, Sc), (Ld, Sd) = [rels[i] for i in q]
        used = set(Sa) | set(Sb) | set(Sc) | set(Sd)
        cand = [i for i in range(cv.N) if i not in used and lam(La, cv.pts[i], p) != 0]
        newp = rng.choice(cand)
        Sa2 = list(Sa)
        Sa2[2] = newp
        r = recip(La, Lb, Lc, Ld, Sa2, Sb, Sc, Sd)
        if not r:
            g3pc_fail += 1
    g4, frac, diff = g4_status(res, p, d)
    row = dict(d=d, m=m, g=cv.g, p=p, c=c, curve_seed=cv.s, n_quads=len(quads), g3_pass=g3_pass,
               g3pc_fail=g3pc_fail, g1_fail=res["g1_fail"], g2_fail=res["g2_fail"],
               split_frac=frac, g4=g4, g4_absdiff=diff, g4_expected=math.comb(p, d - 2) / p ** (d - 2),
               eligible=res["eligible"], n_relations_fb=len(rels), N=cv.N, fb_exhausted=res["exhausted"],
               g5_total=None, g5_E=None, g5_rank=None, g5_pass=None)
    if c == 0:
        r2 = collect(cv, "2LP", 2, 1)
        Etot = len(r2["edges"])
        rk = g5_rank(r2["edges"])
        row.update(g5_total=r2["uf"].total, g5_E=Etot, g5_rank=rk, g5_pass=(r2["uf"].total == Etot - rk),
                   g5_exhausted=r2["exhausted"], g5_g1_fail=r2["g1_fail"], g5_g2_fail=r2["g2_fail"])
    ok = (len(quads) == 200 and g3_pass == 200 and g3pc_fail >= 190 and res["g1_fail"] == 0
          and res["g2_fail"] == 0 and g4 in ("pass", "n/a") and (c != 0 or row["g5_pass"]))
    if c == 0:
        ok = ok and row["g5_g1_fail"] == 0 and row["g5_g2_fail"] == 0
    row["all_pass"] = bool(ok)
    return row


def cmd_gate(args):
    ok = True
    with open(args.out, "w") as fo:
        for p in [int(x) for x in args.primes.split(",")]:
            for c in range(args.curves):
                row = gate_curve(args.d, args.m, p, c)
                fo.write(json.dumps(row, sort_keys=True) + "\n")
                fo.flush()
                print(json.dumps(row, sort_keys=True), file=sys.stderr)
                ok = ok and row["all_pass"]
    print("GATE ALL PASS" if ok else "GATE FAILURES")
    return 0 if ok else 1


# ---------------- selftest ----------------
def cmd_selftest(args):
    fails = []
    out = []

    def check(name, cond, detail=""):
        if cond:
            out.append(f"PASS {name}")
        else:
            out.append(f"FAIL {name} {detail}")
            fails.append(name)

    set_p(7)
    C = Ctr()
    r = powmod(3, 5, C)
    check("t1_powmod", r == 5 and C.mul == 3, f"r={r} mul={C.mul}")
    check("t2_legendre", [legendre(a, Ctr()) for a in range(1, 7)] == [1, 1, -1, 1, -1, -1])
    C = Ctr()
    a3, b3, c3, e3 = 1, 1, 4, 1  # coef [1,4,1,1] low->high => coef[3]=1,coef[2]=1,coef[1]=4,coef[0]=1
    coef = [1, 4, 1, 1]
    a3, b3, c3, e3 = coef[3], coef[2], coef[1], coef[0]
    ab = M(a3, b3, C); ce = M(c3, e3, C); bbb = M(b3, b3, C); ccc = M(c3, c3, C)
    aaa = M(a3, a3, C); eee = M(e3, e3, C)
    Delta = (M(18, M(ab, ce, C), C) - M(4, M(M(bbb, b3, C), e3, C), C) + M(bbb, ccc, C)
             - M(4, M(a3, M(ccc, c3, C), C), C) - M(27, M(aaa, eee, C), C)) % 7
    check("t3_disc_cubic", Delta == 4 and C.mul == 17, f"D={Delta} mul={C.mul}")
    rr = sorted(rational_roots([1, 4, 1, 1], random.Random("t4"), Ctr()))
    check("t4_roots", rr == [1, 2, 3], str(rr))
    coef = [5, 0, 0, 1]
    a3, b3, c3, e3 = coef[3], coef[2], coef[1], coef[0]
    D5 = (-27 * a3 * a3 * e3 * e3) % 7
    rr = rational_roots(coef, random.Random("t5"), Ctr())
    check("t5_cubic_noroots", D5 == 4 and legendre(D5, Ctr()) == 1 and rr == [], f"D={D5} roots={rr}")
    dd = sylvester_det([-1 % 7, 0, 1], [0, 2], Ctr())
    check("t6_sylvester", dd == 3, str(dd))
    check("t7_unrank", [unrank(k) for k in range(6)] == [(0, 1), (0, 2), (1, 2), (0, 3), (1, 3), (2, 3)])
    u = UF(); u.add_edge("a", "b"); t_a = u.total
    u = UF()
    for x, y in [("a", "b"), ("b", "c"), ("c", "d"), ("d", "a")]:
        u.add_edge(x, y)
    t_b = u.total
    u = UF()
    for x, y in [("a", "b"), ("b", "c"), ("c", "a")]:
        u.add_edge(x, y)
    t_c = u.total
    u = UF(); u.add_edge("a", GROUND); u.add_edge("a", GROUND); t_d = u.total
    u = UF(); u.add_edge("a", "b"); u.add_edge("a", GROUND); u.add_edge("b", GROUND); t_e = u.total
    check("t8_unionfind", [t_a, t_b, t_c, t_d, t_e] == [0, 1, 0, 1, 1], str([t_a, t_b, t_c, t_d, t_e]))
    ks = sorted(pair_ks(10, random.Random("t9")))
    check("t9_pairperm", ks == list(range(10)), str(ks))
    ok10 = True
    for (p_, a_, exp) in [(7, 2, {3, 4}), (13, 10, {6, 7}), (17, 2, {6, 11})]:
        set_p(p_)
        ok10 = ok10 and ({sqrt_ts(a_, Ctr()), (-sqrt_ts(a_, Ctr())) % p_} == exp)
    check("t10_sqrt_ts", ok10)
    set_p(131)
    a = gen_coeffs(5, 2, 131, 1)
    cv = Curve()
    cv.d, cv.p = 5, 131
    cv.ml = [(i, j, 5 - i - j, a[(i, j)]) for (i, j) in mons(5)] if not isinstance(a, str) else []
    okn = (not isinstance(a, str)) and a[(0, 5)] != 0
    if okn:
        for nd in [(0, 0), (1, 1)]:
            f, fx, fy, fz = plain_F_grad(cv, (nd[0], nd[1], 1))
            okn = okn and f == 0 and fx == 0 and fy == 0
    check("t11_nodes", okn)
    # t12
    p13 = 13
    a12 = {mo: 0 for mo in mons(4)}
    a12[(0, 4)] = 1
    a12[(4, 0)] = 1
    a12[(0, 0)] = 1
    r12 = genus_test(a12, 4, 0, p13)
    check("t12_T_sqfree", r12 == "T_sqfree", str(r12))
    a13 = {mo: 0 for mo in mons(4)}
    a13[(0, 4)] = 1
    a13[(3, 1)] = 1
    a13[(0, 1)] = 1
    r13 = genus_test(a13, 4, 0, 131)
    check("t13_reducible_rejected", r13 is not None, str(r13))
    l5 = admitted(5, 2, 131, 0)
    l4 = admitted(4, 0, 131, 0)
    check("t14_admitted", len(l5) >= 1 and len(l4) >= 1 and genus_of(5, 2) == 4 and genus_of(4, 0) == 3,
          f"{len(l5)} {len(l4)}")
    cv5 = build_curve(5, 2, 131, l5[0][0], l5[0][1])
    cv4 = build_curve(4, 0, 131, l4[0][0], l4[0][1])
    C = Ctr()
    set_p(131)
    evalF(cv5, (3, 5, 7), C)
    c5 = C.mul
    C = Ctr()
    evalF(cv4, (3, 5, 7), C)
    check("t15_evalF_cost", c5 == 75 and C.mul == 54, f"{c5} {C.mul}")
    okc = True
    det = ""
    for cvx in (cv4, cv5):
        res = collect(cvx, "FB", 0, 1, needed_override=50)
        okc = okc and res["g1_fail"] == 0 and res["g2_fail"] == 0 and \
            all(len(idxs) == cvx.d for (_, idxs) in res["relations"]) and len(res["relations"]) > 0
        det += f" d={cvx.d}: rels={len(res['relations'])} g1={res['g1_fail']} g2={res['g2_fail']}"
    check("t16_fb_collection", okc, det)
    okn = True
    det = ""
    for cvx in (cv4, cv5):
        bound = 2 * cvx.g * math.sqrt(131) + 2 * cvx.m
        okn = okn and abs(cvx.N - 132) <= bound
        det += f" d={cvx.d}: N={cvx.N} bound={bound:.1f}"
    check("t17_pointcount", okn, det)
    for line in out:
        print(line)
    print("ALL PASS" if not fails else f"FAILURES {len(fails)}")
    return 0 if not fails else 1


# ---------------- analyze ----------------
def ols_slope(xs, ys):
    n = len(xs)
    if n < 2:
        return None
    mx = sum(xs) / n
    my = sum(ys) / n
    den = sum((x - mx) ** 2 for x in xs)
    if den == 0:
        return None
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / den


def slope_of(series):
    ps = sorted(series)
    return ols_slope([math.log(p) for p in ps], [math.log(series[p]) for p in ps])


def pct(sorted_vals, q):
    return sorted_vals[round(q * (len(sorted_vals) - 1))]


NUMKEYS = ["fpm_total", "fpm_enum", "fpm_rel", "la_fpm", "lines_tried", "useful", "needed", "mat_rows",
           "mat_cols", "mat_weight", "split_frac", "node_lines", "tangent_lines", "fpm_total_kappa",
           "rho_fpm_393", "rho_fpm_8900", "jlo", "jhi", "nb", "N", "rel_full", "rel_1lp", "rel_2lp",
           "rel_discard3", "eligible", "split", "rel_dup_lines"]


def cmd_analyze(args):
    rows = []
    for fn in sorted(glob.glob(args.glob)):
        with open(fn) as f:
            for line in f:
                line = line.strip()
                if line:
                    rows.append(json.loads(line))
    groups = {}
    for r in rows:
        groups.setdefault((r["d"], r["m"]), []).append(r)
    out = {"python_version": sys.version.split()[0], "groups": {}}
    tsv = ["\t".join("d m arm cF rep p n_valid mean_fpm_total mean_fpm_enum mean_fpm_rel mean_la_fpm "
                     "mean_lines_tried mean_useful mean_needed mean_mat_rows mean_mat_cols mean_mat_weight "
                     "mean_split_frac mean_node_lines mean_tangent_lines n_exhausted ratio_393 "
                     "ratio_8900".split())]
    allfail = False
    for (d, m), rs in sorted(groups.items()):
        g = genus_of(d, m)
        cells = {}
        for r in rs:
            cells.setdefault((r["arm"], r["cF"], r["rep"], r["p"]), []).append(r)
        for k in cells:
            cells[k].sort(key=lambda r: r["curve_index"])
        valid = {}
        for k, v in cells.items():
            idxs = [r["curve_index"] for r in v]
            if len(v) == 4 and len(set(idxs)) == 4 and all((not r["exhausted"]) and r["g1_fail"] == 0
                                                          and r["g2_fail"] == 0 for r in v):
                valid[k] = v
        anyfail = any(r["g1_fail"] or r["g2_fail"] for r in rs)

        def mean(v, key):
            vals = [r[key] for r in v if r[key] is not None]
            return sum(vals) / len(vals) if vals else None

        for k in sorted(cells):
            v = cells[k]
            arm, cF, rep, p = k
            nval = sum(1 for r in v if (not r["exhausted"]) and r["g1_fail"] == 0 and r["g2_fail"] == 0)
            ex = sum(1 for r in v if r["exhausted"])
            mt = mean(v, "fpm_total")

            def fm(x):
                return "" if x is None else repr(x)
            tsv.append("\t".join([str(d), str(m), arm, str(cF), str(rep), str(p), str(nval),
                                  fm(mt), fm(mean(v, "fpm_enum")), fm(mean(v, "fpm_rel")),
                                  fm(mean(v, "la_fpm")), fm(mean(v, "lines_tried")), fm(mean(v, "useful")),
                                  fm(mean(v, "needed")), fm(mean(v, "mat_rows")), fm(mean(v, "mat_cols")),
                                  fm(mean(v, "mat_weight")), fm(mean(v, "split_frac")),
                                  fm(mean(v, "node_lines")), fm(mean(v, "tangent_lines")), str(ex),
                                  fm(mt / mean(v, "rho_fpm_393")), fm(mt / mean(v, "rho_fpm_8900"))]))
        primes = sorted({k[3] for k in cells})
        # series A_2LP
        def series_A(idx_draw=None):
            ser = {}
            arg = {}
            for p in primes:
                best = None
                for cF in (1, 2, 4):
                    k = ("2LP", cF, 1, p)
                    if k in valid:
                        v = valid[k]
                        if idx_draw is not None:
                            v = [v[i] for i in idx_draw[p]]
                        mv = sum(r["fpm_total"] for r in v) / 4
                        if best is None or mv < best[0]:
                            best = (mv, cF)
                if best:
                    ser[p] = best[0]
                    arg[p] = best[1]
            return ser, arg
        ser, arg = series_A()
        info = {"d": d, "m": m, "g": g, "primes_present": primes, "n_rows": len(rs),
                "anyfail_g1g2": anyfail,
                "A_2LP": {str(p): {"value": ser[p], "argmin_cF": arg[p]} for p in sorted(ser)}}
        info["slope_A_all"] = slope_of(ser) if len(ser) >= 2 else None
        sp = sorted(ser)
        info["slope_A_largest4"] = slope_of({p: ser[p] for p in sp[-4:]}) if len(sp) >= 4 else None
        # per cF
        per = {}
        for cF in (1, 2, 4):
            s_ = {}
            for p in primes:
                k = ("2LP", cF, 1, p)
                if k in valid:
                    s_[p] = sum(r["fpm_total"] for r in valid[k]) / 4
            per[str(cF)] = {"primes": sorted(s_), "slope": slope_of(s_) if len(s_) >= 2 else None}
        info["slope_per_cF"] = per
        fbs = {}
        for p in primes:
            k = ("FB", 0, 1, p)
            if k in valid:
                fbs[p] = sum(r["fpm_total"] for r in valid[k]) / 4
        info["FB_values"] = {str(p): fbs[p] for p in sorted(fbs)}
        info["slope_FB_all"] = slope_of(fbs) if len(fbs) >= 2 else None
        sf = sorted(fbs)
        info["slope_FB_largest4"] = slope_of({p: fbs[p] for p in sf[-4:]}) if len(sf) >= 4 else None
        s1, s2 = {}, {}
        for p in primes:
            k1, k2 = ("2LP", 2, 1, p), ("2LP", 2, 2, p)
            if k1 in valid and k2 in valid:
                s1[p] = sum(r["fpm_total"] for r in valid[k1]) / 4
                s2[p] = sum(r["fpm_total"] for r in valid[k2]) / 4
        sa1 = slope_of(s1) if len(s1) >= 2 else None
        sa2 = slope_of(s2) if len(s2) >= 2 else None
        info["AA"] = {"primes": sorted(s1), "slope_rep1": sa1, "slope_rep2": sa2,
                      "absdiff": (abs(sa1 - sa2) if sa1 is not None and sa2 is not None else None)}
        # ratios
        rat = {}
        for p in sorted(ser):
            kk = ("2LP", arg[p], 1, p)
            v = valid[kk]
            r393 = sum(r["rho_fpm_393"] for r in v) / 4
            r8900 = sum(r["rho_fpm_8900"] for r in v) / 4
            jlo = v[0]["jlo"]
            jhi = v[0]["jhi"]
            rat[str(p)] = {"ratio_393": ser[p] / r393, "ratio_8900": ser[p] / r8900,
                           "ratio_393_band": [ser[p] / r393 / math.sqrt(jhi), ser[p] / r393 / math.sqrt(jlo)],
                           "ratio_8900_band": [ser[p] / r8900 / math.sqrt(jhi), ser[p] / r8900 / math.sqrt(jlo)]}
        info["ratios_A_2LP"] = rat
        # enumeration share of the A_2LP argmin cell
        info["enum_share"] = {str(p): (sum(r["fpm_enum"] for r in valid[("2LP", arg[p], 1, p)]) /
                                       sum(r["fpm_total"] for r in valid[("2LP", arg[p], 1, p)]))
                              for p in sorted(ser)}
        # bootstrap
        if len(ser) >= 2:
            brng = random.Random("L08-boot")
            sl = []
            for _ in range(2000):
                draw = {p: [brng.randrange(4) for _ in range(4)] for p in sorted(ser)}
                s_, _a = series_A(draw)
                if len(s_) >= 2:
                    sl.append(slope_of(s_))
            sl.sort()
            info["bootstrap"] = {"reps": len(sl), "lo": pct(sl, 0.025), "hi": pct(sl, 0.975)}
        else:
            info["bootstrap"] = None
        out["groups"][f"d{d}_m{m}"] = info
        allfail = allfail or anyfail
    # verdict
    g5 = out["groups"].get("d5_m2")
    g4 = out["groups"].get("d4_m0")
    if g5 is not None and g4 is not None:
        v = {}
        if g5["anyfail_g1g2"] or g4["anyfail_g1g2"]:
            v["verdict"] = "KILLED-verification"
        else:
            pc1 = g4["slope_A_all"] is not None and 0.55 <= g4["slope_A_all"] <= 1.25
            pc2 = g5["slope_FB_largest4"] is not None and g5["slope_FB_largest4"] >= 1.75
            aa = g5["AA"]["absdiff"] is not None and g5["AA"]["absdiff"] <= 0.10
            v.update(PC1=pc1, PC2=pc2, AA=aa)
            e = g5["slope_A_all"]
            if len(g5["A_2LP"]) < 5:
                v["verdict"] = "INCONCLUSIVE-coverage"
            elif not (pc1 and pc2 and aa):
                v["verdict"] = "INCONCLUSIVE-instrument"
            elif e > 1.60:
                v["verdict"] = "KILLED-exponent"
            elif e <= 1.45 and g5["bootstrap"] and g5["bootstrap"]["hi"] <= 1.60:
                v["verdict"] = "SUPPORTED"
            else:
                v["verdict"] = "INCONCLUSIVE"
            v["point_slope_e"] = e
        out["verdict"] = v
    else:
        out["verdict"] = "n/a (optional)" if any(k[0] == 6 for k in groups) else "n/a (missing group)"
    with open(args.out, "w") as f:
        json.dump(out, f, indent=1, sort_keys=True)
    with open(args.table, "w") as f:
        f.write("\n".join(tsv) + "\n")
    print(json.dumps(out.get("verdict"), sort_keys=True))
    return 0


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    sub.add_parser("selftest")
    g = sub.add_parser("gate")
    g.add_argument("--d", type=int, required=True)
    g.add_argument("--m", type=int, required=True)
    g.add_argument("--primes", required=True)
    g.add_argument("--curves", type=int, required=True)
    g.add_argument("--out", required=True)
    r = sub.add_parser("run")
    r.add_argument("--d", type=int, required=True)
    r.add_argument("--m", type=int, required=True)
    r.add_argument("--p", type=int, required=True)
    r.add_argument("--curves", required=True)
    r.add_argument("--out", required=True)
    a = sub.add_parser("analyze")
    a.add_argument("--glob", required=True)
    a.add_argument("--out", required=True)
    a.add_argument("--table", required=True)
    args = ap.parse_args()
    if args.cmd == "selftest":
        sys.exit(cmd_selftest(args))
    elif args.cmd == "gate":
        sys.exit(cmd_gate(args))
    elif args.cmd == "run":
        cmd_run(args)
    else:
        cmd_analyze(args)


if __name__ == "__main__":
    main()
