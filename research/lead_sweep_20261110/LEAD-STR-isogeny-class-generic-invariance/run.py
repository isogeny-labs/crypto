import sys, json, math, random
SMOKE = "--smoke" in sys.argv
P_BITS = 10 if SMOKE else 14
RUNS = 40 if SMOKE else 1500
NCURVES = 3 if SMOKE else 8
MARGIN = 0.08           # equivalence band: ratio in [1/1.08, 1.08]
SEED = 20261110

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def nextprime(n):
    while not isprime(n): n += 1
    return n

def tables(p):
    chi = [0] * p
    root = {}
    for y in range(1, p):
        s = y * y % p
        chi[s] = 1
        root.setdefault(s, y)
    for s in range(p):
        if chi[s] == 0 and s != 0: chi[s] = -1
    return chi, root

def count(p, chi, a, b):
    return p + 1 + sum(chi[(x * x * x + a * x + b) % p] for x in range(p))

def jinv(p, a, b):
    d = (4 * a ** 3 + 27 * b * b) % p
    if d == 0: return None
    return 1728 * 4 * a ** 3 * pow(d, -1, p) % p

class Curve:
    def __init__(s, p, a, b, N, root):
        s.p, s.a, s.b, s.N, s.root = p, a, b, N, root
    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        p = s.p
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            l = (3 * P[0] * P[0] + s.a) * pow(2 * P[1], -1, p) % p
        else:
            l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
        x = (l * l - P[0] - Q[0]) % p
        return (x, (l * (P[0] - x) - P[1]) % p)
    def mul(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R
    def point(s, rng):
        while True:
            x = rng.randrange(s.p)
            f = (x ** 3 + s.a * x + s.b) % s.p
            if f in s.root: return (x, s.root[f])

def rho_steps(C, rng):
    """total walk steps until a verified discrete log is found."""
    N = C.N
    P = C.point(rng)
    k = rng.randrange(1, N)
    Q = C.mul(k, P)
    jumps = []
    for _ in range(16):
        u, v = rng.randrange(1, N), rng.randrange(1, N)
        jumps.append((u, v, C.add(C.mul(u, P), C.mul(v, Q))))
    total = 0
    while True:
        a, b = rng.randrange(N), rng.randrange(N)
        R = C.add(C.mul(a, P), C.mul(b, Q))
        seen = {}
        while True:
            if R is None: break
            if R in seen:
                a2, b2 = seen[R]
                db = (b2 - b) % N
                if db:
                    kk = (a - a2) * pow(db, -1, N) % N
                    if C.mul(kk, P) == Q: return total
                break
            seen[R] = (a, b)
            u, v, M = jumps[R[0] & 15]
            R = C.add(R, M); a = (a + u) % N; b = (b + v) % N
            total += 1

def ci_ratio(xs, ys, z):
    """mean(xs)/mean(ys) with delta-method CI (independent samples)."""
    def ms(v):
        m = sum(v) / len(v)
        return m, sum((t - m) ** 2 for t in v) / (len(v) - 1) / len(v)
    mx, vx = ms(xs); my, vy = ms(ys)
    r = mx / my
    se = r * math.sqrt(vx / mx ** 2 + vy / my ** 2)
    return r, r - z * se, r + z * se

def main():
    rng = random.Random(SEED)
    p = nextprime(1 << P_BITS)
    chi, root = tables(p)
    # target trace: smallest |t| giving prime order N, reachable
    cands = [(abs(t), t) for t in range(-int(2 * p ** .5), int(2 * p ** .5) + 1)
             if isprime(p + 1 - t)]
    cands.sort()
    target = None; curves = []
    seen_j = set()
    for _, t in cands:
        N = p + 1 - t
        curves = []; seen_j = set(); a = 1
        tries = 0
        while len(curves) < NCURVES and tries < (400 if SMOKE else 4000):
            b = rng.randrange(1, p); a = rng.randrange(1, p); tries += 1
            j = jinv(p, a, b)
            if j is None or j in seen_j: continue
            if count(p, chi, a, b) == N:
                seen_j.add(j); curves.append(Curve(p, a, b, N, root))
        if len(curves) >= NCURVES:
            target = N; break
    if target is None:
        print(json.dumps({"verdict": "inconclusive", "error": "no class found"})); return
    # positive control: different prime, ~2.25x larger order
    p2 = nextprime(int(2.25 * p))
    chi2, root2 = tables(p2)
    pc = None
    while pc is None:
        a, b = rng.randrange(1, p2), rng.randrange(1, p2)
        if jinv(p2, a, b) is None: continue
        n2 = count(p2, chi2, a, b)
        if isprime(n2): pc = Curve(p2, a, b, n2, root2)
    samples = []
    for i, C in enumerate(curves):
        r = random.Random(SEED * 1000 + i)
        samples.append([rho_steps(C, r) for _ in range(RUNS)])
    rp = random.Random(SEED * 1000 + 99)
    pos = [rho_steps(pc, rp) for _ in range(RUNS)]
    # A/A: two halves of one curve's independent seed streams
    ra = random.Random(SEED * 1000 + 77); rb = random.Random(SEED * 1000 + 78)
    aa1 = [rho_steps(curves[0], ra) for _ in range(RUNS)]
    aa2 = [rho_steps(curves[0], rb) for _ in range(RUNS)]
    lo, hi = 1 / (1 + MARGIN), 1 + MARGIN
    rows = []
    for i, s in enumerate(samples):
        others = [t for j, v in enumerate(samples) if j != i for t in v]
        r, l90, h90 = ci_ratio(s, others, 1.645)
        _, l99, h99 = ci_ratio(s, others, 2.576)
        rows.append({"curve": i, "a": curves[i].a, "b": curves[i].b,
                     "j": jinv(p, curves[i].a, curves[i].b),
                     "mean_steps": sum(s) / len(s), "ratio_vs_rest": r,
                     "ci90": [l90, h90], "equivalent": l90 > lo and h90 < hi,
                     "excluded": l99 > hi or h99 < lo})
    r, l90, h90 = ci_ratio(aa1, aa2, 1.645)
    aa = {"ratio": r, "ci90": [l90, h90], "equivalent": l90 > lo and h90 < hi}
    prr, pl, ph = ci_ratio(pos, samples[0], 2.576)
    expected = math.sqrt(pc.N / target)
    pos_ok = pl > hi
    all_eq = all(x["equivalent"] for x in rows)
    any_ex = any(x["excluded"] for x in rows)
    if SMOKE: verdict = "inconclusive"   # sizes too small to decide
    elif not pos_ok or not aa["equivalent"]: verdict = "inconclusive"
    elif all_eq: verdict = "consistent_with_prediction"
    elif any_ex: verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-STR-isogeny-class-generic-invariance",
        "smoke": SMOKE, "seed": SEED, "p": p, "N_class": target,
        "n_curves": len(curves), "runs_per_curve": RUNS, "margin": MARGIN,
        "class_curves": rows, "a_a": aa,
        "positive_control": {"p": p2, "N": pc.N, "expected_ratio_sqrt": expected,
            "ratio_vs_curve0": prr, "ci99": [pl, ph], "detected": pos_ok},
        "statistic": "per-curve mean rho steps / mean over other class curves, 90% CI in [1/1.08,1.08]",
        "verdict": verdict}))
main()
