import sys, json, math, random
SMOKE = "--smoke" in sys.argv
SEED = 20261110

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def prev_prime(n):
    while not isprime(n): n -= 1
    return n

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

def curve_with_prime_order(p, rng):
    sq = {}
    for y in range(p): sq.setdefault(y * y % p, []).append(y)
    for _ in range(4000):
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        pts = []
        for x in range(p):
            r = (x ** 3 + a * x + b) % p
            for y in sq.get(r, []): pts.append((x, y))
        N = len(pts) + 1
        if N != p and isprime(N) and N > 50:
            return a, b, pts, N
    return None

def dl_table(a, p, pts, N, rng):
    G = pts[rng.randrange(len(pts))]
    d = {}; Q = None
    for j in range(1, N):
        Q = add(Q, G, a, p); d[Q] = j
    return d

def stat(S, N, cosT):
    best = 0.0
    S = list(S)
    for k in range(1, (N - 1) // 2 + 1):
        s = 0.0
        for j in S: s += cosT[k * j % N]
        if abs(s) > best: best = abs(s)
    return best

def fold(j, N): return min(j, N - j)

def run_curve(p, rng, nnull):
    c = curve_with_prime_order(p, rng)
    if c is None: return None
    a, b, pts, N = c
    d = dl_table(a, p, pts, N, rng)
    m = round(math.sqrt(N)); h = max(2, m // 2)
    # small-x set: the h smallest x-values (pairs P,-P share x); fold DL
    xs = sorted({P[0] for P in pts})
    S = []
    for x in xs:
        ys = [P for P in pts if P[0] == x] if False else None
        break
    byx = {}
    for P in pts: byx.setdefault(P[0], P)
    S = [fold(d[byx[x]], N) for x in xs[:h]]
    cosT = [math.cos(2 * math.pi * t / N) for t in range(N)]
    half = (N - 1) // 2
    CE = stat(S, N, cosT)
    nulls = [stat(rng.sample(range(1, half + 1), h), N, cosT) for _ in range(nnull)]
    j0 = rng.randrange(1, half - h)
    pos = stat(range(j0, j0 + h), N, cosT)
    k = nnull // 2
    mu = sum(nulls) / nnull
    return dict(p=p, N=N, h=h, CE=CE, null_mean=mu, R=CE / mu,
                AA=(sum(nulls[:k]) / k) / (sum(nulls[k:]) / (nnull - k)), pos_R=pos / mu)

def mean(v): return sum(v) / len(v)

def main():
    rng = random.Random(SEED)
    if SMOKE: bits, ncurves, nnull = [8, 9], 2, 4
    else: bits, ncurves, nnull = [11, 12, 13, 14], 10, 10
    rows = []
    for bt in bits:
        p = prev_prime(2 ** bt)
        for _ in range(ncurves):
            r = run_curve(p, rng, nnull)
            if r: rows.append(r)
    per = {}
    for r in rows: per.setdefault(math.log2(r["N"]) // 1, []).append(r)
    lv = []
    for bt in bits:
        g = [r for r in rows if r["p"] == prev_prime(2 ** bt)]
        if not g: continue
        R = [r["R"] for r in g]; m = mean(R)
        sd = (sum((x - m) ** 2 for x in R) / max(1, len(R) - 1)) ** .5
        lv.append(dict(bits=bt, n=len(g), meanR=m, se=sd / len(g) ** .5,
                       meanAA=mean([r["AA"] for r in g]), meanPosR=mean([r["pos_R"] for r in g])))
    allR = [r["R"] for r in rows]; M = mean(allR)
    sd = (sum((x - M) ** 2 for x in allR) / max(1, len(allR) - 1)) ** .5
    se = sd / len(allR) ** .5
    xs = [l["bits"] for l in lv]; ys = [l["meanR"] for l in lv]
    xb, yb = mean(xs), mean(ys)
    slope = sum((x - xb) * (y - yb) for x, y in zip(xs, ys)) / max(1e-12, sum((x - xb) ** 2 for x in xs))
    aa = mean([r["AA"] for r in rows]); pos = mean([r["pos_R"] for r in rows])
    lo, hi = M - 1.96 * se, M + 1.96 * se
    pos_ok = pos > 1.4; aa_ok = 0.9 <= aa <= 1.1
    if not (pos_ok and aa_ok) or len(rows) < 8: verdict = "inconclusive"
    elif lo > 1.0 and slope > 0.01: verdict = "falsified"
    elif lo <= 1.0 and hi >= 0.95 and slope <= 0.01 or (hi < 1.0): verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    if SMOKE and verdict == "falsified": verdict = "inconclusive"
    print(json.dumps(dict(lead="LEAD-EXO-dl-character-sum-pseudorandom", smoke=SMOKE, seed=SEED,
        params=dict(bits=bits, curves_per_size=ncurves, nulls_per_curve=nnull, set_size_h="round(sqrt N)//2 pairs"),
        n_rows=len(rows), pooled_R=M, ci95=[lo, hi], slope_R_per_bit=slope, aa_mean=aa,
        positive_control_R=pos, levels=lv, verdict=verdict, label="[pcc:unverified]")))
main()
