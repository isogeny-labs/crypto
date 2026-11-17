#!/usr/bin/env sage -python
"""Check whether all degree-three hits are K-isogenous to k-defined curves."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, ZZ


HERE = Path(__file__).resolve().parent
SCAN = HERE / "degree3_all_q16_scan_receipt.json"
BASE_WITNESSES = {10: (8, 6), 24: (0, 6), 16: (0, 2), 18: (8, 2)}


def main():
    raw = SCAN.read_bytes()
    hits = json.loads(raw)["hits"]
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embed = k.embeddings(K)[0]
    R = PolynomialRing(ZZ, "T")
    T = R.gen()
    rows = []
    for label in sorted({hit["class"] for hit in hits}):
        subset = [hit for hit in hits if hit["class"] == label]
        order = subset[0]["expected_order"]
        assert all(hit["expected_order"] == order for hit in subset)
        t = 4097-order
        f = T**6-t*T**3+4096
        candidates = [s for s in range(-8, 9) if s**3-48*s == t]
        assert len(candidates) <= 1
        row = {"class": label, "source_order": order, "source_trace": t,
               "source_equations": len({(h["u_mask"],h["b_mask"]) for h in subset}),
               "cover_descriptions": len(subset), "relative_trace_b_masks": sorted({h["trace_b_mask"] for h in subset}),
               "is_k_defined_isogeny_class": bool(candidates)}
        if candidates:
            s = candidates[0]
            base_order = 17-s
            a_mask, b_mask = BASE_WITNESSES[base_order]
            E0 = EllipticCurve(k, [k(1), k.from_integer(a_mask), k(0), k(0), k.from_integer(b_mask)])
            E0K = EllipticCurve(K, [K(1), embed(k.from_integer(a_mask)), K(0), K(0), embed(k.from_integer(b_mask))])
            assert E0.cardinality() == base_order
            assert E0K.cardinality() == order
            quartic = T**4+s*T**3+(s*s-16)*T**2+16*s*T+256
            assert f == (T**2-s*T+16)*quartic
            row.update({"base_trace": s, "base_order": base_order,
                        "base_elliptic_a2_mask": a_mask, "base_elliptic_b_mask": b_mask,
                        "cubic_extension_order_checked": int(E0K.cardinality()),
                        "trace_zero_factor_order": int(quartic(1)),
                        "weil_polynomial_factorization": str(f.factor())})
        else:
            assert f.is_irreducible()
            row["weil_polynomial_irreducible_over_Q"] = True
        rows.append(row)
    assert len(rows) == 6
    assert sum(row["cover_descriptions"] for row in rows if row["is_k_defined_isogeny_class"]) == 36
    assert all(row["is_k_defined_isogeny_class"] == (row["relative_trace_b_masks"] != [0]) for row in rows)
    receipt = {"schema": "ghs-q16-degree3-base-isogeny-gate-v1",
               "scan_sha256": hashlib.sha256(raw).hexdigest(),
               "rows": rows,
               "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    (HERE / "base_isogeny_degree3_gate_receipt.json").write_text(json.dumps(receipt, indent=2)+"\n")
    print("six classes; all 36 nonzero-trace cover descriptions are K-isogenous to k-defined elliptic curves")


if __name__ == "__main__":
    main()
