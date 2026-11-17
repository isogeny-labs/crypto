#!/usr/bin/env python3
"""LEAD-TRF-anomalous-padic-lift: Smart p-adic lift positive control + null. stdlib only."""
import sys, json, random, math, time

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11):
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

# ---- affine arithmetic over F_p (None = infinity)
def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1,y1 = P; x2,y2 = Q
    if x1 == x2:
        if (y1+y2) % p == 0: return None
        l = (3*x1*x1+a) * pow(2*y1, -1, p) % p
    else:
        l = (y2-y1) * pow(x2-x1, -1, p) % p
    x3 = (l*l-x1-x2) % p
    return (x3, (l*(x1-x3)-y1) % p)

def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R

def rand_point(a, b, p, rng):
    while True:
        x = rng.randrange(p); r = (x**3+a*x+b) % p
        if r and pow(r, (p-1)//2, p) == 1:
            # p = 3 mod 4 not assumed: Tonelli-free via Cipolla-less brute: use sqrt for p%4==3 else search
            y = sqrt_mod(r, p)
            if y is not None: return (x, y)

def sqrt_mod(n, p):
    if p % 4 == 3: return pow(n, (p+1)//4, p)
    # Tonelli-Shanks
    q, s = p-1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(n, q, p), pow(n, (q+1)//2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        bb = pow(c, 1 << (m-i-1), p)
        m, c, t, r = i, bb*bb % p, t*bb*bb % p, r*bb % p
    return r

def find_anomalous(p, rng):
    """random curves until p*P = O for a random P (then #E = p, since p > 7)."""
    tries = 0
    while True:
        tries += 1
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3+27*b*b) % p == 0: continue
        P = rand_point(a, b, p, rng)
        if mul(p, P, a, p) is None: return a, b, P, tries

def point_order(P, a, p):
    """order of P via BSGS in the Hasse interval then prime-factor reduction."""
    s = math.isqrt(p)
    lo = p+1-2*s-2; W = 4*s+6; m = math.isqrt(W)+1
    baby = {}; R = None
    for j in range(m+1):
        if R is not None: baby.setdefault(R, j)
        R = add(R, P, a, p)
    G = mul(m, P, a, p); Q0 = mul(lo, P, a, p)
    cur = Q0
    for i in range(m+1):
        if cur is None: n0 = lo + i*m; break
        neg = (cur[0], (-cur[1]) % p)
        if neg in baby: n0 = lo + i*m + baby[neg]; break
        cur = add(cur, G, a, p)
    else: return None
    if n0 <= 0 or mul(n0, P, a, p) is not None: return None
    n, f = n0, 2
    while f*f <= n:
        while n % f == 0 and mul(n0//f, P, a, p) is None: n0 //= f; n //= f
        if n % f: f += 1 if f == 2 else 2
        elif mul(n0//f, P, a, p) is not None:
            n //= f  # f divides n0 fully needed; strip from cofactor tracker
    # final: reduce any remaining prime factor
    for f in prime_factors(n0):
        while n0 % f == 0 and mul(n0//f, P, a, p) is None: n0 //= f
    return n0

def prime_factors(n):
    out, f = [], 2
    while f*f <= n:
        if n % f == 0:
            out.append(f)
            while n % f == 0: n //= f
        f += 1 if f == 2 else 2
    if n > 1: out.append(n)
    return out

# ---- Jacobian arithmetic over Z/p^2 (inversion free)
def jdbl(P, a, M):
    X,Y,Z = P
    S = 4*X*Y*Y % M; Mm = (3*X*X + a*pow(Z,4,M)) % M
    X3 = (Mm*Mm - 2*S) % M
    return (X3, (Mm*(S-X3) - 8*pow(Y,4,M)) % M, 2*Y*Z % M)

def jadd(P, Q, a, M):
    X1,Y1,Z1 = P; X2,Y2,Z2 = Q
    Z1s, Z2s = Z1*Z1 % M, Z2*Z2 % M
    U1, U2 = X1*Z2s % M, X2*Z1s % M
    S1, S2 = Y1*Z2s*Z2 % M, Y2*Z1s*Z1 % M
    H, r = (U2-U1) % M, (S2-S1) % M
    H2 = H*H % M; H3 = H2*H % M
    X3 = (r*r - H3 - 2*U1*H2) % M
    return (X3, (r*(U1*H2 - X3) - S1*H3) % M, Z1*Z2*H % M)

def jmul(k, P, a, M, ops):
    R = P
    for bit in bin(k)[3:]:
        R = jdbl(R, a, M); ops[0] += 1
        if bit == '1': R = jadd(R, P, a, M); ops[0] += 1
    return R

def lift(P, a, b, p):
    M = p*p; x, y1 = P
    rhs = (x**3 + a*x + b) % M
    t = ((rhs - y1*y1) // p) * pow(2*y1, -1, p) % p
    return (x, (y1 + p*t) % M, 1)

def smart_log(n, P, Q, a, b, p, ops):
    """return d mod p from psi(n*Q~)/psi(n*P~), or None if degenerate. n kills P mod p."""
    M = p*p
    def psi(R):
        X, Y, Z = R
        if Z % p or Y % p == 0 or Z % M == 0: return None
        return (X*Z*pow(Y, -1, M) % M) // p % p
    tp = psi(jmul(n, lift(P, a, b, p), a, M, ops))
    tq = psi(jmul(n, lift(Q, a, b, p), a, M, ops))
    if tp is None or tq is None or tp == 0: return None
    return tq * pow(tp, -1, p) % p

def run(sizes, n_inst, seed):
    out = {"sizes": []}
    for si, k in enumerate(sizes):
        rng = random.Random(seed*1000 + k)
        p = next_prime(rng.randrange(1 << k, 1 << (k+1)))
        row = {"bits": k, "p": p, "arms": {}}
        rho_ref = math.sqrt(math.pi*p/2)
        for arm in ("A", "A_prime"):
            ok = deg = 0; ops = [0]; tries_tot = 0
            for _ in range(n_inst):
                a, b, P, tr = find_anomalous(p, rng); tries_tot += tr
                d = rng.randrange(1, p); Q = mul(d, P, a, p)
                r = smart_log(p, P, Q, a, b, p, ops)
                if r is None: deg += 1
                elif r == d: ok += 1
            row["arms"][arm] = {"n": n_inst, "success": ok, "degenerate": deg,
                                "ops_per_instance": ops[0]/n_inst, "curve_search_tries_mean": tries_tot/n_inst}
        ok = deg = n = 0; ops = [0]
        while n < n_inst:
            a, b = rng.randrange(p), rng.randrange(p)
            if (4*a**3+27*b*b) % p == 0: continue
            P = rand_point(a, b, p, rng); r_ = point_order(P, a, p)
            if r_ is None or r_ == p or r_ < 1000: continue
            n += 1
            d = rng.randrange(1, r_); Q = mul(d, P, a, p)
            r = smart_log(r_, P, Q, a, b, p, ops)
            if r is None: deg += 1
            elif r == d % p: ok += 1
        row["null_nonanomalous"] = {"n": n_inst, "success": ok, "degenerate": deg,
                                    "expected_chance_successes": n_inst/p}
        row["rho_ref_ops_sqrt_pi_p_over_2"] = rho_ref
        row["smart_ops_over_rho_ref"] = row["arms"]["A"]["ops_per_instance"]/rho_ref
        out["sizes"].append(row)
    return out

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time(); seed = 20261110
    sizes, n = ([12], 3) if smoke else ([12, 14, 16, 18, 20, 22], 25)
    res = run(sizes, n, seed)
    tot = lambda f: sum(f(r) for r in res["sizes"])
    N = tot(lambda r: r["arms"]["A"]["n"])
    sA = tot(lambda r: r["arms"]["A"]["success"]) / N
    sB = tot(lambda r: r["arms"]["A_prime"]["success"]) / N
    nullS = tot(lambda r: r["null_nonanomalous"]["success"])
    deg = tot(lambda r: r["arms"]["A"]["degenerate"] + r["arms"]["A_prime"]["degenerate"])
    ratios = [r["smart_ops_over_rho_ref"] for r in res["sizes"]]
    if min(sA, sB) < 0.95 or nullS > 5: verdict = "falsified"
    elif deg > 0.2*2*N or abs(sA-sB) > 0.1: verdict = "inconclusive"
    else: verdict = "consistent_with_prediction"
    res.update({"lead": "LEAD-TRF-anomalous-padic-lift", "smoke": smoke, "seed": seed,
                "params": {"sizes_bits": sizes, "instances_per_arm": n},
                "statistic": {"anomalous_success_A": sA, "anomalous_success_A_prime": sB,
                              "null_total_successes": nullS, "degenerate_total": deg,
                              "smart_over_rho_ratio_by_size": ratios},
                "thresholds": {"pos_min": 0.95, "null_max": 5},
                "verdict": verdict, "label": "[pcc:unverified]",
                "wall_s": time.time()-t0})
    print(json.dumps(res)); return 0

if __name__ == "__main__":
    sys.exit(main())
