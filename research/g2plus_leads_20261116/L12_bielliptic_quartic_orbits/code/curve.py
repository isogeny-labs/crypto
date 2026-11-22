"""Curves, counted enumeration and counted elliptic arithmetic (SPEC.md section 3)."""
from fp import Ops, Fp


class Curve:
    def __init__(self, p, a1, a3, a2, a4, a6):
        self.p = p
        self.a1, self.a3, self.a2, self.a4, self.a6 = a1 % p, a3 % p, a2 % p, a4 % p, a6 % p
        a1, a3, a2, a4, a6 = self.a1, self.a3, self.a2, self.a4, self.a6
        self.b2 = (a1 * a1 + 4 * a2) % p
        self.b4 = (2 * a4 + a1 * a3) % p
        self.b6 = (a3 * a3 + 4 * a6) % p
        self.b8 = (a1 * a1 * a6 + 4 * a2 * a6 - a1 * a3 * a4 + a2 * a3 * a3 - a4 * a4) % p
        b2, b4, b6, b8 = self.b2, self.b4, self.b6, self.b8
        self.DeltaE = (-b2 * b2 * b8 - 8 * b4 ** 3 - 27 * b6 * b6 + 9 * b2 * b4 * b6) % p
        self.c4 = (b2 * b2 - 24 * b4) % p
        self.c6 = (-b2 ** 3 + 36 * b2 * b4 - 216 * b6) % p
        self.disc_f = (a2 * a2 * a4 * a4 - 4 * a4 ** 3 - 4 * a2 ** 3 * a6
                       - 27 * a6 * a6 + 18 * a2 * a4 * a6) % p

    def h(self, x):
        return (self.a1 * x + self.a3) % self.p

    def f(self, x):
        p = self.p
        return (((x + self.a2) * x + self.a4) * x + self.a6) % p


class Setup:
    pass


def setup_points(F, curve):
    p = F.p
    a1, a3, a2, a4, a6 = curve.a1, curve.a3, curve.a2, curve.a4, curve.a6
    F.build_sqrt()
    inv2 = F.inv(2)
    points = []
    E_aff = 0
    for x in range(p):
        hx = F.add(F.mul(a1, x), a3)
        fx = F.add(F.mul(F.add(F.mul(F.add(x, a2), x), a4), x), a6)
        D = F.add(F.mul(hx, hx), F.times4(fx))
        if D == 0:
            ws = [F.mul(F.neg(hx), inv2)]
        elif F.sqrt(D) > 0:
            s = F.sqrt(D)
            ws = [F.mul(F.add(F.neg(hx), s), inv2), F.mul(F.sub(F.neg(hx), s), inv2)]
        else:
            ws = []
        E_aff += len(ws)
        for w in ws:
            if w == 0:
                points.append((x, 0, 0, hx))
            elif F.sqrt(w) > 0:
                y = F.sqrt(w)
                points.append((x, y, w, hx))
                points.append((x, p - y, w, hx))
    points.sort()
    S = Setup()
    S.points = points
    S.E_aff = E_aff
    S.N_aff = len(points)
    S.N_ram = sum(1 for q in points if q[1] == 0)
    S.index = {(q[0], q[1]): i for i, q in enumerate(points)}
    S.nE = E_aff + 1
    S.nC = S.N_aff + 1
    return S


def eadd(F, curve, P, Q):
    if P is None:
        return Q
    if Q is None:
        return P
    a1, a3, a2, a4 = curve.a1, curve.a3, curve.a2, curve.a4
    x1, w1 = P
    x2, w2 = Q
    if x1 == x2:
        s = F.add(F.add(F.add(w1, w2), F.mul(a1, x2)), a3)
        if s == 0:
            return None
        num = F.sub(F.add(F.add(F.times3(F.mul(x1, x1)), F.dbl(F.mul(a2, x1))), a4),
                    F.mul(a1, w1))
        den = F.add(F.add(F.dbl(w1), F.mul(a1, x1)), a3)
        lam = F.mul(num, F.inv(den))
    else:
        lam = F.mul(F.sub(w2, w1), F.inv(F.sub(x2, x1)))
    x3 = F.sub(F.sub(F.sub(F.add(F.mul(lam, lam), F.mul(a1, lam)), a2), x1), x2)
    nu = F.sub(w1, F.mul(lam, x1))
    w3 = F.sub(F.sub(F.neg(F.mul(F.add(lam, a1), x3)), nu), a3)
    return (x3, w3)


def eneg(F, curve, P):
    if P is None:
        return None
    x, w = P
    return (x, F.sub(F.neg(w), F.add(F.mul(curve.a1, x), curve.a3)))


def emul(F, curve, P, k):
    R = None
    if k == 0:
        return None
    for bit in bin(k)[2:]:
        R = eadd(F, curve, R, R)
        if bit == '1':
            R = eadd(F, curve, R, P)
    return R


def free_field(p):
    """Throw-away counted field (never charged)."""
    return Fp(p, Ops())
