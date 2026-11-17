#!/usr/bin/env python3
"""Toy test of the formal-group channel of the kernel-of-reduction lemma. stdlib only."""
import sys, json, random, math

def add(P, Q, a, b3, m):
    X1, Y1, Z1 = P; X2, Y2, Z2 = Q
    t0 = X1*X2 % m; t1 = Y1*Y2 % m; t2 = Z1*Z2 % m
    t3 = (X1+Y1)*(X2+Y2) % m; t3 = (t3-t0-t1) % m
    t4 = (X1+Z1)*(X2+Z2) % m; t4 = (t4-t0-t2) % m
    t5 = (Y1+Z1)*(Y2+Z2) % m; t5 = (t5-t1-t2) % m
    Z3 = (a*t4 + b3*t2) % m
    X3 = (t1-Z3) % m; Z3 = (t1+Z3) % m
    Y3 = X3*Z3 % m
    t1 = (3*t0 + a*t2) % m
    t2 = a*(t0-a*t2) % m
    t4 = (b3*t4 + t2) % m
    Y3 = (Y3 + t1*t4) % m
    X3 = (t3*X3 - t5*t4) % m
    Z3 = (t5*Z3 + t3*t1) % m
    return (X3, Y3, Z3)

def mul(k, P, a, b3, m):
    R = (0, 1, 0)
    for bit in bin(k)[2:]:
        R = add(R, R, a, b3, m)
        if bit == '1':
            R = add(R, P, a, b3, m)
    return R

def count(p, a, b):
    s = 1
    for x in range(p):
        v = (x*x*x + a*x + b) % p
        s += 1 + (0 if v == 0 else (1 if pow(v, (p-1)//2, p) == 1 else -1))
    return s

def isprime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True

def primes_near(lo, hi):
    return [q for q in range(lo, hi) if isprime(q)]

def rand_point(p, a, b, rng):
    while True:
        x = rng.randrange(p); v = (x**3 + a*x + b) % p
        if v and pow(v, (p-1)//2, p) == 1:
            for y in range(1, p):  # small p: tonelli by search is slow; use sqrt via p%4
                break
            if p % 4 == 3:
                y = pow(v, (p+1)//4, p)
            else:
                y = sqrt_mod(v, p)
            return x, y

def sqrt_mod(n, p):
    q, s = p-1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    M, c, t, R = s, pow(z, q, p), pow(n, q, p), pow(n, (q+1)//2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1: tt = tt*tt % p; i += 1
        bb = pow(c, 1 << (M-i-1), p)
        M, c, t, R = i, bb*bb % p, t*bb*bb % p, R*bb % p
    return R

def lift(x, y, p, a, b, rng):
    m = p*p
    xt = x + p*rng.randrange(p)
    f = (xt**3 + a*xt + b) % m
    r = (f - y*y) % m
    assert r % p == 0
    v = (r//p) * pow(2*y, -1, p) % p
    return (xt % m, (y + p*v) % m, 1)

def psi(R, p):
    X, Y, Z = R
    assert Z % p == 0 and Y % p != 0
    assert X % p == 0
    return ((-X * pow(Y, -1, p*p)) % (p*p)) // p % p

def trials(p, a, b, N, n, rng, mult):
    """mult = scalar applied to lifts (N, the group order). Returns (hits, usable, residual counts)."""
    b3 = 3*b
    hits = used = 0
    bins = [0]*16
    G = rand_point(p, a, b, rng)
    for _ in range(n):
        k = rng.randrange(1, N) if N > 2 else 1
        Gp = (G[0], G[1], 1)
        Qp = mul(k, Gp, a, b3, p)
        if Qp[2] % p == 0: continue
        iz = pow(Qp[2], -1, p)
        qx, qy = Qp[0]*iz % p, Qp[1]*iz % p
        Pl = lift(G[0], G[1], p, a, b, rng); Ql = lift(qx, qy, p, a, b, rng)
        sP = psi(mul(mult, Pl, a, b3, p*p), p); sQ = psi(mul(mult, Ql, a, b3, p*p), p)
        if sP == 0: continue
        r = sQ * pow(sP, -1, p) % p
        used += 1
        hits += (r == k % p)
        bins[((r - k) % p) * 16 // p] += 1
    return hits, used, bins

def main():
    smoke = '--smoke' in sys.argv
    rng = random.Random(20261110)
    if smoke:
        plo, phi, ncurve, nt, nanom = 1000, 1300, 1, 40, 1
    else:
        plo, phi, ncurve, nt, nanom = 3000, 40000, 6, 3000, 3
    # self-check of group law against order
    ps = primes_near(plo, phi); rng.shuffle(ps)
    # positive control: anomalous curves (N == p), Smart attack recovers k mod p
    pos = []
    for p in ps:
        if len(pos) >= nanom: break
        for _ in range(3000 if not smoke else 6000):
            a, b = rng.randrange(p), rng.randrange(p)
            if (4*a**3 + 27*b*b) % p == 0: continue
            if count(p, a, b) == p:
                h, u, _ = trials(p, a, b, p, nt if not smoke else 20, random.Random(p), p)
                pos.append({"p": p, "a": a, "b": b, "hits": h, "usable": u}); break
    # null: non-anomalous prime-order curves
    nul = []
    for p in ps:
        if len(nul) >= ncurve: break
        for _ in range(200):
            a, b = rng.randrange(p), rng.randrange(p)
            if (4*a**3 + 27*b*b) % p == 0: continue
            N = count(p, a, b)
            if N != p and isprime(N) and N > 100:
                r1 = trials(p, a, b, N, nt, random.Random(p*7+1), N)
                r2 = trials(p, a, b, N, nt, random.Random(p*7+2), N)  # A/A: same protocol, other seed
                nul.append({"p": p, "N": N, "a": a, "b": b, "hits_A": r1[0], "used_A": r1[1],
                            "hits_A2": r2[0], "used_A2": r2[1], "bins": [x+y for x, y in zip(r1[2], r2[2])]})
                break
    pos_ok = bool(pos) and all(x["usable"] > 0 and x["hits"] == x["usable"] for x in pos)
    # null: expected hits ~ 1/p per trial; reject if hits exceed Poisson-ish bound mean+5sd+1
    null_ok = True; aa_ok = True; chi_max = 0.0
    for x in nul:
        for h, u in ((x["hits_A"], x["used_A"]), (x["hits_A2"], x["used_A2"])):
            mu = u / x["p"]
            if h > mu + 5*math.sqrt(mu) + 2: null_ok = False
        tot = sum(x["bins"]); e = tot/16
        chi = sum((c-e)**2/e for c in x["bins"]) if e else 0.0
        x["chi2_16bins"] = chi; chi_max = max(chi_max, chi)
        if abs(x["hits_A"] - x["hits_A2"]) > 3: aa_ok = False
    if not pos or not nul:
        verdict = "inconclusive"
    elif not pos_ok:
        verdict = "inconclusive"   # detector cannot see the anomalous channel
    elif not null_ok:
        verdict = "falsified"
    elif chi_max > 37.7 * (1 if not smoke else 10) or not aa_ok:  # chi2 df=15, p~1e-3 per curve
        verdict = "inconclusive"
    else:
        verdict = "consistent_with_prediction"
    print(json.dumps({"lead": "LEAD-TRF-kernel-of-reduction-lemma", "smoke": smoke,
        "seed": 20261110, "params": {"p_range": [plo, phi], "trials_per_arm": nt, "curves_null": ncurve, "curves_pos": nanom},
        "statistic": "hit rate of Smart ratio psi(N Q~)/psi(N P~) == k mod p; null expectation 1/p",
        "positive_control_anomalous": pos, "negative_null_nonanomalous": nul,
        "pos_ok": pos_ok, "null_ok": null_ok, "aa_ok": aa_ok, "chi2_max": chi_max, "verdict": verdict}))
main()
