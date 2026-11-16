#!/usr/bin/env sage -python
"""Factor every cubic Weil-restriction Frobenius polynomial in the frozen census."""

import hashlib
import json
from pathlib import Path

from sage.all import GF, PolynomialRing, ZZ, prime_range

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "ghs_cubic_hess_closure_q16_20261027/certificate.json"
FOCUS = (4006, 4054, 4078, 4126)


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    source = json.loads(SOURCE.read_text())
    ring = PolynomialRing(ZZ, "T")
    tvar = ring.gen()
    rows = []
    for item in source["orders"]:
        order = item["order"]
        trace = 4097 - order
        polynomial = tvar**6 - trace * tvar**3 + 4096
        factors = polynomial.factor()
        row = {
            "order": order,
            "trace": trace,
            "source_models": item["source_models"],
            "largest_prime": item["largest_prime"],
            "outside_hess_closure": not item["in_hess_isogeny_closure"],
            "catalog_jacobian": bool(item["catalog_has_jacobian"]),
            "irreducible": bool(polynomial.is_irreducible()),
            "factor_degrees": [int(f.degree()) for f, exponent in factors for _ in range(int(exponent))],
        }
        if row["irreducible"]:
            for prime in prime_range(2, 100):
                rp = PolynomialRing(GF(prime), "T")
                xp = rp.gen()
                if (xp**6 - trace * xp**3 + 4096).is_irreducible():
                    row["irreducible_mod_prime"] = int(prime)
                    break
            assert "irreducible_mod_prime" in row
            if order in FOCUS:
                assert row["irreducible_mod_prime"] == {4006: 5, 4054: 11, 4078: 5, 4126: 5}[order]
        rows.append(row)
    assert len(rows) == 128
    assert sum(r["source_models"] for r in rows) == 8190
    summary = {}
    for label, predicate in {
        "simple": lambda r: r["irreducible"],
        "split": lambda r: not r["irreducible"],
        "simple_outside": lambda r: r["irreducible"] and r["outside_hess_closure"],
        "split_outside": lambda r: not r["irreducible"] and r["outside_hess_closure"],
        "simple_outside_catalog": lambda r: r["irreducible"] and r["outside_hess_closure"] and r["catalog_jacobian"],
        "simple_outside_no_catalog": lambda r: r["irreducible"] and r["outside_hess_closure"] and not r["catalog_jacobian"],
        "prime_over_625": lambda r: r["largest_prime"] > 625,
        "prime_over_625_outside": lambda r: r["largest_prime"] > 625 and r["outside_hess_closure"],
    }.items():
        selected = [r for r in rows if predicate(r)]
        summary[label] = {"orders": len(selected), "models": sum(r["source_models"] for r in selected)}
    output = {
        "schema": "ghs-q16-simple-restriction-bound-v1",
        "source_certificate_sha256": sha256(SOURCE),
        "census_script_sha256": sha256(Path(__file__)),
        "field": "k=GF(16), K=GF(4096), [K:k]=3",
        "frobenius_polynomial": "T^6-(4097-#E(K))*T^3+4096",
        "summary": summary,
        "rows": rows,
    }
    (HERE / "certificate.json").write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
