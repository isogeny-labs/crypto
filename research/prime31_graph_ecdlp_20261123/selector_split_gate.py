"""Exact coprime-selector factorization prerequisite for a split RR5 circuit.

This checks public selector polynomials and a grader-side positive quartic.
It does not extract an unknown query relation.
"""

import hashlib
import json
from pathlib import Path

from flint import nmod_poly

from verify_mitm_graph import A, B, P, ROOT

SOURCE = ROOT / "evidence/rr5-b677-prerequisite-01/structured-positive.stdout"
OUT = ROOT / "evidence/rr5-b677-prerequisite-01/selector-split-gate.json"


def main():
    source = SOURCE.read_bytes()
    fixture = json.loads(next(line.split(" ", 1)[1]
                              for line in source.decode().splitlines()
                              if line.startswith("B677_INPUT ")))
    x = nmod_poly([0, 1], P)
    dickson = x
    for _ in range(10):
        dickson = dickson * dickson - 2
    dickson -= fixture["dickson_terminal"]
    t = (x*x*x + A*x + B) ** 7
    y7 = nmod_poly([1], P)
    for root in fixture["public_coset_squares"]:
        y7 *= t - root
    assert (dickson.degree(), y7.degree(), dickson.gcd(y7).degree()) == (1024, 168, 0)

    expected = fixture["control_expected_row"]["terms"]
    u = nmod_poly([1], P)
    for index, _ in expected:
        u *= x - fixture["public_base"][index][0]
    u_d, u_y = u.gcd(dickson), u.gcd(y7)
    assert (u.degree(), u_d.degree(), u_y.degree()) == (4, 2, 2)
    assert u_d * u_y == u
    assert (dickson % u_d).degree() == -1
    assert (y7 % u_y).degree() == -1
    assert ((dickson * y7) % u).degree() == -1
    output = {
        "source_sha256": hashlib.sha256(source).hexdigest(),
        "curve": fixture["curve"],
        "selector_degrees": {"dickson": dickson.degree(), "y7": y7.degree()},
        "selector_gcd_degree": dickson.gcd(y7).degree(),
        "positive_quartic_degree": u.degree(),
        "positive_split_degrees": [u_d.degree(), u_y.degree()],
        "positive_factor_product_exact": True,
        "claim_boundary": "structural prerequisite only; no unknown-query extraction or cost result",
    }
    OUT.write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps(output, indent=2))


if __name__ == "__main__":
    main()
