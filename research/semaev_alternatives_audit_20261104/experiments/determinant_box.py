"""Idea 18: is the determinant method even non-empty in the box?

C_R : S_3(x1,x2,x_R) = 0.  For each x1 in [1,B] solve the quadratic in x2 and
count roots that also lie in [1,B].  The determinant method needs enough box
points to form a nontrivial vanishing determinant; at the decomposition scale
B = p^(1/5) there should be none.

Reports the count at B = p^(1/5) (decomposition scale) and at B = p^(1/2)
(where the curve meets the box), for several primes.
"""
import json
import random
import sys

import sympy as sp

from lib.ec import Curve


def count_box(pbits, B, seed=7, a=0, b=7):
    rng = random.Random(seed)
    p = int(sp.nextprime(2**pbits))
    E = Curve(p, a, b)
    base = E.points_with_x_upto(B, rng)
    xR = base[0][0] if base else rng.randrange(1, p)
    cnt = 0
    for x1 in range(1, B+1):
        A2 = (xR*xR - 2*xR*x1 + x1*x1) % p
        B2 = (-2*x1*xR*xR - 2*xR*(x1*x1 + a) + 2*a*x1 - 4*b) % p
        C2 = (x1*x1*xR*xR - 2*xR*(a*x1 + 2*b) + a*a - 4*b*x1) % p
        if A2 % p == 0:
            if B2 % p == 0:
                continue
            r = (-C2 * pow(B2, -1, p)) % p
            if 1 <= r <= B:
                cnt += 1
            continue
        disc = (B2*B2 - 4*A2*C2) % p
        root = sp.sqrt_mod(disc, p, all_roots=False)
        if root is None:
            continue
        inv2 = pow(2*A2 % p, -1, p)
        for sg in (1, -1):
            r = ((-B2 + sg*int(root)) * inv2) % p
            if 1 <= r <= B:
                cnt += 1
    return {"p_bits": pbits, "B": B, "box_points": cnt, "B2_over_p": round(B*B/p, 4)}


if __name__ == "__main__":
    cfg = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {}
    out = []
    for pbits in cfg.get("pbits", [20, 24, 28]):
        p = int(sp.nextprime(2**pbits))
        for delta in (0.2, 0.5):
            r = count_box(pbits, max(4, int(round(p**delta))))
            print(r, flush=True); out.append(r)
    json.dump(out, open(cfg.get("out", "determinant_box.json"), "w"), indent=1)
