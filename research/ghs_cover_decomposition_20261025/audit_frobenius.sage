"""Independent point-count audit of the ten exact quadratic GHS covers.

For each genus-two curve C/k in the frozen attack receipts, count C(k) and
C(k^2) directly in the exact polynomial-basis field K=k^2. These determine
the degree-four Frobenius polynomial of Jac(C). Integer factorization tests
whether that isogeny class has an elliptic factor over k. No map, group order,
or Frobenius coefficient is imported from the attack executable.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, ZZ


HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "ghs_generalized_20261024" / "results"
SPECS = (
    (6, 0, 32, ("direct", "two_isogeny")),
    (8, 0, 32, ("direct", "two_isogeny")),
    (10, 0, 32, ("direct", "two_isogeny", "three_isogeny")),
    (12, 1, 512, ("direct", "two_isogeny", "five_isogeny")),
)


def field_from_words(n, modulus_word):
    f2 = GF(2)
    poly_ring = PolynomialRing(f2, "t")
    t = poly_ring.gen()
    modulus = sum(t**i for i in range(n + 1) if (modulus_word >> i) & 1)
    field = GF(2**n, "z", modulus=modulus)
    generator = field.gen()

    def elt(word):
        return sum(generator**i for i in range(n) if (word >> i) & 1)

    elements = [elt(word) for word in range(2**n)]
    assert len(set(elements)) == 2**n
    return field, elements, elt


def evaluate(coefficients, x, zero):
    result = zero
    for coefficient in reversed(coefficients):
        result = result * x + coefficient
    return result


def trace_to_f2(value, degree, zero, one):
    total = zero
    term = value
    for _ in range(degree):
        total += term
        term *= term
    assert total == zero or total == one
    return total


def count_curve(elements, h, f, degree, field):
    # The checked genus-two model has odd degree-five f and one rational
    # point at infinity over every extension considered here.
    count = 1
    zero, one = field.zero(), field.one()
    for x in elements:
        hx = evaluate(h, x, zero)
        if hx == zero:
            # y^2=f(x) has exactly one solution in a finite binary field.
            count += 1
            continue
        fx = evaluate(f, x, zero)
        trace = trace_to_f2(fx / (hx * hx), degree, zero, one)
        if trace == zero:
            count += 2
    return count


def count_curve_bruteforce(elements, h, f, field):
    """A separate equation-enumeration check at degrees at most eight."""
    count = 1
    zero = field.zero()
    for x in elements:
        hx = evaluate(h, x, zero)
        fx = evaluate(f, x, zero)
        count += sum(y*y + hx*y == fx for y in elements)
    return count


def main():
    ring = PolynomialRing(ZZ, "X")
    X = ring.gen()
    cases = []
    by_degree = {}
    for n, a, b, edges in SPECS:
        direct_name = f"exact-n{n}-a{a}-b{b}-direct.json"
        direct = json.loads((SOURCE / direct_name).read_text())
        modulus_word = int(direct["source"]["modulus"], 0)
        field, all_elements, elt = field_from_words(n, modulus_word)
        s = n // 2
        q = 2**s
        base_elements = [x for x in all_elements if x**q == x]
        assert len(base_elements) == q
        source_curve = EllipticCurve(field, [field.one(), elt(a), field.zero(), field.zero(), elt(b)])
        source_order = int(source_curve.cardinality())
        assert source_order == direct["source"]["group_order"]
        by_degree[n] = (field, all_elements, base_elements, elt, source_order)

        for edge in edges:
            name = f"exact-n{n}-a{a}-b{b}-{edge}.json"
            path = SOURCE / name
            receipt = json.loads(path.read_text())
            assert receipt["ghs_status"] == "verified"
            assert receipt["setup_phases"]["full_subgroup_checked_scalars"] == receipt["source"]["subgroup_order"]
            data = receipt["ghs"]["map"]
            # The receipt's cover coefficients are local k-coordinate words,
            # not K polynomial-basis words. Its explicit k -> K embedding
            # basis is part of the exact identity and must be applied here.
            basis = [elt(int(word, 0)) for word in data["subfield_embedding_basis"]]
            assert len(basis) == s and all(value**q == value for value in basis)

            def cover_coefficient(word):
                bits = int(word, 0)
                assert 0 <= bits < 2**s
                return sum(basis[i] for i in range(s) if (bits >> i) & 1)

            h = [cover_coefficient(word) for word in data["h"]]
            f = [cover_coefficient(word) for word in data["f"]]
            assert len(f) == 6 and f[-1] != 0 and len(h) <= 3
            assert all(c**q == c for c in h + f)

            n1 = count_curve(base_elements, h, f, s, field)
            n2 = count_curve(all_elements, h, f, n, field)
            brute_crosscheck = None
            if n <= 8:
                brute_crosscheck = (
                    n1 == count_curve_bruteforce(base_elements, h, f, field)
                    and n2 == count_curve_bruteforce(all_elements, h, f, field)
                )
                assert brute_crosscheck
            t1 = q + 1 - n1
            t2 = q*q + 1 - n2
            numerator = t1*t1 - t2
            assert numerator % 2 == 0
            a2 = numerator // 2
            frobenius = X**4 - t1*X**3 + a2*X**2 - q*t1*X + q*q
            factors = [
                {"coefficients_low_to_high": [int(c) for c in factor], "multiplicity": int(multiplicity)}
                for factor, multiplicity in frobenius.factor()
            ]
            jacobian_order = int(frobenius(1))
            source_trace = q*q + 1 - source_order
            restriction_polynomial = X**4 - source_trace*X**2 + q*q
            # Squaring the q-Frobenius roots gives the q^2-Frobenius
            # polynomial after extending constants from k to K.
            elliptic_polynomial = X**2 - source_trace*X + q*q
            extension_polynomial = elliptic_polynomial**2
            jacobian_order_over_extension = int(frobenius(1) * frobenius(-1))
            # For an even quartic X^4+A X^2+q^2 with |A|<2q, a
            # rational quadratic factor would require either a square
            # (2q-A) or a square (-2q-A). The u=0 option has negative
            # discriminant A^2-4q^2. Record the integer obstruction.
            plus_candidate = 2*q - a2
            minus_candidate = -2*q - a2
            zero_linear_discriminant = a2*a2 - 4*q*q
            irreducibility_certificate = {
                "plus_candidate": plus_candidate,
                "plus_candidate_is_square": bool(plus_candidate >= 0 and ZZ(plus_candidate).is_square()),
                "minus_candidate": minus_candidate,
                "minus_candidate_is_square": bool(minus_candidate >= 0 and ZZ(minus_candidate).is_square()),
                "zero_linear_discriminant": zero_linear_discriminant,
            }
            if frobenius[3] == 0 and frobenius[1] == 0 and abs(a2) < 2*q:
                assert not irreducibility_certificate["plus_candidate_is_square"]
                assert not irreducibility_certificate["minus_candidate_is_square"]
                assert zero_linear_discriminant < 0
                assert frobenius.is_irreducible()
            prime_order = int(receipt["source"]["subgroup_order"])
            relations = receipt["ghs"]["relation_counts"]
            assert relations["attempts"] == prime_order - 1
            row = {
                "receipt": name,
                "receipt_sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                "absolute_degree": n,
                "base_degree": s,
                "base_field_size": q,
                "source_modulus": f"0x{modulus_word:x}",
                "source_a": a,
                "source_b": b,
                "source_group_order_independent_sage": source_order,
                "source_subgroup_order": prime_order,
                "cover_h": data["h"],
                "cover_f": data["f"],
                "cover_n1": n1,
                "cover_n2": n2,
                "brute_force_equation_crosscheck": brute_crosscheck,
                "frobenius_coefficients_low_to_high": [int(c) for c in frobenius],
                "frobenius_factors_over_Z": factors,
                "irreducible_over_Q": bool(frobenius.is_irreducible()),
                "irreducibility_certificate": irreducibility_certificate,
                "jacobian_order_from_point_counts": jacobian_order,
                "source_order_equals_jacobian_order": jacobian_order == source_order,
                "source_subgroup_divides_jacobian_order": jacobian_order % prime_order == 0,
                "divisor_decomposition_receipt_metadata": {
                    "factor_base_size": receipt["ghs"]["factor_base_size"],
                    "smooth": relations["smooth"],
                    "attempts": relations["attempts"],
                    "matrix_rank": receipt["ghs"]["matrix_rank"],
                    "exhausts_nonzero_selected_subgroup": True,
                },
                "restriction_coefficients_low_to_high": [int(c) for c in restriction_polynomial],
                "restriction_polynomial_matches": frobenius == restriction_polynomial,
                "extension_frobenius_coefficients_low_to_high": [int(c) for c in extension_polynomial],
                "jacobian_order_over_extension_from_frobenius": jacobian_order_over_extension,
                "source_order_squared_matches_extension_jacobian_order": jacobian_order_over_extension == source_order**2,
            }
            cases.append(row)
            print(
                f"n={n} {edge}: #C(k)={n1} #C(k^2)={n2} "
                f"P={list(frobenius)} factors={factors} #J(k)={jacobian_order}"
            )

    assert len(cases) == 10
    report = {
        "schema": "ghs-cover-decomposition-audit/v1",
        "method": "Count points of each exact degree-five genus-two model over k and k^2 via Artin-Schreier trace; derive and factor its degree-four Frobenius polynomial over Z.",
        "criterion": "Irreducible degree-four Frobenius polynomial excludes a proper elliptic isogeny factor over the base field k; factorization alone would not certify that the transferred source subgroup projects nontrivially.",
        "source_note": "Cover coefficients and field basis are exact receipt inputs; point counts, Frobenius coefficients, integer factorization, source curve order and comparisons are recomputed independently in Sage. Divisor-decomposition counts and ranks are copied from each bound attack receipt, not recomputed by this script.",
        "cases": cases,
        "summary": {
            "cases": len(cases),
            "irreducible_over_Q": sum(row["irreducible_over_Q"] for row in cases),
            "restriction_polynomial_matches": sum(row["restriction_polynomial_matches"] for row in cases),
            "source_order_matches": sum(row["source_order_equals_jacobian_order"] for row in cases),
            "extension_source_square_matches": sum(row["source_order_squared_matches_extension_jacobian_order"] for row in cases),
            "brute_force_crosschecked": sum(row["brute_force_equation_crosscheck"] is True for row in cases),
        },
    }
    output = HERE / "results.json"
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"wrote {output}")


if __name__ == "__main__":
    main()
