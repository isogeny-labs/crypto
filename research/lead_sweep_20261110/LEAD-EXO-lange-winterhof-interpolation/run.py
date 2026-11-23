#!/usr/bin/env python3
"""LEAD-EXO-lange-winterhof-interpolation: degree of the interpolant of k -> x(kG)-labelling. stdlib only."""
import json, random, sys, time

def is_prime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True

def curve_points_count(p, a, b):
    t = 0
    for x in range(p):
        r = (x*x*x + a*x + b) % p
        if r == 0: t += 1
        elif pow(r, (p-1)//2, p) == 1: t += 2
    return t + 1

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2 and (y1 + y2) % p == 0: return None
    if P == Q: l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
    else: l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l*l - x1 - x2) % p
    return (x3, (l*(x1 - x3) - y1) % p)

def find_curve(p, rng):
    for _ in range(4000):
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        n = curve_points_count(p, a, b)
        if n > 4 and is_prime(n):
            # find a point
            while True:
                x = rng.randrange(p); r = (x**3 + a*x + b) % p
                if r and pow(r, (p-1)//2, p) == 1:
                    for y in range(p):
                        if y*y % p == r: return a, b, n, (x, y)
    return None

def xs_of(p, a, n, G):
    M = (n-1)//2
    xs = []; P = None
    for _ in range(M):
        P = add(P, G, a, p); xs.append(P[0])
    return xs

def interp_degree(xs, ys, p):
    """degree of the unique interpolant through (xs,ys) over F_p (Newton divided differences)."""
    n = len(xs); c = list(ys); deg = 0 if c[0] or n == 1 else -1
    top = -1
    if c[0] % p: top = 0
    for j in range(1, n):
        for i in range(n-1, j-1, -1):
            c[i] = (c[i] - c[i-1]) * pow(xs[i] - xs[i-j], -1, p) % p
        if c[j] % p: top = j
    return top

def run(primes, seed, tag):
    rng = random.Random(seed); rows = []
    for p in primes:
        cv = find_curve(p, rng)
        if cv is None: continue
        a, b, n, G = cv
        xs = xs_of(p, a, n, G); M = len(xs)
        if len(set(xs)) != M: continue
        real = [k % p for k in range(1, M+1)]
        def labels_null():
            l = real[:]; rng.shuffle(l); return l
        planted = [(3*x**5 + 7*x**2 + 1) % p for x in xs]  # degree-5 positive control
        dr = interp_degree(xs, real, p)
        dn = [interp_degree(xs, labels_null(), p) for _ in range(2)]  # null + A/A
        dp = interp_degree(xs, planted, p)
        rows.append(dict(p=p, a=a, b=b, n=n, M=M, deficit_real=M-1-dr,
                         deficit_null=[M-1-d for d in dn], deg_planted=dp))
    return rows

def main():
    smoke = "--smoke" in sys.argv; t0 = time.time()
    primes = [p for p in range(40, 60) if is_prime(p)][:2] if smoke else \
             [p for p in range(300, 1100) if is_prime(p)][::25][:12]
    seed = 20261110
    rows = run(primes, seed, "lw")
    ok_pos = all(r["deg_planted"] == 5 for r in rows) and rows != []
    real_def = [r["deficit_real"] for r in rows]
    null_def = [d for r in rows for d in r["deficit_null"]]
    if not ok_pos: verdict = "inconclusive"
    elif max(real_def) >= 2 or sum(1 for d in real_def if d >= 1) > 1 + len(real_def)//4: verdict = "falsified"
    else: verdict = "consistent_with_prediction"
    print(json.dumps(dict(lead="LEAD-EXO-lange-winterhof-interpolation", smoke=smoke, seed=seed,
        primes=primes, statistic="deficit = (M-1) - deg of F_p interpolant of k->x(kG), M=(n-1)/2",
        real_deficits=real_def, null_aa_deficits=null_def, positive_control_ok=ok_pos,
        threshold="falsified if any real deficit>=2 or >25%+1 curves have deficit>=1",
        rows=rows, elapsed_s=round(time.time()-t0, 2), verdict=verdict)))
main()
