#!/usr/bin/env python3
"""Toy proxy for LEAD-RHO-ecc2k130-iteration-function (stdlib only, deterministic)."""
import sys, json, random, math

def is_prime(n):
    if n < 2: return False
    for p in (2,3,5,7,11,13):
        if n % p == 0: return n == p
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

def prime_1mod(m, lo):
    n = lo - lo % m + 1
    while not (n >= lo and is_prime(n)): n += m
    return n

def mult_of_order(n, m, rng):
    while True:
        g = pow(rng.randrange(2, n-1), (n-1)//m, n)
        if all(pow(g, m//q, n) != 1 for q in range(2, m+1) if m % q == 0 and is_prime(q)) and pow(g,m,n)==1 and g!=1:
            return g

def trial(n, g, m, R, rng):
    """Walk on classes of Z_n under x->g^k x; table step; count steps to first revisit."""
    pw = [1]
    for _ in range(m-1): pw.append(pw[-1]*g % n)
    T = [rng.randrange(1, n) for _ in range(R)]
    canon = (lambda x: min(x*p % n for p in pw)) if m > 1 else (lambda x: x)
    x = canon(rng.randrange(n)); seen = {x}; steps = 0
    while True:
        x = canon((x + T[(x*2654435761 >> 7) % R]) % n); steps += 1
        if x in seen: return steps
        seen.add(x)

def arm(n, m, R, trials, seed):
    rng = random.Random(seed)
    g = mult_of_order(n, m, rng) if m > 1 else 1
    v = [trial(n, g, m, R, rng) for _ in range(trials)]
    mean = sum(v)/len(v)
    sd = math.sqrt(sum((a-mean)**2 for a in v)/(len(v)-1))
    return mean, sd/math.sqrt(len(v))

def main():
    smoke = "--smoke" in sys.argv
    size, trials, R = (1<<12, 40, 16) if smoke else (1<<18, 1200, 16)
    ms = (1, 2, 8)
    res = {}
    for m in ms:
        n = prime_1mod(8, size)
        res[f"m{m}"] = arm(n, m, R, trials, 1000+m)
    n = prime_1mod(8, size)
    aa = arm(n, 1, R, trials, 7777)  # A/A: same arm, new seed
    base, se0 = res["m1"]
    out = {"lead": "LEAD-RHO-ecc2k130-iteration-function", "smoke": smoke, "n": n,
           "table_size": R, "trials": trials, "seeds": {"m1":1001,"m2":1002,"m8":1008,"aa":7777},
           "mean_steps": {k: v[0] for k, v in res.items()}, "se": {k: v[1] for k, v in res.items()},
           "positive_control_theory_m1_sqrt_pi_n_over_2": math.sqrt(math.pi*n/2),
           "aa_ratio": aa[0]/base}
    ratios = {k: res[k][0]/base for k in res}
    out["ratio_to_m1"] = ratios
    out["predicted_ratio"] = {"m2": 1/math.sqrt(2), "m8": 1/math.sqrt(8)}
    out["rel_dev"] = {k: ratios[k]/out["predicted_ratio"][k]-1 for k in ("m2","m8")}
    if smoke:
        v = "inconclusive"
    else:
        aa_ok = abs(out["aa_ratio"]-1) < 0.10
        pos_ok = abs(base/out["positive_control_theory_m1_sqrt_pi_n_over_2"]-1) < 0.25
        if not (aa_ok and pos_ok): v = "inconclusive"
        elif abs(out["rel_dev"]["m8"]) <= 0.15 and abs(out["rel_dev"]["m2"]) <= 0.15: v = "consistent_with_prediction"
        else: v = "falsified"  # two-sided: class reduction misses ~1/sqrt(m)
    out["verdict"] = v
    print(json.dumps(out))

main()
