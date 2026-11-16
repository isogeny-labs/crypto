from .rng import M64, GOLD, mix64
from .poly import pgcd, pderiv, ev
from .field import horner_F


def _gen(p, g, s, extra, count):
    st = ((s * GOLD) & M64) ^ p ^ (g << 48) ^ extra
    f = []
    for _ in range(count):
        st = (st + GOLD) & M64
        f.append(mix64(st) % p)
    f.append(1)
    return f


def random_f(p, g, s):
    return _gen(p, g, s, 0, 2 * g + 1)


def random_f_even(p, g, s):
    return _gen(p, g, s, 0xE5 << 40, 2 * g + 2)


def squarefree(f, p):
    from .poly import trim
    d = pgcd(f, pderiv(f, p), p)
    return len(d) - 1 == 0


def roots(f, p):
    return [x for x in range(p) if ev(f, x, p) == 0]


def select_curve(p, g, cls, c):
    """Return (s, f, alpha) for the c-th (1-based) kept curve, or None."""
    kept = 0
    for s in range(1, 10001):
        if cls == "oddrand":
            f = random_f(p, g, s)
            if not squarefree(f, p):
                continue
            alpha = None
        else:
            f = random_f_even(p, g, s)
            if not squarefree(f, p):
                continue
            rts = roots(f, p)
            if cls == "root" and not rts:
                continue
            if cls == "rootless" and rts:
                continue
            alpha = rts[0] if rts else None
        kept += 1
        if kept == c:
            return s, f, alpha
    return None


class Tables:
    def __init__(self, h, p, F_):
        self.h = h
        self.p = p
        sq = [-1] * p
        sq[0] = 0
        for y in range(1, (p - 1) // 2 + 1):
            sq[F_.mul(y, y)] = y
        self.sq = sq
        self.chi = [0 if a == 0 else (1 if sq[a] > 0 else -1) for a in range(p)]
        self.inv2 = (p + 1) // 2
        self.hx = [horner_F(F_, h, x) for x in range(p)]
        self.B = [x for x in range(p) if self.chi[self.hx[x]] == 1]
        self.col = [-1] * p
        for i, x in enumerate(self.B):
            self.col[x] = i
        self.n = len(self.B)
        self.ylift = [0] * p
        for x in self.B:
            self.ylift[x] = sq[self.hx[x]]
