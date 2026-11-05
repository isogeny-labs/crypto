#!/usr/bin/env sage -python
"""Exact F4096 elliptic subgroup -> F16 genus-three Jacobian map."""

import hashlib
import json
from pathlib import Path
from time import perf_counter

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing, is_prime


HERE = Path(__file__).resolve().parent


def mask(z):
    return int(z.to_integer())


def polynomial_masks(p):
    return [mask(c) for c in p.list()]


def main():
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embedding = k.embeddings(K)[0]
    ak, a, w = k.gen(), embedding(k.gen()), K.gen()
    lam = K.from_integer(1609)
    alpha = lam.sqrt()
    beta = lam**2
    b = beta**2
    c = K.from_integer(932)
    d0 = a**3
    assert mask(b) == 1384 and c**2+c == w**5+d0

    Ep = EllipticCurve(K, [1, d0, 0, lam, 0])
    E = EllipticCurve(K, [1, w**5, 0, 0, b])
    r = 2081
    assert is_prime(r) and Ep.cardinality() == E.cardinality() == 4162
    Gp = Ep(K.from_integer(3633), K.from_integer(2425))
    Qp = 7*Gp
    assert Gp != Ep(0) and r*Gp == Ep(0)

    def to_source(P):
        if P == Ep(0):
            return E(0)
        Xp, Yp = P[0], P[1]
        return E(Xp**2, Xp*Yp+Xp**3+lam*Xp+c*Xp**2+lam**2)

    G, Q = to_source(Gp), to_source(Qp)
    assert G != E(0) and r*G == E(0) and Q == 7*G

    Rt = PolynomialRing(k, "t")
    t = Rt.gen()
    hk = ak**2 + ak*t**3
    fk = (ak**3+ak+1) + (ak**2+ak+1)*t**4 + (ak**2+ak)*t**6 + (ak**3+ak)*t**7
    Ck = HyperellipticCurve(fk, hk)
    assert Ck.genus() == 3 and Ck.count_points(3) == [17, 257, 4292]
    RK = PolynomialRing(K, "t")
    tt = RK.gen()
    h = RK([embedding(z) for z in hk.list()])
    f = RK([embedding(z) for z in fk.list()])
    C = HyperellipticCurve(f, h)
    J = C.jacobian()

    Rx = PolynomialRing(K, "x")
    x = Rx.gen()
    roots = [z for z, _ in (a*a*x**4+a*x).roots()]
    N = (x-roots[2])*(x-roots[3])
    D = (x-roots[0])*(x-roots[1])
    V = Rx.fraction_field()(N)/D
    S = alpha*(V+1/V)
    X = lam*V**2
    assert S+d0+S**2+S+c**2+c+(beta/X)**2+beta/X == X+w**5+b/X**2
    Nt = RK([N[2], N[1], N[0]])
    Dt = RK([D[2], D[1], D[0]])
    # The separable cover C -> Ep is X'=alpha*(Nt/Dt), Y'=X'*(y/h).
    # The purely inseparable isogeny Ep -> E is to_source above.
    def pullback(P):
        assert P != Ep(0) and P[0] != 0
        Xp, Yp = P[0], P[1]
        u = (alpha*Nt + Xp*Dt).monic()
        v = (RK(Yp/Xp)*h) % u
        assert u.degree() == 2 and (v*v+h*v+f) % u == 0
        return J(u, v)

    def frobenius(Z, power):
        return J(RK([z**power for z in Z[0].list()]),
                 RK([z**power for z in Z[1].list()]))

    def transfer(P):
        # [P-2P]=[-P] in Pic^0(Ep); the two degree-two fibers have
        # finite support, avoiding the point at infinity in Mumford form.
        Z = pullback(P)-pullback(2*P)
        return Z+frobenius(Z, 16)+frobenius(Z, 256)

    start = perf_counter()
    image_G, image_Q = transfer(Gp), transfer(Qp)
    map_seconds = perf_counter()-start
    assert image_G != J(0) and r*image_G == J(0)
    assert image_Q == 7*image_G
    assert frobenius(image_G, 16) == image_G
    assert all(z**16 == z for z in image_G[0].list()+image_G[1].list())

    receipt = {
        "schema": "ghs-q16-genus3-subgroup-transfer-4162-v1",
        "field_modulus": str(K.modulus()),
        "base_generator_embedding_mask": mask(a),
        "source_equation": "Y^2+X*Y=X^3+w^5*X^2+b",
        "source_b_mask": mask(b),
        "source_order": 4162,
        "subgroup_prime": r,
        "source_G_xy_masks": [mask(G[0]), mask(G[1])],
        "source_Q_equals_7G_xy_masks": [mask(Q[0]), mask(Q[1])],
        "intermediate_equation": "Y'^2+X'*Y'=X'^3+a^3*X'^2+lambda*X'",
        "lambda_mask": mask(lam),
        "alpha_mask": mask(alpha),
        "c_mask": mask(c),
        "intermediate_G_xy_masks": [mask(Gp[0]), mask(Gp[1])],
        "cover_numerator_masks": polynomial_masks(Nt),
        "cover_denominator_masks": polynomial_masks(Dt),
        "base_curve_odd_model_h": str(hk),
        "base_curve_odd_model_f": str(fk),
        "base_curve_point_counts_1_2_3": [17, 257, 4292],
        "jacobian_order_from_weil_polynomial": 4162,
        "image_G_u_masks": polynomial_masks(image_G[0]),
        "image_G_v_masks": polynomial_masks(image_G[1]),
        "image_Q_u_masks": polynomial_masks(image_Q[0]),
        "image_Q_v_masks": polynomial_masks(image_Q[1]),
        "checks": {
            "source_Q_equals_7G": True,
            "image_G_nonzero": True,
            "image_G_order_2081": True,
            "image_Q_equals_7_image_G": True,
            "image_G_frobenius16_fixed": True,
        },
        "transfer_two_points_wall_seconds": map_seconds,
        "search_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    }
    (HERE/"map4162_receipt.json").write_text(json.dumps(receipt, indent=2)+"\n")
    print(json.dumps({"b": mask(b), "G": receipt["source_G_xy_masks"],
                      "Q": receipt["source_Q_equals_7G_xy_masks"],
                      "image_G_u": receipt["image_G_u_masks"],
                      "image_G_v": receipt["image_G_v_masks"],
                      "map_seconds": map_seconds}, indent=2))


if __name__ == "__main__":
    main()
