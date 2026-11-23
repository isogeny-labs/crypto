#!/usr/bin/env python3
"""LEAD-ICP-isogeny-transported-relation-yield toy: stdlib only, 1 core.
Pair-relation (m=2) coverage of an x-interval factor base on isogenous curves
(same trace, distinct j) vs a matched random-log null in the same cyclic group."""
import sys, json, math, random, time

def primes_factor(n):
    f, d = [], 2
    while d * d <= n:
        if n % d == 0:
            f.append(d)
            while n % d == 0: n //= d
        d += 1
    if n > 1: f.append(n)
    return f

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R

def sqrt_mod(n, p, rng):  # p odd prime; brute-free via Tonelli-lite using pow for p%4==3 else search
    n %= p
    if n == 0: return 0
    if p % 4 == 3:
        r = pow(n, (p + 1) // 4, p)
        return r if r * r % p == n else None
    # Tonelli-Shanks
    if pow(n, (p - 1) // 2, p) != 1: return None
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(n, q, p), pow(n, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2 * t2 % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r

def classes(p, rng, maxper):
    chi = [-1] * p
    chi[0] = 0
    for y in range(1, p): chi[y * y % p] = 1
    chi = [0 if i == 0 else chi[i] for i in range(p)]
    byN, seen = {}, set()
    for j in range(2, p):
        if j == 1728 % p: continue
        c = j * (1728 - j) % p
        a, b = 3 * c % p, 2 * c * (1728 - j) % p
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        N = p + 1 + sum(chi[(x * x * x + a * x + b) % p] for x in range(p))
        byN.setdefault(N, []).append((j, a, b))
    return byN

def find_generator(a, b, p, N, rng):
    fs = primes_factor(N)
    for _ in range(40):
        x = rng.randrange(p)
        y = sqrt_mod(x ** 3 + a * x + b, p, rng)
        if y is None: continue
        P = (x, y)
        if mul(N, P, a, p) is not None: return None
        if all(mul(N // q, P, a, p) is not None for q in fs): return P
    return None

def logs_by_x(a, p, N, G):
    out, Q = [], G
    for k in range(1, N):
        out.append((Q[0], k)); Q = add(Q, G, a, p)
    out.sort()
    return [k for _, k in out]

def coverage(S, N):
    seen = bytearray(N); L = list(S)
    for i, u in enumerate(L):
        for v in L[i:]:
            seen[(u + v) % N] = 1
    return sum(seen) / N

def mean(v): return sum(v) / len(v)
def var(v):
    m = mean(v); return sum((x - m) ** 2 for x in v) / max(1, len(v) - 1)

def one_prime(p, seed, maxcurves, K, boot, rng_tag):
    rng = random.Random(seed * 1000003 + p)
    byN = classes(p, rng, maxcurves)
    best = None
    for N, lst in sorted(byN.items(), key=lambda t: -len(t[1])):
        cyc = []
        for (j, a, b) in lst:
            if len(cyc) >= maxcurves: break
            G = find_generator(a, b, p, N, rng)
            if G: cyc.append((j, a, b, G))
        if len(cyc) >= 8 and (best is None or len(cyc) > len(best[1])): best = (N, cyc)
        if best and len(best[1]) >= maxcurves: break
        if best and N != best[0] and len(best[1]) >= 8 and len(lst) < len(best[1]): break
    if best is None: return {"p": p, "status": "no_class"}
    N, cyc = best
    B = max(4, round(1.2 * math.sqrt(N)))
    cA, cN, cPos = [], [], []
    for (j, a, b, G) in cyc:
        order = logs_by_x(a, p, N, G)
        cA.append(coverage(order[:B], N))
        cN.append([coverage(rng.sample(range(1, N), B), N) for _ in range(K)])
        ap = (rng.randrange(1, N), rng.randrange(1, N))
        pos = [(ap[0] + ap[1] * t) % N for t in range(B // 2)] + rng.sample(range(1, N), B - B // 2)
        cPos.append(coverage(pos, N))
    nflat = [x for r in cN for x in r]
    d = [cA[i] - mean(cN[i]) for i in range(len(cA))]
    se = math.sqrt(var(cA) / len(cA) + var(nflat) / len(nflat)) or 1e-12
    z = (mean(cA) - mean(nflat)) / se
    # A/A: odd vs even index curves
    h1, h2 = cA[0::2], cA[1::2]
    seaa = math.sqrt(var(h1) / len(h1) + var(h2) / len(h2)) or 1e-12
    zaa = (mean(h1) - mean(h2)) / seaa
    vr = var(cA) / max(var(nflat), 1e-18)
    bs = []
    for _ in range(boot):
        ra = [cA[rng.randrange(len(cA))] for _ in cA]
        rn = [nflat[rng.randrange(len(nflat))] for _ in nflat]
        bs.append(var(ra) / max(var(rn), 1e-18))
    bs.sort()
    zpos = (mean(cPos) - mean(nflat)) / (math.sqrt(var(cPos) / len(cPos) + var(nflat) / len(nflat)) or 1e-12)
    return {"p": p, "N": N, "B": B, "n_curves": len(cyc), "mean_cov_A": mean(cA),
            "mean_cov_null": mean(nflat), "z_mean": z, "z_AA": zaa, "var_ratio": vr,
            "var_ratio_ci95": [bs[int(.025 * boot)], bs[int(.975 * boot) - 1]],
            "z_positive_control": zpos, "status": "ok"}

def main():
    smoke = "--smoke" in sys.argv
    seed = 20261110
    t0 = time.time()
    if smoke: primes, maxc, K, boot, budget = [211, 307], 8, 3, 50, 8
    else: primes, maxc, K, boot, budget = [1009, 2003, 4001, 8009, 16001], 24, 5, 400, 420
    rows = []
    for p in primes:
        if rows and time.time() - t0 > budget * (0.45 if p >= 16001 else 0.8): break
        rows.append(one_prime(p, seed, maxc, K, boot, 0))
    ok = [r for r in rows if r.get("status") == "ok"]
    verdict = "inconclusive"
    big = ok[-2:]
    if len(ok) >= (1 if smoke else 3):
        pos_ok = all(r["z_positive_control"] < -3 for r in ok)
        aa_ok = all(abs(r["z_AA"]) < 3.5 for r in ok)
        if pos_ok and aa_ok:
            excess = all(r["var_ratio_ci95"][0] > 1.0 for r in big) and len(big) == 2 and big[1]["var_ratio"] > ok[0]["var_ratio"]
            shift = any(abs(r["z_mean"]) > 4 for r in ok)
            if excess or shift: verdict = "falsified"
            elif all(0.4 <= r["var_ratio"] <= 2.5 and abs(r["z_mean"]) < 4 for r in ok): verdict = "consistent_with_prediction"
    print(json.dumps({"lead": "LEAD-ICP-isogeny-transported-relation-yield", "seed": seed,
        "smoke": smoke, "params": {"primes": primes, "max_curves_per_class": maxc, "null_draws_per_curve": K,
        "bootstrap": boot, "B": "round(1.2*sqrt(N))", "fb": "x-sorted points of the cyclic group"},
        "rows": rows, "elapsed_s": round(time.time() - t0, 2), "verdict": verdict, "label": "pcc:unverified"}))
if __name__ == "__main__": main()
