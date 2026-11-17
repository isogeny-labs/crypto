#!/usr/bin/env sage -python
"""Necessary Frobenius-module screen for one odd-degree (ell,ell,ell) isogeny.

The source rows and class labels are frozen in the earlier cubic census.
Factorization is performed in Sage; audit.py independently enumerates cubics.
"""
import hashlib
import json
from pathlib import Path

from sage.all import GF, PolynomialRing

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "ghs_q16_simple_restriction_bound_20261027" / "certificate.json"
CLOSURE = HERE.parent / "ghs_cubic_hess_closure_q16_20261027" / "certificate.json"
PRIMES = (3, 5, 7, 11, 13)


def run():
    raw = SOURCE.read_bytes()
    census = json.loads(raw)
    closure_raw = CLOSURE.read_bytes()
    closure = {row["order"]: row for row in json.loads(closure_raw)["orders"]}
    rows = [
        row for row in census["rows"]
        if row["irreducible"] and row["outside_hess_closure"]
        and row["catalog_jacobian"]
    ]
    assert len(rows) == 44 and sum(row["source_models"] for row in rows) == 2643
    records = []
    for row in rows:
        record = {"order": row["order"], "trace": row["trace"],
                  "catalog_label": closure[row["order"]]["catalog_label"],
                  "source_models": row["source_models"], "primes": {}}
        for prime in PRIMES:
            ring = PolynomialRing(GF(prime), "T")
            T = ring.gen()
            poly = T**6 - row["trace"] * T**3 + 4096
            factors = [(int(f.degree()), int(e)) for f, e in poly.factor()]
            degrees = sorted(deg for deg, exponent in factors for _ in range(exponent))
            subset = {0}
            for degree in degrees:
                subset |= {s + degree for s in tuple(subset)}
            record["primes"][str(prime)] = {
                "factor_degrees": degrees,
                "monic_cubic_divisor": 3 in subset,
            }
        records.append(record)
    summary = {}
    for prime in PRIMES:
        excluded = [r for r in records if not r["primes"][str(prime)]["monic_cubic_divisor"]]
        summary[str(prime)] = {
            "excluded_orders": len(excluded),
            "excluded_models": sum(r["source_models"] for r in excluded),
            "orders": [r["order"] for r in excluded],
        }
    all_excluded = [r for r in records if all(
        not r["primes"][str(p)]["monic_cubic_divisor"] for p in PRIMES)]
    result = {
        "schema": "ghs-q16-odd-isogeny-necessary-screen-v1",
        "source_certificate_sha256": hashlib.sha256(raw).hexdigest(),
        "closure_certificate_sha256": hashlib.sha256(closure_raw).hexdigest(),
        "field": "k=GF(16), K=GF(4096), [K:k]=3",
        "scope": "44 simple outside-Hess classes with catalog genus-three Jacobian carriers",
        "primes": list(PRIMES),
        "necessary_condition": "T^6-(4097-N)T^3+4096 has a monic cubic divisor modulo ell",
        "summary": summary,
        "all_screened_primes_excluded": {
            "orders": [r["order"] for r in all_excluded],
            "models": sum(r["source_models"] for r in all_excluded),
        },
        "rows": records,
    }
    (HERE / "certificate.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"summary": summary, "all_screened_primes_excluded":
                      result["all_screened_primes_excluded"]}, indent=2))


if __name__ == "__main__":
    run()
