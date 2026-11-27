"""Screen rational two-step isogeny endpoints for the P-192 m=8 sign.

Run from the repository root:
  sage -python -u research/p192_ray_character_20261123/two_hop_m8_screen.sage \
    > research/p192_ray_character_20261123/TWO_HOP_M8_SCREEN.jsonl

The public scalar labels are controls. All enumerated rational edges are
horizontal: the source Frobenius discriminant is fundamental, and each
F_p-isogenous curve has the same Frobenius trace and maximal order.
"""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, ZZ, kronecker

P = 6277101735386680763835789423207666416083908700390324961279
R = 6277101735386680763835789423176059013767194773182842284081
N = (R - 1) // 2
H = pow(3, N // 8, R)
REPS = sorted({min(pow(H, j, R), R - pow(H, j, R)) for j in range(8)})
assert len(REPS) == 8 and pow(H, 8, R) == R - 1
E = EllipticCurve(GF(P), [-3, 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1])
G = E(
    0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
    0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811,
)
assert R * G == E(0)
TRACE = P + 1 - R
DISCRIMINANT = ZZ(TRACE * TRACE - 4 * P)
assert DISCRIMINANT.is_fundamental_discriminant()
MARKS = (2, 3, 5, 7, 11, 13, 4, 10, 14, 15, 17, 19, 23, 29, 31)


def sign(value):
    residue = pow(int(value), (P - 1) // 2, P)
    assert residue in (1, P - 1)
    return 1 if residue == 1 else -1


def product_sign(points):
    value = GF(P)(1)
    for point in points:
        assert point != point.curve()(0)
        value *= point[1]
    return sign(value)


def sorted_edges(curve, degree):
    return sorted(curve.isogenies_prime_degree(degree),
                  key=lambda phi: int(phi.codomain().j_invariant()))


print(json.dumps({"metadata": {
    "source_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    "p": str(P), "r": str(R), "m": 8, "reps": [str(x) for x in REPS],
    "frobenius_discriminant": str(DISCRIMINANT),
    "degrees": [5, 11, 13], "marks": list(MARKS),
}}, sort_keys=True), flush=True)

for first_degree in (5, 11, 13):
    for first_index, first in enumerate(sorted_edges(E, first_degree)):
        intermediate = first(G)
        assert intermediate != intermediate.curve()(0)
        assert R * intermediate == intermediate.curve()(0)
        for second_degree in (5, 11, 13):
            for second_index, second in enumerate(sorted_edges(first.codomain(), second_degree)):
                target = second(intermediate)
                assert target != target.curve()(0)
                assert R * target == target.curve()(0)
                # Check the composite on a point supplied independently of target.
                assert second(first(7 * G)) == 7 * target
                points = [int(rep) * target for rep in REPS]
                base = product_sign(points)
                rows = []
                for scalar in MARKS:
                    observed = base * product_sign([scalar * point for point in points])
                    expected = int(kronecker(scalar, R))
                    rows.append({"scalar": scalar, "feature_sign": observed,
                                 "scalar_sign": expected, "agree": observed == expected})
                print(json.dumps({"path": [first_degree, first_index,
                                            second_degree, second_index],
                                  "intermediate_j": str(first.codomain().j_invariant()),
                                  "endpoint_j": str(target.curve().j_invariant()),
                                  "base_sign": base, "marks": rows},
                                 sort_keys=True), flush=True)
