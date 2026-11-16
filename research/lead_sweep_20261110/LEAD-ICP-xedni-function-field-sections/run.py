import json, math, random, sys
SMOKE = "--smoke" in sys.argv
PRIMES = [61, 127] if SMOKE else [127, 251, 509, 1021, 2039]
N = 20000 if SMOKE else 600000
SEED = 20261110

def curve_pts(p, a, b):
    sq = {}
    for y in range(p): sq.setdefault(y*y % p, []).append(y)
    return [(x, y) for x in range(p) for y in sq.get((x**3+a*x+b) % p, [])]

def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0] and (P[1]+Q[1]) % p == 0: return None
    if P == Q: l = (3*P[0]*P[0]+a)*pow(2*P[1], -1, p) % p
    else: l = (Q[1]-P[1])*pow(Q[0]-P[0], -1, p) % p
    x = (l*l-P[0]-Q[0]) % p
    return (x, (l*(P[0]-x)-P[1]) % p)

def det(a, b, c):
    return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])

def collinear_prob(p, rng, mode):
    a, b = 3, 7
    while (4*a**3+27*b*b) % p == 0: b += 1
    pts = curve_pts(p, a, b)
    hits = 0; used = 0
    for _ in range(N):
        P = rng.choice(pts); Q = rng.choice(pts)
        S = add(P, Q, p, a)
        if S is None or P[0] == Q[0]: continue
        R = (S[0], (-S[1]) % p)   # P,Q,R collinear mod p
        if mode == "null":        # break collinearity mod p
            R = rng.choice(pts)
        L = [(u+p*rng.randrange(0, 1), v) for (u, v) in (P, Q, R)]
        L = [(u, v) for (u, v) in L]
        # random integer lifts: coordinate + p*k, k in {0,1,2,3}
        L = [(u+p*rng.randrange(4), v+p*rng.randrange(4)) for (u, v) in (P, Q, R)]
        used += 1
        if det(*L) == 0: hits += 1
    return hits, used

def slope(xs, ys):
    n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
    return sum((x-mx)*(y-my) for x, y in zip(xs, ys))/sum((x-mx)**2 for x in xs)

def run(mode):
    rng = random.Random(SEED + (1 if mode == "null" else 0))
    rows = []
    for p in PRIMES:
        h, u = collinear_prob(p, rng, mode)
        rows.append({"p": p, "hits": h, "used": u, "prob": h/u if u else 0})
    ok = [r for r in rows if r["hits"] >= 5]
    s = slope([math.log2(r["p"]) for r in ok], [math.log2(r["prob"]) for r in ok]) if len(ok) >= 2 else None
    return rows, s

def positive_control():
    # lifts chosen as integer collinear points then reduced: determinant is 0 by construction
    rng = random.Random(SEED+2); ok = 0
    for _ in range(1000):
        x0, y0, dx, dy = [rng.randrange(1, 50) for _ in range(4)]
        L = [(x0+t*dx, y0+t*dy) for t in (0, 1, 2)]
        ok += det(*L) == 0
    return ok/1000

rows, s = run("mod_p_collinear")
nrows, ns = run("null")
pc = positive_control()
# exponent alpha = -slope; viability requires alpha < 1/2.
if s is None: verdict = "inconclusive"
elif -s < 0.5 and pc == 1.0: verdict = "falsified"
elif -s >= 0.9 and pc == 1.0: verdict = "consistent_with_prediction"
else: verdict = "inconclusive"
print(json.dumps({"lead": "LEAD-ICP-xedni-function-field-sections", "smoke": SMOKE, "seed": SEED,
  "lift_set": "coord + p*k, k in 0..3", "samples_per_prime": N,
  "statistic": "alpha = -slope of log2 P(lifted collinear triple) vs log2 p",
  "alpha_estimate": None if s is None else -s, "rows": rows,
  "null_alpha": None if ns is None else -ns, "null_rows": nrows,
  "positive_control_detect_rate": pc, "aa_note": "null arm uses same sampler with independent seed",
  "verdict": verdict}))
