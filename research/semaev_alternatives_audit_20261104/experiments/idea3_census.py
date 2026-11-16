"""Idea 3, corrected census: how special are decomposition forms in L_R?

For a planted 2-decomposition R = P + Q, enumerate every vector of
L_R = {(c0,c1,c2): c0 + c1 x_R + c2 x_R^2 = 0 mod p} with |c_i| <= B^4, and
count those that look like S_3(x1,x2,.):

  * c2 is a perfect square          (necessary: c2 = (x1-x2)^2)
  * D = c1^2 - 4 c0 c2 is in S_B     (necessary: D = 16 f(x1) f(x2))
  * both

Feasibility: the box holds ~8B^12/p points, so this runs only for delta <~ 0.10
(the point of the correction).  We also report the work exponent, since the
enumeration is (2B^4+1)^2 candidate pairs.

Output: JSON lines, one per (p, delta).
"""
import json
import math
import random
import sys

import sympy as sp

from lib.ec import Curve


def s3_xy_val(z, a, b, x, y):
    return ((x - y)**2 * z**2 - 2*((x + y)*(x*y + a) + 2*b)*z
            + (x*y - a)**2 - 4*b*(x + y))


def is_square(n: int) -> bool:
    if n < 0:
        return False
    r = math.isqrt(n)
    return r*r == n


def census(pbits, delta, seed=5, a=0, b=7):
    rng = random.Random(seed)
    p = int(sp.nextprime(2**pbits))
    B = max(2, int(round(p ** delta)))
    E = Curve(p, a, b)
    base = E.points_with_x_upto(B, rng)
    if len(base) < 2:
        return None
    P, Q = rng.sample(base, 2)
    R = E.add(P, Q)
    xR = R[0]
    B4 = B**4

    # structured discriminants from pairs x1,x2 <= B
    S = {16 * (x1**3 + a*x1 + b) * (x2**3 + a*x2 + b)
         for x1 in range(1, B+1) for x2 in range(1, B+1)}
    # actual decompositions of R (there is at least one: P+Q)
    dec = [(x1, x2) for x1 in range(1, B+1) for x2 in range(1, B+1)
           if s3_xy_val(xR, a, b, x1, x2) % p == 0]

    total = sq_lead = str_disc = both = 0
    xR2 = (xR*xR) % p
    for c2 in range(-B4, B4+1):
        sq2 = is_square(abs(c2))
        for c1 in range(-B4, B4+1):
            c0 = (-(c1*xR + c2*xR2)) % p
            if c0 > p//2:
                c0 -= p
            if abs(c0) > B4:
                continue
            total += 1
            s_ok = sq2
            d_ok = (c1*c1 - 4*c0*c2) in S
            sq_lead += s_ok
            str_disc += d_ok
            both += s_ok and d_ok
    return {"p_bits": pbits, "delta": delta, "p": p, "B": B, "B4": B4,
            "lattice_points_in_box": total,
            "square_leading": sq_lead, "structured_disc": str_disc,
            "both": both, "decompositions": len(dec),
            "S_size": len(S),
            "work_iters": (2*B4+1)**2,
            "ratio_both_over_sqlead": (both / sq_lead) if sq_lead else None}


if __name__ == "__main__":
    cfg = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {}
    out = []
    for pbits in cfg.get("pbits", [20, 22]):
        for delta in cfg.get("deltas", [0.08, 0.10, 0.12]):
            r = census(pbits, delta)
            if r:
                print(json.dumps(r), flush=True)
                out.append(r)
    json.dump(out, open(cfg.get("out", "idea3_census.json"), "w"), indent=1)
