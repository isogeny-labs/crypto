"""F_{p^2} arithmetic and polynomials over it (pure Python).

An element a + b*theta of F_q = F_p[theta]/(theta^2 - n) is the integer
a + p*b.  F_p is the set of integers < p.  Multiplication uses discrete-log
tables of a primitive element; addition works on the two coordinates.

Python prototype under a task-level exception to the repository's no-Python
rule (see PROTOCOL.md section 1); not a final harness.
"""
from __future__ import annotations

from math import gcd


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True


def factorint(n: int) -> dict[int, int]:
    """Trial-division factorisation of a positive integer (inputs here are small)."""
    out: dict[int, int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            out[d] = out.get(d, 0) + 1
            n //= d
        d += 1 if d == 2 else 2
    if n > 1:
        out[n] = out.get(n, 0) + 1
    return out


class Fq:
    """The field F_{p^2}, p an odd prime >= 5."""

    def __init__(self, p: int):
        if p < 5 or not is_prime(p):
            raise ValueError("p must be a prime >= 5")
        self.p = p
        self.q = p * p
        # least quadratic non-residue mod p: theta^2 = n
        n = 2
        while pow(n, (p - 1) // 2, p) != p - 1:
            n += 1
        self.n = n
        self._build_tables()

    # ---- slow reference arithmetic (independent of the log tables) ----
    def mul_slow(self, x: int, y: int) -> int:
        p = self.p
        xb, xa = divmod(x, p)
        yb, ya = divmod(y, p)
        return (xa * ya + self.n * xb * yb) % p + p * ((xa * yb + xb * ya) % p)

    def add_slow(self, x: int, y: int) -> int:
        return self.add(x, y)

    def _build_tables(self) -> None:
        p, q = self.p, self.q
        order_primes = list(factorint(q - 1))
        for g in range(p, q):  # skip F_p itself, whose elements have order | p-1
            # primitive iff g^((q-1)/r) != 1 for every prime r | q-1 (slow pow)
            ok = True
            for r in order_primes:
                e = (q - 1) // r
                acc, base = 1, g
                while e:
                    if e & 1:
                        acc = self.mul_slow(acc, base)
                    base = self.mul_slow(base, base)
                    e >>= 1
                if acc == 1:
                    ok = False
                    break
            if ok:
                self.gen = g
                break
        else:  # pragma: no cover
            raise RuntimeError("no primitive element found")
        exp = [0] * (2 * (q - 1) + 2)
        log = [-1] * q
        x = 1
        for i in range(q - 1):
            exp[i] = x
            log[x] = i
            x = self.mul_slow(x, self.gen)
        assert x == 1
        for i in range(q - 1, 2 * (q - 1) + 2):
            exp[i] = exp[i - (q - 1)]
        self.EXP = exp
        self.LOG = log

    # ---- arithmetic ----
    def add(self, x: int, y: int) -> int:
        p = self.p
        xb, xa = divmod(x, p)
        yb, ya = divmod(y, p)
        return (xa + ya) % p + p * ((xb + yb) % p)

    def neg(self, x: int) -> int:
        p = self.p
        xb, xa = divmod(x, p)
        return (-xa) % p + p * ((-xb) % p)

    def sub(self, x: int, y: int) -> int:
        return self.add(x, self.neg(y))

    def mul(self, x: int, y: int) -> int:
        if x == 0 or y == 0:
            return 0
        return self.EXP[self.LOG[x] + self.LOG[y]]

    def sq(self, x: int) -> int:
        return self.mul(x, x)

    def inv(self, x: int) -> int:
        if x == 0:
            raise ZeroDivisionError("inverse of 0 in F_q")
        return self.EXP[(self.q - 1) - self.LOG[x]]

    def div(self, x: int, y: int) -> int:
        return self.mul(x, self.inv(y))

    def pow(self, x: int, e: int) -> int:
        if e == 0:
            return 1
        if x == 0:
            return 0
        return self.EXP[(self.LOG[x] * e) % (self.q - 1)]

    def const(self, c: int) -> int:
        """Image of the integer c in F_p, as an element of F_q."""
        return c % self.p

    def is_square(self, x: int) -> bool:
        return x == 0 or self.LOG[x] % 2 == 0

    def sqrt(self, x: int) -> int | None:
        """A square root of x in F_q, or None."""
        if x == 0:
            return 0
        if self.LOG[x] % 2:
            return None
        return self.EXP[self.LOG[x] // 2]

    def comps(self, x: int) -> tuple[int, int]:
        """(a, b) with x = a + b*theta."""
        b, a = divmod(x, self.p)
        return a, b

    def in_fp(self, x: int) -> bool:
        return x < self.p


# ---------------------------------------------------------------- polynomials
# A polynomial is a list of field elements, low degree first, no trailing zeros.

def ptrim(f: list[int]) -> list[int]:
    while f and f[-1] == 0:
        f.pop()
    return f


def pdeg(f: list[int]) -> int:
    return len(f) - 1  # deg(0) = -1


def padd(F: Fq, f: list[int], g: list[int]) -> list[int]:
    n = max(len(f), len(g))
    out = [0] * n
    for i in range(n):
        a = f[i] if i < len(f) else 0
        b = g[i] if i < len(g) else 0
        out[i] = F.add(a, b) if (a and b) else (a or b)
    return ptrim(out)


def psub(F: Fq, f: list[int], g: list[int]) -> list[int]:
    return padd(F, f, [F.neg(c) for c in g])


def pscale(F: Fq, f: list[int], c: int) -> list[int]:
    if c == 0:
        return []
    return [F.mul(a, c) for a in f]


def pmul(F: Fq, f: list[int], g: list[int]) -> list[int]:
    if not f or not g:
        return []
    EXP, LOG, p = F.EXP, F.LOG, F.p
    res = [0] * (len(f) + len(g) - 1)
    lg = [LOG[b] if b else -1 for b in g]
    for i, a in enumerate(f):
        if a == 0:
            continue
        la = LOG[a]
        for j, lb in enumerate(lg):
            if lb < 0:
                continue
            c = EXP[la + lb]
            r = res[i + j]
            if r == 0:
                res[i + j] = c
            else:
                rb, ra = divmod(r, p)
                cb, ca = divmod(c, p)
                res[i + j] = (ra + ca) % p + p * ((rb + cb) % p)
    return ptrim(res)


def pdivmod(F: Fq, f: list[int], g: list[int]) -> tuple[list[int], list[int]]:
    if not g:
        raise ZeroDivisionError("polynomial division by 0")
    r = list(f)
    dg = len(g) - 1
    if len(r) - 1 < dg:
        return [], ptrim(r)
    lead_inv = F.inv(g[-1])
    quo = [0] * (len(r) - dg)
    for k in range(len(r) - 1, dg - 1, -1):
        c = r[k]
        if c == 0:
            continue
        coef = F.mul(c, lead_inv)
        quo[k - dg] = coef
        for j in range(dg + 1):
            if g[j]:
                r[k - dg + j] = F.sub(r[k - dg + j], F.mul(coef, g[j]))
    return ptrim(quo), ptrim(r[:dg])


def pmod(F: Fq, f: list[int], g: list[int]) -> list[int]:
    return pdivmod(F, f, g)[1]


def pmonic(F: Fq, f: list[int]) -> list[int]:
    if not f:
        return f
    return pscale(F, f, F.inv(f[-1]))


def pgcd(F: Fq, f: list[int], g: list[int]) -> list[int]:
    f, g = list(f), list(g)
    while g:
        f, g = g, pmod(F, f, g)
    return pmonic(F, f)


def ppowmod(F: Fq, base: list[int], e: int, mod: list[int]) -> list[int]:
    result = [1]
    b = pmod(F, base, mod)
    while e:
        if e & 1:
            result = pmod(F, pmul(F, result, b), mod)
        e >>= 1
        if e:
            b = pmod(F, pmul(F, b, b), mod)
    return result


def peval(F: Fq, f: list[int], x: int) -> int:
    acc = 0
    for c in reversed(f):
        acc = F.add(F.mul(acc, x), c)
    return acc


def proots(F: Fq, f: list[int]) -> list[int]:
    """All roots of f in F_q by evaluation (q is tiny)."""
    return [x for x in range(F.q) if peval(F, f, x) == 0]


def pderiv(F: Fq, f: list[int]) -> list[int]:
    return ptrim([F.mul(c, F.const(i)) for i, c in enumerate(f)][1:])


def ddf(F: Fq, f: list[int]) -> list[tuple[list[int], int]]:
    """Distinct-degree factorisation of a monic squarefree f: [(product, degree)]."""
    out = []
    h = [0, 1]
    fs = list(f)
    d = 0
    while 2 * (d + 1) <= pdeg(fs):
        d += 1
        h = ppowmod(F, h, F.q, fs)  # h = x^(q^d) mod fs
        g = pgcd(F, fs, psub(F, h, [0, 1]))
        if pdeg(g) > 0:
            out.append((g, d))
            fs, _ = pdivmod(F, fs, g)
            h = pmod(F, h, fs)
    if pdeg(fs) > 0:
        out.append((pmonic(F, fs), pdeg(fs)))
    return out


def edf(F: Fq, g: list[int], d: int, rng) -> list[list[int]]:
    """Cantor-Zassenhaus: split a monic product g of irreducibles of degree d."""
    if pdeg(g) == d:
        return [pmonic(F, g)]
    e = (F.q ** d - 1) // 2
    while True:
        r = ptrim([rng.randrange(F.q) for _ in range(pdeg(g))])
        if pdeg(r) < 1:
            continue
        b = ppowmod(F, r, e, g)
        gg = pgcd(F, g, psub(F, b, [1]))
        if 0 < pdeg(gg) < pdeg(g):
            other, rem = pdivmod(F, g, gg)
            assert not rem
            return edf(F, pmonic(F, gg), d, rng) + edf(F, pmonic(F, other), d, rng)


def factor_squarefree(F: Fq, f: list[int], rng) -> list[tuple[list[int], int]]:
    """Irreducible factors (monic, degree) of a monic squarefree f."""
    out = []
    for g, d in ddf(F, pmonic(F, f)):
        for fac in edf(F, g, d, rng):
            out.append((fac, d))
    out.sort(key=lambda t: (t[1], t[0]))
    return out
