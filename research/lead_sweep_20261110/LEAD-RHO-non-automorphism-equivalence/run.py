import json, sys, math, random
SMOKE = '--smoke' in sys.argv
BITS = 12 if SMOKE else 20
TRIALS = 20 if SMOKE else 400
HS = [1, 2, 4, 8, 16] if SMOKE else [1, 2, 3, 4, 6, 8, 12, 16, 24, 32]
SEED = 20261110

def isprime(n):
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

def prime_for(h, bits):
    n = (1 << bits) + 1
    while not (isprime(n) and (n-1) % h == 0): n += 1
    return n

def subgroup(N, h):
    if h == 1: return [1]
    if h == 2: return [1, N-1]
    for a in range(2, N):
        g = pow(a, (N-1)//h, N)
        if all(pow(g, h//q, N) != 1 for q in range(2, h+1) if h % q == 0 and isprime(q)):
            return [pow(g, i, N) for i in range(h)]

def steps_to_collision(N, H, rng, R=16):
    jumps = [rng.randrange(1, N) for _ in range(R)]
    x = rng.randrange(N)
    seen = set(); n = 0
    while True:
        c = min(h*x % N for h in H)
        if c in seen: return n
        seen.add(c); n += 1
        x = (x + jumps[c % R]) % N

def arm(h, rng):
    N = prime_for(h, BITS); H = subgroup(N, h)
    s = [steps_to_collision(N, H, rng) for _ in range(TRIALS)]
    m = sum(s)/len(s); v = sum((a-m)**2 for a in s)/(len(s)-1)
    return N, m, math.sqrt(v/len(s))

def main():
    out = {"lead": "LEAD-RHO-non-automorphism-equivalence", "seed": SEED, "bits": BITS,
           "trials": TRIALS, "smoke": SMOKE, "arms": {}}
    base = arm(1, random.Random(SEED))
    aa = arm(1, random.Random(SEED + 1))   # A/A: same arm, different seed
    out["aa_ratio"] = aa[1]/base[1]
    ok = True; bad_pos = False
    cgen = 1.5 * BITS  # group ops per non-trivial image h.x for generic H element
    for h in HS:
        N, m, se = arm(h, random.Random(SEED + 100 + h))
        ratio = m / base[1]
        pred = 1/math.sqrt(h)
        cstar = (1/ratio - 1)/(h-1) if h > 1 else None  # break-even image cost in steps
        cost_generic = ratio*(1+(h-1)*cgen) if h > 2 else ratio  # h=2 negation is free
        cost_free_img = ratio
        out["arms"][h] = {"N": N, "mean_steps": m, "step_ratio": ratio, "pred_ratio": pred,
                          "rel_err_vs_pred": ratio/pred-1, "breakeven_image_cost_in_steps": cstar,
                          "cost_ratio_generic_images": cost_generic,
                          "cost_ratio_free_images_null": cost_free_img}
        if h == 2 and not (0.6 < ratio < 0.85): bad_pos = True
        if h > 6:
            if cost_generic <= 1.0: ok = False
            if abs(ratio/pred - 1) > 0.25: ok = False
    out["controls"] = {"positive": "h=2 negation (free automorphism) step ratio near 0.707",
                       "null_detector": "free-image cost ratio equals step ratio (<1): detector sees gain",
                       "aa": "h=1 reseeded"}
    out["controls_ok"] = (not bad_pos) and abs(out["aa_ratio"]-1) < (0.3 if SMOKE else 0.1)
    out["verdict"] = ("inconclusive" if not out["controls_ok"] else
                      "consistent_with_prediction" if ok else "falsified")
    print(json.dumps(out))
main()
