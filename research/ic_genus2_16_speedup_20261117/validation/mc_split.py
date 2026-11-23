#!/usr/bin/env python3
"""Check 6: the core mechanism, independent of genus_ladder.

For a committed instance, estimate the fraction of reduced degree-g divisors
whose u splits into degree-1 factors (1-smooth) versus into factors of degree
<= 2 (2-smooth), and compare the ratio with the producer's measured
trials-per-relation ratio between arms C_double_lp+pre and D_deg2_full.

Two estimates, both with the validator's own code (hyperell.py):
  (MC)    walk-style sampling: a random start D_0 and D_{i+1} = D_i + B_{j(i)}
          over a table of 32 random elements B_j (random sums of prime
          divisors), j(i) uniform; classify u of each D_i with deg u = g.
  (exact) the exact counts over all reduced divisors of degree g, from place
          counts: with n1 (resp. z1) the number of a in F_p with f(a) a nonzero
          square (resp. zero), and n2, z2 the same for monic irreducible
          quadratics,
            #1-smooth  = [t^g] ((1+t)/(1-t))^n1 (1+t)^z1
            #2-smooth  = [t^g] ((1+t)/(1-t))^n1 (1+t)^z1 ((1+t^2)/(1-t^2))^n2 (1+t^2)^z2
          (squarefree variants: (1+2t)^n1 ... (1+2t^2)^n2), and the total
            #deg-g    = [t^g] L(t) (1 - p t^2) / (1 - p t),
          with L(t) from the validator's own point counts when K = g, else
          from PARI (flagged).

usage: python3 mc_split.py G P CURVE_SEED [--samples N]
"""
import argparse
import glob
import json
import math
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from hyperell import (HCurve, count_points_ext, is_irreducible, lpoly_from_counts,  # noqa: E402
                      pdivmod, peval, pgcd, ppowmod, psub, trim)

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)


def series_mul(a, b, n):
    out = [0] * (n + 1)
    for i, x in enumerate(a[: n + 1]):
        if x:
            for j, y in enumerate(b[: n + 1 - i]):
                out[i + j] += x * y
    return out


def series_pow(a, e, n):
    r = [1] + [0] * n
    for _ in range(e):
        r = series_mul(r, a, n)
    return r


def geom_ratio(d, n, sq_free):
    """(1+t^d)/(1-t^d) truncated, or 1+2t^d if squarefree only."""
    s = [0] * (n + 1)
    s[0] = 1
    if sq_free:
        if d <= n:
            s[d] = 2
        return s
    k = d
    while k <= n:
        s[k] = 2
        k += d
    return s


def smoothness(u, p, frob1, frob2):
    """(is 1-smooth, is 2-smooth, squarefree) for monic u (deg >= 1)."""
    du = trim([(i * c) % p for i, c in enumerate(u)][1:])
    sqf = len(pgcd(u, du, p)) == 1

    def strip(v, frob_poly_fn):
        while len(v) > 1:
            h = frob_poly_fn(v)
            d = pgcd(v, psub(h, [0, 1], p), p)
            if len(d) == 1:
                return False
            v = pdivmod(v, d, p)[0]
        return True

    s1 = strip(list(u), lambda v: ppowmod([0, 1], p, v, p))
    s2 = s1 or strip(list(u), lambda v: ppowmod([0, 1], p * p, v, p))
    return s1, s2, sqf


def exact_counts(f, p, g, Lpoly):
    n1 = z1 = 0
    for a in range(p):
        y = peval(f, a, p)
        if y == 0:
            z1 += 1
        elif pow(y, (p - 1) // 2, p) == 1:
            n1 += 1
    n2 = z2 = 0
    for q0 in range(p):
        for q1 in range(p):
            q = [q0, q1, 1]
            if not is_irreducible(q, p):
                continue
            fr = pdivmod(f, q, p)[1]
            if not fr:
                z2 += 1
                continue
            # chi over F_{p^2} = F_p[x]/q
            if ppowmod(fr, (p * p - 1) // 2, q, p) == [1]:
                n2 += 1
    res = {"n1": n1, "z1": z1, "n2": n2, "z2": z2}
    for sq in (False, True):
        one = series_mul(series_pow(geom_ratio(1, g, sq), n1, g), series_pow([1, 1], z1, g), g)
        two = series_mul(one, series_mul(series_pow(geom_ratio(2, g, sq), n2, g),
                                         series_pow([1, 0, 1], z2, g), g), g)
        tag = "sqf" if sq else "any"
        res[f"cnt1_{tag}"] = one[g]
        res[f"cnt2_{tag}"] = two[g]
    # total number of degree-g reduced divisors: [t^g] L(t)(1 - p t^2)/(1 - p t)
    num = series_mul(Lpoly, [1, 0, -p], g)
    geo = [p ** i for i in range(g + 1)]
    A = series_mul(num, geo, g)
    res["A_g"] = A[g]
    res["sum_A_le_g"] = sum(A[: g + 1])
    return res


def producer_tpr(g, p, cs):
    """trials/relations per seed for arms c and D in the committed JSONL."""
    out = {}
    for fn in glob.glob(os.path.join(ROOT, "P1/main/*.jsonl")) + glob.glob(os.path.join(ROOT, "P1/supp/*.jsonl")):
        with open(fn) as fh:
            for line in fh:
                d = json.loads(line)
                if d["kind"] == "run" and d["g"] == g and d["p"] == p and d["curve_seed"] == cs:
                    if d["arm"] in ("C_double_lp+pre", "D_deg2_full", "E_deg2_lp"):
                        out[(d["arm"], d["seed"])] = (d["trials"], d["relations"], d["rels_by_lp"], d["exhausted"])
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("g", type=int)
    ap.add_argument("p", type=int)
    ap.add_argument("curve_seed", type=int)
    ap.add_argument("--samples", type=int, default=200000)
    ap.add_argument("--seed", type=int, default=7)
    args = ap.parse_args()
    g, p, cs = args.g, args.p, args.curve_seed
    f = None
    for fn in glob.glob(os.path.join(ROOT, "P1/main/*.jsonl")):
        with open(fn) as fh:
            for line in fh:
                d = json.loads(line)
                if d["kind"] == "instance" and (d["g"], d["p"], d["curve_seed"]) == (g, p, cs):
                    f = d["f"]
    assert f is not None, "instance not found"
    C = HCurve(f, p)
    rng = random.Random(args.seed)
    # L(t): own point counts if p^g is small enough, else PARI (flagged)
    if p ** g <= 10_000_000:
        N = [count_points_ext(f, p, k) for k in range(1, g + 1)]
        _, Lfull = lpoly_from_counts(N, p, g)
        src = "own point counts"
    else:
        import cypari2
        P = cypari2.Pari()
        fx = " + ".join(f"{c}*x^{i}" for i, c in enumerate(f))
        pol = P(f"lift(hyperellcharpoly(Mod(1,{p})*({fx})))")
        chi = [int(P.polcoef(pol, i)) for i in range(2 * g + 1)]
        Lfull = [chi[2 * g - i] for i in range(2 * g + 1)]
        src = "PARI hyperellcharpoly"
    ex = exact_counts(f, p, g, Lfull)
    print(f"instance g={g} p={p} curve={cs}; L(t) from {src}; L(1)={sum(Lfull)}; "
          f"sum_(n<=g) A_n = {ex['sum_A_le_g']} (must equal L(1))")
    print(f"exact: n1={ex['n1']} z1={ex['z1']} n2={ex['n2']} z2={ex['z2']} A_g={ex['A_g']}")
    for tag in ("any", "sqf"):
        P1e, P2e = ex[f"cnt1_{tag}"] / ex["A_g"], ex[f"cnt2_{tag}"] / ex["A_g"]
        print(f"exact ({tag}): P(1-smooth)={P1e:.5g}  P(2-smooth)={P2e:.5g}  ratio={P2e / P1e:.4g}")

    # Monte Carlo, walk-style
    table = [C.random_element(rng, parts=4) for _ in range(32)]
    D = C.random_element(rng, parts=4)
    n = ng = c1 = c2 = c1s = c2s = 0
    for _ in range(args.samples):
        D = C.add(D, table[rng.randrange(32)])
        n += 1
        u = D[0]
        if len(u) - 1 != g:
            continue
        ng += 1
        s1, s2, sqf = smoothness(u, p, None, None)
        c1 += s1
        c2 += s2
        c1s += s1 and sqf
        c2s += s2 and sqf
    print(f"MC: {n} walk samples, {ng} with deg u = g")
    for tag, a, b in (("any", c1, c2), ("sqf", c1s, c2s)):
        P1m, P2m = a / ng, b / ng
        se1 = math.sqrt(a) / ng
        print(f"MC ({tag}): 1-smooth {a} ({P1m:.5g} +- {se1:.2g})  2-smooth {b} ({P2m:.5g})  "
              f"ratio={(P2m / P1m) if a else float('inf'):.4g}")
    tpr = producer_tpr(g, p, cs)
    print("producer records (arm, seed): (trials, relations, rels_by_lp, exhausted)")
    for k in sorted(tpr):
        print("  ", k, tpr[k])
    cs_ = [tpr[k][0] / tpr[k][1] for k in tpr if k[0] == "C_double_lp+pre"]
    ds_ = [tpr[k][0] / tpr[k][1] for k in tpr if k[0] == "D_deg2_full"]
    if cs_ and ds_:
        gmc = math.exp(sum(map(math.log, cs_)) / len(cs_))
        gmd = math.exp(sum(map(math.log, ds_)) / len(ds_))
        print(f"producer trials/relation: c={gmc:.4g} D={gmd:.4g} ratio c/D={gmc / gmd:.4g}")


if __name__ == "__main__":
    main()
