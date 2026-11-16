"""Exact q=16 cubic Weil-restriction / genus-two Jacobian gate.

Run with: sage -python research/ghs_q16_tracezero_genus2_20261028/audit.py
The HNR classification and finite-field isogeny theorem are external inputs;
this script certifies their exact integer and polynomial hypotheses.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

from sage.all import EllipticCurve, PolynomialRing, QQ, ZZ

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "ghs_nonsplit_order4222_20261028"))
from frontier_q16 import K, source_order, walsh_trace_inverse_sums  # noqa: E402

R = PolynomialRing(QQ, "X")
X = R.gen()

# 2026-10-19 local public.av_fq_isog snapshot, matched by full Weil polynomial.
CATALOG = {
    3980: ("2.16.ad_ah", -1), 3982: ("2.16.af_j", 1),
    4050: ("2.16.ab_ap", 1), 4090: ("2.16.h_bh", 1),
    4104: ("2.16.ah_bh", 1), 4144: ("2.16.b_ap", 1),
    4212: ("2.16.f_j", 1), 4214: ("2.16.d_ah", -1),
}


def point_coordinates(p):
    if p.is_zero():
        return None
    return [int(p[0].to_integer()), int(p[1].to_integer())]


def base_defined_map_receipt(models, n, r):
    """Evaluate [1-Frob_16] on an exact base-defined representative."""
    twist, b_mask = next((twist, b) for twist, b in models
                         if K.from_integer(b)**16 == K.from_integer(b))
    a_mask = 372 if twist else 0
    a, b = K.from_integer(a_mask), K.from_integer(b_mask)
    assert a**16 == a and b**16 == b
    curve = EllipticCurve(K, [1, a, 0, 0, b])
    assert int(curve.cardinality()) == n
    zero = curve(0)
    r_power = int(r ** ZZ(n).valuation(r))
    for x_mask in range(4096):
        points = sorted(curve.lift_x(K.from_integer(x_mask), all=True),
                        key=lambda p: int(p[1].to_integer()))
        for p in points:
            g = (n // r_power) * p
            while g != zero and r * g != zero:
                g = r * g
            if g == zero or r * g != zero:
                continue
            frob = lambda v: (zero if v == zero else
                              curve([v[0]**16, v[1]**16, 1]))
            h = g - frob(g)
            q = 7 * g
            hq = q - frob(q)
            assert h != zero and r * h == zero and hq == 7 * h
            assert g + frob(g) + frob(frob(g)) == zero
            assert h + frob(h) + frob(frob(h)) == zero
            return {"a_mask": a_mask, "b_mask": b_mask,
                    "G": point_coordinates(g), "Q_equals_7G": point_coordinates(q),
                    "tracezero_G": point_coordinates(h),
                    "tracezero_Q": point_coordinates(hq),
                    "map_formula": "P - Frob_16(P)",
                    "checks": ["nonzero", "order_r", "trace_zero", "Q_map_equals_7G"]}
    raise AssertionError("prime-order representative point not found")


def main() -> None:
    sums = walsh_trace_inverse_sums()
    groups: dict[int, list[tuple[int, int]]] = {}
    for twist in (0, 1):
        for b in range(1, 4096):
            n = int(source_order(twist, b, sums))
            groups.setdefault(n, []).append((twist, b))
    assert len(groups) == 128 and sum(map(len, groups.values())) == 8190
    assert set(groups) == set(range(3970, 4225, 2))

    rows = []
    simple_count = 0
    for n, models in sorted(groups.items()):
        t = 4097 - n
        p = X**6 - t * X**3 + 4096
        if p.is_irreducible():
            simple_count += len(models)
            continue
        fs = p.factor()
        assert len(fs) == 2 and all(e == 1 for _, e in fs)
        e = next(f for f, _ in fs if f.degree() == 2)
        bpoly = next(f for f, _ in fs if f.degree() == 4)
        assert bpoly.is_irreducible() and int(e(1) * bpoly(1)) == n
        a = -int(e[1])
        assert a in (-7, -5, -3, -1, 1, 3, 5, 7)
        assert t == a**3 - 48 * a
        assert e == X**2 - a * X + 16
        assert bpoly == X**4 + a*X**3 + (a*a-16)*X**2 + 16*a*X + 256
        r = int(ZZ(n).factor()[-1][0])
        assert r > 2 and ZZ(r).is_prime()
        assert int(e(1)) % r != 0 and int(bpoly(1)) % r == 0
        hnr_obstruction = (a*a - int(bpoly[2]) == 16
                           and int(bpoly[2]) < 0
                           and all(int(pr) % 3 == 1
                                   for pr, _ in ZZ(-int(bpoly[2])).factor()))
        label, status = CATALOG[n]
        assert hnr_obstruction == (status == -1)
        rows.append({
            "source_order": n, "source_trace": t, "models": len(models),
            "models_twist_b": models, "base_trace": a,
            "elliptic_factor": list(map(int, e.list())),
            "surface_factor": list(map(int, bpoly.list())),
            "elliptic_order": int(e(1)), "surface_order": int(bpoly(1)),
            "selected_prime": r,
            "surface_derivative_at_one": int(bpoly.derivative()(1)),
            "simple_root_gate": ZZ(r).gcd(ZZ(bpoly.derivative()(1))) == 1,
            "hnr_no_genus2_jacobian": hnr_obstruction,
            "lmfdb_label": label, "lmfdb_has_jacobian_snapshot": status,
            "base_defined_representative": base_defined_map_receipt(models, n, r),
        })
    assert [x["source_order"] for x in rows] == sorted(CATALOG)
    assert simple_count == 7584
    assert sum(x["models"] for x in rows) == 606
    no_jac_count = sum(x["models"] for x in rows if x["hnr_no_genus2_jacobian"])
    possible_count = sum(x["models"] for x in rows if not x["hnr_no_genus2_jacobian"])
    simple_root_count = sum(x["models"] for x in rows
                            if not x["hnr_no_genus2_jacobian"] and x["simple_root_gate"])
    assert (no_jac_count, possible_count, simple_root_count) == (56, 550, 379)
    out = {
        "schema": "ghs-q16-tracezero-genus2-v1",
        "field": "K=F_2[w]/(w^12+w^7+w^6+w^5+w^3+w+1); k=F_16",
        "population": {"ordinary_models": 8190, "source_orders": 128,
                       "simple_threefold_models": 7584,
                       "split_elliptic_surface_models": 606,
                       "surface_no_jacobian_models": 56,
                       "genus2_jacobian_class_models": 550,
                       "simple_root_certified_models": 379,
                       "repeated_root_unresolved_models": 171},
        "rows": rows,
        "external_inputs": [
            "Tate finite-field isogeny theorem",
            "Howe-Nart-Ritzenthaler 2009 Theorem 1.2 and Table 1.2",
            "LMFDB public.av_fq_isog snapshot extracted 2026-10-19 16:45 UTC",
        ],
        "claim": "Necessary genus-at-most-two Jacobian transfer gate for the selected largest-prime source subgroups; no attack cost inferred.",
    }
    path = HERE / "certificate.json"
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"population": out["population"], "orders": [x["source_order"] for x in rows]}))


if __name__ == "__main__":
    main()
