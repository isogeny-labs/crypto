"""Frozen Sage producer for the primitive-j 23-isogeny discovery."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, ZZ, is_prime, set_random_seed


HERE = Path(__file__).resolve().parent
SOURCE = Path(__file__).read_bytes()
K = GF(3**5, "a", modulus=[1, 2, 0, 0, 0, 1])
L = GF(3**55, "z")
embed = K.embeddings(L)[0]
a = embed(K.gen())
E = EllipticCurve(L, [0, L(2), 0, 0, a])

def digits(x):
    return [int(c) for c in x.polynomial().list()]


def point_record(P):
    return None if P.is_zero() else [digits(P[0]), digits(P[1])]


t0, t1 = ZZ(2), ZZ(26)
for _ in range(2, 12):
    t0, t1 = t1, 26 * t1 - 243 * t0
order = ZZ(243) ** 11 + 1 - t1
r = ZZ(333566377062870325133)
cofactor = order // r
assert cofactor * r == order and is_prime(r) and r % 23 != 0

isogenies = E.isogenies_prime_degree(23)
primitive = [f for f in isogenies if f.codomain().j_invariant() ** 243 != f.codomain().j_invariant()]
assert len(isogenies) == 24 and len(primitive) == 22
f = primitive[0]
dual = f.dual()
assert dual.codomain().j_invariant() == E.j_invariant()

set_random_seed(20261116)
P = None
for _ in range(30):
    candidate = cofactor * E.random_point()
    if not candidate.is_zero():
        P = candidate
        break
assert P is not None and r * P == E(0)
Q = f(P)
assert not Q.is_zero() and r * Q == f.codomain()(0)
assert dual(Q) == 23 * P

out = {
    "schema": "ghs.odd5.primitive-s11/v1",
    "source_sha256": hashlib.sha256(SOURCE).hexdigest(),
    "field_modulus": [int(c) for c in L.modulus().list()],
    "a_embedding": digits(a),
    "source_ainvs": [digits(c) for c in E.ainvs()],
    "source_order": int(order),
    "source_order_factors": [[2, 1], [109, 1], [2399, 1], [int(r), 1]],
    "r": int(r),
    "cofactor": int(cofactor),
    "isogeny_degree": 23,
    "isogeny_count": len(isogenies),
    "primitive_j_count": len(primitive),
    "codomain_ainvs": [digits(c) for c in f.codomain().ainvs()],
    "codomain_j": digits(f.codomain().j_invariant()),
    "codomain_j_fixed_by_243": bool(f.codomain().j_invariant() ** 243 == f.codomain().j_invariant()),
    "kernel_polynomial": [digits(c) for c in f.kernel_polynomial().list()],
    "P": point_record(P),
    "Q": point_record(Q),
    "dual_Q": point_record(dual(Q)),
    "subgroup_checks": {"rP_zero": True, "Q_nonzero": True, "rQ_zero": True, "dual_Q_eq_23P": True},
}
(HERE / "primitive_isogeny_s11.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print(json.dumps({k: out[k] for k in ["schema", "source_order", "r", "isogeny_count", "primitive_j_count", "codomain_j_fixed_by_243", "subgroup_checks"]}, sort_keys=True))
