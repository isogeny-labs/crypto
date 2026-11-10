import sys, json, random, math, time
from collections import Counter
SMOKE = '--smoke' in sys.argv
B = 16
LAGS = list(range(1, 9)) + ['x2', 'x3']

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not isprime(n): n += 1
    return n

def curve_orbit(p, rng):
    sq = set((y * y) % p for y in range(p))
    for _ in range(400):
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a**3 + 27 * b * b) % p == 0: continue
        cnt = 1
        for x in range(p):
            r = (x * x * x + a * x + b) % p
            if r == 0: cnt += 1
            elif r in sq: cnt += 2
        if not isprime(cnt): continue
        while True:   # find a point
            x = rng.randrange(p); r = (x**3 + a * x + b) % p
            if r in sq or r == 0:
                y = next(yy for yy in range(p) if (yy * yy) % p == r) if p < 3000 else sqrt_mod(r, p)
                break
        xs = [None]; X, Y = x, y
        cx, cy = X, Y
        xs.append(cx)
        for k in range(2, cnt):
            # add (X,Y) to (cx,cy)
            if cx == X:
                if cy != Y: break
                lam = (3 * cx * cx + a) * pow(2 * cy, -1, p) % p
            else:
                lam = (Y - cy) * pow(X - cx, -1, p) % p
            nx = (lam * lam - X - cx) % p
            ny = (lam * (cx - nx) - cy) % p
            cx, cy = nx, ny
            xs.append(cx)
        if len(xs) == cnt:
            return cnt, xs
    raise RuntimeError('no curve')

def sqrt_mod(r, p):
    if p % 4 == 3: return pow(r, (p + 1) // 4, p)
    for y in range(p):
        if (y * y) % p == r: return y

def digits(xs, p):
    top = int(math.log(p, B))
    out = []
    for j in (0, 1, top - 1):
        out.append([(v // B**j) % B if v is not None else 0 for v in xs])
    return out

def stat(dl, N):
    """max over digit positions and lags of |mean e((d_k - d_k')/B)| * sqrt(N)"""
    T = 0.0
    cs = [(math.cos(2 * math.pi * t / B), math.sin(2 * math.pi * t / B)) for t in range(B)]
    for d in dl:
        d = d[1:]
        n = len(d)
        for L in LAGS:
            if L == 'x2': pairs = zip(d[0::1][:n // 2], [d[2 * k + 1] for k in range(n // 2)])
            elif L == 'x3': pairs = zip(d[:n // 3], [d[3 * k + 2] for k in range(n // 3)])
            else: pairs = zip(d[:-L], d[L:])
            c = Counter((a - b) % B for a, b in pairs); m = sum(c.values())
            re = sum(c[t] * cs[t][0] for t in c); im = sum(c[t] * cs[t][1] for t in c)
            T = max(T, math.hypot(re, im) / math.sqrt(m))
    return T

def null_labels(N, p, rng):
    h = N // 2
    half = [rng.randrange(p) for _ in range(h + 1)]
    return [None] + [half[min(k, N - k)] for k in range(1, N)]

def leaky_labels(xs, N, p, rng):
    # positive control: low digit replaced by a slow function of k with prob 0.3
    out = [None]
    for k in range(1, N):
        v = xs[k]
        if rng.random() < 0.3:
            v = v - v % B + (min(k, N - k) // 7) % B  # symmetric in k
        out.append(v)
    return out

def main():
    t0 = time.time()
    sizes = [(2**8, 1, 6)] if SMOKE else [(2**12, 6, 30), (2**16, 3, 8), (2**18, 1, 3)]
    res = []; seed = 20261110
    flags = []
    for (target, ncurve, nnull) in sizes:
        p = next_prime(target)
        row = {'p': p, 'real': [], 'null': [], 'null2_AA': [], 'leaky': []}
        for c in range(ncurve):
            rng = random.Random(seed + p * 7 + c)
            N, xs = curve_orbit(p, rng)
            row['N'] = N
            row['real'].append(stat(digits(xs, p), N))
            row['leaky'].append(stat(digits(leaky_labels(xs, N, p, rng), p), N))
            if c == 0:
                for i in range(nnull):
                    r2 = random.Random(seed + 99 + i + p)
                    row['null'].append(stat(digits(null_labels(N, p, r2), p), N))
                for i in range(max(3, nnull // 3)):
                    r2 = random.Random(seed + 5555 + i + p)
                    row['null2_AA'].append(stat(digits(null_labels(N, p, r2), p), N))
        nl = sorted(row['null'])
        thr = nl[min(len(nl) - 1, int(0.95 * len(nl)))] if len(nl) > 2 else nl[-1]
        row['null_q95'] = thr
        row['real_exceeds_q95'] = sum(v > thr for v in row['real'])
        row['leaky_detected'] = all(v > thr for v in row['leaky'])
        row['aa_exceeds_q95'] = sum(v > thr for v in row['null2_AA'])
        res.append(row)
    pc_ok = all(r['leaky_detected'] for r in res)
    nreal = sum(len(r['real']) for r in res); nex = sum(r['real_exceeds_q95'] for r in res)
    aa_ok = all(r['aa_exceeds_q95'] <= max(1, len(r['null2_AA']) // 3) for r in res)
    if not pc_ok or not aa_ok: verdict = 'inconclusive'
    elif nex > max(1, 0.05 * nreal + 2): verdict = 'falsified'
    else: verdict = 'consistent_with_prediction'
    print(json.dumps({'lead': 'LEAD-EXO-coordinate-statistic-leakage', 'smoke': SMOKE, 'statistic': 'max |digit-character lag autocorr|*sqrt(m)', 'base': B, 'lags': [str(l) for l in LAGS], 'seed': seed, 'rows': res, 'positive_control_ok': pc_ok, 'aa_ok': aa_ok, 'real_exceed_count': nex, 'real_total': nreal, 'verdict': verdict, 'seconds': round(time.time() - t0, 1)}))
main()
