"""Exact Frobenius-sign / quadratic-twist sector extractor for S_(m+1).

For specialized x_i in F_p, every ordinate lives in F_(p^2).
Frobenius sends each point P_i to +P_i or -P_i. If A and B
are the corresponding partial sums, A+B=O implies A=B in E(F_p)[2].
Conversely, if A=B=T is common rational two-torsion then A+B=O.

This computes *projected F_p roots* of S_(m+1), not an elimination ideal,
GB solving degree or an asymptotically faster index calculus algorithm.
Only the summation polynomial is in scope: no factor-base map.
"""
from __future__ import annotations
from functools import lru_cache


def legendre(t: int, p: int) -> int:
    t %= p
    if t == 0:
        return 0
    return 1 if pow(t, (p - 1) // 2, p) == 1 else -1


@lru_cache(maxsize=100)
def nonsquare(p: int) -> int:
    return next(d for d in range(2, p) if legendre(d, p) == -1)


def sqrt_mod(a: int, p: int) -> int | None:
    """Deterministic Tonelli-Shanks; requires odd prime p."""
    a %= p
    if a == 0:
        return 0
    if legendre(a, p) != 1:
        return None
    if p % 4 == 3:
        return pow(a, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    c, t, r, m = pow(nonsquare(p), q, p), pow(a, q, p), pow(a, (q + 1) // 2, p), s
    while t != 1:
        i, t2 = 1, t * t % p
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
            if i >= m:
                raise ArithmeticError("Tonelli-Shanks invariant")
        b = pow(c, 1 << (m - i - 1), p)
        r, t, c, m = r * b % p, t * b * b % p, b * b % p, i
    return r


def f(x: int, p: int, a: int, b: int) -> int:
    return (x * x * x + a * x + b) % p


def classify(x: int, p: int, a: int, b: int) -> str:
    """y=0 torsion is assigned to + even though both signs act identically."""
    return "minus" if legendre(f(x, p, a, b), p) == -1 else "plus"


def point_neg(P, p):
    return None if P is None else (P[0], -P[1] % p)


def add(P, Q, p, a, scale):
    """Group law on scale*y²=x³+a*x+b; None is the identity.

    scale=1 is E(Fp), scale=d is the rational model of anti-Frobenius
    points via original ordinate omega*y, omega²=d.
    """
    if P is None:
        return Q
    if Q is None:
        return P
    x, y = P
    u, v = Q
    if x == u:
        if (y + v) % p == 0:
            return None
        if y != v:
            raise ArithmeticError("invalid same-x chart")
        slope = (3 * x * x + a) * pow(2 * scale * y % p, -1, p) % p
    else:
        slope = (v - y) * pow((u - x) % p, -1, p) % p
    w = (scale * slope * slope - x - u) % p
    z = (slope * (x - w) - y) % p
    return w, z


def ordinates(x, p, a, b, scale):
    rhs = f(x, p, a, b) * pow(scale, -1, p) % p
    y = sqrt_mod(rhs, p)
    if y is None:
        raise AssertionError("wrong twist sector for leaf")
    return (y,) if y == 0 else (y, -y % p)


def signed_sums(xs, p, a, b, scale):
    states = {None}
    for x in xs:
        states = {add(P, (x, y), p, a, scale)
                  for P in states for y in ordinates(x, p, a, b, scale)}
    return states


@lru_cache(maxsize=128)
def two_torsion(p, a, b):
    """Shared E(Fp)[2], with cold per-curve setup separately chargeable."""
    if p < 1000:
        roots = {x for x in range(p) if f(x, p, a, b) == 0}
    else:
        import sympy as sp
        z = sp.Symbol("z")
        poly = sp.Poly(z**3 + (a % p) * z + b % p, z, modulus=p)
        roots = set()
        for factor, _ in poly.factor_list()[1]:
            if factor.degree() == 1:
                u, v = map(int, factor.all_coeffs())
                x = -v * pow(u % p, -1, p) % p
                if f(x, p, a, b) != 0:
                    raise ArithmeticError("bad cubic linear factor")
                roots.add(x)
    return frozenset({None} | {(x, 0) for x in roots})


def frobenius_sector_roots(p, a, b, xs, *, return_stats=False):
    """Compute exact Fp X satisfying S_(m+1)(*xs, X)=0, m>=2.

    Requires odd prime p>3 and 4a³+27b² != 0. A torsion point with
    ordinate zero is placed in the base sector. All nonidentity 2-torsion
    branches are retained; omitting them yields a known counterexample.
    """
    if p <= 3 or (4 * a**3 + 27 * b**2) % p == 0:
        raise ValueError("nonsingular short Weierstrass E/Fp required")
    d = nonsquare(p)
    plus = [x % p for x in xs if classify(x, p, a, b) == "plus"]
    minus = [x % p for x in xs if classify(x, p, a, b) == "minus"]
    A = signed_sums(plus, p, a, b, 1)
    B = signed_sums(minus, p, a, b, d)
    T = two_torsion(p, a, b)
    roots = set()
    plus_match, minus_match = B & T, A & T
    for t in plus_match:
        for P in A:
            Q = add(t, point_neg(P, p), p, a, 1)
            if Q is not None:
                roots.add(Q[0])
    for t in minus_match:
        for P in B:
            Q = add(t, point_neg(P, p), p, a, d)
            if Q is not None:
                roots.add(Q[0])
    if return_stats:
        return roots, {
            "plus_leaves": len(plus), "minus_leaves": len(minus),
            "plus_states": len(A), "minus_states": len(B),
            "common_torsion": len(T), "eligible_plus_torsion": len(plus_match),
            "eligible_minus_torsion": len(minus_match),
            "candidate_roots": len(roots),
            "torsion_setup": "cached_scan" if p < 1000 else "cached_cubic_factor",
        }
    return roots


def zero_only_roots(p, a, b, xs):
    """Intentionally incomplete O-only comparison; regression counterexample."""
    d = nonsquare(p)
    plus = [x % p for x in xs if classify(x, p, a, b) == "plus"]
    minus = [x % p for x in xs if classify(x, p, a, b) == "minus"]
    A, B = signed_sums(plus, p, a, b, 1), signed_sums(minus, p, a, b, d)
    roots = set()
    if None in B:
        roots.update(Q[0] for P in A if (Q := point_neg(P, p)) is not None)
    if None in A:
        roots.update(Q[0] for P in B if (Q := point_neg(P, p)) is not None)
    return roots
