#!/usr/bin/env sage -python
"""Construct and test all 15 q16 branch-supported genus-three transfers."""

import hashlib
import json
from pathlib import Path
from time import perf_counter

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing, factor


HERE = Path(__file__).resolve().parent
DESCENT = HERE / "descent_receipt.json"


def mask(z):
    return int(z.to_integer())


def masks(p):
    return [mask(z) for z in p.list()]


def main():
    descent = json.loads(DESCENT.read_text())
    pairs = descent["rows"][0]["pairs"]
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embed = k.embeddings(K)[0]
    ak, a, w = k.gen(), embed(k.gen()), K.gen()
    lookup = {embed(z): z for z in k}
    Rx = PolynomialRing(K, "x")
    x = Rx.gen()
    h = a*a*x**4+a*x
    roots = [z for z, _ in h.roots()]
    N = (x-roots[2])*(x-roots[3])
    D = (x-roots[0])*(x-roots[1])
    V = Rx.fraction_field()(N)/D
    Rk = PolynomialRing(k, "t")
    t = Rk.gen()
    hk = ak*ak+ak*t**3
    RK = PolynomialRing(K, "t")
    Nt = RK([N[2], N[1], N[0]])
    Dt = RK([D[2], D[1], D[0]])
    c = K.from_integer(932)
    d0 = a**3
    assert c*c+c == w**5+d0
    rows = []
    for pair in pairs:
        start = perf_counter()
        lam = K.from_integer(pair["lambda_mask"])
        beta = lam*lam
        b = beta*beta
        assert mask(b) == pair["b_mask"] and pair["lambda_equals_mu"]
        alpha = lam.sqrt()
        S = alpha*(V+1/V)
        X = lam*V**2
        assert S+d0+S**2+S+c**2+c+(beta/X)**2+beta/X == X+w**5+b/X**2
        fquot = h*h*(S+d0)
        assert fquot.denominator() == 1
        fx = fquot.numerator()
        assert fx.degree() <= 8 and all(z in lookup for z in fx.list())
        # t=1/x, y_new=t^4*y gives a ramified odd-degree model.
        fk = Rk([lookup[fx[8-i]] for i in range(9)])
        Ck = HyperellipticCurve(fk, hk)
        assert Ck.genus() == 3
        counts = Ck.count_points(3)
        assert counts[:2] == [17, 257]
        n = 4097-(4097-counts[2])//3
        assert (4097-counts[2]) % 3 == 0
        E = EllipticCurve(K, [1, w**5, 0, 0, b])
        Ep = EllipticCurve(K, [1, d0, 0, lam, 0])
        assert E.cardinality() == Ep.cardinality() == n
        r = max(int(p) for p, _ in factor(n))
        cofactor = n//r
        Gp = Ep(0)
        for xmask in range(1, 4096):
            for candidate in Ep.lift_x(K.from_integer(xmask), all=True):
                candidate = cofactor*candidate
                if candidate != Ep(0) and r*candidate == Ep(0):
                    Gp = candidate
                    break
            if Gp != Ep(0):
                break
        assert Gp != Ep(0) and all((j*Gp)[0] != 0 for j in (1,2,7,14))

        def to_source(P):
            if P == Ep(0):
                return E(0)
            Xp, Yp = P[0], P[1]
            return E(Xp*Xp, Xp*Yp+Xp**3+lam*Xp+c*Xp*Xp+lam*lam)

        G, Q = to_source(Gp), to_source(7*Gp)
        assert G != E(0) and r*G == E(0) and Q == 7*G
        hK = RK([embed(z) for z in hk.list()])
        fK = RK([embed(z) for z in fk.list()])
        J = HyperellipticCurve(fK, hK).jacobian()

        def pullback(P):
            Xp, Yp = P[0], P[1]
            u = (alpha*Nt+Xp*Dt).monic()
            v = (RK(Yp/Xp)*hK) % u
            assert u.degree() == 2 and (v*v+hK*v+fK) % u == 0
            return J(u,v)

        def frob(Z, power):
            return J(RK([z**power for z in Z[0].list()]),
                     RK([z**power for z in Z[1].list()]))

        def transfer(P):
            Z = pullback(P)-pullback(2*P)
            return Z+frob(Z,16)+frob(Z,256)

        image_G = transfer(Gp)
        image_Q = transfer(7*Gp)
        assert image_G != J(0) and r*image_G == J(0)
        assert image_Q == 7*image_G and frob(image_G,16) == image_G
        assert all(z**16 == z for z in image_G[0].list()+image_G[1].list())
        elapsed = perf_counter()-start
        rows.append({
            "b_mask": mask(b), "lambda_mask": mask(lam),
            "source_order": n, "subgroup_prime": r, "cofactor": cofactor,
            "base_curve_odd_model_h": str(hk),
            "base_curve_odd_model_f": str(fk),
            "base_curve_odd_model_h_coeff_masks": masks(hk),
            "base_curve_odd_model_f_coeff_masks": masks(fk),
            "base_curve_point_counts_1_2_3": counts,
            "source_G_xy_masks": [mask(G[0]),mask(G[1])],
            "source_Q_equals_7G_xy_masks": [mask(Q[0]),mask(Q[1])],
            "image_G_u_masks": masks(image_G[0]),
            "image_G_v_masks": masks(image_G[1]),
            "image_Q_u_masks": masks(image_Q[0]),
            "image_Q_v_masks": masks(image_Q[1]),
            "verified_nonzero_base_field_subgroup_map": True,
            "construction_wall_seconds": elapsed,
        })
        print("b",mask(b),"order",n,"r",r,"map",round(elapsed,3),flush=True)
    rows.sort(key=lambda row: row["b_mask"])
    receipt = {"schema": "ghs-q16-genus3-all15-subgroup-maps-v1",
               "field_modulus": str(K.modulus()),
               "base_generator_embedding_mask": mask(a),
               "descent_receipt_sha256": hashlib.sha256(DESCENT.read_bytes()).hexdigest(),
               "rows": rows,
               "search_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    (HERE/"map15_receipt.json").write_text(json.dumps(receipt,indent=2)+"\n")
    print("all15 verified")


if __name__ == "__main__":
    main()
