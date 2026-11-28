#!/usr/bin/env python3
"""Independent certificates for genus4_followup_20261118 at p = 16411 (validator's arithmetic, jac.py).

For each curve seed (172, 180, 2640), from the committed instance row (b_wx.jsonl.gz at COMMIT):
  - f = random_f(16411, 4, cs) equals the row's f; f monic of degree 9 and squarefree (genus 4);
  - r prime (deterministic Miller-Rabin), order = h*r, h <= 2;
  - Hasse-Weil: lo <= h*r <= hi and r > hi - lo, so with G != 0, [r]G = 0, r prime: #J = h*r;
  - [order]D = 0 for 6 random divisors (validator's own random stream);
  - no automorphism (x, y) -> (a x + b, c y) over F_p other than a = 1, b = 0, c = +-1: the only
    rational automorphism of y^2 = f, f of odd degree 9 (one point at infinity, which every
    automorphism fixes, so x -> a x + b) is the hyperelliptic involution; Frobenius is the identity
    on J(F_p) for prime p.  So negation is the strongest symmetry (A = 2).
For every target k = 0..17 of the study (target seed cs + k*1000003):
  - G, t re-derived from instance_from's seeding; G valid, G != 0, [r]G = 0; T = [t]G (NAF chain);
    [t]G again as [t-k]G + [k]G; T valid and non-zero; 1 <= t < r.
Usage: certs.py REPO OUT_JSON [COMMIT]"""
import gzip, json, os, random, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from jac import Jac, random_f, instance_from, is_prime, hasse_weil_bounds, random_divisor, peval

REPO, OUT = sys.argv[1], sys.argv[2]
COMMIT = sys.argv[3] if len(sys.argv) > 3 else "6213ecbc0da0ed4917ab75805258201d6629b477"
P = 16411
raw = subprocess.run(["git", "-C", REPO, "show", f"{COMMIT}:research/genus4_followup_20261118/b_wx.jsonl.gz"],
                     check=True, capture_output=True).stdout
inst = {r["curve_seed"]: r for r in map(json.loads, gzip.decompress(raw).decode().splitlines())
        if r["kind"] == "instance"}
rng = random.Random(0x5EED_20261118)


def automorphisms(f, p):
    """All (a, b) with f(a x + b) = a^9 f(x) (f monic deg 9) and a^9 a square (c^2 = a^9)."""
    out = []
    f8 = f[8]
    inv9 = pow(9, p - 2, p)
    for a in range(1, p):
        # x^8 coefficient: a^8 (9 b + f8) = a^9 f8  ->  b = (a f8 - f8) / 9
        b = (a * f8 - f8) * inv9 % p
        # compare f(a x + b) with a^9 f(x) at a few points first, then fully
        a9 = pow(a, 9, p)
        if all(peval(f, (a * x + b) % p, p) == a9 * peval(f, x, p) % p for x in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29)):
            if all(peval(f, (a * x + b) % p, p) == a9 * peval(f, x, p) % p for x in range(p)):
                if pow(a9, (p - 1) // 2, p) == 1:
                    out.append((a, b))
    return out


def swap_automorphisms(f, p):
    """Rational roots e of f; for each, every c != 0 with x -> e + c/(x - e) (swapping infinity and e)
    mapping y^2 = f to y^2 = mu f with mu a non-zero square, i.e. (x - e)^10 f(e + c/(x - e)) = mu f(x).
    An F_p-rational automorphism that moves infinity must send it to a rational branch point (a rational
    root of f); with one rational root e it swaps infinity and e, so this and the affine scan cover all."""
    from jac import pmul, padd, trim
    out = []
    roots = [x for x in range(p) if peval(f, x, p) == 0]
    for e in roots:
        xe = ((-e) % p, 1)  # x - e
        pw = [(1,)]
        for _ in range(10):
            pw.append(pmul(pw[-1], xe, p))
        for c in range(1, p):
            lin = trim([(c - e * e) % p, e % p])  # e (x - e) + c
            F, lp = (), (1,)
            for i in range(10):
                F = padd(F, pmul(tuple(f[i] * z % p for z in lp), pw[10 - i], p), p)
                lp = pmul(lp, lin, p)
            if len(F) == 10 and F[9]:
                mu = F[9]
                if all(F[i] == mu * f[i] % p for i in range(10)) and pow(mu, (p - 1) // 2, p) == 1:
                    out.append((e, c))
    return roots, out


res = {"commit": COMMIT, "curves": {}, "targets": []}
allok = True
for cs in (172, 180, 2640):
    ir = inst[cs]
    f = random_f(P, 4, cs)
    J = Jac(P, f)
    h, r, order = ir["h"], ir["r"], ir["order"]
    lo, hi = hasse_weil_bounds(P, 4)
    st = rng.getrandbits(64)
    killed = []
    for _ in range(6):
        D, st = random_divisor(J, st)
        killed.append(J.is_zero(J.mul(D, order)) and not J.is_zero(D))
    auts = automorphisms(f, P)
    roots, swaps = swap_automorphisms(f, P)
    chk = {"f_matches_row": f == ir["f"], "deg9_monic": len(f) == 10 and f[-1] == 1,
           "squarefree_genus4": J.squarefree and J.g == 4, "r_prime": is_prime(r), "order_eq_h_r": order == h * r,
           "h_le_2": h <= 2, "order_in_hasse_weil": lo <= order <= hi, "r_gt_interval_width": r > hi - lo,
           "order_kills_6_random_divisors": all(killed),
           "only_automorphisms_identity_and_involution": auts == [(1, 0)] and swaps == []}
    ok = all(chk.values())
    allok &= ok
    res["curves"][cs] = {"h": h, "r": r, "order": order, "hasse_weil": [lo, hi], "automorphisms_a_b": auts, "rational_roots": roots, "swap_automorphisms_e_c": swaps,
                         "checks": chk, "all_pass": ok}
    print("curve", cs, "PASS" if ok else "FAIL " + str([k for k, v in chk.items() if not v]), flush=True)
    for k in range(18):
        ts = cs + k * 1000003
        G, t = instance_from(J, h, r, ts)
        T = J.mul(G, t)
        kk = rng.randrange(1, r)
        tc = {"G_valid_nonzero": J.valid(G) and not J.is_zero(G), "rG_zero": J.is_zero(J.mul(G, r)),
              "tG_two_chains_agree": J.add(J.mul(G, (t - kk) % r), J.mul(G, kk)) == T,
              "T_valid_nonzero": J.valid(T) and not J.is_zero(T), "t_in_range": 1 <= t < r}
        tok = all(tc.values())
        allok &= tok
        res["targets"].append({"p": P, "curve_seed": cs, "k": k, "target_seed": ts, "t": t,
                               "G": [list(G[0]), list(G[1])], "T": [list(T[0]), list(T[1])],
                               "checks": tc, "all_pass": tok})
        print(" ", cs, k, ts, "t =", t, "PASS" if tok else "FAIL " + str([a for a, v in tc.items() if not v]), flush=True)
res["all_pass"] = allok
json.dump(res, open(OUT, "w"), indent=1)
print("targets:", len(res["targets"]), "all pass:", allok)
