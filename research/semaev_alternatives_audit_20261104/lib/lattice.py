"""Integer shift lattices with a deterministic cost model.

The lattice is the row span of shift polynomials evaluated on a monomial basis,
scaled column-wise by X^u Y^v.  Reduction is delegated to fplll (subprocess);
the reduction itself is charged by the documented model n^3 * L / 64 word-ops
(n = rank, L = max entry bit length), because fplll exposes no op counter.
"""
from __future__ import annotations

import itertools
import subprocess


def poly_mul(f: dict, g: dict) -> dict:
    out = {}
    for (u1, v1), c1 in f.items():
        for (u2, v2), c2 in g.items():
            k = (u1+u2, v1+v2)
            out[k] = out.get(k, 0) + c1*c2
    return out


def build(f: dict, shifts, X: int, Y: int):
    """shifts: iterable of (monomial_shift (i,j), polynomial dict)."""
    rows, monoms = [], set()
    for (i, j), g in shifts:
        row = {(u+i, v+j): c for (u, v), c in g.items()}
        rows.append(row); monoms |= set(row)
    monoms = sorted(monoms)
    M = [[r.get(mn, 0) * X**mn[0] * Y**mn[1] for mn in monoms] for r in rows]
    return M, monoms


def reduce_fplll(M, delta: float = 0.99):
    if not M:
        raise ValueError("empty lattice")
    if any(not isinstance(c, int) for row in M for c in row):
        raise TypeError("lattice entries must be ints")
    txt = "[" + "\n".join("[" + " ".join(str(c) for c in row) + "]" for row in M) + "]"
    r = subprocess.run(["fplll", "-a", "lll", "-d", str(delta)],
                       input=txt, capture_output=True, text=True)
    if r.returncode != 0:
        raise RuntimeError("fplll: " + r.stderr.strip()[:300])
    rows, cur = [], None
    for tok in r.stdout.replace("[", " [ ").replace("]", " ] ").split():
        if tok == "[":
            cur = []
        elif tok == "]":
            if cur is not None:
                rows.append(cur); cur = None
        else:
            cur.append(int(tok))
    return rows


def to_poly(row, monoms, X: int, Y: int) -> dict:
    out = {}
    for c, (u, v) in zip(row, monoms):
        s = X**u * Y**v
        if c % s:
            raise ValueError("scaling not exact: reduce before dividing")
        out[(u, v)] = c // s
    return out


def eval_int(poly: dict, x0: int, y0: int) -> int:
    return sum(c * x0**u * y0**v for (u, v), c in poly.items())


def lll_charge(rank: int, bit_length: int) -> int:
    return rank**3 * max(bit_length, 1) // 64


def recover(polys, p: int, x0: int, y0: int) -> bool:
    """Recover a known root from two polynomials that vanish on it over Z.

    The resultant must be taken over Z: modulo p it is frequently identically
    zero because every vanishing polynomial shares the S_3 component mod p.
    """
    try:
        import sympy as sp
    except ImportError:                      # pragma: no cover
        return False
    x, y = sp.symbols('x y')
    exprs = []
    for P in polys[:8]:
        e = sum(sp.Integer(c) * x**u * y**v for (u, v), c in P.items())
        exprs.append(sp.Poly(e, x, y))
    for A, B in itertools.combinations(exprs, 2):
        for var, other, target in ((y, x, x0), (x, y, y0)):
            try:
                res = sp.resultant(A, B, var)
            except Exception:
                continue
            if res == 0:
                continue
            rp = sp.Poly(sp.expand(res), other)
            if rp.degree() is None or rp.degree() > 32:
                continue
            try:
                roots = rp.ground_roots()
            except Exception:
                roots = {}
            for root in roots:
                if getattr(root, "is_Integer", False) and int(root) % p == target % p:
                    return True
    return False


def is_multiple_of(poly: dict, base: dict, nvars: int = 2) -> bool:
    """True if poly is divisible by base as an integer polynomial."""
    try:
        import sympy as sp
    except ImportError:                      # pragma: no cover
        return False
    if nvars == 1:
        x = sp.Symbol('x')
        P = sp.Poly(sum(sp.Integer(c)*x**u for (u, _), c in poly.items()), x)
        Q = sp.Poly(sum(sp.Integer(c)*x**u for (u, _), c in base.items()), x)
    else:
        x, y = sp.symbols('x y')
        P = sp.Poly(sum(sp.Integer(c)*x**u*y**v for (u, v), c in poly.items()), x, y)
        Q = sp.Poly(sum(sp.Integer(c)*x**u*y**v for (u, v), c in base.items()), x, y)
    if P.is_zero:
        return True
    return sp.rem(P, Q).is_zero
