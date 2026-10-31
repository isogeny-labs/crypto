#!/usr/bin/env sage -python
"""Exact 3-torsion census and subgroup-preserving source isogenies."""
import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, is_prime

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "ghs_cubic_hess_closure_q16_20261027" / "certificate.json"
SIMPLE = HERE.parent / "ghs_q16_simple_restriction_bound_20261027" / "certificate.json"

R = PolynomialRing(GF(2), "w")
w = R.gen()
K = GF(2**12, modulus=w**12+w**7+w**6+w**5+w**3+w+1, name="w")
u = K.gen()
P = PolynomialRing(K, "x")
x = P.gen()


def dec(code):
    return sum(((code >> i) & 1) * u**i for i in range(12))


def code(element):
    return sum(int(c) << i for i, c in enumerate(element.polynomial().list()))


def point_codes(point):
    return [code(point[0]), code(point[1])]


def frobenius_on_three_torsion(E, L):
    EL = E.change_ring(L)
    pts = [point for xval, _ in EL.division_polynomial(3).roots(L)
           for point in EL.lift_x(xval, all=True)]
    assert len(pts) == 8 and all(3 * pt == EL(0) for pt in pts)
    pts.sort(key=lambda pt: (code(pt[0]), code(pt[1])))
    first = pts[0]
    second = next(pt for pt in pts if pt != first and pt != -first)
    coordinates = {i*first+j*second: [i, j]
                   for i in range(3) for j in range(3)}
    ffirst = EL([first[0]**4096, first[1]**4096])
    fsecond = EL([second[0]**4096, second[1]**4096])
    c1, c2 = coordinates[ffirst], coordinates[fsecond]
    return {
        "basis_P": point_codes(first), "basis_Q": point_codes(second),
        "frobenius_columns": [c1, c2],
        "scalar": c1 == [2, 0] and c2 == [0, 2],
    }


def main():
    source_raw = SOURCE.read_bytes()
    source = json.loads(source_raw)
    simple_raw = SIMPLE.read_bytes()
    simple = json.loads(simple_raw)
    simple_orders = {row["order"] for row in simple["rows"] if row["irreducible"]}
    factor_patterns = {}
    pattern_count = Counter()
    scalar_b = set()
    for b in range(1, 4096):
        f = x**4+x**3+dec(b)
        degrees = tuple(sorted(int(g.degree()) for g, e in f.factor() for _ in range(int(e))))
        factor_patterns[b] = degrees
        pattern_count[str(degrees)] += 1
        if degrees == (1, 1, 1, 1):
            scalar_b.add(b)
    assert pattern_count == {"(1, 1, 1, 1)": 341, "(1, 3)": 2730, "(2, 2)": 1024}
    assert len(scalar_b) == 341

    rows = []
    for item in source["orders"]:
        selected = {a: sorted(set(v["hess"]+v["outside"]) & scalar_b)
                    for a, v in item["by_a"].items()}
        count = sum(map(len, selected.values()))
        rows.append({
            "order": item["order"], "catalog_label": item["catalog_label"],
            "outside_hess_closure": not item["in_hess_isogeny_closure"],
            "catalog_jacobian": bool(item["catalog_has_jacobian"]),
            "simple_restriction": item["order"] in simple_orders,
            "source_models": item["source_models"],
            "largest_prime": item["largest_prime"],
            "scalar_three_models": count,
            "scalar_b_by_a": {a: bs for a, bs in selected.items() if bs},
        })
    assert sum(row["scalar_three_models"] for row in rows) == 682
    residual = [row for row in rows if row["outside_hess_closure"]
                and row["catalog_jacobian"] and row["simple_restriction"]]
    residual_positive = [row for row in residual if row["scalar_three_models"]]
    assert len(residual) == 44
    assert len(residual_positive) == 8
    assert sum(row["scalar_three_models"] for row in residual_positive) == 177
    all_congruence = [row for row in rows if row["order"] % 9 in (0, 4)]
    assert len(all_congruence) == 28
    assert sum(row["source_models"] for row in all_congruence) == 2530
    residual_congruence = [row for row in residual if row["order"] % 9 in (0, 4)]
    assert {row["order"] for row in residual_congruence} == {row["order"] for row in residual_positive}
    assert sum(row["source_models"] for row in residual_congruence) == 651

    L6 = K.extension(6, "v")
    exact_controls = []
    for b, order, r, g_codes, q_codes in (
        (198, 4006, 2003, [199, 2783], [738, 3018]),
        (32, 4054, 2027, [33, 349], [2098, 1182]),
        (44, 4078, 2039, [45, 1367], [47, 3257]),
        (63, 4126, 2063, [62, 1499], [2633, 1798]),
    ):
        E = EllipticCurve(K, [K(1), dec(32), K(0), K(0), dec(b)])
        assert E.cardinality() == order
        G = E([dec(c) for c in g_codes])
        Q = E([dec(c) for c in q_codes])
        assert is_prime(r) and r*G == E(0) and G != E(0) and Q == 7*G
        torsion = frobenius_on_three_torsion(E, L6)
        assert factor_patterns[b] == (1, 3)
        assert not torsion["scalar"]
        row = {"source_b": b, "order": order, "subgroup_prime": r,
               "source_G": g_codes, "source_Q": q_codes,
               "three_division_factor_degrees": list(factor_patterns[b]),
               "three_torsion": torsion}
        if b in (32, 63):
            candidates = E.isogenies_prime_degree(3)
            assert len(candidates) == 1
            phi = candidates[0]
            target_b = code(1 / phi.codomain().j_invariant())
            target = EllipticCurve(K, [K(1), dec(32), K(0), K(0), dec(target_b)])
            iso = phi.codomain().isomorphism_to(target)
            image_G, image_Q = iso(phi(G)), iso(phi(Q))
            assert target_b in scalar_b
            assert target.cardinality() == order
            assert image_G != target(0) and r*image_G == target(0) and image_Q == 7*image_G
            assert int(phi.degree()) == 3
            row["isogeny_to_scalar_model"] = {
                "target_b": target_b,
                "kernel_x": code(phi.kernel_polynomial()[0]),
                "target_G": point_codes(image_G), "target_Q": point_codes(image_Q),
                "isomorphism_urst": [code(z) for z in iso.tuple()],
                "target_division_factor_degrees": list(factor_patterns[target_b]),
                "phi_rational_maps": [str(m) for m in phi.rational_maps()],
            }
        exact_controls.append(row)

    one_step_witnesses = []
    source_by_order = {item["order"]: item for item in source["orders"]}
    for row in residual_congruence:
        N = row["order"]
        r = row["largest_prime"]
        assert is_prime(r) and r > 3
        item = source_by_order[N]
        all_b = sorted(set(item["by_a"]["32"]["hess"] + item["by_a"]["32"]["outside"]))
        b0 = next(b for b in all_b if b not in scalar_b)
        E0 = EllipticCurve(K, [K(1), dec(32), K(0), K(0), dec(b0)])
        assert E0.cardinality() == N and factor_patterns[b0] == (1, 3)
        phis = E0.isogenies_prime_degree(3)
        assert len(phis) == 1
        phi = phis[0]
        b1 = code(1 / phi.codomain().j_invariant())
        assert b1 in scalar_b
        E1 = EllipticCurve(K, [K(1), dec(32), K(0), K(0), dec(b1)])
        iso = phi.codomain().isomorphism_to(E1)
        assert E1.cardinality() == N
        G0 = None
        for xcode in range(4096):
            for point in sorted(E0.lift_x(dec(xcode), all=True), key=lambda pt: code(pt[1])):
                candidate = (N // r) * point
                if candidate != E0(0) and r * candidate == E0(0):
                    G0 = candidate
                    break
            if G0 is not None:
                break
        assert G0 is not None
        Q0 = 7 * G0
        G1, Q1 = iso(phi(G0)), iso(phi(Q0))
        assert G1 != E1(0) and r * G1 == E1(0) and Q1 == 7 * G1
        one_step_witnesses.append({
            "order": N, "subgroup_prime": r,
            "source_b": b0, "target_b": b1,
            "kernel_x": code(phi.kernel_polynomial()[0]),
            "source_G": point_codes(G0), "source_Q": point_codes(Q0),
            "target_G": point_codes(G1), "target_Q": point_codes(Q1),
            "isomorphism_urst": [code(z) for z in iso.tuple()],
            "phi_rational_maps": [str(m) for m in phi.rational_maps()],
        })

    b = 116
    E = EllipticCurve(K, [K(1), dec(32), K(0), K(0), dec(b)])
    assert E.cardinality() == 4198 and is_prime(2099)
    r = 2099
    G = None
    for xcode in range(4096):
        choices = E.lift_x(dec(xcode), all=True)
        for point in sorted(choices, key=lambda pt: code(pt[1])):
            candidate = 2*point
            if candidate != E(0) and r*candidate == E(0):
                G = candidate
                break
        if G is not None:
            break
    assert G is not None
    Q = 7*G
    assert factor_patterns[b] == (1, 1, 1, 1)
    L2 = K.extension(2, "z")
    new_focus = {
        "a": 32, "b": b, "order": 4198, "subgroup_prime": r,
        "G": point_codes(G), "Q": point_codes(Q),
        "three_division_factor_degrees": [1, 1, 1, 1],
        "three_torsion": frobenius_on_three_torsion(E, L2),
        "catalog_label": next(row["catalog_label"] for row in rows if row["order"] == 4198),
    }
    assert new_focus["three_torsion"]["scalar"]

    output = {
        "schema": "ghs-q16-scalar-three-torsion-gate-v1",
        "source_certificate_sha256": hashlib.sha256(source_raw).hexdigest(),
        "simple_certificate_sha256": hashlib.sha256(simple_raw).hexdigest(),
        "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "field_modulus": "w^12+w^7+w^6+w^5+w^3+w+1",
        "extension6_modulus": str(L6.modulus()),
        "extension6_base_generator_code": code(L6(u)),
        "factor_patterns": dict(pattern_count),
        "scalar_b": sorted(scalar_b),
        "summary": {
            "all_source_models": 8190, "scalar_three_source_models": 682,
            "all_one_step_classes": len(all_congruence),
            "all_one_step_models": 2530,
            "residual_catalog_classes": len(residual),
            "residual_scalar_classes": len(residual_positive),
            "residual_scalar_models": 177,
            "residual_one_step_classes": len(residual_congruence),
            "residual_one_step_models": 651,
        },
        "residual_scalar_rows": residual_positive,
        "rows": rows,
        "exact_controls": exact_controls,
        "one_step_witnesses": one_step_witnesses,
        "new_focus_4198": new_focus,
    }
    (HERE / "certificate.json").write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps(output["summary"], indent=2))


if __name__ == "__main__":
    main()
