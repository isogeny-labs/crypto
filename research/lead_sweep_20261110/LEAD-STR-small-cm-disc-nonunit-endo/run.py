#!/usr/bin/env python3
"""Stdlib-only toy for LEAD-STR-small-cm-disc-nonunit-endo. See PROTOCOL.md."""
import sys, json, math, random

def is_prime(n):
    if n < 2: return False
    for p in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % p == 0: return n == p
    d, s = n-1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def factor(n):
    f = {}; p = 2
    while p*p <= n:
        while n % p == 0: f[p] = f.get(p,0)+1; n //= p
        p += 1 if p == 2 else 2
    if n > 1: f[n] = f.get(n,0)+1
    return f

def order(x, N):
    o = N-1
    for p in factor(N-1):
        while o % p == 0 and pow(x, o//p, N) == 1: o //= p
    return o

def sqrt_mod(a, p):
    a %= p
    if a == 0: return 0
    if pow(a, (p-1)//2, p) != 1: return None
    for r in range(p) if p < 50 else ():
        if r*r % p == a: return r
    # Tonelli-Shanks
    q, s = p-1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q+1)//2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        b = pow(c, 1 << (m-i-1), p)
        m, c, t, r = i, b*b % p, t*b*b % p, r*b % p
    return r

def elements(nmax, dmax):
    """alpha = a+b*sqrt(-d), norm n in 2..nmax; return (d,a,b,n)."""
    out = []
    for d in range(1, dmax+1):
        if any(k*k > 1 and d % (k*k) == 0 for k in range(2, 6)): continue
        for b in range(1, 6):
            for a in range(0, 8):
                n = a*a + d*b*b
                if 2 <= n <= nmax: out.append((d, a, b, n))
    return out

def mul(x, y, d):  # (a,b)*(c,e) in Z[sqrt(-d)]
    return (x[0]*y[0] - d*x[1]*y[1], x[0]*y[1] + x[1]*y[0])

def norm_pow_minus1(d, a, b, h):
    z = (1, 0)
    for _ in range(h): z = mul(z, (a, b), d)
    return (z[0]-1)**2 + d*z[1]**2

def eig_orders(d, a, b, n, N):
    """roots of x^2 - 2a x + n mod N and their orders (prime N, gcd(n,N)=1)."""
    t = 2*a
    s = sqrt_mod(t*t - 4*n, N)
    if s is None: return None
    inv2 = pow(2, N-2, N)
    lam = (t + s) * inv2 % N
    if lam == 0 or lam == 1: return None
    return order(lam, N)

def main():
    smoke = "--smoke" in sys.argv
    rng = random.Random(20261110)
    lo, hi = (10, 14) if smoke else (12, 22)
    nprime = 40 if smoke else 400
    els = elements(8, 12 if smoke else 40)
    if smoke: els = els[:6]
    bound_viol = 0; tested = 0; ratios = []; null_ratios = []
    for _ in range(nprime):
        N = rng.randrange(1 << lo, 1 << hi) | 1
        while not is_prime(N): N += 2
        for (d, a, b, n) in els:
            if N % n == 0: continue
            h = eig_orders(d, a, b, n, N)
            if h is None: continue
            tested += 1
            bound = 2*math.log(math.sqrt(N)-1)/math.log(n)  # h >= bound (exact consequence of norm(alpha^h-1)>=N)
            if h < bound - 1e-9: bound_viol += 1
            ratios.append(h/(math.log(N)/math.log(n)))
        lam = rng.randrange(2, N-1)
        null_ratios.append(order(lam, N)/(math.log(N)/math.log(2)))
    # positive control: primes N | Norm(alpha^h-1) force lambda^h=1 with small h
    pos = []
    for (d, a, b, n) in els[: (4 if smoke else 30)]:
        for h in range(2, 9 if smoke else 14):
            M = norm_pow_minus1(d, a, b, h)
            if M < 2: continue
            for p in factor(M) if M < 10**12 else []:
                if p > 2 and p % n and p < (1 << hi) * 4 and (1 << 8) < p:
                    hh = eig_orders(d, a, b, n, p)
                    if hh is not None and hh <= h:
                        pos.append((p, hh, h, hh/(math.log(p)/math.log(n))))
    pos_min = min((x[3] for x in pos), default=None)
    pos_small = sum(1 for x in pos if x[1] <= 12)
    gen_med = sorted(ratios)[len(ratios)//2] if ratios else None
    # net factor model: orbit size h, canonicalise = h-1 endomorphism applications (cost 1 each): net = sqrt(h)/h
    net_eq = [math.sqrt(x[1])/x[1] for x in pos if x[1] >= 2]
    net_max = max(net_eq, default=None)
    ok = (bound_viol == 0 and tested > 0 and pos_min is not None and net_max is not None
          and net_max <= 0.71 and gen_med is not None and gen_med > 20 * (pos_min or 1))
    if smoke and tested == 0: verdict = "inconclusive"
    elif bound_viol > 0: verdict = "falsified"
    else: verdict = "consistent_with_prediction" if ok else "inconclusive"
    print(json.dumps({
        "lead": "LEAD-STR-small-cm-disc-nonunit-endo", "smoke": smoke, "seed": 20261110,
        "params": {"log2N_range": [lo, hi], "n_primes": nprime, "n_elements": len(els)},
        "statistic": {"tested_pairs": tested, "bound_violations": bound_viol,
                      "generic_median_h_over_log_n_N": gen_med,
                      "null_random_lambda_median_ratio": sorted(null_ratios)[len(null_ratios)//2],
                      "positive_control_n": len(pos), "positive_control_min_ratio": pos_min,
                      "positive_control_small_h_count": pos_small,
                      "max_net_factor_sqrt_h_over_h": net_max},
        "controls": {"negative": "random lambda in (Z/N)*", "positive": "primes dividing Norm(alpha^h-1)",
                     "aa": "not applicable: exact integer arithmetic, no noise"},
        "verdict": verdict}))
    return 0

if __name__ == "__main__":
    sys.exit(main())
