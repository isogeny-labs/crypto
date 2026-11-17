"""Idea 5: height-closed Kummer ladders -- measure the yield.

Criterion (from the Kummer identity): for a target R and a ladder-closed base,
x_R * x_{P-Q} must be a square over Q, so with x_R = u/v in lowest terms the
minimal admissible denominator height is

    h_R = max(sqf(u), sqf(v))

(the choice x_{P-Q} = sqf(u)/sqf(v) always works).  The route is useful only if
h_R is small -- polylog or p^c with small c -- for a positive density of
targets.  We measure the distribution of h_R on random targets.

If the fraction with h_R <= p^c collapses like p^{-2+o(1)}, the ladder base is
just the global-lift base and the route is closed.
"""
import json
import random
import sys

import sympy as sp

from lib.ec import Curve


def sqf(n: int) -> int:
    n = abs(n)
    if n in (0, 1):
        return n
    out = 1
    for q, e in sp.factorint(n).items():
        if e % 2:
            out *= q
    return out


def ratrec(x: int, p: int, bound: int):
    """u/v = x mod p with |u|,|v| <= bound (Wang)."""
    r0, r1, s0, s1 = p, x % p, 0, 1
    while r1 > bound:
        q = r0 // r1
        r0, r1 = r1, r0 - q*r1
        s0, s1 = s1, s0 - q*s1
    if r1 == 0:
        return None
    return r1, s1


def run(pbits, n_targets=300, seed=3, cuts=(0.05, 0.10, 0.15, 0.20, 0.25)):
    rng = random.Random(seed)
    p = int(sp.nextprime(2**pbits))
    E = Curve(p, 0, 7)
    bound = int(sp.integer_nthroot(p, 2)[0])
    hs = []
    for _ in range(n_targets):
        # random point R on the curve
        while True:
            x = rng.randrange(1, p)
            rhs = (x*x*x + 7) % p
            if rhs and pow(rhs, (p-1)//2, p) == 1:
                break
        uv = ratrec(x, p, bound)
        if uv is None:
            continue
        u, v = uv
        hs.append(max(sqf(u), sqf(v)))
    hs.sort()
    frac = {f"{c}": sum(1 for h in hs if h <= p**c)/len(hs) for c in cuts}
    return {"p_bits": pbits, "n": len(hs),
            "median_h_bits": sorted(h.bit_length() for h in hs)[len(hs)//2],
            "fraction_le_p^c": frac,
            "min_h_bits": min(h.bit_length() for h in hs),
            "max_h_bits": max(h.bit_length() for h in hs)}


if __name__ == "__main__":
    cfg = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {}
    out = []
    for pbits in cfg.get("pbits", [32, 40]):
        r = run(pbits, cfg.get("n", 300))
        print(r, flush=True)
        out.append(r)
    json.dump(out, open(cfg.get("out", "ladder_yield.json"), "w"), indent=1)
