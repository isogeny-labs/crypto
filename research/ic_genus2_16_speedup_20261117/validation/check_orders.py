#!/usr/bin/env python3
"""Check 2: group orders of every committed `kind: instance` record,
independently of genus_ladder.

Two independent sources of #J(F_p) per instance:
  (a) PARI/GP hyperellcharpoly (cypari2; PARI's own algorithm, unrelated to
      the Rust instrument): #J = chi_Frob(1).
  (b) The validator's own pure-Python/numpy code (hyperell.py):
      - N_k by enumeration of F_{p^k} for every k with p^k <= ENUM_LIMIT
        -> exact a_1..a_K of the L-polynomial;
      - the Cartier-Manin (Hasse-Witt) matrix -> every a_i mod p and #J mod p;
      - Cantor arithmetic: [#J_pari] D = 0 on random D, exact element orders,
        e' = lcm of orders (a proven divisor of the group exponent);
      - an own-only determination: the integers N in the Hasse-Weil window
        that are consistent with a_1..a_K (+ Weil bounds on the rest), with
        #J mod p, and divisible by e'.  If exactly one survives, #J is pinned
        by own code alone.
Then, against the record: order == #J ?  else r prime, r | #J, and
[order] D = 0 on the random D (order is an exponent multiple).

usage: venv/bin/python check_orders.py [--max-instances N] [--json OUT]
"""
import argparse
import glob
import json
import math
import os
import random
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from hyperell import (HCurve, charpoly_mod_p, count_points_ext, element_order,  # noqa: E402
                      factor, hasse_witt, is_prime, lpoly_from_counts)

import cypari2  # noqa: E402

PARI = cypari2.Pari()
PARI.allocatemem(1 << 30)  # hyperellcharpoly at p=211, g=6 overflows the 8 MB default stack
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
ENUM_LIMIT = 10_000_000
N_RANDOM = 8


def instances():
    paths = (glob.glob(os.path.join(ROOT, "P1/main/*.jsonl")) + glob.glob(os.path.join(ROOT, "P1/supp/*.jsonl"))
             + glob.glob(os.path.join(ROOT, "P2/crossover/*.jsonl")) + glob.glob(os.path.join(ROOT, "P2/crossover_supp/*.jsonl"))
             + glob.glob(os.path.join(ROOT, "P2/pilot_phi*/*.jsonl")))
    inst, conflicts, nrec = {}, [], 0
    for fn in sorted(paths):
        with open(fn) as fh:
            for line in fh:
                d = json.loads(line)
                if d["kind"] != "instance":
                    continue
                nrec += 1
                k = (d["p"], d["g"], d["curve_seed"])
                rec = {x: d[x] for x in ("p", "g", "curve_seed", "f", "order", "h", "r")}
                if k in inst and inst[k] != rec:
                    conflicts.append(k)
                inst.setdefault(k, rec)
    return inst, conflicts, nrec


def pari_charpoly(f, p):
    fx = " + ".join(f"{c}*x^{i}" for i, c in enumerate(f))
    pol = PARI(f"lift(hyperellcharpoly(Mod(1,{p})*({fx})))")
    deg = int(PARI.poldegree(pol))
    coeffs = [int(PARI.polcoef(pol, i)) for i in range(deg + 1)]  # low first
    return coeffs


def hw_window(p, g):
    # widened by 1 on each side against float rounding
    lo = math.floor((math.sqrt(p) - 1) ** (2 * g)) - 1
    hi = math.ceil((math.sqrt(p) + 1) ** (2 * g)) + 1
    return lo, hi


def own_candidates(p, g, a_exact, hw_a_modp, eprime):
    """Integers N consistent with own data.  a_exact = [1, a_1..a_K] exact;
    hw_a_modp = a_1..a_g mod p (from Hasse-Witt)."""
    K = len(a_exact) - 1
    lo, hi = hw_window(p, g)
    # interval for L(1) = sum_{i<g} a_i (1 + p^{g-i}) + a_g
    known = sum(a_exact[i] * ((1 + p ** (g - i)) if i < g else 1) for i in range(K + 1))
    slack = 0.0
    for i in range(K + 1, g + 1):
        bnd = math.comb(2 * g, i) * p ** (i / 2)
        slack += bnd * ((1 + p ** (g - i)) if i < g else 1)
    lo = max(lo, math.floor(known - slack))
    hi = min(hi, math.ceil(known + slack))
    # L(1) mod p from Hasse-Witt: L(1) = 1 + sum_{i=1}^{2g} a_i ; a_{g+j} = p^j a_{g-j} = 0 mod p
    l1modp = (1 + sum(hw_a_modp)) % p
    m = eprime * p // math.gcd(eprime, p)
    # N = 0 mod e', N = l1modp mod p  -> solve CRT if consistent
    sols = []
    if eprime % p == 0:
        if l1modp % p != 0:
            return lo, hi, []
        base = 0
    else:
        # N = e' * t, e' t = l1modp mod p
        t = (l1modp * pow(eprime, -1, p)) % p
        base = eprime * t
    first = base + ((lo - base + m - 1) // m) * m
    n = first
    while n <= hi:
        sols.append(n)
        n += m
        if len(sols) > 50:
            break
    return lo, hi, sols


def check(k, rec, rng):
    p, g, cs = k
    f, order, h, r = rec["f"], rec["order"], rec["h"], rec["r"]
    t0 = time.time()
    out = dict(p=p, g=g, curve_seed=cs, order=order, h=h, r=r)
    assert len(f) == 2 * g + 2 and f[-1] == 1
    chi = pari_charpoly(f, p)
    assert len(chi) == 2 * g + 1 and chi[-1] == 1
    J = sum(chi)
    out["J_pari"] = J
    a_pari = [chi[2 * g - i] for i in range(2 * g + 1)]  # a_0..a_2g of L(t)
    # functional equation check on PARI's output
    out["pari_funceq"] = all(a_pari[g + j] == p ** j * a_pari[g - j] for j in range(g + 1))
    # own point counts
    N = []
    kk = 1
    while kk <= g and p ** kk <= ENUM_LIMIT:
        N.append(count_points_ext(f, p, kk, rng=random.Random(kk)))
        kk += 1
    a_own, full = lpoly_from_counts(N, p, g)
    out["K_own"] = len(N)
    out["own_a_match_pari"] = all(a_own[i] == a_pari[i] for i in range(len(a_own)))
    if full is not None:
        out["J_own_full"] = sum(full)
    # own Hasse-Witt
    cp = charpoly_mod_p(hasse_witt(f, p, g), p)  # det(tI - A), low first
    hw_a = [cp[g - i] % p for i in range(1, g + 1)]
    out["hw_match_pari"] = all((a_pari[i] - hw_a[i - 1]) % p == 0 for i in range(1, g + 1))
    # own Cantor arithmetic
    C = HCurve(f, p)
    Jf = factor(J)
    out["J_factors"] = {str(q): e for q, e in sorted(Jf.items())}
    kills_J = kills_order = 0
    eprime = 1
    for _ in range(N_RANDOM):
        D = C.random_element(rng)
        assert C.is_valid(D)
        if C.is_zero(C.mul(J, D)):
            kills_J += 1
            o = element_order(C, D, J, list(Jf))
            eprime = eprime * o // math.gcd(eprime, o)
        if C.is_zero(C.mul(order, D)):
            kills_order += 1
    out["random_D"] = N_RANDOM
    out["J_kills_D"] = kills_J
    out["order_kills_D"] = kills_order
    out["e_lower"] = eprime
    out["exp_ratio_J_over_e"] = J // eprime if J % eprime == 0 else None
    # own-only determination
    lo, hi, sols = own_candidates(p, g, a_own, hw_a, eprime)
    out["own_window"] = [lo, hi]
    out["own_candidates"] = len(sols) if len(sols) <= 50 else ">50"
    out["own_unique_equals_pari"] = (len(sols) == 1 and sols[0] == J)
    out["pari_in_own_candidates"] = J in sols
    # record checks
    out["order_eq_J"] = order == J
    out["order_eq_hr"] = order == h * r
    out["r_prime"] = is_prime(r)
    out["r_divides_J"] = J % r == 0
    out["r2_divides_J"] = J % (r * r) == 0
    out["order_div_J"] = J % order == 0
    out["J_over_order"] = (J / order)
    out["secs"] = round(time.time() - t0, 1)
    if out["order_eq_J"]:
        out["verdict"] = "PASS(order=#J)"
    elif out["r_prime"] and out["r_divides_J"] and kills_order == N_RANDOM and eprime and order % eprime == 0:
        out["verdict"] = "PASS(order=exponent multiple; r prime, r|#J)"
    else:
        out["verdict"] = "FAIL"
    if not (out["pari_funceq"] and out["own_a_match_pari"] and out["hw_match_pari"] and kills_J == N_RANDOM):
        out["verdict"] = "FAIL(#J cross-check)"
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--max-instances", type=int, default=10 ** 9)
    ap.add_argument("--offset", type=int, default=0)
    ap.add_argument("--json", default=None)
    ap.add_argument("--only", default=None, help="p,g,curve_seed")
    args = ap.parse_args()
    inst, conflicts, nrec = instances()
    print(f"instance records: {nrec}; unique (p, g, curve_seed): {len(inst)}; conflicting duplicates: {conflicts}")
    print(f"PARI {PARI.version()}  cypari2")
    keys = sorted(inst, key=lambda k: (k[0] ** k[1], k))
    if args.only:
        want = tuple(int(x) for x in args.only.split(","))
        keys = [k for k in keys if k == want]
    results = []
    for k in keys[args.offset: args.offset + args.max_instances]:
        rng = random.Random(hash(k) & 0xFFFFFFFF)
        res = check(k, inst[k], rng)
        results.append(res)
        print(json.dumps(res), flush=True)
    if args.json:
        with open(args.json, "w") as fh:
            json.dump(results, fh, indent=1)
    n = len(results)
    print(f"\nchecked {n}/{len(inst)}")
    print(" order == #J:", sum(r["order_eq_J"] for r in results))
    print(" order != #J but exponent multiple, r prime, r | #J:",
          sum((not r["order_eq_J"]) and r["verdict"].startswith("PASS") for r in results))
    print(" FAIL:", [(r["p"], r["g"], r["curve_seed"], r["verdict"]) for r in results if not r["verdict"].startswith("PASS")])
    print(" #J pinned by own code alone (unique candidate == PARI):", sum(r["own_unique_equals_pari"] for r in results))
    print(" PARI #J inside own candidate set:", sum(r["pari_in_own_candidates"] for r in results))


if __name__ == "__main__":
    main()
