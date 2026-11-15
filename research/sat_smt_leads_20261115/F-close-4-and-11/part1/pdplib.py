"""Self-contained F_{2^n} + binary curve + S4 + ANF tools for the idea-4 pilot."""
import random
import numpy as np

A_CURVE = 1
B_CURVE = 1


class Field:
    def __init__(self, n):
        self.n = n
        self.N = (1 << n) - 1
        m = (1 << n) + 1
        while True:
            t = self._try(m)
            if t is not None:
                break
            m += 2
        self.mod = m
        self.exp, self.log = t

    def _try(self, m):
        n, N = self.n, self.N
        exp = np.zeros(2 * N + 2, dtype=np.int64)
        log = np.zeros(N + 1, dtype=np.int64)
        v = 1
        top = 1 << n
        for i in range(N):
            if i and v == 1:
                return None
            exp[i] = v
            log[v] = i
            v <<= 1
            if v & top:
                v ^= m
        if v != 1:
            return None
        exp[N:2 * N] = exp[:N]
        return exp, log

    def mul(self, a, b):
        a = np.asarray(a, dtype=np.int64)
        b = np.asarray(b, dtype=np.int64)
        r = self.exp[self.log[a] + self.log[b]]
        return np.where((a == 0) | (b == 0), 0, r)

    def smul(self, a, b):
        if a == 0 or b == 0:
            return 0
        return int(self.exp[self.log[a] + self.log[b]])

    def inv(self, a):
        return int(self.exp[(self.N - self.log[a]) % self.N])

    def sq(self, a):
        return self.mul(a, a)

    def trace(self, c):
        t, x = 0, c
        for _ in range(self.n):
            t ^= x
            x = self.smul(x, x)
        return t

    def half_trace(self, c):
        h, x = 0, c
        for i in range((self.n - 1) // 2 + 1):
            h ^= x
            x = self.smul(x, x)
            x = self.smul(x, x)
        return h


# ---------- curve y^2 + xy = x^3 + a x^2 + b ----------
def lift_x(F, x):
    """Return y with (x,y) on curve, or None."""
    if x == 0:
        return None  # 2-torsion point, excluded
    rhs = F.smul(F.smul(x, x), x) ^ F.smul(A_CURVE, F.smul(x, x)) ^ B_CURVE
    c = F.smul(rhs, F.inv(F.smul(x, x)))
    if F.trace(c) != 0:
        return None
    z = F.half_trace(c)
    assert F.smul(z, z) ^ z == c
    return F.smul(z, x)


def on_curve(F, P):
    if P is None:
        return True
    x, y = P
    return F.smul(y, y) ^ F.smul(x, y) == F.smul(F.smul(x, x), x) ^ F.smul(A_CURVE, F.smul(x, x)) ^ B_CURVE


def neg(P):
    return None if P is None else (P[0], P[0] ^ P[1])


def add(F, P, Q):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if y1 != y2 or x1 == 0:
            return None
        lam = x1 ^ F.smul(y1, F.inv(x1))
        x3 = F.smul(lam, lam) ^ lam ^ A_CURVE
        y3 = F.smul(x1, x1) ^ F.smul(lam ^ 1, x3)
        return (x3, y3)
    lam = F.smul(y1 ^ y2, F.inv(x1 ^ x2))
    x3 = F.smul(lam, lam) ^ lam ^ x1 ^ x2 ^ A_CURVE
    y3 = F.smul(lam, x1 ^ x3) ^ x3 ^ y1
    return (x3, y3)


# ---------- Semaev ----------
def S4(F, x1, x2, x3, x4):
    """Vectorised S4 = Res_X(S3(x1,x2,X), S3(x3,x4,X)); b = B_CURVE."""
    m = F.mul
    s = x1 ^ x2
    p = m(x1, x2)
    t = x3 ^ x4
    q = m(x3, x4)
    s2, t2, p2, q2 = m(s, s), m(t, t), m(p, p), m(q, q)
    u = m(s2, q2 ^ B_CURVE) ^ m(t2, p2 ^ B_CURVE)
    v = m(m(s2, q) ^ m(t2, p), m(p, q2 ^ B_CURVE) ^ m(q, p2 ^ B_CURVE))
    return m(u, u) ^ v


def span_table(basis):
    l = len(basis)
    tab = np.zeros(1 << l, dtype=np.int64)
    for j, v in enumerate(basis):
        tab[1 << j:1 << (j + 1)] = tab[:1 << j] ^ v
    return tab


def rank_f2(vals):
    piv = {}
    r = 0
    for v in vals:
        v = int(v)
        while v:
            h = v.bit_length() - 1
            if h in piv:
                v ^= piv[h]
            else:
                piv[h] = v
                r += 1
                break
    return r


def random_basis(F, l, rng):
    basis = []
    while len(basis) < l:
        v = rng.randrange(1, 1 << F.n)
        if rank_f2(basis + [v]) == len(basis) + 1:
            basis.append(v)
    return basis


def make_instance(F, l, seed, planted):
    """Returns dict(basis, xR, planted tuple or None)."""
    rng = random.Random(seed)
    while True:  # redraw V if it holds fewer than 3 liftable x (only matters at l=3)
        basis = random_basis(F, l, rng)
        tab = span_table(basis)
        cand = [int(x) for x in tab if x and lift_x(F, int(x)) is not None]
        if len(cand) >= 3:
            break
    if planted:
        while True:
            xs = rng.sample(cand, 3)
            pts = []
            for x in xs:
                y = lift_x(F, x)
                P = (x, y)
                if rng.random() < 0.5:
                    P = neg(P)
                pts.append(P)
            R = add(F, add(F, pts[0], pts[1]), pts[2])
            if R is not None and R[0] != 0:
                return dict(basis=basis, xR=R[0], planted=xs)
    while True:
        x = rng.randrange(1, 1 << F.n)
        if lift_x(F, x) is not None:
            return dict(basis=basis, xR=x, planted=None)


def coords(basis, x):
    """coordinates (bitmask) of x in span(basis), or None."""
    tab = span_table(basis)
    w = np.nonzero(tab == x)[0]
    return int(w[0]) if len(w) else None


def lifts_to_points(F, xs, xR):
    ys = [lift_x(F, x) for x in xs]
    if any(y is None for y in ys):
        return False
    P = [(x, y) for x, y in zip(xs, ys)]
    for sg in range(4):
        Q = add(F, add(F, P[0], neg(P[1]) if sg & 1 else P[1]), neg(P[2]) if sg & 2 else P[2])
        if Q is not None and Q[0] == xR:
            return True
    return False


# ---------- evaluation tables / ANF ----------
def eval_table(F, basis, xR, k=0, kind=None, c=0):
    """S4 on the residual variable set.
    kind None: vars = x1(l) | x2(l) | x3(l)                     (3l bits)
    kind 'xor': x3_j = c_j ^ x1_j ^ x2_j for j<k; vars = x1 | x2 | x3_{k..l-1}
    kind 'lit': x1_j = c_j for j<k;              vars = x1_{k..l-1} | x2 | x3
    Bit layout of the index: listed blocks from least significant."""
    l = len(basis)
    tab = span_table(basis)
    lm = (1 << l) - 1
    km = (1 << k) - 1
    if kind is None or k == 0:
        idx = np.arange(1 << (3 * l), dtype=np.int64)
        u1, u2, u3 = idx & lm, (idx >> l) & lm, (idx >> (2 * l)) & lm
    elif kind == 'xor':
        idx = np.arange(1 << (3 * l - k), dtype=np.int64)
        u1, u2 = idx & lm, (idx >> l) & lm
        hi = idx >> (2 * l)
        u3 = (hi << k) | ((u1 ^ u2 ^ c) & km)
    elif kind == 'lit':
        idx = np.arange(1 << (3 * l - k), dtype=np.int64)
        u1 = ((idx & ((1 << (l - k)) - 1)) << k) | c
        u2 = (idx >> (l - k)) & lm
        u3 = (idx >> (2 * l - k)) & lm
    else:
        raise ValueError(kind)
    vals = S4(F, tab[u1], tab[u2], tab[u3], np.int64(xR))
    return vals, (u1, u2, u3)


def moebius(vals):
    a = vals.copy()
    nb = a.size.bit_length() - 1
    for i in range(nb):
        a = a.reshape(-1, 2, 1 << i)
        a[:, 1, :] ^= a[:, 0, :]
        a = a.reshape(-1)
    return a


_PC = {}


def popcounts(nb):
    if nb not in _PC:
        pc = np.zeros(1, dtype=np.int8)
        for _ in range(nb):
            pc = np.concatenate([pc, pc + 1])
        _PC[nb] = pc
    return _PC[nb]


def anf_stats(anf):
    nb = anf.size.bit_length() - 1
    pc = popcounts(nb)
    nz = anf != 0
    deg = int(pc[nz].max()) if nz.any() else 0
    nl = nz & (pc >= 2)
    d3 = nz & (pc >= 3)
    return dict(
        nvars=nb,
        max_deg=deg,
        n_nonlin_monos=int(nl.sum()),
        n_deg3_monos=int(d3.sum()),
        rank_nonlin=rank_f2(np.unique(anf[nl])),
        rank_deg3=rank_f2(np.unique(anf[d3])),
    )
