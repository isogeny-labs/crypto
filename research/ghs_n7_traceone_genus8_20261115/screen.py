#!/usr/bin/env sage -python
"""Exact small-field census for the trace-one n=7 genus-eight slice."""

import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing


ROOT = Path(__file__).resolve().parent
F2 = GF(2)
R = PolynomialRing(F2, "x")
x = R.gen()


def mask(value):
    return sum(int(bit) << i for i, bit in enumerate(value.polynomial().list()))


def screen(s, modulus):
    q = 2**s
    K = GF(2 ** (7 * s), "z", modulus=modulus)
    elements = list(K)
    k_star = [v for v in elements if v and v**q == v]
    assert len(k_star) == q - 1
    a = next(v for v in elements if v**q == v and sum((v ** (2**i) for i in range(s)), K(0)) == 1)
    kernels = [
        [v for v in elements if v and v ** (q**3) + v**q + v == 0],
        [v for v in elements if v and v ** (q**3) + v ** (q**2) + v == 0],
    ]
    assert [len(v) for v in kernels] == [q**3 - 1, q**3 - 1]
    S8 = {(v + c) for arm in kernels for v in arm for c in k_star}
    W1 = {(u * (v + c)) ** 2 for arm in kernels for u in arm for v in arm for c in k_star}
    trace = lambda v: sum((v ** (q**i) for i in range(7)), K(0))
    assert all(trace(v) for v in S8)
    assert all(trace(v) == 0 for v in W1)
    assert len(S8 & W1) == 0
    assert len(S8) == 2 * (q**3 - 1) * (q - 1)
    proper_degrees = [d for d in range(1, 7 * s) if (7 * s) % d == 0]
    proper = {b for b in S8 if any(b ** (2**d) == b for d in proper_degrees)}
    orders = Counter(int(EllipticCurve(K, [1, a, 0, 0, b]).cardinality()) % 8 for b in S8)
    result = {
        "q": q,
        "field_modulus": str(modulus),
        "a_mask": mask(a),
        "cubic_kernel_sizes": [len(v) for v in kernels],
        "genus8_coefficients": len(S8),
        "w1_coefficients": len(W1),
        "intersection": len(S8 & W1),
        "proper_F2_subfield_coefficients": len(proper),
        "full_F2_field_coefficients": len(S8 - proper),
        "genus8_order_mod8": {str(k): v for k, v in sorted(orders.items())},
    }
    if s == 1:
        z = K.gen()
        b = z + 1
        E = EllipticCurve(K, [1, 1, 0, 0, b])
        assert mask(b) == 3 and b in S8 and E.cardinality() == 142
        result["example"] = {"a": 1, "b_mask": 3, "order": 142, "prime_subgroup": 71}
    return result


output = {
    "schema": "ghs-traceone-genus8-screen/v1",
    "q2": screen(1, x**7 + x + 1),
    "q4": screen(2, x**14 + x**5 + 1),
}
assert output["q2"]["genus8_order_mod8"] == {"6": 14}
assert output["q4"]["genus8_order_mod8"] == {"2": 126, "6": 252}
assert output["q2"]["full_F2_field_coefficients"] == 14
assert output["q4"]["full_F2_field_coefficients"] == 364
(ROOT / "screen.json").write_text(json.dumps(output, indent=2) + "\n")
print(json.dumps(output, indent=2))
