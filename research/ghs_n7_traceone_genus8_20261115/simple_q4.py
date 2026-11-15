#!/usr/bin/env sage -python
"""Factor every q4 Weil-restriction polynomial from the frozen S8 strata."""

import json
from pathlib import Path

from sage.all import PolynomialRing, QQ


ROOT = Path(__file__).resolve().parent
source = json.loads((ROOT / "strata.json").read_text())
rows = source["q4"]["strata"]
R = PolynomialRing(QQ, "X")
X = R.gen()
result = []
for row in rows:
    t = row["trace"]
    polynomial = X**14 - t * X**7 + 4**7
    factors = [(str(f), int(e)) for f, e in polynomial.factor()]
    irreducible = len(factors) == 1 and factors[0][1] == 1
    result.append(
        {
            "trace": t,
            "coefficients": row["coefficients"],
            "full_F2_field_coefficients": row["full_F2_field_coefficients"],
            "polynomial": str(polynomial),
            "irreducible": irreducible,
            "factors": factors,
        }
    )
assert sum(r["coefficients"] for r in result) == 378
assert sum(r["full_F2_field_coefficients"] for r in result) == 364
assert all(r["irreducible"] for r in result if r["full_F2_field_coefficients"])
assert next(r for r in result if r["trace"] == -249)["irreducible"]
output = {"schema": "ghs-traceone-genus8-simplicity/v1", "q": 4, "strata": result}
(ROOT / "simple_q4.json").write_text(json.dumps(output, indent=2) + "\n")
print(
    json.dumps(
        {
            "strata": len(result),
            "irreducible_strata": sum(r["irreducible"] for r in result),
            "irreducible_coefficients": sum(r["coefficients"] for r in result if r["irreducible"]),
            "full_field_irreducible_coefficients": sum(
                r["full_F2_field_coefficients"] for r in result if r["irreducible"]
            ),
            "candidate_trace_minus_249": next(r for r in result if r["trace"] == -249),
        },
        indent=2,
    )
)
