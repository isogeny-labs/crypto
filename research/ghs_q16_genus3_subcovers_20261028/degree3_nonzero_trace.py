#!/usr/bin/env sage -python
"""Exact degree-three cover and subgroup transfer for one nonzero-trace q16 hit."""

import json
import hashlib
import sys
from pathlib import Path
from time import perf_counter

from sage.all import EllipticCurve, GF, HyperellipticCurve, LaurentSeriesRing, PolynomialRing, sage_eval


HERE = Path(__file__).resolve().parent


def mask(z):
    return int(z.to_integer())


def main(model_index=5):
    start = perf_counter()
    row = next(json.loads(s) for s in (HERE / "catalog_all_q16_weil_classes.jsonl").read_text().splitlines()
               if json.loads(s)["label"] == "3.16.a_a_ah")
    assert model_index in (5, 12)
    equation = row["curves"][model_index]
    scan = json.loads((HERE / "degree3_all_q16_scan_receipt.json").read_text())
    hit = next(h for h in scan["hits"] if h["class"] == row["label"]
               and h["model_index"] == model_index and h["exponents"] == [-1, -2, 1, 2])
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embed = k.embeddings(K)[0]
    R2 = PolynomialRing(k, names=("x", "y"))
    x, y = R2.gens()
    left, right = equation.split("=")
    p = sage_eval(left, locals={"a": k.gen(), "x": x, "y": y}) - sage_eval(
        right, locals={"a": k.gen(), "x": x, "y": y})
    h0 = R2(p.coefficient({y: 1}))
    f0 = R2(-p.coefficient({y: 0}))
    RK = PolynomialRing(K, "t")
    t = RK.gen()
    h = RK([embed(h0.coefficient({x: i})) for i in range(h0.degree(x) + 1)])
    f = RK([embed(f0.coefficient({x: i})) for i in range(f0.degree(x) + 1)])
    roots = [z for z, multiplicity in h.roots() if multiplicity == 1]
    assert len(roots) == 4 and roots[0] == 0
    F = RK.fraction_field()
    U = F(1)
    for z, exponent in zip(roots, (-1, -2, 1, 2)):
        U *= F(t-z)**exponent
    lam, mu = K.from_integer(hit["lambda_mask"]), K.from_integer(hit["mu_mask"])
    beta, b = lam*mu, (lam*mu)**2
    X = lam*U
    D = F(f/h**2+X+beta/X)
    L = LaurentSeriesRing(K, "s", default_prec=8)
    s = L.gen()
    H = sum((F(L(D(t=s+z))[-2].sqrt())/F(t-z) for z in roots), F(0))
    T = D+H**2+H
    assert T.denominator().degree() == T.numerator().degree() == 0
    u = T.numerator()[0]/T.denominator()[0]
    assert (mask(u), mask(b)) == (hit["u_mask"], hit["b_mask"])
    E = EllipticCurve(K, [K(1), u, K(0), K(0), b])
    assert E.cardinality() == sum(row["poly"]) == 4090

    # The catalog model is even degree. Move its branch point x=0 to infinity.
    hodd = RK([h[4-i] for i in range(5)])
    ftemp = RK([f[8-i] for i in range(9)])
    q = f[0].sqrt()*t**4
    fodd = ftemp+q**2+hodd*q
    assert fodd.degree() == 7 and hodd.degree() == 3
    C = HyperellipticCurve(fodd, hodd)
    assert C.genus() == 3
    J = C.jacobian()

    G = E(0)
    for i in range(1, 4096):
        for candidate in E.lift_x(K.from_integer(i), all=True):
            candidate = 10*candidate
            if candidate != E(0) and 409*candidate == E(0):
                G = candidate
                break
        if G != E(0):
            break
    assert G != E(0)

    def pull(point):
        xp, yp = point[0], point[1]
        assert xp != 0
        function = X(t=1/t)
        poly = (function.numerator()-xp*function.denominator()).monic()
        zvalue = (yp+beta)/xp
        yfunction = F(hodd)*(zvalue+H(t=1/t))+F(q)
        v = (yfunction.numerator()*yfunction.denominator().inverse_mod(poly)) % poly
        assert poly.degree() == 3 and (v*v+hodd*v+fodd) % poly == 0
        return J(poly, v)

    def frob(Z, power):
        return J(RK([z**power for z in Z[0].list()]),
                 RK([z**power for z in Z[1].list()]))

    def transfer(point):
        Z = pull(point)-pull(2*point)
        return Z+frob(Z, 16)+frob(Z, 256)

    image_G = transfer(G)
    image_Q = transfer(7*G)
    assert image_G != J(0) and 409*image_G == J(0)
    assert image_Q == 7*image_G and frob(image_G, 16) == image_G
    assert all(z**16 == z for z in image_G[0].list()+image_G[1].list())
    receipt = {
        "schema": "ghs-q16-degree3-nonzero-trace-cover-v1",
        "scan_receipt_sha256": hashlib.sha256((HERE / "degree3_all_q16_scan_receipt.json").read_bytes()).hexdigest(),
        "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "catalog_label": row["label"], "catalog_model_index": model_index, "catalog_equation": equation,
        "source_equation": "Y^2+X*Y=X^3+u*X^2+b",
        "field_modulus": str(K.modulus()), "base_embedding_mask": mask(embed(k.gen())),
        "u_mask": mask(u), "b_mask": mask(b), "relative_trace_b_mask": mask(sum((b**(16**i) for i in range(3)), K(0))),
        "lambda_mask": mask(lam), "mu_mask": mask(mu), "beta_mask": mask(beta),
        "branch_root_masks": [mask(z) for z in roots], "branch_exponents": [-1, -2, 1, 2],
        "cover_X_numerator_masks": [mask(z) for z in X.numerator().list()],
        "cover_X_denominator_masks": [mask(z) for z in X.denominator().list()],
        "cover_H_numerator_masks": [mask(z) for z in H.numerator().list()],
        "cover_H_denominator_masks": [mask(z) for z in H.denominator().list()],
        "odd_model_h_masks": [mask(z) for z in hodd.list()],
        "odd_model_f_masks": [mask(z) for z in fodd.list()],
        "source_order": 4090, "subgroup_prime": 409,
        "G_xy_masks": [mask(G[0]), mask(G[1])],
        "Q_equals_7G_xy_masks": [mask((7*G)[0]), mask((7*G)[1])],
        "image_G_u_masks": [mask(z) for z in image_G[0].list()],
        "image_G_v_masks": [mask(z) for z in image_G[1].list()],
        "image_Q_u_masks": [mask(z) for z in image_Q[0].list()],
        "image_Q_v_masks": [mask(z) for z in image_Q[1].list()],
        "verified_exact_cover": True, "verified_subgroup_transfer": True,
        "construction_wall_seconds": perf_counter()-start,
    }
    filename = "degree3_nonzero_trace_receipt.json" if model_index == 5 else f"degree3_nonzero_trace_receipt_{model_index}.json"
    (HERE / filename).write_text(json.dumps(receipt, indent=2)+"\n")
    print("model", model_index, "exact degree-three cover; nonzero trace; subgroup map verified; seconds", round(perf_counter()-start, 3))


if __name__ == "__main__":
    main(int(sys.argv[1]) if len(sys.argv) > 1 else 5)
