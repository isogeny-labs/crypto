#!/usr/bin/env python3
"""Toy: cost of transporting a precomputed x-table across an odd l-isogeny (Velu) vs regenerating it.
Stdlib only, deterministic, one core."""
import json, random, sys, time
SMOKE = "--smoke" in sys.argv
SEED = 20261110
def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True
def nextprime(n):
    while not isprime(n): n += 1
    return n
class C:  # field op counter
    M = 0; I = 0
def inv(a, p): C.I += 1; return pow(a, -1, p)
def mul(a, b, p): C.M += 1; return a * b % p
def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        lam = mul(3 * mul(x1, x1, p) + a, inv(2 * y1 % p, p), p)
    else:
        lam = mul((y2 - y1) % p, inv((x2 - x1) % p, p), p)
    x3 = (mul(lam, lam, p) - x1 - x2) % p
    return (x3, (mul(lam, (x1 - x3) % p, p) - y1) % p)
def smul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R
def order_count(a, b, p, sq):
    n = 1
    for x in range(p):
        r = (x * x * x + a * x + b) % p
        n += 1 if r == 0 else (2 if r in sq else 0)
    return n
def rand_point(a, b, p, rng):
    while True:
        x = rng.randrange(p); r = (x**3 + a*x + b) % p
        if r == 0: continue
        if pow(r, (p-1)//2, p) == 1:
            # p = 3 mod 4 assumed
            return (x, pow(r, (p+1)//4, p))
def find_curve(l, p, sq, rng, tries=4000):
    for _ in range(tries):
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        n = order_count(a, b, p, sq)
        if n % l == 0 and isprime(n // l) and n // l > 60 and n // l != l:
            q = n // l
            while True:
                R = rand_point(a, b, p, rng)
                K = smul(q, R, a, p)
                if K is not None:
                    P = smul(l, R, a, p)  # order q (R has order dividing l*q)
                    if P is not None and smul(q, P, a, p) is None:
                        return a, b, q, K, P
    return None
def velu(a, b, K, l, p):
    S = []; Q = K
    for _ in range((l - 1) // 2):
        S.append(Q); Q = add(Q, K, a, p)
    # drop counted setup ops: precompute constants
    t = w = 0; terms = []
    for (xq, yq) in S:
        tq = 2 * (3*xq*xq + a) % p; uq = 4*yq*yq % p
        terms.append((xq, tq, uq)); t += tq; w += uq + xq*tq
    return (a - 5*t) % p, (b - 7*w) % p, terms
def velu_x(x, terms, p):
    r = x
    for (xq, tq, uq) in terms:
        d = inv((x - xq) % p, p); d2 = mul(d, d, p)
        r = (r + mul(tq, d, p) + mul(uq, d2, p)) % p; C.M += 0
    return r
def run_l(l, p, sq, seed, T):
    rng = random.Random(seed)
    cur = find_curve(l, p, sq, rng)
    if cur is None: return None
    a, b, q, K, P = cur
    a2, b2, terms = velu(a, b, K, l, p)
    # table on E: x(kP), k=1..T (build via repeated add; not charged to either arm)
    pts = []; R = P
    for _ in range(T): pts.append(R); R = add(R, P, a, p)
    C.M = C.I = 0
    tx = [velu_x(x, terms, p) for (x, _) in pts]
    tr = (C.M, C.I)
    # fresh arm: build T-table on E' from a generator image: T affine adds
    x0 = tx[0]; r0 = (x0**3 + a2*x0 + b2) % p
    ok0 = r0 == 0 or pow(r0, (p-1)//2, p) == 1
    y0 = pow(r0, (p+1)//4, p); G = (x0, y0)
    C.M = C.I = 0; R = G
    for _ in range(T - 1): R = add(R, G, a2, p)
    fr = (C.M, C.I)
    # correctness: on-curve rate of transported x, injectivity over half the N-part
    oncurve = sum(1 for x in tx if pow((x**3 + a2*x + b2) % p, (p-1)//2, p) in (0, 1)) / T
    qs = []; R = P; seen = set(); inj = True
    for k in range(1, (q - 1)//2 + 1):
        x = velu_x(R[0], terms, p)
        if x in seen: inj = False; break
        seen.add(x); R = add(R, P, a, p)
    # negative control: perturbed b' (same x map)
    bad = sum(1 for x in tx if pow((x**3 + a2*x + (b2 + 1 + l) % p) % p, (p-1)//2, p) in (0, 1)) / T
    out = {"l": l, "p": p, "q": q, "T": T, "transport_M_I": tr, "fresh_M_I": fr,
           "oncurve_rate": oncurve, "injective_half_Npart": inj, "negctl_oncurve_rate": bad, "ok0": ok0}
    return out
def cost(mi, I): return mi[0] + I * mi[1]
def main():
    ls = [3, 5] if SMOKE else [3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]
    ls = [l for l in ls if isprime(l)]
    P0 = 3001 if SMOKE else 40039  # p = 3 mod 4 enforced below
    p = nextprime(P0)
    while p % 4 != 3: p = nextprime(p + 1)
    sq = {x * x % p for x in range(1, p)}
    T = 32 if SMOKE else 256
    seeds = [SEED] if SMOKE else [SEED, SEED + 1]  # A/A: two independent curve draws
    t0 = time.time(); rows = []
    for l in ls:
        for s in seeds:
            r = run_l(l, p, sq, s * 1000 + l, T)
            if r: rows.append(r)
            if time.time() - t0 > 520: break
    stats = {}
    for I in (0, 10, 30):
        rat = {}
        for r in rows:
            rat.setdefault(r["l"], []).append(cost(r["transport_M_I"], I) / cost(r["fresh_M_I"], I))
        stats[str(I)] = {str(l): round(sum(v)/len(v), 3) for l, v in rat.items()}
    # A/A: per-l ratio spread between the two seeds at I=10
    aa = 0.0
    for l in ls:
        v = [cost(r["transport_M_I"], 10)/cost(r["fresh_M_I"], 10) for r in rows if r["l"] == l]
        if len(v) == 2: aa = max(aa, abs(v[0]-v[1]) / (sum(v)/2))
    r10 = stats["10"]; vals = list(r10.values())
    allok = all(r["oncurve_rate"] == 1.0 and r["injective_half_Npart"] for r in rows)
    negok = all(r["negctl_oncurve_rate"] < 0.8 for r in rows)
    if SMOKE or not rows: verdict = "inconclusive"
    elif not (allok and negok): verdict = "falsified" if not allok else "inconclusive"
    else:
        lo, hi = min(vals), max(vals)
        cond = all(v > 1.0 for v in vals) and 1.0 < lo <= 2.0 and 10.0 <= hi <= 25.0 and all(r10[k] <= r10[j] + 0.05 for j, k in zip(list(r10)[:-1], list(r10)[1:]))
        verdict = "consistent_with_prediction" if cond else "falsified"
    print(json.dumps({"lead": "LEAD-STR-isogeny-transport-preprocessing", "smoke": SMOKE, "seed": SEED,
        "params": {"p": p, "T": T, "ls": ls, "inversion_cost_in_M": [0, 10, 30]},
        "statistic": "mean transport/fresh field-cost ratio per l", "ratios": stats,
        "A_A_max_rel_spread_I10": aa, "positive_control_all_oncurve_and_injective": allok,
        "negative_control_perturbed_curve_oncurve_below_0.8": negok, "rows": rows,
        "seconds": round(time.time() - t0, 2), "verdict": verdict}))
main()
