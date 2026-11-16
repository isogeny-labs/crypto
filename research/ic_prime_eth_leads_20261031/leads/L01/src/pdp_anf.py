"""Point-decomposition (m = 3) systems over GF(2^n) in WDSat's ANF format.

The encoding is the one of the Trimoska EC-Index-Calculus-Benchmarks corpus
(Trimoska, Ionica, Dequen, ePrint 2019/313), itself the symmetrised encoding of
Faugere-Gaudry-Huot-Renault and Galbraith-Gebregiyorgis: the curve is
y^2 + xy = x^3 + x^2 + a6, the factor base is {P : x(P) in V},
V = span{1, z, .., z^(l-1)} in the polynomial basis of GF(2)[z]/(f), and the
unknowns are

* variables 1 .. 3l: the coordinates of x_1, x_2, x_3 in V (block i holds x_(i+1));
* then the coordinates of e1 (l of them), e2 (2l - 1), e3 (3l - 2), the
  elementary symmetric functions of the x_i, which have degree < k(l-1)+1 in z
  and so need no reduction when 3l - 2 <= n.

Equations: the linking equations e_k = sigma_k(x_1, x_2, x_3) coordinate by
coordinate (degree k in the x bits), then the n coordinates of
S_4(e1, e2, e3, x_R) = 0, which is quadratic in the e bits. A boolean
equation f = 0 is written "x [T] monomials 0", with T present iff f has
constant term 0 (WDSat reads the line as an XOR equal to 1).

Only a6 = 1 is implemented (the corpus curve); S_4 below is the frozen
checker's (crypto:research/index_calculus_baseline_20260914/pilot/run_pilot.py,
GF.s4) with the substitution a = e1^2, b = e2^2, c = e3^2.
"""
from __future__ import annotations

from itertools import combinations


class Field:
    def __init__(self, n: int, modulus: int):
        assert modulus >> n == 1
        self.n, self.mod = n, modulus

    def mul(self, a: int, b: int) -> int:
        z = 0
        while b:
            if b & 1:
                z ^= a
            b >>= 1
            a <<= 1
            if a >> self.n:
                a ^= self.mod
        return z

    def frob(self, a: int, t: int) -> int:
        for _ in range(t):
            a = self.mul(a, a)
        return a


# A polynomial over GF(2^n) in boolean variables: {frozenset(vars): coeff}.
def _add(p: dict, mono: frozenset, c: int) -> None:
    v = p.get(mono, 0) ^ c
    if v:
        p[mono] = v
    else:
        p.pop(mono, None)


def _mul(F: Field, p: dict, q: dict) -> dict:
    r: dict = {}
    for m1, c1 in p.items():
        for m2, c2 in q.items():
            _add(r, m1 | m2, F.mul(c1, c2))
    return r


def _scale(F: Field, p: dict, c: int) -> dict:
    return {m: F.mul(v, c) for m, v in p.items() if F.mul(v, c)}


def _sum(*ps: dict) -> dict:
    r: dict = {}
    for p in ps:
        for m, c in p.items():
            _add(r, m, c)
    return r


def _form(F: Field, vars_: list[int], t: int = 0) -> dict:
    """(sum_j c_j z^j)^(2^t) as a linear form in the bits c_j."""
    return {frozenset([v]): F.frob(1 << j, t) for j, v in enumerate(vars_) if F.frob(1 << j, t)}


def layout(l: int) -> dict:
    x = [list(range(1 + i * l, 1 + (i + 1) * l)) for i in range(3)]
    nxt = 3 * l + 1
    e = []
    for k in (1, 2, 3):
        d = k * (l - 1) + 1
        e.append(list(range(nxt, nxt + d)))
        nxt += d
    return {'x': x, 'e': e, 'nvars': nxt - 1}


def linking_equations(l: int) -> list[set]:
    """Boolean equations e_k[j] + sigma_k(x)[j] = 0, as sets of monomials (frozensets)."""
    L = layout(l)
    x, e = L['x'], L['e']
    eqs = []
    for k, ek in zip((1, 2, 3), e):
        for j, v in enumerate(ek):
            eq = {frozenset([v])}
            for blocks in combinations(range(3), k):
                # coefficient of z^j in prod_{b in blocks} x_b: sum over index tuples with sum j
                def rec(bi, rem, acc):
                    if bi == len(blocks):
                        if rem == 0:
                            eq.symmetric_difference_update({frozenset(acc)})
                        return
                    for i in range(min(l - 1, rem) + 1):
                        rec(bi + 1, rem - i, acc + [x[blocks[bi]][i]])
                rec(0, j, [])
            eqs.append(eq)
    return eqs


def s4_equations(F: Field, l: int, xR: int) -> list[set]:
    """The n coordinate equations of S_4(e1, e2, e3, xR) = 0 (a6 = 1)."""
    assert 3 * l - 2 <= F.n
    L = layout(l)
    e1, e2, e3 = L['e']
    one = {frozenset(): 1}
    r = xR
    r2 = F.mul(r, r)
    r3 = F.mul(r2, r)
    r4 = F.mul(r2, r2)
    E3 = _form(F, e3)
    a = _form(F, e1, 1)     # e1^2
    b = _form(F, e2, 1)     # e2^2
    c = _form(F, e3, 1)     # e3^2
    a2 = _form(F, e1, 2)    # a^2
    c2 = _form(F, e3, 2)    # c^2
    b2 = _form(F, e2, 2)    # b^2
    poly = _sum(
        _scale(F, one, r4), a2, c2, _scale(F, b2, r4),
        _scale(F, _mul(F, c, E3), r), _scale(F, _mul(F, E3, b), r3),
        _scale(F, _mul(F, E3, a), r), _scale(F, E3, r3),
        _scale(F, _mul(F, a, c), r2), _scale(F, c, r4), c, _scale(F, b, r2),
    )
    eqs = []
    for bit in range(F.n):
        eqs.append({m for m, v in poly.items() if (v >> bit) & 1})
    return eqs


def system(n: int, modulus: int, l: int, xR: int) -> tuple[int, list[set]]:
    F = Field(n, modulus)
    L = layout(l)
    return L['nvars'], linking_equations(l) + s4_equations(F, l, xR)


# ------------------------------------------------------------- the curve, a6 = 1
def _inv(F: Field, a: int) -> int:
    assert a
    z, k = 1, (1 << F.n) - 2
    while k:
        if k & 1:
            z = F.mul(z, a)
        a = F.mul(a, a)
        k >>= 1
    return z


def lift(F: Field, x: int) -> list[tuple[int, int]]:
    """Points of y^2 + xy = x^3 + x^2 + 1 above x (n odd: half-trace solves z^2 + z = c)."""
    assert F.n % 2 == 1
    if x == 0:
        return [(0, 1)]
    c = x ^ 1 ^ F.mul(_inv(F, x), _inv(F, x))
    z = v = c
    for _ in range((F.n - 1) // 2):
        v = F.frob(v, 2)
        z ^= v
    if F.mul(z, z) ^ z != c:
        return []
    y = F.mul(x, z)
    return [(x, y), (x, y ^ x)]


def add(F: Field, P, Q):
    if P is None:
        return Q
    if Q is None:
        return P
    (x, y), (u, v) = P, Q
    if x == u:
        if y != v or x == 0:
            return None
        lam = x ^ F.mul(y, _inv(F, x))
        w = F.mul(lam, lam) ^ lam ^ 1
        return (w, F.mul(x, x) ^ F.mul(lam ^ 1, w))
    lam = F.mul(y ^ v, _inv(F, x ^ u))
    w = F.mul(lam, lam) ^ lam ^ x ^ u ^ 1
    return (w, F.mul(lam, x ^ w) ^ w ^ y)


def sample_target(F: Field, l: int, rng, mode: str) -> dict:
    """x_R for one instance. 'uniform': R a uniformly random affine point with x != 0
    (the relation-collection distribution: most instances are UNSAT). 'planted':
    R = P1 + P2 + P3 with x(Pi) uniform over the liftable abscissas of V (SAT by
    construction unless R is O or has x = 0, which are redrawn)."""
    while True:
        if mode == 'uniform':
            x = rng.randrange(1, 1 << F.n)
            if lift(F, x):
                return {'xR': x, 'planted': None}
        elif mode == 'planted':
            fb = [x for x in range(1, 1 << l) if lift(F, x)]
            xs = [rng.choice(fb) for _ in range(3)]
            R = None
            for x in xs:
                R = add(F, R, rng.choice(lift(F, x)))
            if R is not None and R[0] != 0:
                return {'xR': R[0], 'planted': xs}
        else:
            raise ValueError(mode)


def to_anf(nvars: int, eqs: list[set]) -> str:
    lines = [f'p cnf {nvars} {len(eqs)}']
    for eq in eqs:
        const = frozenset() in eq
        toks = ['x'] + ([] if const else ['T'])
        for mono in sorted((m for m in eq if m), key=lambda m: (len(m), sorted(m))):
            vs = sorted(mono)
            toks += [str(vs[0])] if len(vs) == 1 else [f'.{len(vs)}', *map(str, vs)]
        lines.append(' '.join(toks + ['0']))
    return '\n'.join(lines) + '\n'


def parse_anf(text: str) -> tuple[int, list[set]]:
    """Inverse of to_anf (any monomial order): equations f = 0 as sets of monomials."""
    lines = [s for s in text.splitlines() if s.strip()]
    nvars = int(lines[0].split()[2])
    eqs = []
    for line in lines[1:]:
        tok = line.split()
        assert tok[0] == 'x'
        eq: set = set()
        has_t = False
        i = 1
        while tok[i] != '0':
            t = tok[i]
            i += 1
            if t == 'T':
                has_t = not has_t
            elif t.startswith('.'):
                d = int(t[1:])
                eq ^= {frozenset(map(int, tok[i:i + d]))}
                i += d
            else:
                eq ^= {frozenset([int(t)])}
        if not has_t:
            eq ^= {frozenset()}
        eqs.append(eq)
    return nvars, eqs
