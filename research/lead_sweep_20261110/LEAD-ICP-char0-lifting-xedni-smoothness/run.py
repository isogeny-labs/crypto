import sys, json, random, math
SMOKE = "--smoke" in sys.argv
N = 3000 if SMOKE else 150000
B = 300
PRIMES = [q for q in range(2, B+1) if all(q % r for r in range(2, int(q**.5)+1))]
SEEDS = {"point": 101, "null": 202, "aa": 303, "planted": 404}
PS = [65521, 1048573, 4194301]  # primes near 2^16, 2^20, 2^22

def smooth(n):
    for q in PRIMES:
        while n % q == 0:
            n //= q
        if n == 1:
            return True
    return n == 1

def is_prime(n):
    return n > 1 and all(n % r for r in range(2, int(n**.5)+1))

def lift_x_point(rng, p):
    a, b = rng.randrange(p), rng.randrange(p)
    while True:
        x = rng.randrange(p)
        r = (x**3 + a*x + b) % p
        if r and pow(r, (p-1)//2, p) == 1:
            return x

def arm(kind, seed, n):
    rng = random.Random(seed); hits = 0; tot = 0
    for i in range(n):
        p = PS[i % 3]
        k = rng.randrange(1, 64)            # selected-lift offset: x + k p
        if kind == "point":
            x = lift_x_point(rng, p)
        elif kind == "planted":
            x = 1
            while x * 2 < p:
                x *= rng.choice(PRIMES[:20])
            k = 0  # planted: smooth integer lift itself
        else:
            x = rng.randrange(p)
        hits += smooth(x + k * p) ; tot += 1
    return hits, tot

def rate_ci(h, t):
    r = h / t; se = math.sqrt(max(r*(1-r), 1e-12) / t); return r, se

res = {k: arm(k, SEEDS[k], N) for k in SEEDS}
r = {k: rate_ci(*v) for k, v in res.items()}
def ratio(a, b):
    ra, sa = r[a]; rb, sb = r[b]
    q = ra / rb if rb else float("nan")
    se = q * math.sqrt((sa/ra)**2 + (sb/rb)**2) if ra and rb else float("nan")
    return q, se
pq, pse = ratio("point", "null"); aq, ase = ratio("aa", "null"); lq, lse = ratio("planted", "null")
ctrl_ok = abs(aq - 1) < 3 * ase + 1e-9 and lq > 3
if SMOKE:
    verdict = "inconclusive"
elif not ctrl_ok:
    verdict = "inconclusive"
elif pq - 3*pse > 1.10:
    verdict = "falsified"
elif abs(pq - 1) <= 3 * pse + 0.03:
    verdict = "consistent_with_prediction"
else:
    verdict = "inconclusive"
print(json.dumps({"lead": "LEAD-ICP-char0-lifting-xedni-smoothness", "smoke": SMOKE,
  "statistic": "smooth-rate ratio point-lift x+kp vs uniform null", "ratio": pq, "se": pse,
  "aa_ratio": aq, "aa_se": ase, "positive_control_ratio": lq, "controls_ok": ctrl_ok,
  "rates": {k: v[0] for k, v in r.items()}, "seeds": SEEDS,
  "params": {"N_per_arm": N, "B": B, "primes": PS, "k_range": [1, 63]}, "verdict": verdict}))
