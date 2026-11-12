#!/usr/bin/env python3
"""Toy for LEAD-STR-class-group-ecdlp-bridge: generic ECDLP cost with a free class-group action oracle.
Group G_c = Z_N on each of h curves c; action a maps G_c -> G_{c*a} as an exponent-preserving
isomorphism (what an isogeny of degree prime to N does on prime-order groups). Cost = group ops + action calls."""
import sys, json, random, math, hashlib

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def prime_near(n):
    while not is_prime(n): n += 1
    return n

def H(seed, x):
    return int.from_bytes(hashlib.blake2b(b"%d:%d" % (seed, x), digest_size=4).digest(), "big")

def solve(N, k, rng, action, h=1, R=16, pswitch=2):
    """Collision walk on exponents x = a + b*k (mod N); returns query cost until k recovered."""
    seed = rng.getrandbits(30)
    mul = [(rng.randrange(N), rng.randrange(N)) for _ in range(R)]
    cost = 0
    while True:
        a, b = rng.randrange(N), rng.randrange(N)
        x = (a + b * k) % N
        c = 0
        seen = {x: (a, b)}
        for _ in range(8 * int(math.sqrt(N)) + 50):
            hx = H(seed, x)
            ma, mb = mul[hx % R]
            a, b = (a + ma) % N, (b + mb) % N
            x = (a + b * k) % N
            cost += 1  # group op
            if action and (hx >> 8) % pswitch == 0:
                c = (c + 1 + (hx >> 12) % max(h - 1, 1)) % h
                cost += 1  # action call; canonicalizing to a common curve is this transport
            if x in seen:
                a0, b0 = seen[x]
                if (b - b0) % N:
                    kk = ((a0 - a) * pow(b - b0, -1, N)) % N
                    if kk == k: return cost
                break
            seen[x] = (a, b)

def arm(N, runs, seed0, action, leak=1):
    costs = []
    for i in range(runs):
        rng = random.Random(seed0 * 1000003 + i)
        Neff = prime_near(N // leak) if leak > 1 else N
        k = rng.randrange(1, Neff)
        costs.append(solve(Neff, k, rng, action, h=max(2, int(N ** 0.5) // 8)))
    return costs

def med(v):
    s = sorted(v); return s[len(s) // 2]

def boot_ratio(x, y, rng, B=400):
    r = []
    for _ in range(B):
        xs = [rng.choice(x) for _ in x]; ys = [rng.choice(y) for _ in y]
        r.append(med(xs) / med(ys))
    r.sort(); return r[int(.025 * B)], r[int(.975 * B)]

def main():
    smoke = "--smoke" in sys.argv
    exps = [10] if smoke else [12, 14, 16, 18, 20, 22]
    runs = 12 if smoke else 120
    out = {"lead": "LEAD-STR-class-group-ecdlp-bridge", "smoke": smoke, "seeds": {"A": 1, "A2": 2, "B": 3, "C": 4, "boot": 99},
           "params": {"sizes_log2": exps, "runs": runs, "walk_multipliers": 16, "switch_prob": 0.5, "leak_m": 8}, "per_size": {}}
    pool = {"A": [], "A2": [], "B": [], "C": []}
    pts = {"A": [], "B": []}
    for e in exps:
        N = prime_near(2 ** e)
        row = {}
        for name, seed, act, leak in (("A", 1, False, 1), ("A2", 2, False, 1), ("B", 3, True, 1), ("C", 4, False, 8)):
            c = arm(N, runs, seed, act, leak)
            row[name] = med(c) / math.sqrt(N)
            pool[name] += [x / math.sqrt(N) for x in c]
            if name in pts: pts[name].append((math.log(N), math.log(med(c))))
        out["per_size"][str(N)] = row
    def slope(p):
        if len(p) < 2: return None
        mx = sum(a for a, _ in p) / len(p); my = sum(b for _, b in p) / len(p)
        return sum((a - mx) * (b - my) for a, b in p) / sum((a - mx) ** 2 for a, _ in p)
    br = random.Random(99)
    rAA = boot_ratio(pool["A2"], pool["A"], br); rB = boot_ratio(pool["B"], pool["A"], br); rC = boot_ratio(pool["C"], pool["A"], br)
    out["stat"] = {"ratio_AA": [med(pool["A2"]) / med(pool["A"]), *rAA], "ratio_B_over_A": [med(pool["B"]) / med(pool["A"]), *rB],
                   "ratio_C_over_A_positive_control": [med(pool["C"]) / med(pool["A"]), *rC],
                   "slope_A": slope(pts["A"]), "slope_B": slope(pts["B"])}
    aa_ok = rAA[0] <= 1 <= rAA[1] or (0.9 <= rAA[0] and rAA[1] <= 1.1)
    pc_ok = rC[1] < 0.6
    if not (aa_ok and pc_ok): v = "inconclusive"
    elif rB[1] < 0.95: v = "falsified"
    elif rB[0] >= 0.95 and (smoke or (slope(pts["B"]) is not None and 0.4 <= slope(pts["B"]) <= 0.6)): v = "consistent_with_prediction"
    else: v = "inconclusive"
    out["controls"] = {"aa_ok": aa_ok, "positive_control_detected": pc_ok}
    out["verdict"] = v
    print(json.dumps(out))

main()
