"""Independent SymPy and elementary modular checks of the Sage certificate."""

import hashlib
import json
from pathlib import Path

import sympy as sp

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "ghs_cubic_hess_closure_q16_20261027/certificate.json"
CERT = HERE / "certificate.json"
FOCUS = {4006: 5, 4054: 11, 4078: 5, 4126: 5}


def main():
    source = json.loads(SOURCE.read_text())
    cert = json.loads(CERT.read_text())
    assert cert["source_certificate_sha256"] == hashlib.sha256(SOURCE.read_bytes()).hexdigest()
    assert cert["census_script_sha256"] == hashlib.sha256((HERE / "census.py").read_bytes()).hexdigest()
    source_by_order = {r["order"]: r for r in source["orders"]}
    x = sp.Symbol("T")
    assert len(cert["rows"]) == len(source_by_order) == 128
    bucket = {name: [0, 0] for name in cert["summary"]}
    split_orders = []
    for row in cert["rows"]:
        n = row["order"]
        original = source_by_order[n]
        assert row["trace"] == 4097 - n
        assert row["source_models"] == original["source_models"]
        assert row["largest_prime"] == original["largest_prime"]
        assert row["outside_hess_closure"] == (not original["in_hess_isogeny_closure"])
        assert row["catalog_jacobian"] == bool(original["catalog_has_jacobian"])
        f = sp.Poly(x**6 - row["trace"] * x**3 + 4096, x, domain=sp.ZZ)
        factors = sp.factor_list(f)[1]
        degrees = sorted([g.degree() for g, exponent in factors for _ in range(exponent)])
        assert degrees == sorted(row["factor_degrees"])
        simple = degrees == [6]
        assert simple == row["irreducible"]
        if simple:
            prime = row["irreducible_mod_prime"]
            assert prime < 100 and sp.isprime(prime)
            assert sp.Poly(f.as_expr(), x, modulus=prime).is_irreducible
        else:
            assert "irreducible_mod_prime" not in row
        if not simple:
            split_orders.append(n)
        if n in FOCUS:
            assert row["irreducible_mod_prime"] == FOCUS[n]
        membership = {
            "simple": simple,
            "split": not simple,
            "simple_outside": simple and row["outside_hess_closure"],
            "split_outside": not simple and row["outside_hess_closure"],
            "simple_outside_catalog": simple and row["outside_hess_closure"] and row["catalog_jacobian"],
            "simple_outside_no_catalog": simple and row["outside_hess_closure"] and not row["catalog_jacobian"],
            "prime_over_625": row["largest_prime"] > 625,
            "prime_over_625_outside": row["largest_prime"] > 625 and row["outside_hess_closure"],
        }
        for name, included in membership.items():
            if included:
                bucket[name][0] += 1
                bucket[name][1] += row["source_models"]
    assert split_orders == [3980, 3982, 4050, 4090, 4104, 4144, 4212, 4214]
    for name, (orders, models) in bucket.items():
        assert cert["summary"][name] == {"orders": orders, "models": models}
    assert sum(r["source_models"] for r in cert["rows"]) == 8190
    print("PASS: 128 independent integer factorizations, 120 modular witnesses, census totals")


if __name__ == "__main__":
    main()
