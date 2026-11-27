"""Screen fixed division-polynomial signs on P-192 known markings.

Run: sage -python research/p192_ray_character_20261123/division_feature_screen.py
The selected trace-minus-two candidate is checked on independent seeded samples.
"""

from functools import lru_cache
import json
from pathlib import Path
import random

from sage.all import EllipticCurve, GF, ZZ, kronecker

P = 6277101735386680763835789423207666416083908700390324961279
R = 6277101735386680763835789423176059013767194773182842284081
TRACE = P + 1 - R
CURVE = EllipticCurve(GF(P), [-3, 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1])
G = CURVE(
    0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
    0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811,
)
assert R * G == CURVE(0)


def division_values(point):
    x, two_y = point[0], 2 * point[1]
    seed = {
        0: GF(P)(0),
        1: GF(P)(1),
        2: two_y,
        3: CURVE.division_polynomial(3)(x),
        4: CURVE.division_polynomial(4)(x) / two_y,
    }

    @lru_cache(None)
    def psi(j):
        if j in seed:
            return seed[j]
        q = j // 2
        if j & 1:
            return psi(q + 2) * psi(q) ** 3 - psi(q - 1) * psi(q + 1) ** 3
        return psi(q) * (
            psi(q + 2) * psi(q - 1) ** 2
            - psi(q - 2) * psi(q + 1) ** 2
        ) / two_y

    return psi


psi_g = division_values(G)
for small in (3, 5, 7, 11):
    assert psi_g(small) == CURVE.division_polynomial(small)(G[0])


def sign_at(d, index):
    value = division_values(d * G)(index)
    assert value != 0
    return int(kronecker(ZZ(value), P))


def scalar_sign(d):
    return int(kronecker(d, R))


indices = {
    "half_plus": (R + 1) // 2,
    "half_minus": (R - 3) // 2,
    "quarter_plus": (R - 1) // 4 + 1,
    "trace": TRACE,
    "trace_minus2": TRACE - 2,
    "r_minus2": R - 2,
    "r_minus4": R - 4,
    "r_minus6": R - 6,
}
for exponent in (2, 3, 4):
    indices[f"inv2pow{exponent}"] = ((2**exponent - 1) * R + 1) // (2**exponent)
indices = {name: k if k & 1 else R - k for name, k in indices.items()}
assert all(0 < k < R and k & 1 for k in indices.values())

output = Path(__file__).with_name("DIVISION_FEATURE_SAMPLES.jsonl")
markings = (1, 2, 3, 5, 7, 11)
with output.open("w") as raw:
    for name, index in indices.items():
        signs = [sign_at(d, index) for d in markings]
        normalized = [sign * signs[0] for sign in signs]
        mismatches = [d for d, sign in zip(markings, normalized)
                      if sign != scalar_sign(d)]
        raw.write(json.dumps({
            "kind": "screen", "name": name, "index": str(index),
            "markings": markings, "normalized": normalized,
            "mismatch_d": mismatches,
        }, sort_keys=True) + "\n")
        print(name, "mismatch_d", mismatches)
    assert not [d for d in markings if sign_at(d, TRACE - 2) * sign_at(1, TRACE - 2)
                != scalar_sign(d)]
    witness = 10
    witness_observed = sign_at(witness, TRACE - 2) * sign_at(1, TRACE - 2)
    assert witness_observed == -1 and scalar_sign(witness) == 1
    raw.write(json.dumps({
        "kind": "holdout_witness", "d": witness, "observed": witness_observed,
        "expected": scalar_sign(witness), "index": str(TRACE - 2),
    }, sort_keys=True) + "\n")
    base = sign_at(1, TRACE - 2)
    for seed, count, expected_correct in ((20261125, 256, 117), (20261126, 4096, 2014)):
        generator = random.Random(seed)
        seen = set()
        correct = 0
        for _ in range(count):
            d = generator.randrange(1, R)
            while d in seen:
                d = generator.randrange(1, R)
            seen.add(d)
            observed = sign_at(d, TRACE - 2) * base
            expected = scalar_sign(d)
            correct += observed == expected
            raw.write(json.dumps({
                "kind": "sample", "seed": seed, "d_hex": f"{d:x}",
                "observed": observed, "expected": expected,
            }, sort_keys=True) + "\n")
        assert correct == expected_correct
        raw.write(json.dumps({
            "kind": "summary", "seed": seed, "count": count,
            "correct": correct, "correlation_numerator": 2 * correct - count,
        }, sort_keys=True) + "\n")
        print("seed", seed, "correct", correct, "count", count)
