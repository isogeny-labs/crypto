#!/usr/bin/env python3
"""LEAD-EXO-split-jacobian-projection: stdlib toy re-derivation of Hasse-Weil inflation.
Genus-2 C: y^2=g(x^2) over F_p (split Jacobian ~ E1 x E2). #J=(N1^2+N2)/2-p from C(F_p),C(F_p^2)."""
import sys, json, random, math

def is_prime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True

def count_sextic(f, p, n):
    """f: coeffs low->high, degree 6 or 4 handled by caller via lead. returns (N1,N2) for y^2=f(x), deg 6, smooth model."""
    deg = len(f)-1
    def ev(x):
        r = 0
        for c in reversed(f): r = (r*x + c) % p
        return r
    leg = [0]*p
    for y in range(1, p): leg[y*y % p] = 1
    N1 = 0
    for x in range(p):
        v = ev(x)
        N1 += 1 if v == 0 else (2 if leg[v] else 0)
    lc = f[-1] % p
    inf = (1 + (1 if leg[lc] else 0)) if deg == 6 else (2 if leg[lc] else 0) + (0)
    if deg == 6: inf = 2 if leg[lc] else 0
    else: inf = 2 if leg[lc] else 0
    N1 += inf
    # F_p^2 = F_p[i]/(i^2-n); square test through norm
    def mul(a, b):
        return ((a[0]*b[0] + n*a[1]*b[1]) % p, (a[0]*b[1] + a[1]*b[0]) % p)
    N2 = 0
    for a in range(p):
        for b in range(p):
            r = (0, 0)
            for c in reversed(f):
                r = mul(r, (a, b)); r = ((r[0]+c) % p, r[1])
            if r == (0, 0): N2 += 1
            else:
                nm = (r[0]*r[0] - n*r[1]*r[1]) % p
                N2 += 2 if leg[nm] else 0
    # points at infinity over F_p^2: leading coeff is a square in F_p^2 always (F_p elements are squares there)
    N2 += 2
    return N1, N2

def count_quartic_E(g, p):
    """E2: v^2 = u*g(u), g monic cubic list low->high; genus 1, deg 4 monic: 2 pts at infinity."""
    h = [0] + list(g)
    leg = [0]*p
    for y in range(1, p): leg[y*y % p] = 1
    N = 2
    for u in range(p):
        v = 0
        for c in reversed(h): v = (v*u + c) % p
        N += 1 if v == 0 else (2 if leg[v] else 0)
    return N

def count_cubic_E(g, p):
    leg = [0]*p
    for y in range(1, p): leg[y*y % p] = 1
    N = 1
    for u in range(p):
        v = 0
        for c in reversed(g): v = (v*u + c) % p
        N += 1 if v == 0 else (2 if leg[v] else 0)
    return N

def disc_ok(f, p):
    # squarefree check by brute force: no repeated root in F_p (cheap necessary check); full check via gcd
    def pmod(a, b):
        a = a[:]
        while len(a) >= len(b) and any(a):
            if a[-1] == 0: a.pop(); continue
            q = a[-1]*pow(b[-1], -1, p) % p
            s = len(a)-len(b)
            for i in range(len(b)): a[s+i] = (a[s+i]-q*b[i]) % p
            a.pop()
        while a and a[-1] == 0: a.pop()
        return a
    df = [(i*f[i]) % p for i in range(1, len(f))]
    while df and df[-1] == 0: df.pop()
    if not df: return False
    a, b = f[:], df
    while b:
        a, b = b, pmod(a, b)
    return len(a) == 1

def largest_pf(n):
    m, f = n, 1
    d = 2
    while d*d <= m:
        while m % d == 0: f = d; m //= d
        d += 1
    return max(f, m) if m > 1 else f

def run(primes, per_p, seed):
    rng = random.Random(seed)
    stats = {"hw_violations": 0, "iso_mismatch": 0, "split_cases": 0, "generic_cases": 0}
    ratios_split = []; min_ambient = None; ratios_gen = []
    for p in primes:
        n = next(c for c in range(2, p) if pow(c, (p-1)//2, p) == p-1)
        for _ in range(per_p):
            # split curve y^2=g(x^2), g monic cubic
            while True:
                g = [rng.randrange(p) for _ in range(3)] + [1]
                f = [0]*7
                for i, c in enumerate(g): f[2*i] = c
                if g[0] != 0 and disc_ok(f, p): break
            N1, N2 = count_sextic(f, p, n)
            J = (N1*N1 + N2)//2 - p
            Ea, Eb = count_cubic_E(g, p), count_quartic_E(g, p)
            stats["split_cases"] += 1
            if Ea*Eb != J: stats["iso_mismatch"] += 1
            lo, hi = (math.sqrt(p)-1)**4, (math.sqrt(p)+1)**4
            if not (lo <= J <= hi): stats["hw_violations"] += 1
            # DL group = largest prime-order part of E1; ambient/N
            for E, other in ((Ea, Eb), (Eb, Ea)):
                N = largest_pf(E)
                amb = J / N
                ratios_split.append(amb / p)
                if min_ambient is None or amb/p < min_ambient: min_ambient = amb/p
            # generic sextic
            while True:
                f6 = [rng.randrange(p) for _ in range(7)]
                if f6[6] and disc_ok(f6, p): break
            M1, M2 = count_sextic(f6, p, n)
            Jg = (M1*M1+M2)//2 - p
            stats["generic_cases"] += 1
            if not ((math.sqrt(p)-1)**4 <= Jg <= (math.sqrt(p)+1)**4): stats["hw_violations"] += 1
            ratios_gen.append(Jg/(p*p))
    ratios_split.sort(); ratios_gen.sort()
    med = lambda a: a[len(a)//2]
    return stats, {"split_ambient_over_N_div_p_min": min_ambient, "split_ambient_over_N_div_p_median": med(ratios_split),
                   "generic_J_over_p2_median": med(ratios_gen), "generic_J_over_p2_min": ratios_gen[0], "generic_J_over_p2_max": ratios_gen[-1]}

def main():
    smoke = "--smoke" in sys.argv
    primes = [p for p in range(11, 20) if is_prime(p)] if smoke else [p for p in range(37, 140) if is_prime(p)]
    per_p = 1 if smoke else 12
    seed = 20261110
    stats, summ = run(primes, per_p, seed)
    # A/A arm: same protocol, second seed; medians should agree within 15%
    stats2, summ2 = run(primes, per_p, seed+1)
    aa = abs(summ["generic_J_over_p2_median"]-summ2["generic_J_over_p2_median"]) < 0.15
    controls_ok = stats["hw_violations"] == 0 and stats["iso_mismatch"] == 0 and stats2["iso_mismatch"] == 0
    # prediction: ambient/N >= p/4 in every split case (no O(N) ambient)
    pred = summ["split_ambient_over_N_div_p_min"] >= 0.25 and summ2["split_ambient_over_N_div_p_min"] >= 0.25
    if not controls_ok or not aa: verdict = "inconclusive"
    elif pred: verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps({"lead": "LEAD-EXO-split-jacobian-projection", "smoke": smoke, "seeds": [seed, seed+1],
        "primes": primes, "curves_per_prime": per_p, "stats_arm1": stats, "summary_arm1": summ,
        "stats_AA_arm2": stats2, "summary_AA_arm2": summ2, "aa_ok": aa,
        "positive_control": "#J == #E1*#E2 (isogeny) and Hasse-Weil band; mismatches counted in stats",
        "negative_control": "generic sextic J/p^2 near 1",
        "threshold": "min over split cases of (#J/N)/p >= 0.25", "verdict": verdict}))
    return 0
sys.exit(main())
