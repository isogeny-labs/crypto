"""Deterministic elliptic-curve arithmetic with operation counters.

Every public method increments a shared Counter, so a run can be priced in
operations rather than wall time (AGENTS.md §5a).  Costs are reported both as
raw class counts and as mul-equivalents:

    mul-equiv = mul + sqr + 100*inv + add/10

The 100x inversion weight and 1/10 addition weight are a documented model, not
a hardware measurement; they are used only to compare arms within one report.
"""
from __future__ import annotations

import random
from dataclasses import dataclass, field


@dataclass
class Counter:
    mul: int = 0
    sqr: int = 0
    inv: int = 0
    add: int = 0
    pow: int = 0          # modular exponentiations (sqrt / Legendre)
    group_add: int = 0    # elliptic additions or doublings

    def total(self) -> int:
        return self.mul + self.sqr + self.inv + self.add + self.pow + self.group_add

    def mul_equiv(self) -> int:
        return self.mul + self.sqr + 100*self.inv + self.add//10 + 40*self.pow

    def as_dict(self) -> dict:
        return {"mul": self.mul, "sqr": self.sqr, "inv": self.inv, "add": self.add,
                "pow": self.pow, "group_add": self.group_add,
                "total": self.total(), "mul_equiv": self.mul_equiv()}


def _jacobi(a: int, n: int, c: Counter) -> int:
    """Legendre symbol for odd prime n; counted as a pow-class operation."""
    c.pow += 1
    return pow(a % n, (n - 1)//2, n)


def sqrt_mod(a: int, p: int, c: Counter) -> int | None:
    """Tonelli-Shanks; returns a square root or None.  Counts one pow + work."""
    a %= p
    if a == 0:
        return 0
    if _jacobi(a, p, c) != 1:
        return None
    if p % 4 == 3:
        c.pow += 1
        return pow(a, (p + 1)//4, p)
    # Tonelli-Shanks
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2; s += 1
    z = 2
    while _jacobi(z, p, c) != p - 1:
        z += 1
    m, cc, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1)//2, p)
    c.pow += 4
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = (t2*t2) % p; i += 1
            if i == m:
                return None
        b = pow(cc, 1 << (m - i - 1), p)
        m, cc = i, (b*b) % p
        t, r = (t*cc) % p, (r*b) % p
        c.pow += 3; c.mul += 2
    return r


class Curve:
    """Short Weierstrass y^2 = x^3 + a x + b over F_p, affine coordinates."""

    def __init__(self, p: int, a: int = 0, b: int = 7, counter: Counter | None = None):
        self.p, self.a, self.b = p, a % p, b % p
        self.c = counter or Counter()

    def on_curve(self, P) -> bool:
        if P is None:
            return True
        x, y = P
        return (y*y - (x*x*x + self.a*x + self.b)) % self.p == 0

    def neg(self, P):
        return None if P is None else (P[0], (-P[1]) % self.p)

    def add(self, P, Q):
        c, p = self.c, self.p
        if P is None: return Q
        if Q is None: return P
        c.group_add += 1
        x1, y1 = P; x2, y2 = Q
        if x1 == x2 and (y1 + y2) % p == 0:
            return None
        if P == Q:
            if y1 % p == 0:
                return None
            lam = (3*x1*x1 + self.a) * pow(2*y1, -1, p) % p
            c.inv += 1; c.mul += 2; c.sqr += 1; c.add += 2
        else:
            lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
            c.inv += 1; c.mul += 1; c.add += 2
        x3 = (lam*lam - x1 - x2) % p
        y3 = (lam*(x1 - x3) - y1) % p
        c.sqr += 1; c.mul += 1; c.add += 2
        return (x3, y3)

    def mul(self, k: int, P):
        Q, R = P, None
        while k:
            if k & 1:
                R = self.add(R, Q)
            Q = self.add(Q, Q)
            k >>= 1
        return R

    def points_with_x_upto(self, B: int, rng: random.Random | None = None):
        """All affine points with 1 <= x <= B (one of the two signs)."""
        rng = rng or random.Random(0)
        out = []
        for x in range(1, B+1):
            rhs = (x*x*x + self.a*x + self.b) % self.p
            if rhs == 0:
                continue
            y = sqrt_mod(rhs, self.p, self.c)
            if y is None:
                continue
            if rng.getrandbits(1):
                y = (-y) % self.p
            out.append((x, y))
        return out
