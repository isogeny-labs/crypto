"""Reconstruct the saved 23-isogeny witness from its kernel polynomials."""

import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, ZZ


path = Path(__file__).with_name("primitive_isogeny_s11.json")
d = json.loads(path.read_text())
F3 = GF(3)
R = PolynomialRing(F3, "z")
L = GF(3**55, "z", modulus=R(d["field_modulus"]))
z = L.gen()


def elem(cs):
    return sum((L(c) * z**i for i, c in enumerate(cs)), L(0))


def point(E, xy):
    return E(elem(xy[0]), elem(xy[1]))


E = EllipticCurve(L, [elem(x) for x in d["source_ainvs"]])
C = EllipticCurve(L, [elem(x) for x in d["codomain_ainvs"]])
j = C.j_invariant()
assert j == elem(d["codomain_j"])
assert j.minpoly().degree() == 55
assert j**243 != j and j**(3**11) != j
r = ZZ(d["r"])
P = point(E, d["P"])
Q = point(C, d["Q"])
assert r * P == E(0) and not P.is_zero()
assert r * Q == C(0) and not Q.is_zero()

Rx = PolynomialRing(L, "x")
forward = E.isogeny(Rx([elem(x) for x in d["kernel_polynomial"]]))
assert forward.codomain().ainvs() == C.ainvs()
assert forward(P) == Q
reverse = C.isogeny(Rx([elem(x) for x in d["reverse_kernel_polynomial"]]))
assert reverse.codomain().ainvs() == tuple(elem(x) for x in d["reverse_codomain_ainvs"])
iso = reverse.codomain().isomorphism_to(E)
assert d["reverse_isomorphism_sign"] * iso(reverse(Q)) == 23 * P
print(json.dumps({"schema": "ghs.odd5.primitive-s11-replay/v1", "field_degree_of_j": 55, "r": int(r), "forward_point_match": True, "reverse_composition_match": True}, sort_keys=True))
