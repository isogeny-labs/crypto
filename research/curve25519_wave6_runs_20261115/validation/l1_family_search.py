#!/usr/bin/env python3
"""Validator, independent of checks/faithful_full.py: for given primes p, scan A = 6, 10, ..., < AMAX (A = 2 mod 4)
and apply the lab's full family rule (engine: build_miniature_at / miniature_for_a):
  A^2 - 4 non-square mod p; #E = 8 r, r prime, r > 2^(k-3) (k = bit length of p); #E' = 2p + 2 - #E = 4 r', r' prime.
The quadratic character is tabulated from powers of a primitive root (chi(w^i) = (-1)^i), not from squares, and
#E = p + 1 + sum_u chi(u^3 + A u^2 + u) (Montgomery curve y^2 = u^3 + A u^2 + u, B = 1).
Also reports the A that pass every condition except the size bound on r, and the first-seven shape list.
Usage: l1_family_search.py <out.json> <AMAX> <time_budget_s> p1 [p2 ...]"""
import json, sys, time
import numpy as np

def is_prime_td(n):  # deterministic trial division (n < 2^32 here)
    if n < 2: return False
    if n % 2 == 0: return n == 2
    f = 3
    while f * f <= n:
        if n % f == 0: return False
        f += 2
    return True

def factor_td(n):
    out, f = [], 2
    while f * f <= n:
        while n % f == 0: out.append(f); n //= f
        f += 1
    if n > 1: out.append(n)
    return out

def prim_root(p):
    qs = sorted(set(factor_td(p - 1)))
    g = 2
    while any(pow(g, (p - 1) // q, p) == 1 for q in qs): g += 1
    return g

def chi_table(p):
    g = prim_root(p); B = 4096
    small = np.empty(B, dtype=np.int64); v = 1
    for i in range(B): small[i] = v; v = v * g % p
    nb = (p - 1 + B - 1) // B; big = np.empty(nb, dtype=np.int64); gb = pow(g, B, p); v = 1
    for j in range(nb): big[j] = v; v = v * gb % p
    pw = (np.outer(big, small) % p).ravel()[: p - 1]          # w^i, i = 0 .. p-2
    chi = np.zeros(p, dtype=np.int8)
    chi[pw] = np.where(np.arange(p - 1) % 2 == 0, 1, -1).astype(np.int8)
    assert np.count_nonzero(chi) == p - 1 and int(chi.sum()) == 0
    return chi, g

def scan(p, amax, budget):
    k = p.bit_length(); chi, g = chi_table(p)
    u = np.arange(p, dtype=np.int64); u2 = u * u % p; c = (u2 * u % p + u) % p
    full, no_rbound, tested, nonsq = [], [], 0, 0; t0 = time.time(); last_a = None; complete = True
    for A in range(6, amax, 4):
        if time.time() - t0 > budget: complete = False; break
        last_a = A; tested += 1
        if pow((A * A - 4) % p, (p - 1) // 2, p) != p - 1: continue
        nonsq += 1
        s = int(chi[(c + A * u2) % p].sum(dtype=np.int64)); n = p + 1 + s; nt = 2 * p + 2 - n
        if n % 8 == 0 and nt % 4 == 0 and is_prime_td(n // 8) and is_prime_td(nt // 4):
            row = {"A": A, "n": n, "r": n // 8, "nt": nt, "r_twist": nt // 4, "r_gt_2^(k-3)": n // 8 > (1 << (k - 3))}
            no_rbound.append(row)
            if row["r_gt_2^(k-3)"]: full.append(row)
    return {"p": p, "k": k, "prim_root": g, "A_range": [6, amax - 1], "A_tested": tested, "A_nonsquare_disc": nonsq,
            "last_A_tested": last_a, "complete": complete, "seconds": round(time.time() - t0, 1),
            "full_rule_hits": full, "hits_without_r_bound": no_rbound}

def first_shape_primes(k, count):
    out, c = [], 1
    while len(out) < count:
        p = 2 ** k - c
        if p % 8 == 5 and (p - 1) % 60 == 0 and is_prime_td(p) and is_prime_td((p - 1) // 60):
            out.append((p, c))
        c += 2
    return out

if __name__ == "__main__":
    out_path, amax, budget = sys.argv[1], int(sys.argv[2]), float(sys.argv[3]); ps = [int(x) for x in sys.argv[4:]]
    res = {"first_seven_k20_shape_primes (p = 60 C + 1, p and C prime, p = 5 mod 8, scanning down from 2^20)":
           [{"p": p, "c": c, "p_minus_1": factor_td(p - 1)} for p, c in first_shape_primes(20, 7)], "scans": []}
    T0 = time.time()
    for p in ps:
        left = budget - (time.time() - T0)
        if left <= 0: res["scans"].append({"p": p, "skipped": "time budget"}); continue
        r = scan(p, amax, left); res["scans"].append(r)
        print(p, "complete" if r["complete"] else "PARTIAL", r["A_tested"], "full-rule hits", [h["A"] for h in r["full_rule_hits"]],
              "hits without r bound", len(r["hits_without_r_bound"]), r["seconds"], "s", flush=True)
    json.dump(res, open(out_path, "w"), indent=1)
