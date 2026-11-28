import json
import math
import random
from fractions import Fraction

PRIMES = [101, 1009, 10007]
DEGREES = [1, 2, 3, 4, 5, 8, 9, 10, 11]
MC_P = 101
MC_DEGREES = [1, 2, 3]
MC_TRIALS = 200000
MC_SEEDS = [20261116, 70261116]


def splitfrac(p, d):
    return Fraction(math.comb(p, d), p ** d)


def eval_poly(coeffs, x, p):
    """f(x) = x^d + c[d-1] x^(d-1) + ... + c[0]; coeffs = [c0..c_{d-1}] (non-leading)."""
    acc = 1
    for c in reversed(coeffs):
        acc = (acc * x + c) % p
    return acc % p


def mc_count(p, d, seed, trials):
    rng = random.Random(seed * 1000 + d)
    n = 0
    for _ in range(trials):
        c = [rng.randrange(p) for _ in range(d)]
        roots = 0
        for x in range(p):
            if eval_poly(c, x, p) == 0:
                roots += 1
        if roots == d:
            n += 1
    return n


def emit(o):
    print(json.dumps(o))


def main():
    for p in PRIMES:
        for d in DEGREES:
            cpd = math.comb(p, d)
            frac = Fraction(cpd, p ** d)
            emit({"kind": "exact", "p": p, "d": d, "C_p_d": cpd, "p_pow_d": p ** d,
                  "splitfrac_num": frac.numerator, "splitfrac_den": frac.denominator,
                  "splitfrac_dec": float(frac),
                  "inv_d_factorial_dec": float(Fraction(1, math.factorial(d)))})
    for p in PRIMES:
        for g in (4, 5):
            ratio = splitfrac(p, g) / splitfrac(p, 2 * g + 1)
            emit({"kind": "penalty", "p": p, "g": g, "d_hyper": g, "d_prym": 2 * g + 1,
                  "ratio_num": ratio.numerator, "ratio_den": ratio.denominator,
                  "ratio_dec": float(ratio)})
    for d in MC_DEGREES:
        for seed in MC_SEEDS:
            n = mc_count(MC_P, d, seed, MC_TRIALS)
            emit({"kind": "mc", "p": MC_P, "d": d, "seed": seed, "derived_seed": seed * 1000 + d,
                  "trials": MC_TRIALS, "n_complete": n,
                  "frac_mc_dec": float(Fraction(n, MC_TRIALS)),
                  "exact_dec": float(splitfrac(MC_P, d))})


if __name__ == "__main__":
    main()
