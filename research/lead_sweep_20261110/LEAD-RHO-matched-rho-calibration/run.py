#!/usr/bin/env python3
"""LEAD-RHO-matched-rho-calibration toy: finite-size bias of plain/negation rho vs birthday floor. stdlib only."""
import sys, json, math, random, time

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l*l - x1 - x2) % p
    return (x3, (l*(x1 - x3) - y1) % p)

def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R

def count_points(a, b, p):
    cnt = [0]*p
    for y in range(p): cnt[y*y % p] += 1
    n = 1
    for x in range(p): n += cnt[(x*x*x + a*x + b) % p]
    return n

def find_curve(bits, rng):
    """prime-order-r subgroup (cofactor <= 4), r ~ 2^bits."""
    while True:
        p = next_prime(rng.randrange(1 << bits, 1 << (bits+1)))
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        n = count_points(a, b, p)
        for h in (1, 2, 3, 4):
            if n % h == 0 and is_prime(n // h) and n // h > 100:
                r = n // h
                for _ in range(50):
                    x = rng.randrange(p); rhs = (x**3 + a*x + b) % p
                    if pow(rhs, (p-1)//2, p) != 1: continue
                    y = pow(rhs, (p+1)//4, p) if p % 4 == 3 else None
                    if y is None or y*y % p != rhs: continue
                    G = mul(h, (x, y), a, p)
                    if G is not None: return p, a, b, r, G
        # else retry

def walk_once(curve, rng, njump, neg, cap):
    p, a, b, r, G = curve
    jumps = [mul(rng.randrange(1, r), G, a, p) for _ in range(njump)]
    canon = (lambda P: (P[0], min(P[1], p-P[1]))) if neg else (lambda P: P)
    P = canon(mul(rng.randrange(1, r), G, a, p))
    seen = {P}; steps = 0
    while steps < cap:
        j = jumps[(P[0]*2654435761 >> 7) % njump]
        Q = add(P, j, a, p)
        if Q is None: return steps, False  # hit infinity; negligible
        P = canon(Q); steps += 1
        if P in seen: return steps, True
        seen.add(P)
    return steps, False

def oracle_once(N, rng, cap):
    """null: random function on N elements; steps to first repeat."""
    seen = set(); x = rng.randrange(N); steps = 0
    while True:
        if x in seen: return steps
        seen.add(x); steps += 1
        x = rng.randrange(N)

def mean_se(v):
    m = sum(v)/len(v); s = (sum((t-m)**2 for t in v)/max(1, len(v)-1))**.5
    return m, s/len(v)**.5

def arm(curve, seed, runs, njump, neg):
    rng = random.Random(seed); r = curve[3]
    N = r//2 if neg else r
    floor = math.sqrt(math.pi*N/2)  # expected steps to first repeat on N states
    cap = int(30*floor)
    vals, trapped = [], 0
    for _ in range(runs):
        s, ok = walk_once(curve, rng, njump, neg, cap)
        vals.append(s/floor)
        trapped += (not ok)
    m, se = mean_se(vals)
    return {"R": m, "se": se, "censored_frac": trapped/runs}

def null_arm(N, seed, runs):
    rng = random.Random(seed); floor = math.sqrt(math.pi*N/2)
    m, se = mean_se([oracle_once(N, rng, 0)/floor for _ in range(runs)])
    return {"R": m, "se": se}

def main():
    smoke = "--smoke" in sys.argv
    bits = [8, 10] if smoke else [12, 14, 16, 18, 20]
    runs = 30 if smoke else 400
    t0 = time.time(); out = {"lead": "LEAD-RHO-matched-rho-calibration", "smoke": smoke,
        "seeds": {"curve": 20261110, "arms": "20261110+1000*bits+arm_offset"}, "njump": 32,
        "runs_per_arm": runs, "sizes": []}
    for bt in bits:
        crng = random.Random(20261110 + bt)
        curve = find_curve(bt, crng); r = curve[3]; base = 20261110 + 1000*bt
        row = {"bits": bt, "r": r, "p": curve[0],
            "plain": arm(curve, base+1, runs, 32, False),
            "plain_AA": arm(curve, base+2, runs, 32, False),
            "neg": arm(curve, base+3, runs, 32, True),
            "positive_control_2jump_plain": arm(curve, base+4, runs, 2, False),
            "null_plain": null_arm(r, base+5, runs*2),
            "null_neg": null_arm(r//2, base+6, runs*2)}
        out["sizes"].append(row)
    L = out["sizes"][-1]; F = out["sizes"][0]
    def ex(a, n): return a["R"]/n["R"]       # excess over null
    aa = abs(L["plain"]["R"] - L["plain_AA"]["R"]) / math.hypot(L["plain"]["se"], L["plain_AA"]["se"])
    pos = L["positive_control_2jump_plain"]["R"]/L["null_plain"]["R"]
    nex = ex(L["neg"], L["null_neg"]); nex0 = ex(F["neg"], F["null_neg"]); pex = ex(L["plain"], L["null_plain"])
    out["statistics"] = {"plain_excess_largest": pex, "neg_excess_largest": nex, "neg_excess_smallest": nex0,
        "AA_z": aa, "positive_control_excess": pos}
    ctrl_ok = aa < 3 and pos > 1.3
    if smoke: v = "inconclusive"
    elif not ctrl_ok: v = "inconclusive"
    elif pex <= 1.10 and nex <= 1.10: v = "consistent_with_prediction"
    elif nex >= 1.30 and nex >= nex0 - 0.05: v = "falsified"
    else: v = "inconclusive"
    out["controls_ok"] = ctrl_ok; out["verdict"] = v; out["wall_s"] = round(time.time()-t0, 2)
    print(json.dumps(out)); return 0

if __name__ == "__main__":
    sys.exit(main())
