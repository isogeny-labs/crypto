"""Certified reach for several presentations, via cuso.symbolic.

Each call returns the asymptotic exponent delta for the *first* variable; the
bounds list gives the relative exponent of every variable (1 = p^delta).
"""
import json
import logging
import sys
import time

from sage.all import PolynomialRing, QQ
from cuso.symbolic import SymbolicCoppersmithProblem, get_asymptotic_bounds

logging.basicConfig(level=logging.ERROR)


def model(name):
    K = PolynomialRing(QQ, 'z').fraction_field(); z = K.gen()
    a, b = 0, 7
    if name == "xcoords":
        R = PolynomialRing(K, 'x,y'); x, y = R.gens()
        f = (x-y)**2*z**2 - 2*((x+y)*(x*y + a) + 2*b)*z + (x*y - a)**2 - 4*b*(x+y)
        return R, [f], [1, 1], [1, x**2, y**2, x**2*y**2]
    if name == "vieta":
        R = PolynomialRing(K, 'e1,e2'); e1, e2 = R.gens()
        f = z**2*(e1**2 - 4*e2) - 2*z*(e1*e2 + a*e1 + 2*b) + e2**2 - 2*a*e2 + a**2 - 4*b*e1
        return R, [f], [1, 2], [1, e1**2, e1*e2, e2**2]
    if name == "chained":
        R = PolynomialRing(K, 'x1,x2,x3,t'); x1, x2, x3, t = R.gens()
        def S3(u, v, w):
            return ((u-v)**2*w**2 - 2*((u+v)*(u*v + a) + 2*b)*w + (u*v - a)**2 - 4*b*(u + v))
        # t is the full-size abscissa of P1+P2
        return R, [S3(x1, x2, t), S3(t, x3, z)], [(0,1), (0,1), (0,1), (1,0)], \
               [1, x1**2, x2**2, x3**2, t, t**2]
    raise ValueError(name)


def run(name, mult=2):
    R, rels, bounds, verts = model(name)
    prob = SymbolicCoppersmithProblem(rels, bounds, modulus_name="p")
    t0 = time.time()
    delta, tau, cond = get_asymptotic_bounds(
        prob, [0.1]*len(bounds), verts,
        precompute_multiplicity=mult, tshifts_to_include=[False]*len(bounds))
    rec = {"presentation": name, "bounds": bounds, "multiplicity": mult,
           "delta": str(delta), "delta_float": float(delta), "tau": str(tau),
           "seconds": round(time.time()-t0, 1)}
    print(rec, flush=True)
    return rec


if __name__ == "__main__":
    names = sys.argv[1].split(",") if len(sys.argv) > 1 else ["vieta", "chained"]
    out = [run(n) for n in names]
    json.dump(out, open(sys.argv[2] if len(sys.argv) > 2 else "cuso_presentations.json", "w"), indent=1)
