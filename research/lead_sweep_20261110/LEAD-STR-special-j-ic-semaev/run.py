import json, math, random, sys, statistics

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def rand_prime(rng, lo, hi, cond):
    while True:
        p = rng.randrange(lo, hi) | 1
        if is_prime(p) and cond(p): return p

def order(p, a, b, leg):
    return 1 + sum(1 + leg[(x * x * x + a * x + b) % p] for x in range(p))

def legendre(p):
    leg = [-1] * p
    leg[0] = 0
    for y in range(1, p): leg[y * y % p] = 1
    return leg

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def sqrt_mod(n, p):
    n %= p
    for y in range(p):
        pass
    return None

def find_point(a, b, p, rng, leg):
    while True:
        x = rng.randrange(1, p)
        if leg[(x**3 + a * x + b) % p] == 1:
            r = (x**3 + a * x + b) % p
            # Tonelli-Shanks-free: brute via pow when p%4==3 else general search
            if p % 4 == 3:
                y = pow(r, (p + 1) // 4, p)
            else:
                y = tonelli(r, p)
            return (x, y)

def tonelli(n, p):
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(n, q, p), pow(n, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1: tt = tt * tt % p; i += 1
        bb = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, bb * bb % p, t * bb * bb % p, r * bb % p
    return r

def make_curve(kind, rng, lo, hi):
    """return (N, lam) with lam the eigenvalue of a nontrivial automorphism (or -1)."""
    while True:
        if kind == "j0": p = rand_prime(rng, lo, hi, lambda q: q % 3 == 1)
        elif kind == "j1728": p = rand_prime(rng, lo, hi, lambda q: q % 4 == 1)
        else: p = rand_prime(rng, lo, hi, lambda q: True)
        leg = legendre(p)
        for _ in range(40):
            if kind == "j0": a, b = 0, rng.randrange(1, p)
            elif kind == "j1728": a, b = rng.randrange(1, p), 0
            else: a, b = rng.randrange(1, p), rng.randrange(1, p)
            if kind == "generic" and (4*a**3 + 27*b*b) % p == 0: continue
            N = order(p, a, b, leg)
            h = next((c for c in ((1, 2, 4) if kind == "j1728" else (1,)) if N % c == 0 and is_prime(N // c)), 0)
            if h and N // h > 100:
                N //= h
                G = find_point(a, b, p, rng, leg)
                for _ in range(h.bit_length() - 1): G = add(G, G, a, p)
                dlog = {}
                P = None
                for k in range(1, N):
                    P = add(P, G, a, p)
                    dlog[P] = k
                lam = N - 1
                if kind == "j0":
                    w = next(w for w in range(2, p) if pow(w, 3, p) == 1)
                    lam = dlog[(w * G[0] % p, G[1])]
                elif kind == "j1728":
                    i = next(i for i in range(2, p) if i * i % p == p - 1)
                    lam = dlog[((-G[0]) % p, i * G[1] % p)]
                return N, lam, p
        # retry with new p

def orbit(k, lam, size, N):
    gen = [k]
    if size == 2: return [k, N - k]
    out, x = set(), k
    for _ in range(size):  # group <-1, lam> ; lam order 3 (j0) or 4 (j1728)
        out.add(x); out.add(N - x); x = x * lam % N
    return list(out)

def coverage(N, lam, kind, B, rng, structured=False):
    size = {"j0": 6, "j1728": 4}.get(kind, 2)
    F = set()
    if structured:
        F = set(range(1, B // 2 + 1)) | set(N - k for k in range(1, B // 2 + 1))
    else:
        while len(F) < B:
            k = rng.randrange(1, N)
            o = orbit(k, lam, size, N)
            if len(o) != size or any(x in F for x in o): continue
            F.update(o)
    L = sorted(F)
    S = set()
    for i, a in enumerate(L):
        for b in L[i:]:
            S.add((a + b) % N)
    return len(S) / N, len(L)

def arm(kind, n, seed, lo, hi, structured=False, reps=3):
    rng = random.Random(seed)
    vals, info = [], []
    for _ in range(n):
        N, lam, p = make_curve(kind, rng, lo, hi)
        B = 12 * max(1, round(1.2 * math.sqrt(N) / 12))
        cs = []
        for _ in range(reps):
            c, b = coverage(N, lam, kind, B, rng, structured)
            cs.append(c / (1 - math.exp(-b * b / (2 * N))))  # normalise by random-model coverage
        vals.append(sum(cs) / reps); info.append((p, N, B))
    return vals, info

def summ(v): return statistics.mean(v), (statistics.stdev(v) / math.sqrt(len(v)) if len(v) > 1 else 0.0)

def main():
    smoke = "--smoke" in sys.argv
    lo, hi, n = (600, 1500, 3) if smoke else (40000, 200000, 12)
    arms = {}
    spec = [("generic_A", "generic", 101, False), ("generic_B_AA", "generic", 202, False),
            ("j0", "j0", 303, False), ("j1728", "j1728", 404, False),
            ("positive_control_structured_base", "generic", 505, True)]
    out_arms = {}
    for name, kind, seed, st in spec:
        v, info = arm(kind, n, seed, lo, hi, st, 1 if smoke else 3)
        m, se = summ(v)
        out_arms[name] = {"mean_norm_coverage": m, "se": se, "n_curves": n, "seed": seed,
                          "curves_p_N_B": info[:4]}
    A, AA = out_arms["generic_A"], out_arms["generic_B_AA"]
    aa_ratio = AA["mean_norm_coverage"] / A["mean_norm_coverage"]
    aa_se = math.hypot(A["se"], AA["se"]) / A["mean_norm_coverage"]
    band = max(3 * aa_se, 0.02)
    ratios = {}
    for k in ("j0", "j1728"):
        r = out_arms[k]["mean_norm_coverage"] / A["mean_norm_coverage"]
        se = math.hypot(out_arms[k]["se"], A["se"]) / A["mean_norm_coverage"]
        ratios[k] = {"ratio_to_generic": r, "se": se, "band": max(band, 3 * se)}
    pc = out_arms["positive_control_structured_base"]["mean_norm_coverage"] / A["mean_norm_coverage"]
    if abs(aa_ratio - 1) > band or pc > 0.5 or aa_se > 0.03:
        verdict = "inconclusive"
    elif all(abs(v["ratio_to_generic"] - 1) <= v["band"] for v in ratios.values()):
        verdict = "consistent_with_prediction"
    else:
        verdict = "falsified"
    print(json.dumps({"lead": "LEAD-STR-special-j-ic-semaev", "smoke": smoke, "statistic":
        "sumset coverage |F+F|/N of an Aut-orbit-closed factor base F (|F|=B fixed ~1.2 sqrt N), normalised by 1-exp(-B^2/2N), ratio to generic prime-order curves",
        "params": {"p_range": [lo, hi], "curves_per_arm": n, "B": "12*round(1.2 sqrt(N)/12)"},
        "arms": out_arms, "aa_ratio": aa_ratio, "aa_band": band, "ratios": ratios,
        "positive_control_ratio": pc, "verdict": verdict, "label": "pcc:unverified"}))

main()
