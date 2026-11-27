"""Screen P-192 division-polynomial characters at indices near p.

Run from the repository root:
  sage -python -u research/p192_ray_character_20261123/characteristic_index_screen.sage \
    > research/p192_ray_character_20261123/CHARACTERISTIC_INDEX_SCREEN.jsonl

The five formulas use a public point only. The scalar labels are controls.
"""

import hashlib
import json
from functools import lru_cache
from pathlib import Path

from sage.all import EllipticCurve, GF, ZZ, kronecker

P = 6277101735386680763835789423207666416083908700390324961279
R = 6277101735386680763835789423176059013767194773182842284081
E = EllipticCurve(GF(P), [-3, 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1])
G = E(
    0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
    0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811,
)
assert R * G == E(0)
INDICES = {"p_minus2": P - 2, "p": P, "p_plus2": P + 2,
           "p_minus1_over_2y": P - 1, "p_plus1_over_2y": P + 1}
MARKS = (1, 2, 3, 5, 7, 11, 13, 4, 10, 14, 15, 17)


def division_values(point):
    x, two_y = point[0], 2 * point[1]
    seed = {0: GF(P)(0), 1: GF(P)(1), 2: two_y,
            3: E.division_polynomial(3)(x),
            4: E.division_polynomial(4)(x) / two_y}

    @lru_cache(None)
    def psi(j):
        if j in seed:
            return seed[j]
        q = j // 2
        if j & 1:
            return psi(q + 2) * psi(q)**3 - psi(q - 1) * psi(q + 1)**3
        return psi(q) * (psi(q + 2) * psi(q - 1)**2
                         - psi(q - 2) * psi(q + 1)**2) / two_y

    return psi


def feature(point, name, index):
    value = division_values(point)(index)
    if name.endswith("over_2y"):
        value /= 2 * point[1]
    assert value != 0
    return int(kronecker(ZZ(value), P))


psi_g = division_values(G)
for small in (3, 5, 7):
    assert psi_g(small) == E.division_polynomial(small)(G[0])

print(json.dumps({"metadata": {
    "source_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    "p": str(P), "r": str(R), "indices": {k: str(v) for k, v in INDICES.items()},
    "marks": list(MARKS), "small_direct_polynomial_checks": [3, 5, 7],
}}, sort_keys=True), flush=True)

base = {name: feature(G, name, index) for name, index in INDICES.items()}
for name, index in INDICES.items():
    assert feature(-G, name, index) == base[name]
for scalar in MARKS:
    point = scalar * G
    expected = int(kronecker(scalar, R))
    row = {"scalar": scalar, "scalar_sign": expected, "features": {}}
    for name, index in INDICES.items():
        observed = base[name] * feature(point, name, index)
        row["features"][name] = {"sign": observed, "agree": observed == expected}
    print(json.dumps(row, sort_keys=True), flush=True)
