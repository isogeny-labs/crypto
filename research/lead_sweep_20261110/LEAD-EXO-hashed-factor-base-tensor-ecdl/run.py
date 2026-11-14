#!/usr/bin/env python3
"""Toy scaling test of exact relation-collection cost (hashed factor base, m=2) vs sqrt(N). stdlib only."""
import json, math, random, sys, time

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13):
        if n % q == 0: return n == q
    i = 17
    while i*i <= n:
        if n % i == 0: return False
        i += 2
    return True

def mkcurve(rng, bits):
    while True:
        p = rng.randrange(1 << (bits-1), 1 << bits) | 1
        if p > 5 and is_prime(p): break
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p: break
    # point counting by QR table
    sq = bytearray(p)
    for y in range(p): sq[y*y % p] = 1
    pts = []
    N = 1
    for x in range(p):
        f = (x*x*x + a*x + b) % p
        if f == 0: N += 1
        elif sq[f]: N += 2
    return p, a, b, N

def sqrtmod(f, p):
    # Tonelli-Shanks (f known QR)
    if p % 4 == 3: return pow(f, (p+1)//4, p)
    q, s = p-1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(f, q, p), pow(f, (q+1)//2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        bb = pow(c, 1 << (m-i-1), p)
        m, c, t, r = i, bb*bb % p, t*bb*bb % p, r*bb % p
    return r

class EC:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b; s.ops = 0
    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        p = s.p; s.ops += 1
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            l = (3*P[0]*P[0] + s.a) * pow(2*P[1], -1, p) % p
        else:
            l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
        x = (l*l - P[0] - Q[0]) % p
        return (x, (l*(P[0]-x) - P[1]) % p)
    def neg(s, P): return None if P is None else (P[0], (-P[1]) % s.p)
    def mul(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R

def rand_point(E, rng):
    p = E.p
    while True:
        x = rng.randrange(p); f = (x**3 + E.a*x + E.b) % p
        if f and pow(f, (p-1)//2, p) == 1: return (x, sqrtmod(f, p))

def relations(E, rng, B, Bmax_trials):
    """FB = points with the B smallest valid x (hashed by x). Trial: random R; success if x(R-f_i) in FB
    for some i (m=2 decomposition up to sign). Cost per trial = B subtractions (counted). Returns trials to B relations."""
    p = E.p; fb = {}; x = 0
    while len(fb) < B:
        f = (x**3 + E.a*x + E.b) % p
        if f and pow(f, (p-1)//2, p) == 1: fb[x] = (x, sqrtmod(f, p))
        x += 1
    fbl = list(fb.values()); rels = 0; trials = 0
    while rels < B and trials < Bmax_trials:
        trials += 1
        R = rand_point(E, rng)
        for f in fbl:
            D = E.add(R, E.neg(f))
            if D is not None and D[0] in fb:
                rels += 1; break
    E.ops += 0
    return trials, rels

def rho_steps(E, P, n, rng):
    # Teske-style 16-partition walk with collision dict (counted steps = group ops), same instrument for all arms
    M = [(rng.randrange(1, n), rng.randrange(1, n)) for _ in range(16)]
    Mp = [E.add(E.mul(a, P), E.mul(b, P)) for a, b in M]
    x = E.mul(rng.randrange(1, n), P); seen = {}; steps = 0
    while True:
        key = x
        if key in seen: return steps
        seen[key] = 1
        j = (x[0] if x else 0) % 16
        x = E.add(x, Mp[j]); steps += 1

def lsq_slope(xs, ys):
    mx, my = sum(xs)/len(xs), sum(ys)/len(ys)
    return sum((a-mx)*(b-my) for a, b in zip(xs, ys)) / sum((a-mx)**2 for a in xs)

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    sizes = [10] if smoke else [12, 14, 16, 18]
    ncurves = 1 if smoke else 3
    seed = 20261110
    rows = []
    for bits in sizes:
        Cs, Cpos, rr, rr2 = [], [], [], []
        for c in range(ncurves):
            rng = random.Random(seed*1000 + bits*10 + c)
            p, a, b, N = mkcurve(rng, bits)
            E = EC(p, a, b); sN = math.sqrt(N)
            best = None
            for frac in (0.25, 0.5, 1.0):  # B = frac*sqrt(N)*k grid
                for k in (1, 2, 4):
                    B = max(4, int(frac*k*sN/2))
                    trials, rels = relations(E, rng, B, 40*N//max(B, 1) + 50)
                    if rels < B:  # estimate trials to B rels from observed rate
                        trials = trials * B / max(rels, 1)
                    cost = trials*B + 3*B*B  # trial subtractions + sparse linear algebra (m=2, ~3 B^2)
                    if best is None or cost < best: best = cost
            Cs.append(best/sN)
            Cpos.append(best/sN/(N**0.4))  # planted advantage: cost divided by N^0.4
            # rho arms (null / A/A) use two independent seeds, order of the point P
            P = rand_point(E, rng); n = N  # use whole-group scale; walk collision ~ sqrt(pi*N/2) if P generates
            rr.append(rho_steps(E, P, N, random.Random(seed+1+c))/sN)
            rr2.append(rho_steps(E, P, N, random.Random(seed+2+c))/sN)
        med = lambda v: sorted(v)[len(v)//2]
        rows.append({"bits": bits, "medC_IC": med(Cs), "medC_planted": med(Cpos),
                     "med_rho_over_sqrtN": med(rr), "med_rho_AA_over_sqrtN": med(rr2), "N_log2": bits})
    xs = [r["N_log2"] for r in rows]
    slope = lsq_slope(xs, [math.log2(r["medC_IC"]) for r in rows]) if len(rows) > 1 else None
    slope_pos = lsq_slope(xs, [math.log2(r["medC_planted"]) for r in rows]) if len(rows) > 1 else None
    slope_rho = lsq_slope(xs, [math.log2(r["med_rho_over_sqrtN"]) for r in rows]) if len(rows) > 1 else None
    lastC = rows[-1]["medC_IC"]
    if slope is None:
        verdict = "inconclusive"
    elif lastC < 1 or slope <= 0.0:
        verdict = "falsified"
    elif all(r["medC_IC"] >= 1 for r in rows) and slope >= 0.25 and slope_pos < 0.25 and abs(slope_rho) < 0.1:
        verdict = "consistent_with_prediction"
    else:
        verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-EXO-hashed-factor-base-tensor-ecdl", "smoke": smoke, "seed": seed,
        "params": {"sizes_bits": sizes, "curves_per_size": ncurves, "decomposition_m": 2, "B_grid": "frac*k*sqrt(N)/2"},
        "rows": rows, "slope_log2C_vs_log2N": slope, "positive_control_slope": slope_pos,
        "negative_control_rho_slope": slope_rho, "thresholds": {"consistent": "slope>=0.25 and all C>=1",
        "falsified": "C<1 at largest size or slope<=0", "positive_control_must_be_below": 0.25},
        "verdict": verdict, "elapsed_s": round(time.time()-t0, 2)}))
main()
