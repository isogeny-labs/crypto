#!/usr/bin/env python3
"""Toy re-derivation of the sqrt(|Aut|)=sqrt(6) ceiling for j=0 (secp256k1 class). stdlib only."""
import json, math, random, sys

def isprime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13):
        if n % q == 0: return n == q
    i = 17
    while i*i <= n:
        if n % i == 0: return False
        i += 2
    return True

def next_prime_1mod3(n):
    n += (1 - n) % 6 if n % 6 != 1 else 0
    while not isprime(n): n += 6
    return n

def find_curve(p, a, rng, want_j0):
    sq = bytearray(p); 
    for x in range(1, p): sq[x*x % p] = 1
    for _ in range(4000):
        b = rng.randrange(1, p)
        cnt = 1
        for x in range(p):
            v = (x*x*x + a*x + b) % p
            cnt += 1 if v == 0 else (2 if sq[v] else 0)
        if isprime(cnt) and cnt > 100:
            return b, cnt
    raise RuntimeError("no prime-order curve")

class Curve:
    def __init__(s, p, a, b, N):
        s.p, s.a, s.b, s.N = p, a, b, N
    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        p = s.p
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            l = (3*P[0]*P[0] + s.a) * pow(2*P[1], -1, p) % p
        else:
            l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
        x = (l*l - P[0] - Q[0]) % p
        return (x, (l*(P[0]-x) - P[1]) % p)
    def mul(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R
    def rand_point(s, rng):
        while True:
            x = rng.randrange(s.p); v = (x**3 + s.a*x + s.b) % s.p
            y = pow(v, (s.p+1)//4, s.p) if s.p % 4 == 3 else None
            if y is None:
                for y in range(s.p):
                    if y*y % s.p == v: break
                else: continue
            if y*y % s.p == v: return (x, y)

def sqrt_mod(v, p):
    # Tonelli-Shanks
    if v == 0: return 0
    if pow(v, (p-1)//2, p) != 1: return None
    q, s = p-1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(v, q, p), pow(v, (q+1)//2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        b = pow(c, 1 << (m-i-1), p)
        m, c, t, r = i, b*b % p, t*b*b % p, r*b % p
    return r

def point(cv, rng):
    while True:
        x = rng.randrange(cv.p); y = sqrt_mod((x**3 + cv.a*x + cv.b) % cv.p, cv.p)
        if y is not None: return (x, y)

def make_canon(cv, mode, omega, lam):
    p, N = cv.p, cv.N
    def canon(X, a, b):
        best = (X, a, b)
        if mode == 'plain': return best
        c = [(X, a, b), ((X[0], (-X[1]) % p), (-a) % N, (-b) % N)]
        if mode == 'orb6':
            x, y, aa, bb = X[0], X[1], a, b
            for _ in range(2):
                x = x*omega % p; aa = aa*lam % N; bb = bb*lam % N
                c.append(((x, y), aa, bb)); c.append(((x, (-y) % p), (-aa) % N, (-bb) % N))
        return min(c, key=lambda t: t[0])
    return canon

def rho(cv, P, Q, d, canon, rng, cap, r=16):
    N = cv.N
    R = []
    for _ in range(r):
        u, v = rng.randrange(1, N), rng.randrange(1, N)
        R.append((cv.add(cv.mul(u, P), cv.mul(v, Q)), u, v))
    seen = {}
    steps = 0
    while steps < cap:
        a, b = rng.randrange(N), rng.randrange(N)
        X = cv.add(cv.mul(a, P), cv.mul(b, Q))
        if X is None: continue
        X, a, b = canon(X, a, b)
        trail = set()
        while steps < cap:
            if X in seen:
                a2, b2 = seen[X]
                if (a2 - a) % N != 0 or (b2 - b) % N != 0:
                    if (b2 - b) % N != 0:
                        dd = (a - a2) * pow(b2 - b, -1, N) % N
                        if dd == d: return steps
                break  # trivial collision / bad: restart
            seen[X] = (a, b); trail.add(X)
            Rj = R[X[0] % r]
            Y = cv.add(X, Rj[0]); steps += 1
            if Y is None: break
            X, a, b = canon(Y, (a + Rj[1]) % N, (b + Rj[2]) % N)
    return None

def run_arm(cv, mode, omega, lam, trials, seed):
    rng = random.Random(seed)
    P = point(cv, rng)
    canon = make_canon(cv, mode, omega, lam)
    out, fails = [], 0
    for t in range(trials):
        d = rng.randrange(2, cv.N)
        Q = cv.mul(d, P)
        s = rho(cv, P, Q, d, canon, rng, cap=int(60*math.sqrt(cv.N)))
        if s is None: fails += 1; out.append(int(60*math.sqrt(cv.N)))
        else: out.append(s)
    return sum(out)/len(out), fails

def main():
    smoke = '--smoke' in sys.argv
    bits = [10] if smoke else [14, 16, 18]
    trials = 6 if smoke else 60
    ncurves = 1 if smoke else 2
    rng = random.Random(20261110)
    sqrt6 = math.sqrt(6)
    j0 = []; gen = []
    for B in bits:
        p = next_prime_1mod3(1 << B)
        p += 0
        while p % 4 != 3 or not isprime(p) or p % 3 != 1:
            p = next_prime_1mod3(p + 1)
        for _ in range(ncurves):
            b, N = find_curve(p, 0, rng, True)
            cv = Curve(p, 0, b, N)
            P = point(cv, rng)
            # find omega (order 3 in F_p) and matching lambda on the order-N group
            g = 2
            while pow(g, (p-1)//3, p) == 1: g += 1
            omega = pow(g, (p-1)//3, p)
            Pm = cv.mul(rng.randrange(1, N), P)
            Pw = (Pm[0]*omega % p, Pm[1])
            lam = next(l for l in range(2, N) if (pow(l, 3, N) == 1 and l != 1 and cv.mul(l, Pm) == Pw)) if N < 1 else None
            if lam is None:
                # solve lam^2+lam+1=0 mod N by Tonelli (N prime, N=1 mod 3)
                s3 = sqrt_mod((-3) % N, N); inv2 = pow(2, -1, N)
                for cand in ((-1+s3)*inv2 % N, (-1-s3)*inv2 % N):
                    if cv.mul(cand, Pm) == Pw: lam = cand
            j0.append((B, cv, omega, lam))
        # generic (a != 0) control curve, same size
        a = 1
        while True:
            b, N = find_curve(p, a, rng, False)
            break
        gen.append((B, Curve(p, a, b, N)))
    res = {"j0": [], "generic": []}
    pl, ob, ng, pl2 = [], [], [], []
    for i, (B, cv, om, lam) in enumerate(j0):
        s = 1000 + i
        m_plain, f1 = run_arm(cv, 'plain', om, lam, trials, s)
        m_plain2, f2 = run_arm(cv, 'plain', om, lam, trials, s + 500)   # A/A
        m_neg, f3 = run_arm(cv, 'neg', om, lam, trials, s + 1)           # positive control sqrt2
        m_orb, f4 = run_arm(cv, 'orb6', om, lam, trials, s + 2)
        res["j0"].append(dict(bits=B, p=cv.p, N=cv.N, plain=m_plain, plain_AA=m_plain2, neg=m_neg, orb6=m_orb, fails=[f1, f2, f3, f4]))
        pl.append(m_plain); pl2.append(m_plain2); ng.append(m_neg); ob.append(m_orb)
    gneg, gpl = [], []
    for i, (B, cv) in enumerate(gen):
        s = 2000 + i
        m_p, _ = run_arm(cv, 'plain', 1, 1, trials, s)
        m_n, _ = run_arm(cv, 'neg', 1, 1, trials, s + 1)  # null: generic curve has only +-1, no sqrt6
        res["generic"].append(dict(bits=B, N=cv.N, plain=m_p, neg=m_n)); gneg.append(m_p/m_n)
    mean = lambda v: sum(v)/len(v)
    R6 = mean(pl)/mean(ob); R2 = mean(pl)/mean(ng); AA = mean(pl)/mean(pl2); G2 = mean(gneg)
    ctrl_ok = (0.7*math.sqrt(2) <= R2 <= 1.15*math.sqrt(2)) and (0.8 <= AA <= 1.25) and (0.7*math.sqrt(2) <= G2 <= 1.15*math.sqrt(2))
    lo, hi = 0.75*sqrt6, 1.10*sqrt6
    if not ctrl_ok: verdict = "inconclusive"
    elif R6 > hi: verdict = "falsified"
    elif R6 >= lo: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    if smoke: verdict = "inconclusive" if verdict != "falsified" else verdict
    print(json.dumps(dict(lead="LEAD-STR-secp256k1-cm-class-audit", smoke=smoke, statistic_R6_plain_over_orb6=R6,
        sqrt6=sqrt6, band=[lo, hi], R2_positive_control=R2, AA_ratio=AA, generic_neg_ratio=G2, controls_ok=ctrl_ok,
        seeds=dict(master=20261110, arms="1000+i..,2000+i.."), params=dict(bits=bits, trials=trials, ncurves=ncurves, walk_r=16),
        rows=res, verdict=verdict, label="[pcc:unverified]")))
main()
