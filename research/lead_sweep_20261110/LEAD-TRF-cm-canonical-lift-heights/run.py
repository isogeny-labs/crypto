#!/usr/bin/env python3
"""LEAD-TRF-cm-canonical-lift-heights toy: does reduction of small-height rational
points of the CM curve y^2=x^3+17 (rank 2) cover E(F_p) faster than |S|/N (x6 for Aut)?
Stdlib only, deterministic."""
import sys, json, random, math

B = 17
G1, G2 = (-2, 3), (-1, 4)  # rational points on y^2=x^3+17, independent (rank 2)

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def add(P, Q, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = 3 * x1 * x1 * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def neg(P, p): return None if P is None else (P[0], (-P[1]) % p)

def count_points(p):
    sq = {}
    for y in range(p):
        sq.setdefault(y * y % p, []).append(y)
    n = 1
    for x in range(p):
        n += len(sq.get((x * x * x + B) % p, ()))
    return n

def sqrt_mod(a, p):
    a %= p
    if a == 0: return 0
    if pow(a, (p - 1) // 2, p) != 1: return None
    if p % 4 == 3: return pow(a, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2 * t2 % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r

def rand_point(rng, p):
    while True:
        x = rng.randrange(p)
        y = sqrt_mod(x ** 3 + B, p)
        if y is not None and y != 0:
            return (x, y if rng.random() < .5 else p - y)

def mult(P, k, p):
    R = None
    if k < 0: P, k = neg(P, p), -k
    while k:
        if k & 1: R = add(R, P, p)
        P = add(P, P, p); k >>= 1
    return R

def red_set(p, k, w):
    """reduction of {aG1+bG2 : |a|,|b|<=k} closed under the CM automorphism x->w x (w^3=1)."""
    g1 = (G1[0] % p, G1[1] % p); g2 = (G2[0] % p, G2[1] % p)
    S = set()
    row0 = mult(g1, -k, p)
    start = add(row0, mult(g2, -k, p), p)
    for a in range(-k, k + 1):
        R = add(mult(g1, a, p), mult(g2, -k, p), p)
        for b in range(-k, k + 1):
            if R is not None:
                S.add(R); S.add((R[0] * w % p, R[1])); S.add((R[0] * w * w % p, R[1]))
            R = add(R, g2, p)
    S.add(None)
    return S

def cube_root_unity(p):
    for g in range(2, p):
        w = pow(g, (p - 1) // 3, p)
        if w != 1: return w

def run_prime(p, frac, trials, seed):
    rng = random.Random(seed)
    N = count_points(p)
    w = cube_root_unity(p)
    k = max(2, int(math.sqrt(frac * N / 6) / 2))
    S = red_set(p, k, w)
    Sn = len(S)
    pred = Sn / N  # claimed bound Pr[Q in red(S u Aut S)] ~ |S|/N
    # arm CM: Q uniform on the curve (x-uniform sampling, y sign random) tested in S
    def rate(seed_off, mode):
        r = random.Random(seed * 1000 + seed_off); h = 0
        for _ in range(trials):
            if mode == "pos":
                Q = r.choice(sorted_S)
            else:
                Q = rand_point(r, p)
            h += Q in S
        return h / trials
    sorted_S = sorted(s for s in S if s is not None)
    # random control set of the same size (null): random points
    R = set()
    while len(R) < Sn - 1: R.add(rand_point(rng, p))
    r0 = random.Random(seed * 1000 + 7)
    nullh = sum(rand_point(r0, p) in R for _ in range(trials)) / trials
    h1, h2 = rate(1, "q"), rate(2, "q")
    pos = rate(3, "pos")
    # sampling by x is non-uniform on points only through y-sign (uniform), x-density ~ (1+chi)/..: fine at toy size
    sd = math.sqrt(max(pred * (1 - pred), 1e-12) / trials)
    return dict(p=p, N=N, k=k, S=Sn, pred=pred, hit_cm=h1, hit_cm_AA=h2, hit_null=nullh,
                hit_pos=pos, z_cm=(h1 - pred) / sd, z_AA=(h1 - h2) / (sd * math.sqrt(2)),
                z_null=(nullh - pred) / sd)

def main():
    smoke = "--smoke" in sys.argv
    sizes = [(1 << 8, 1)] if smoke else [(1 << 12, 6), (1 << 14, 5), (1 << 16, 4), (1 << 18, 2)]
    frac, trials = (0.1, 300) if smoke else (0.1, 20000)
    rows = []
    for base, cnt in sizes:
        p = base + 1; got = 0
        while got < cnt:
            p += 1
            if p % 3 == 1 and is_prime(p) and p > 7:
                rows.append(run_prime(p, frac, trials, seed=p)); got += 1
    n = len(rows)
    zs = [r["z_cm"] for r in rows]
    zbar = sum(zs) / n * math.sqrt(n)  # Stouffer
    ratio = sum(r["hit_cm"] for r in rows) / max(1e-12, sum(r["pred"] for r in rows))
    zaa = sum(r["z_AA"] for r in rows) / math.sqrt(n)
    posok = all(r["hit_pos"] == 1.0 for r in rows)
    # Prediction: ratio hit/pred <= 1.25 and Stouffer z < 4 (no enrichment); A/A |z|<4; positive control = 1.0
    if not posok or abs(zaa) > 4:
        v = "inconclusive"
    elif zbar > 4 and ratio > 1.25:
        v = "falsified"
    elif smoke:
        v = "inconclusive"
    else:
        v = "consistent_with_prediction"
    print(json.dumps(dict(lead="LEAD-TRF-cm-canonical-lift-heights", smoke=smoke, curve="y^2=x^3+17",
        generators=[G1, G2], seeds="seed=p per prime, +1/+2/+3/+7 offsets", frac=frac, trials=trials,
        stouffer_z_enrichment=zbar, pooled_hit_over_pred=ratio, stouffer_z_AA=zaa,
        positive_control_ok=posok, rows=rows, verdict=v)))
if __name__ == "__main__": main()
