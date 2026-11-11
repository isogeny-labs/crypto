#!/usr/bin/env python3
"""LEAD-STR-safecurves-parameter-audit: toy test that a SafeCurves twist criterion carries no ECDLP rho cost."""
import json, math, random, sys, time

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def rand_prime(rng, lo, hi):
    while True:
        p = rng.randrange(lo, hi) | 1
        if p > 3 and is_prime(p): return p

def largest_pf(n):
    f, m, d = 1, n, 2
    while d*d <= m:
        while m % d == 0: f, m = d, m//d
        d += 1 if d == 2 else 2
    return max(f, m) if m > 1 else f

def count_points(p, a, b, squares):
    c = 1
    for x in range(p):
        r = (x*x*x + a*x + b) % p
        if r == 0: c += 1
        elif r in squares: c += 2
    return c

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0: return None
        l = (3*P[0]*P[0] + a) * pow(2*P[1], -1, p) % p
    else:
        l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
    x = (l*l - P[0] - Q[0]) % p
    return (x, (l*(P[0]-x) - P[1]) % p)

def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R

def rand_point(rng, a, b, p):
    while True:
        x = rng.randrange(p); r = (x**3 + a*x + b) % p
        if r and pow(r, (p-1)//2, p) == 1 and p % 4 == 3:
            return (x, pow(r, (p+1)//4, p))
        if r and pow(r,(p-1)//2,p) == 1:
            for y in range(1, p):  # fallback for p%4==1 (rare path avoided by p%4==3 sampling)
                if y*y % p == r: return (x, y)

def rho_steps(rng, P, Q, N, a, p):
    """Floyd r-adding rho for k with Q=kP; returns total walk steps (3 per Floyd iter) incl. restarts."""
    total = 0
    while True:
        R = 16
        tab = []
        for _ in range(R):
            u, v = rng.randrange(N), rng.randrange(N)
            tab.append((u, v, add(mul(u,P,a,p), mul(v,Q,a,p), a, p)))
        def step(X, A, B):
            if X is None: j = 0
            else: j = (X[0] * 2654435761 >> 5) % R
            u, v, T = tab[j]
            return add(X, T, a, p), (A+u) % N, (B+v) % N
        a0, b0 = rng.randrange(N), rng.randrange(N)
        X0 = add(mul(a0,P,a,p), mul(b0,Q,a,p), a, p)
        x1, A1, B1 = X0, a0, b0; x2, A2, B2 = X0, a0, b0
        for it in range(1, 40*int(math.sqrt(N))+100):
            x1, A1, B1 = step(x1, A1, B1)
            x2, A2, B2 = step(*step(x2, A2, B2))
            total += 3
            if x1 == x2: break
        else:
            continue
        db = (B1 - B2) % N
        if db == 0: continue
        k = (A2 - A1) * pow(db, -1, N) % N
        if mul(k, P, a, p) == Q: return total

def boot_ci(x, y, rng, B=1000):
    ds = []
    for _ in range(B):
        mx = sum(rng.choice(x) for _ in x)/len(x); my = sum(rng.choice(y) for _ in y)/len(y)
        ds.append(mx-my)
    ds.sort()
    return ds[int(.025*B)], ds[int(.975*B)]

def mean(v): return sum(v)/len(v)

def collect(rng, lo, hi, n, budget_s, t0, tag):
    """Sample prime-order curves; classify twist; run one rho each. Returns list of dict rows."""
    rows = []
    while len(rows) < n and time.time() - t0 < budget_s:
        p = rand_prime(rng, lo, hi)
        if p % 4 != 3: continue
        sq = {x*x % p for x in range(1, p)}
        for _ in range(12):
            a, b = rng.randrange(p), rng.randrange(p)
            if (4*a**3 + 27*b*b) % p == 0: continue
            N = count_points(p, a, b, sq)
            if not is_prime(N) or N < 50: continue
            tw = 2*p + 2 - N
            lp = largest_pf(tw)
            cls = 'secure' if lp*4 >= tw else ('insecure' if lp*64 <= tw else 'mid')
            P = rand_point(rng, a, b, p)
            k = rng.randrange(1, N); Q = mul(k, P, a, p)
            s = rho_steps(rng, P, Q, N, a, p)
            rows.append(dict(p=p, N=N, cls=cls, steps=s, trace=p+1-N, tag=tag))
            break
    return rows

def main():
    smoke = '--smoke' in sys.argv
    t0 = time.time()
    seed = 20261110
    rng = random.Random(seed)
    if smoke: n, lo, hi, budget, margin = 6, 2**10, 2**11, 5, 0.2
    else: n, lo, hi, budget, margin = 150, 2**14, 2**17, 480, 0.2
    # main population (twist classes), then a small-size arm for the positive control
    main_rows = collect(rng, lo, hi, 4*n, budget*0.8, t0, 'main')
    small_rows = collect(rng, lo//4, hi//4, n, budget*0.98, t0, 'small')
    stat = lambda r: math.log(r['steps']/math.sqrt(r['N']))
    sec = [stat(r) for r in main_rows if r['cls']=='secure']
    ins = [stat(r) for r in main_rows if r['cls']=='insecure']
    # A/A: random halves of secure arm
    sh = sec[:]; rng.shuffle(sh); h = len(sh)//2
    aa1, aa2 = sh[:h], sh[h:]
    out = dict(lead='LEAD-STR-safecurves-parameter-audit', seed=seed, smoke=smoke,
               params=dict(p_range=[lo, hi], target_n_per_arm=n, margin_log=margin, rho='Floyd 16-adding', curves='y^2=x^3+ax+b, p=3 mod 4, prime order'),
               n_secure=len(sec), n_insecure=len(ins), n_prime_order=len(main_rows))
    out['audit_tally'] = dict(anomalous_trace1=sum(1 for r in main_rows if r['trace']==1),
                              mid_twist=sum(1 for r in main_rows if r['cls']=='mid'))
    ok = len(sec) >= 3 and len(ins) >= 3 and len(aa1) >= 2 and len(aa2) >= 2 and len(small_rows) >= 3
    if ok:
        brng = random.Random(seed+1)
        d = mean(ins) - mean(sec); ci = boot_ci(ins, sec, brng)
        out['primary'] = dict(diff_log=d, ci95=ci)
        d2 = mean(aa1)-mean(aa2); ci2 = boot_ci(aa1, aa2, brng)
        out['AA'] = dict(diff_log=d2, ci95=ci2)
        big = [math.log(r['steps']) for r in main_rows]; sm = [math.log(r['steps']) for r in small_rows]
        dp = mean(big)-mean(sm); cip = boot_ci(big, sm, brng)
        expected = 0.5*(mean([math.log(r['N']) for r in main_rows]) - mean([math.log(r['N']) for r in small_rows]))
        out['positive_control'] = dict(raw_log_steps_diff=dp, ci95=cip, expected_sqrtN_diff=expected, detected=(cip[0] > 0))
        equiv = ci[0] > -margin and ci[1] < margin
        aa_ok = ci2[0] > -margin and ci2[1] < margin
        pc = cip[0] > 0
        if not (aa_ok and pc): v = 'inconclusive'
        elif equiv: v = 'consistent_with_prediction'
        elif ci[0] > 0 or ci[1] < 0: v = 'falsified' if abs(d) > 0.1 else 'inconclusive'
        else: v = 'inconclusive'
    else:
        v = 'inconclusive'
    out['verdict'] = v
    out['elapsed_s'] = round(time.time()-t0, 2)
    print(json.dumps(out))
    return 0

if __name__ == '__main__':
    sys.exit(main())
