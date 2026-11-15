#!/usr/bin/env sage -python
"""Exact q2/q4 isogeny strata inside the published n=7 genus-eight selector."""

import json
from collections import defaultdict
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, factor


ROOT = Path(__file__).resolve().parent
F2 = GF(2)
R = PolynomialRing(F2, "x")
x = R.gen()


def mask(value):
    return sum(int(bit) << i for i, bit in enumerate(value.polynomial().list()))


def seventh_trace(t, q):
    previous, current = 2, t
    for _ in range(2, 8):
        previous, current = current, t * current - q * previous
    return current


def screen(s, modulus, expected):
    q = 2**s
    K = GF(2 ** (7 * s), "z", modulus=modulus)
    elements = list(K)
    k = [v for v in elements if v**q == v]
    a = next(v for v in k if sum((v ** (2**i) for i in range(s)), K(0)) == 1)
    kernels = [
        [v for v in elements if v and v ** (q**3) + v**q + v == 0],
        [v for v in elements if v and v ** (q**3) + v ** (q**2) + v == 0],
    ]
    S8 = {v + c for arm in kernels for v in arm for c in k if c}
    assert len(S8) == expected["genus8_coefficients"]
    k_field = GF(q, "w") if s > 1 else F2
    ka = next(v for v in k_field if sum((v ** (2**i) for i in range(s)), k_field(0)) == 1)
    base_traces = sorted(
        q + 1 - int(EllipticCurve(k_field, [1, ka, 0, 0, b]).cardinality())
        for b in k_field
        if b
    )
    base_seventh = sorted({seventh_trace(t, q) for t in base_traces})
    assert base_seventh == ({1: [-13], 2: [87, 251]}[s])
    strata = defaultdict(list)
    proper_degrees = [d for d in range(1, 7 * s) if (7 * s) % d == 0]
    mod8 = defaultdict(int)
    for b in sorted(S8, key=mask):
        order = int(EllipticCurve(K, [1, a, 0, 0, b]).cardinality())
        trace = q**7 + 1 - order
        full_field = all(b ** (2**d) != b for d in proper_degrees)
        strata[trace].append((mask(b), order, full_field))
        mod8[order % 8] += 1
    assert sum(map(len, strata.values())) == expected["genus8_coefficients"]
    assert sum(full for rows in strata.values() for _, _, full in rows) == expected["full_F2_field_coefficients"]
    assert {str(k): v for k, v in sorted(mod8.items())} == expected["genus8_order_mod8"]
    rows = []
    for trace, data in sorted(strata.items()):
        full = [row for row in data if row[2]]
        selected = min(full or data)
        order = selected[1]
        rows.append(
            {
                "trace": trace,
                "order": order,
                "coefficients": len(data),
                "full_F2_field_coefficients": len(full),
                "base_field_isogeny_class": trace in base_seventh,
                "representative_b_mask": selected[0],
                "representative_full_F2_field": selected[2],
                "largest_prime_subgroup": int(max(p for p, _ in factor(order))),
            }
        )
    return {
        "q": q,
        "field_modulus": str(modulus),
        "a_mask": mask(a),
        "base_field_traces": base_traces,
        "base_change_seventh_traces": base_seventh,
        "strata": rows,
    }


saved = json.loads((ROOT / "screen.json").read_text())
output = {
    "schema": "ghs-traceone-genus8-strata/v1",
    "q2": screen(1, x**7 + x + 1, saved["q2"]),
    "q4": screen(2, x**14 + x**5 + 1, saved["q4"]),
}
(ROOT / "strata.json").write_text(json.dumps(output, indent=2) + "\n")
print(json.dumps(output, indent=2))
