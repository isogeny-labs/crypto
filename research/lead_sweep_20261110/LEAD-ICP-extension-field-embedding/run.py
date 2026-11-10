#!/usr/bin/env python3
"""LEAD-ICP-extension-field-embedding toy: Gaudry-style subfield factor base over F_{p^2}.
stdlib only, deterministic. --smoke = trivial sizes."""
import sys, json, random, math

def fm(u, v, p): return ((u[0]*v[0]-u[1]*v[1]) % p, (u[0]*v[1]+u[1]*v[0]) % p)
def finv(u, p):
    n = pow((u[0]*u[0]+u[1]*u[1]) % p, -1, p)
    return (u[0]*n % p, -u[1]*n % p)

class Curve:
    def __init__(s, p, A, B): s.p, s.A, s.B = p, A, B
    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        p = s.p; x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or y1 == (0, 0): return None
            num = fm(fm(x1, x1, p), (3, 0), p); num = ((num[0]+s.A) % p, num[1])
            den = ((2*y1[0]) % p, (2*y1[1]) % p)
        else:
            num = ((y2[0]-y1[0]) % p, (y2[1]-y1[1]) % p)
            den = ((x2[0]-x1[0]) % p, (x2[1]-x1[1]) % p)
        l = fm(num, finv(den, p), p)
        x3 = fm(l, l, p); x3 = ((x3[0]-x1[0]-x2[0]) % p, (x3[1]-x1[1]-x2[1]) % p)
        t = fm(l, ((x1[0]-x3[0]) % p, (x1[1]-x3[1]) % p), p)
        return (x3, ((t[0]-y1[0]) % p, (t[1]-y1[1]) % p))
    def mul(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R

def factor(n):
    f = {}; d = 2
    while d*d <= n:
        while n % d == 0: f[d] = f.get(d, 0)+1; n //= d
        d += 1
    if n > 1: f[n] = f.get(n, 0)+1
    return f

def setup(p, rng):
    """p = 3 mod 4. Returns curve, FB list (x in F_p, both signs), N2, G, r."""
    while True:
        A, B = rng.randrange(p), rng.randrange(p)
        if (4*A**3+27*B*B) % p == 0: continue
        fs = []; n1 = 1
        for x in range(p):
            f = (x**3+A*x+B) % p
            if f == 0: n1 += 1; continue
            if pow(f, (p-1)//2, p) == 1:
                y = pow(f, (p+1)//4, p); fs.append((x, (y, 0))); n1 += 2
            else:
                c = pow((-f) % p, (p+1)//4, p)   # (c*i)^2 = -c^2 = f
                fs.append((x, (0, c)))
        N2 = n1*(2*p+2-n1)
        C = Curve(p, A, B)
        FB = []
        for x, y in fs:
            FB.append(((x, 0), y)); FB.append(((x, 0), ((-y[0]) % p, (-y[1]) % p)))
        fac = factor(N2)
        re_ = [P for P in FB if P[1][1] == 0]; im_ = [P for P in FB if P[1][0] == 0]
        if not re_ or not im_: continue
        G = C.add(re_[rng.randrange(len(re_))], im_[rng.randrange(len(im_))])
        if G is None: continue
        r = N2
        for q in fac:
            while r % q == 0 and C.mul(r//q, G) is None: r //= q
        if r * 8 < N2 or r < 150: continue
        return C, FB, N2, G, r

def coverage(C, S, N2):
    seen = set(); n = len(S)
    for i in range(n):
        for j in range(i, n):
            R = C.add(S[i], S[j])
            if R is not None: seen.add(R)
    return len(seen)/N2

def rho_ops(C, G, r, rng):
    Q = C.mul(rng.randrange(1, r), G)
    tabs = []
    for _ in range(16):
        a, b = rng.randrange(r), rng.randrange(r)
        tabs.append((a, b, C.add(C.mul(a, G), C.mul(b, Q))))
    def step(st):
        P, a, b = st
        if P is None: i = 0
        else: i = (P[0][0]*31+P[0][1]) % 16
        ta, tb, T = tabs[i]
        return (C.add(P, T), (a+ta) % r, (b+tb) % r)
    while True:
        a0, b0 = rng.randrange(r), rng.randrange(r)
        s0 = (C.add(C.mul(a0, G), C.mul(b0, Q)), a0, b0)
        t = h = s0; ops = 0
        for _ in range(40*int(math.sqrt(r))+100):
            t = step(t); h = step(step(h)); ops += 3
            if t[0] == h[0]:
                if (t[1], t[2]) != (h[1], h[2]): return ops
                break

def fit(xs, ys):
    n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
    return sum((x-mx)*(y-my) for x, y in zip(xs, ys))/sum((x-mx)**2 for x in xs)

def main():
    smoke = "--smoke" in sys.argv
    primes = [67, 83] if smoke else [67, 103, 151, 211, 307, 431, 607]
    trials = 4 if smoke else 40
    rows = []
    for p in primes:
        rng = random.Random(20261110 + p)
        C, FB, N2, G, r = setup(p, rng)
        f_sub = coverage(C, FB, N2)
        # null: 2p random multiples of G (negation closed), same count
        null = []
        while len(null) < len(FB):
            P = C.mul(rng.randrange(1, r), G)
            if P is None or P[1] == (0, 0): continue
            null.append(P); null.append((P[0], ((-P[1][0]) % p, (-P[1][1]) % p)))
        f_null = coverage(C, null, N2)
        lam = len(FB)**2/2/N2
        ra = random.Random(1000+p); rb = random.Random(2000+p)
        A1 = sum(rho_ops(C, G, r, ra) for _ in range(trials))/trials
        A2 = sum(rho_ops(C, G, r, rb) for _ in range(trials))/trials
        rho = (A1+A2)/2
        ncols = p; att = ncols/f_sub
        nat = att*len(FB) + 2*ncols*ncols
        row = dict(p=p, N2=N2, r=r, fb=len(FB), f_sub=f_sub, f_null=f_null,
                   poisson_model=1-math.exp(-lam), rho_ops=rho, aa_ratio=A1/A2,
                   ic_ops=att, ic_native=nat)
        for w in (0, 0.1, 1):
            row["ratio_w%s" % w] = (att + w*nat)/rho
        rows.append(row)
    lx = [math.log(x["N2"]) for x in rows]
    slope = {w: fit(lx, [math.log(x["ratio_w%s" % w]) for x in rows]) for w in (0.1, 1)}
    aa_ok = all(0.7 < x["aa_ratio"] < 1.43 for x in rows)
    last = rows[-1]
    cross = all(x["ratio_w0.1"] > 1 and x["ratio_w1"] > 1 for x in rows[len(rows)//2:])
    if not aa_ok: v = "inconclusive"
    elif min(slope.values()) < 0.1 or (not smoke and last["ratio_w0.1"] < 1): v = "falsified"
    elif min(slope.values()) >= 0.4 and (cross or smoke): v = "consistent_with_prediction"
    else: v = "inconclusive"
    print(json.dumps(dict(lead="LEAD-ICP-extension-field-embedding", smoke=smoke,
        seeds="20261110+p; rho 1000+p/2000+p", primes=primes, trials=trials, rows=rows,
        slope_log_ratio_vs_logN={str(k): v2 for k, v2 in slope.items()},
        aa_ok=aa_ok, verdict=v)))
main()
