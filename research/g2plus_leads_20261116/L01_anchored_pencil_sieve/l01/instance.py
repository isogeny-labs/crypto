from .rng import mix64, M64, GOLD
from .field import Counter, F


def random_f(p, g, s):
    st = ((s * GOLD) & M64) ^ p ^ (g << 48)
    f = []
    for _ in range(2 * g + 1):
        st = (st + GOLD) & M64
        f.append(mix64(st) % p)
    f.append(1)
    return f


def _trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def _prem(a, b, p):
    a = list(a)
    inv = pow(b[-1], p - 2, p)
    while len(a) >= len(b):
        c = a[-1] * inv % p
        sh = len(a) - len(b)
        for j in range(len(b)):
            a[sh + j] = (a[sh + j] - c * b[j]) % p
        _trim(a)
    return a


def squarefree(f, p):
    d = [(i * f[i]) % p for i in range(1, len(f))]
    _trim(d)
    a, b = list(f), d
    while b:
        a, b = b, _prem(a, b, p)
    return len(a) == 1


def curve_seeds(p, g, count):
    out = []
    s = 0
    while len(out) < count:
        s += 1
        if squarefree(random_f(p, g, s), p):
            out.append(s)
    return out


def sqtab_plain(p):
    sq = [-1] * p
    sq[0] = 0
    for y in range(1, (p - 1) // 2 + 1):
        sq[y * y % p] = y
    return sq


def chitab_plain(p):
    sq = sqtab_plain(p)
    return [0 if a == 0 else (1 if sq[a] > 0 else -1) for a in range(p)]


class Tables:
    """Base, square-root and character tables; setup work counted on self.ctr."""

    def __init__(self, p, g, f):
        self.p, self.g, self.f = p, g, f
        self.ctr = Counter()
        self.F = F(p, self.ctr)
        Fq = self.F
        sq = [-1] * p
        sq[0] = 0
        for y in range(1, (p - 1) // 2 + 1):
            a = Fq.mul(y, y)
            sq[a] = y
        self.sq = sq
        self.chi = [0 if a == 0 else (1 if sq[a] > 0 else -1) for a in range(p)]
        self.inv2 = (p + 1) // 2
        self.fx = [Fq.horner(f, x) for x in range(p)]
        self.B = [x for x in range(p) if self.chi[self.fx[x]] == 1]
        self.col = [-1] * p
        for i, x in enumerate(self.B):
            self.col[x] = i
        self.Z = [x for x in range(p) if self.fx[x] == 0]
        self.n = len(self.B)
        self.N_target = self.n + 8 + self.n // 100


# ---------- point counts and Jacobian order (uncounted) ----------

def _least_nonres(p, chi):
    for a in range(2, p):
        if chi[a] == -1:
            return a


def N1(f, p, chi):
    tot = 1
    for x in range(p):
        v = 0
        for c in reversed(f):
            v = (v * x + c) % p
        tot += 1 + chi[v]
    return tot


def N2(f, p, chi):
    nr = _least_nonres(p, chi)
    tot = 1
    for a in range(p):
        for b in range(p):
            x0, x1 = 1, 0  # leading coeff 1 (f monic)
            for c in reversed(f[:-1]):
                # (x0 + x1 w)(a + b w) = x0 a + nr x1 b + (x0 b + x1 a) w
                x0, x1 = (x0 * a + nr * x1 * b + c) % p, (x0 * b + x1 * a) % p
            nm = (x0 * x0 - nr * x1 * x1) % p
            tot += 1 + chi[nm]
    return tot


def N3(f, p, chi):
    assert p % 3 == 1
    cn = None
    for a in range(2, p):
        if pow(a, (p - 1) // 3, p) != 1:
            cn = a
            break
    om = pow(cn, (p - 1) // 3, p)
    om2 = om * om % p
    cn2 = cn * cn % p
    fl = list(reversed(f[:-1]))
    tot = 1 + p  # F_p elements: b = c = 0 (norm = a^3 * ... = f(a)^3 -> chi(f(a)))
    tot = 1
    for a in range(p):
        v = 0
        for c in reversed(f):
            v = (v * a + c) % p
        # Norm of an F_p element is its cube: chi(v^3) = chi(v)
        tot += 1 + chi[v]
    for b in range(p):
        for c in range(p):
            if b == 0 and c == 0:
                continue
            orbit = min((b, c), (om * b % p, om2 * c % p), (om2 * b % p, om * c % p))
            if (b, c) != orbit:
                continue
            s = 0
            for a in range(p):
                x0, x1, x2 = 1, 0, 0
                for cc in fl:
                    # (x0 + x1 w + x2 w^2)(a + b w + c w^2), w^3 = cn
                    n0 = x0 * a + cn * (x1 * c + x2 * b) + cc
                    n1 = x0 * b + x1 * a + cn * x2 * c
                    n2 = x0 * c + x1 * b + x2 * a
                    x0, x1, x2 = n0 % p, n1 % p, n2 % p
                nm = (x0 * x0 * x0 + cn * x1 * x1 * x1 + cn2 * x2 * x2 * x2
                      - 3 * cn * x0 * x1 * x2) % p
                s += 1 + chi[nm]
            tot += 3 * s
    return tot


def point_counts(f, p, g, chi=None):
    if chi is None:
        chi = chitab_plain(p)
    out = [N1(f, p, chi)]
    if g >= 2:
        out.append(N2(f, p, chi))
    if g >= 3:
        out.append(N3(f, p, chi))
    return out


def jac_order(N, p, g):
    t = [None] + [p ** k + 1 - N[k - 1] for k in range(1, g + 1)]
    e = [1]
    for k in range(1, g + 1):
        s = 0
        for i in range(1, k + 1):
            s += (-1) ** (i - 1) * e[k - i] * t[i]
        assert s % k == 0
        e.append(s // k)
    c = [0] * (2 * g + 1)
    for j in range(g + 1):
        c[j] = (-1) ** j * e[j]
    for j in range(g):
        c[2 * g - j] = p ** (g - j) * c[j]
    return sum(c)


def is_prime(n):
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True


def split_order(J):
    for h in range(1, 65):
        if J % h == 0:
            r = J // h
            if r > h and is_prime(r):
                return h, r
    return None
