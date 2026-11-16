"""Idea 1 certified reach via cuso's symbolic asymptotic bound.

Requires: SageMath + cuso (https://github.com/keeganryan/cuso).
Each call returns (delta, tau, cond) where delta is the certified exponent
B = p^delta of the shift-lattice strategy in the given presentation.
"""
import json
import logging
import sys
import time

from sage.all import PolynomialRing, QQ

from cuso.symbolic import SymbolicCoppersmithProblem, get_asymptotic_bounds

logging.basicConfig(level=logging.ERROR)


def make_S(m, a=0, b=7):
    """S_{m+1} in m variables plus the target coefficient z, over QQ(z)."""
    K = PolynomialRing(QQ, 'z').fraction_field()
    z = K.gen()
    names = ",".join(f"x{i}" for i in range(1, m+1))
    R = PolynomialRing(K, names)
    S = PolynomialRing(R, 'T')
    X = R.gens(); T = S.gen()

    def S3(u, v, w):
        u, v, w = S(u), S(v), S(w)
        return ((u-v)**2*w**2 - 2*((u+v)*(u*v + a) + 2*b)*w
                + (u*v - a)**2 - 4*b*(u + v))

    if m == 2:
        poly = R(S3(X[0], X[1], z))
    elif m == 3:
        poly = R(S3(X[0], X[1], T).resultant(S3(X[2], z, T)))
    elif m == 4:
        # S4(x1,x2,x3,T) = Res_U(S3(x1,x2,U), S3(x3,T,U))
        # S5(x1..x4,z)   = Res_T(S4(x1..x3,T), S3(x4,z,T))
        Uring = PolynomialRing(S, 'U')
        U = Uring.gen()

        def S3U(u, v, w):
            u, v, w = Uring(u), Uring(v), Uring(w)
            return ((u-v)**2*w**2 - 2*((u+v)*(u*v + a) + 2*b)*w
                    + (u*v - a)**2 - 4*b*(u + v))

        S4 = S3U(X[0], X[1], U).resultant(S3U(X[2], T, U)).numerator()
        poly = R(S4.resultant(S3(X[3], z, T)).numerator())
    else:
        raise NotImplementedError
    return R, poly


def bounds_for(m, deg, R):

    X = R.gens()
    verts = [R(1)]
    import itertools
    for r in range(1, m+1):
        for comb in itertools.combinations(X, r):
            v = R(1)
            for x in comb:
                v *= x**deg
            verts.append(v)
    return verts


def run(m, deg, mult, out):
    R, poly = make_S(m)
    prob = SymbolicCoppersmithProblem([poly], [1]*m, modulus_name="p")
    t0 = time.time()
    delta, tau, cond = get_asymptotic_bounds(
        prob, [0.1]*m, bounds_for(m, deg, R),
        precompute_multiplicity=mult, tshifts_to_include=[False]*m)
    rec = {"m": m, "degree": deg, "multiplicity": mult,
           "delta": str(delta), "delta_float": float(delta),
           "tau": str(tau), "cond": str(cond), "seconds": round(time.time()-t0, 1)}
    print(rec, flush=True)
    json.dump(rec, open(out, "w"), indent=1)
    return rec


if __name__ == "__main__":
    m = int(sys.argv[1]) if len(sys.argv) > 1 else 2
    deg = {2: 2, 3: 4, 4: 8}[m]
    run(m, deg, int(sys.argv[2]) if len(sys.argv) > 2 else 2,
        sys.argv[3] if len(sys.argv) > 3 else f"cuso_m{m}.json")
