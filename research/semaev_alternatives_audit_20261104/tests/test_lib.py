"""Unit tests for the audit library and harness.

Run:  PYTHONPATH=. python3 -m pytest tests -q
"""
import random

import sympy as sp

from lib import lattice as L
from lib.ec import Curve, Counter, sqrt_mod
from lib.accounting import Record
from experiments.control_univariate import univariate_shifts


def test_vieta_conic_identity():
    a, b, x1, x2, xr, e1, e2 = sp.symbols('a b x1 x2 xr e1 e2')
    S3 = ((x1-x2)**2*xr**2 - 2*((x1+x2)*(x1*x2+a)+2*b)*xr
          + (x1*x2-a)**2 - 4*b*(x1+x2))
    rem = sp.rem(sp.Poly(S3.subs(x2, e1-x1), x1), sp.Poly(x1**2 - e1*x1 + e2, x1))
    Q = sp.expand(rem.as_expr())
    target = (xr**2*(e1**2-4*e2) - 2*xr*(e1*e2 + a*e1 + 2*b)
              + e2**2 - 2*a*e2 + a**2 - 4*b*e1)
    assert sp.simplify(Q - target) == 0
    assert sp.Poly(Q, e1, e2).total_degree() == 2


def test_discriminant_identity():
    a, b, x1, x2, T = sp.symbols('a b x1 x2 T')
    g = ((x1-x2)**2*T**2 - 2*((x1+x2)*(x1*x2+a)+2*b)*T
         + (x1*x2-a)**2 - 4*b*(x1+x2))
    disc = sp.discriminant(sp.Poly(g, T))
    assert sp.simplify(disc - 16*(x1**3+a*x1+b)*(x2**3+a*x2+b)) == 0


def test_curve_group_and_counter():
    p = 1000003
    E = Curve(p, -1, 1)
    base = E.points_with_x_upto(50, random.Random(1))
    assert base and all(E.on_curve(P) for P in base[:20])
    P, Q = base[0], base[1]
    R = E.add(P, Q)
    assert E.on_curve(R)
    assert E.add(P, E.neg(P)) is None
    assert E.add(R, E.neg(Q)) == P          # associativity/inverse
    c = E.c
    assert c.group_add > 0 and c.inv > 0


def test_sqrt_mod_roundtrip():
    p = 1000003
    c = Counter()
    for v in (2, 3, 5, 7, 11):
        y = sqrt_mod(v, p, c)
        if y is not None:
            assert y*y % p == v % p


def test_lattice_roundtrip():
    # 2x2 identity lattice scaled by B: rows [B,0],[0,B]
    M, mons = L.build({}, [((0, 0), {(0, 0): 1}), ((1, 1), {(0, 0): 2})], 3, 3)
    rows = L.reduce_fplll(M)
    assert len(rows) == 2
    for row in rows:
        h = L.to_poly(row, mons, 3, 3)
        assert isinstance(h, dict)


def test_is_multiple_of():
    f = {(0, 0): 7, (1, 0): 3, (2, 0): 1}
    assert L.is_multiple_of({(0, 0): 14, (1, 0): 6, (2, 0): 2}, f)
    assert not L.is_multiple_of({(1, 0): 1}, f)


def test_univariate_control_recovers():
    from experiments.control_univariate import run
    rec, ok = run(32, 0.40, 2)
    assert ok, rec.to_dict()
    rec2, ok2 = run(32, 0.55, 2)
    assert not ok2, "harness must fail above the 1/2 bound"


def test_record_pricing():
    r = Record("t", counts={"a": 10})
    d = r.to_dict()
    assert d["gae_0"] == 0 and d["gae_0.1"] == 1 and d["gae_1"] == 10
