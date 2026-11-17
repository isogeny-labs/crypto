#!/usr/bin/env python3
"""LEAD-ICB-retained-setup-online toy: S*T^2/r for generic precomputed rho vs a non-generic online IC control."""
import json, random, sys, time, math

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def prime_near(n):
    while not is_prime(n): n += 1
    return n

NJ = 32
def mkwalk(r, rng):
    return [rng.randrange(1, r) for _ in range(NJ)]

def hsh(x):
    return ((x * 2654435761) ^ (x >> 7)) & 0xFFFFFFFF

def generic_cell(r, S, W, trials, seed):
    """Generic group Z_r (P=1). Walk x -> x + m[h(x)%NJ]; state tracks only (x, a). Group-law only, no division."""
    rng = random.Random(seed)
    m = mkwalk(r, rng)
    isdp = lambda x: hsh(x) % W == 0
    cap = 20 * W
    table = {}
    tries = 0
    while len(table) < S and tries < 50*S:
        tries += 1
        a = rng.randrange(r); x = a
        for _ in range(cap):
            j = hsh(x) % NJ
            x = (x + m[j]) % r; a = (a + m[j]) % r
            if x % 1 == 0 and isdp(x):
                table.setdefault(x, a); break
    steps_total = 0; solved = 0
    for _ in range(trials):
        k = rng.randrange(1, r); Q = k  # secret known only to check
        ok = False
        while not ok and steps_total < 4000 * W + 10**7:
            c = rng.randrange(r); x = (Q + c) % r; a = c
            for _ in range(cap):
                j = hsh(x) % NJ
                x = (x + m[j]) % r; a = (a + m[j]) % r
                steps_total += 1
                if isdp(x):
                    if x in table:
                        ka = (table[x] - a) % r  # table: x=aP ; online: x=Q+aP
                        if ka == k: ok = True
                    break
        solved += ok
    T = steps_total / max(solved, 1)
    return {"r": r, "S": len(table), "W": W, "T": T, "solved": solved, "ratio": len(table) * T * T / r}

def ic_control(logp, trials, seed, B):
    """Non-generic online IC in Z_p^*: retained logs of B small primes, T charged = smoothness trials (1 op each)."""
    rng = random.Random(seed)
    p = prime_near(1 << logp)
    # generator
    fac = []; n = p-1; d = 2
    while d*d <= n:
        if n % d == 0:
            fac.append(d)
            while n % d == 0: n//=d
        d += 1
    if n > 1: fac.append(n)
    g = next(c for c in range(2, p) if all(pow(c, (p-1)//f, p) != 1 for f in fac))
    dl = {}; x = 1
    for i in range(p-1):
        dl[x] = i; x = x*g % p
    fb = [q for q in range(2, 10**4) if is_prime(q)][:B]
    L = {q: dl[q] for q in fb}  # uncharged setup
    del dl
    steps = 0; solved = 0
    for _ in range(trials):
        k = rng.randrange(1, p-1); h = pow(g, k, p); got = False
        while not got:
            steps += 1
            e = rng.randrange(p-1); y = h * pow(g, e, p) % p
            t = y; s = 0
            for q in fb:
                while t % q == 0: t//=q; s += L[q] * 0 + L[q]
            if t == 1:
                if (s - e) % (p-1) == k: got = True
                else: raise RuntimeError("log mismatch")
        solved += 1
    T = steps / solved; r = p-1
    return {"p": p, "S": len(fb), "T": T, "ratio": len(fb) * T * T / r}

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    if smoke:
        cfgs = [(1<<12, 16, 16)]; trials = 5; icp = (12, 20, 5)
    else:
        cfgs = []
        for lr in (14, 16, 18):
            r = prime_near(1 << lr)
            for sf in (0.5, 1.0, 2.0):
                S = max(4, int(sf * 2 ** (lr/3)))
                W = max(2, int(0.5 * math.sqrt(r / S)))
                cfgs.append((r, S, W))
        trials = 60; icp = (18, 150, 40)
    seeds = {"A": 1001, "A2": 2002, "ic": 3003}
    rows = []
    for (r, S, W) in cfgs:
        a = generic_cell(r, S, W, trials, seeds["A"] + r)
        b = generic_cell(r, S, W, trials, seeds["A2"] + r)
        rows.append({"A": a, "A2": b})
        if time.time() - t0 > 420: break
    ic = ic_control(icp[0], icp[2], seeds["ic"], icp[1])
    gm = min(min(x["A"]["ratio"], x["A2"]["ratio"]) for x in rows)
    aa = max(max(x["A"]["ratio"], x["A2"]["ratio"]) / min(x["A"]["ratio"], x["A2"]["ratio"]) for x in rows)
    if smoke: verdict = "inconclusive"
    elif gm < 0.25: verdict = "falsified"
    elif gm >= 0.5 and ic["ratio"] <= 0.25 and aa <= 2.0: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICB-retained-setup-online", "smoke": smoke, "seeds": seeds,
        "params": {"cfgs": cfgs, "trials": trials, "ic": icp, "NJ": NJ},
        "generic_rows": rows, "ic_positive_control": ic,
        "stat_generic_min_ratio": gm, "stat_aa_max_ratio_of_ratios": aa,
        "thresholds": {"generic_min_ge": 0.5, "falsify_below": 0.25, "ic_le": 0.25, "aa_le": 2.0},
        "elapsed_s": time.time() - t0, "verdict": verdict}))
main()
