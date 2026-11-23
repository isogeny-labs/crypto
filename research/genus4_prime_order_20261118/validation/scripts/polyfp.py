"""Validator's own factor-degree tools over F_p (independent of the producer's Rabin test).

Distinct-degree factorisation: g_d = gcd(x^(p^d) - x, f / (g_1 ... g_{d-1})) is the product of the
irreducible factors of degree d.  A reducible f of degree 9 has an irreducible factor of degree <= 4,
so f is reducible iff some g_d (d <= 4) is non-trivial, or f is not squarefree.

Certificate of reducibility: a monic w | f with 1 <= deg w <= 4 (checked by exact division).  For
y^2 = f(x), deg f = 2g + 1 = 9, f squarefree, the reduced Mumford divisor (w, 0) is F_p-rational,
non-zero (u != 1) and 2-torsion (2(w, 0) = div(w)), so #J(F_p) is even.
"""
import random
from jac import trim, padd, psub, pmul, pdivmod, pxgcd, pderiv, pmonic

X = (0, 1)


def pmod(a, f, p):
    return pdivmod(a, f, p)[1]


def ppowmod(a, e, f, p):
    res, base = (1,), pmod(a, f, p)
    while e:
        if e & 1:
            res = pmod(pmul(res, base, p), f, p)
        base = pmod(pmul(base, base, p), f, p)
        e >>= 1
    return res


def squarefree(f, p):
    return pxgcd(f, pderiv(f, p), p)[0] == (1,)


def ddf(f, p):
    """[(d, g_d)] for the non-trivial distinct-degree parts of a squarefree monic f."""
    f = trim(f)
    out, rest, h, d = [], f, X, 0
    while len(rest) - 1 >= 2 * (d + 1):
        d += 1
        h = ppowmod(h, p, rest, p)
        g = pxgcd(rest, psub(h, X, p), p)[0]
        if len(g) > 1:
            out.append((d, g))
            rest = pdivmod(rest, g, p)[0]
            h = pmod(h, rest, p)
    if len(rest) > 1:
        out.append((len(rest) - 1, pmonic(rest, p)))
    return out


def n_factors(f, p):
    return sum((len(g) - 1) // d for d, g in ddf(f, p))


def edf_one(g, d, p, rnd):
    """One non-trivial monic factor of g, a product of >= 2 distinct irreducibles of degree d (p odd)."""
    n = len(g) - 1
    e = (p ** d - 1) // 2
    while True:
        a = trim([rnd.randrange(p) for _ in range(n)])
        if len(a) < 2:
            continue
        w = pxgcd(g, a, p)[0]
        if 1 < len(w) < len(g):
            return w
        b = psub(ppowmod(a, e, g, p), (1,), p)
        w = pxgcd(g, b, p)[0]
        if 1 < len(w) < len(g):
            return w


def reducible_certificate(f, p, rnd=None):
    """None if f is irreducible; else ("not_squarefree", gcd(f, f')) or ("factor", w), w | f monic, 1 <= deg w <= 4."""
    f = trim(f)
    rnd = rnd or random.Random(1)
    gs = pxgcd(f, pderiv(f, p), p)[0]
    if gs != (1,):
        return ("not_squarefree", gs)
    parts = ddf(f, p)
    if len(parts) == 1 and parts[0][0] == len(f) - 1:
        return None
    d, g = parts[0]                       # smallest degree present
    n = len(f) - 1
    while len(g) - 1 > 4:
        co = pdivmod(f, g, p)[0]
        if 1 <= len(co) - 1 <= 4:
            g = pmonic(co, p)
            break
        g = edf_one(g, d, p, rnd)
    return ("factor", g)


def check_certificate(f, p, cert):
    """Independent check: w monic, 1 <= deg w <= 4, w | f; or gcd(f, f') non-trivial and divides f."""
    kind, w = cert
    if kind == "factor":
        return w[-1] == 1 and 1 <= len(w) - 1 <= 4 and not pdivmod(trim(f), w, p)[1]
    if kind == "not_squarefree":
        return len(w) > 1 and not pdivmod(trim(f), w, p)[1] and not pdivmod(pderiv(trim(f), p), w, p)[1]
    return False
