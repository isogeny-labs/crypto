#!/usr/bin/env python3
"""Certify a prime-to-r isogeny existence gate for the old q16 frontier.

This computes the arithmetic hypotheses only. The conclusion uses the
Honda-Tate classification and Tate's integral homomorphism theorem:
if two threefolds have the same Frobenius polynomial f, and f mod r is
squarefree, their r-adic Tate modules are free rank-one modules over the
same finite etale Z_r-algebra. An equivariant isomorphism then lifts to
an isogeny whose degree is prime to r. This does not evaluate that map.
"""

import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
FRONTIER = HERE.parent / "ghs_q16_genus4_frontier_20261028" / "summary.json"
CATALOG = HERE / "catalog_models.json"
MODULUS = (1 << 12) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 3) | (1 << 1) | 1


def mul(x, y):
    out = 0
    while y:
        if y & 1:
            out ^= x
        y >>= 1
        x <<= 1
        if x & (1 << 12):
            x ^= MODULUS
    return out


def power(x, exponent):
    out = 1
    while exponent:
        if exponent & 1:
            out = mul(out, x)
        x = mul(x, x)
        exponent >>= 1
    return out


def relative_trace(x):
    return x ^ power(x, 16) ^ power(x, 256)


def is_prime(n):
    if n < 2:
        return False
    return all(n % d for d in range(2, int(n**0.5) + 1))


def main():
    source = json.loads(FRONTIER.read_text())
    catalog = json.loads(CATALOG.read_text())
    classes = {row["label"]: row for row in catalog["rows"]}
    rows = []
    for class_row in source["classes"]:
        order = class_row["source_order"]
        r = class_row["subgroup_prime"]
        t = 16**3 + 1 - order
        f = [1, 0, 0, -t, 0, 0, 16**3]
        label = class_row["lmfdb_class"]
        assert classes[label]["poly"] == f and classes[label]["curves"]
        assert is_prime(r) and order % r == 0 and (order // r) % r != 0
        discriminant_mod_r = (3**6 * 16**6 * (t*t - 4*16**3)**3) % r
        assert discriminant_mod_r != 0
        exact = []
        for model in class_row["models"]:
            b = model["b_mask"]
            trace = relative_trace(b)
            assert trace != 0
            exact.append({"a_mask": model["a_mask"], "b_mask": b,
                          "relative_trace_b_mask": trace})
        rows.append({"source_order": order, "subgroup_prime": r,
                     "source_trace_t": t, "frobenius_polynomial_coefficients": f,
                     "lmfdb_jacobian_class": label,
                     "catalog_model_count": len(classes[label]["curves"]),
                     "discriminant_mod_r": discriminant_mod_r,
                     "f_squarefree_mod_r": True,
                     "prime_to_r_isogeny_exists_by_tate": True,
                     "explicit_isogeny_evaluated": False,
                     "models": exact})
    assert sum(len(row["models"]) for row in rows) == 66
    receipt = {"schema": "ghs-q16-genus3-prime-to-r-isogeny-feasibility-v1",
               "scope": "abstract existence for 66 exact source equations in three cataloged genus-three Jacobian classes; no explicit map or attack cost",
               "discriminant_formula": "disc(T^6-t*T^3+q^3)=3^6*q^6*(t^2-4*q^3)^3",
               "theorem_source": "Tate, Endomorphisms of abelian varieties over finite fields; Honda-Tate classification",
               "frontier_sha256": hashlib.sha256(FRONTIER.read_bytes()).hexdigest(),
               "catalog_sha256": hashlib.sha256(CATALOG.read_bytes()).hexdigest(),
               "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
               "classes": rows}
    (HERE / "isogeny_feasibility_receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("66 non-trace-zero source equations; all three classes pass the prime-to-r isogeny existence gate")


if __name__ == "__main__":
    main()
