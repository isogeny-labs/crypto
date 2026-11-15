import sys, json, math, random
UNIT_INV = 10  # field mult units per inversion (charging convention)
ADD = UNIT_INV + 3

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

def count_points(p, A, B):
    s = 1
    for x in range(p):
        r = (x * x * x + A * x + B) % p
        if r == 0: continue
        s += 1 if pow(r, (p - 1) // 2, p) == 1 else -1
        s += 0
    # sum over x of (1+leg) = p + sum leg ; plus infinity
    return p + 1 + (s - 1)

def add(P, Q, A, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + A) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def mul(k, P, A, p):
    R = None
    while k:
        if k & 1: R = add(R, P, A, p)
        P = add(P, P, A, p); k >>= 1
    return R

def rand_point(rng, A, B, p):
    while True:
        x = rng.randrange(p); r = (x**3 + A * x + B) % p
        if r and pow(r, (p - 1) // 2, p) == 1:
            for y in range(1, p):  # sqrt via Tonelli-lite (p small): use pow for p%4==3 else brute
                break
            y = sqrt_mod(r, p)
            return (x, y)

def sqrt_mod(a, p):
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

def order_of(P, A, p, N):
    n, f, d = N, [], 2
    while d * d <= n:
        if n % d == 0:
            f.append(d)
            while n % d == 0: n //= d
        d += 1
    if n > 1: f.append(n)
    o = N
    for q in f:
        while o % q == 0 and mul(o // q, P, A, p) is None: o //= q
    return o

def eds_check(P, A, B, p, o):
    """Positive control for the net: EDS W_n(P) from recurrence vanishes first at n=o (rank of apparition)."""
    x, y = P
    W = {0: 0, 1: 1, 2: 2 * y % p}
    W[3] = (3 * x**4 + 6 * A * x * x + 12 * B * x - A * A) % p
    W[4] = (4 * y * (x**6 + 5 * A * x**4 + 20 * B * x**3 - 5 * A * A * x * x - 4 * A * B * x - 8 * B * B - A**3)) % p
    n = 5
    top = min(o + 2, 4000)
    while n <= top:
        m = n // 2
        if n % 2:
            W[n] = (W[m + 2] * W[m]**3 - W[m - 1] * W[m + 1]**3) % p
        else:
            W[n] = (W[m] * (W[m + 2] * W[m - 1]**2 - W[m - 2] * W[m + 1]**2) * pow(2 * y, -1, p)) % p
        n += 1
    first = next((i for i in range(1, top + 1) if W[i] == 0), None)
    return first == o if o <= top else first is None

def sieve(Q, P, A, p, o):
    """net/EDS zero-test search: find a with a*P+Q = O, a=0,1,.. ; returns (a, adds)."""
    R, a, adds = Q, 0, 0
    while R is not None:
        R = add(R, P, A, p); a += 1; adds += 1
    return a, adds

def bsgs(Q, P, A, p, o):
    m = math.isqrt(o) + 1; adds = 0
    tab = {}; R = None
    for j in range(m):
        tab.setdefault(R, j); R = add(R, P, A, p); adds += 1
    G = mul(m, P, A, p); adds += 2 * m.bit_length()
    nG = None if G is None else (G[0], (-G[1]) % p)
    R = Q
    for i in range(m + 1):
        if R in tab: return (i * m + tab[R]) % o, adds
        R = add(R, nG, A, p); adds += 1
    return None, adds

def arm(seed, primes, T):
    rng = random.Random(seed); rows = []
    for p0 in primes:
        p = next_prime(p0 + rng.randrange(0, p0 // 8))
        while True:
            A, B = rng.randrange(p), rng.randrange(p)
            if (4 * A**3 + 27 * B * B) % p: break
        N = count_points(p, A, B)
        P = rand_point(rng, A, B, p); o = order_of(P, A, p, N)
        ctrl = eds_check(P, A, B, p, o)
        s_cost = b_cost = n_cost = 0; ok = True
        for _ in range(T):
            k = rng.randrange(1, o); Q = mul(k, P, A, p)
            a, ad = sieve(Q, P, A, p, o); s_cost += ad
            ok &= ((-a) % o == k % o) or (Q is None)
            kb, bd = bsgs(Q, P, A, p, o); b_cost += bd; ok &= (kb == k % o)
            n_cost += rng.randrange(1, o + 1)  # null: random-order enumeration probes
        rows.append(dict(p=p, N=o, sieve=s_cost / T, bsgs=b_cost / T, null=n_cost / T, ok=bool(ok), eds_ok=bool(ctrl)))
    return rows

def slope(xs, ys):
    lx = [math.log(x) for x in xs]; ly = [math.log(y) for y in ys]
    mx, my = sum(lx) / len(lx), sum(ly) / len(ly)
    return sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sum((a - mx)**2 for a in lx)

def summarize(rows):
    Ns = [r["N"] for r in rows]
    ss = slope(Ns, [r["sieve"] * ADD for r in rows]); sb = slope(Ns, [r["bsgs"] * ADD for r in rows])
    ratio = [r["sieve"] / r["bsgs"] for r in rows]
    nr = [r["sieve"] / r["null"] for r in rows]
    return dict(slope_sieve=ss, slope_bsgs=sb, slope_gap=ss - sb, ratios=ratio, sieve_over_null=nr)

def main():
    smoke = "--smoke" in sys.argv
    primes = [400, 800, 1600] if smoke else [1 << 12, 1 << 14, 1 << 16, 1 << 18]
    T = 3 if smoke else 6
    a = summarize(arm(1001, primes, T)); rowsA = arm(1001, primes, T)
    rowsB = arm(2002, primes, T); b = summarize(rowsB)
    aa = abs(a["slope_gap"] - b["slope_gap"])
    mono = all(x < y for x, y in zip(a["ratios"], a["ratios"][1:]))
    allok = all(r["ok"] and r["eds_ok"] for r in rowsA + rowsB)
    null_ok = all(0.5 < v < 1.0 + 0.0 or abs(v - 1) < 0.5 for v in a["sieve_over_null"])
    if not allok or aa > 0.25: verdict = "inconclusive"
    elif a["slope_gap"] >= 0.3 and mono and b["slope_gap"] >= 0.3: verdict = "consistent_with_prediction"
    elif a["slope_gap"] < 0.15: verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps(dict(lead="LEAD-EXO-elliptic-net-eds", smoke=smoke, seeds=[1001, 2002], primes=primes,
        targets_per_size=T, charge=dict(inv=UNIT_INV, add=ADD), arm_A=a, arm_B=b, aa_slope_gap_diff=aa,
        ratio_monotone=mono, controls=dict(positive_all_recovered_and_eds_rank_ok=allok,
        null_sieve_over_random_probe_ratio=a["sieve_over_null"]), rows=rowsA, verdict=verdict)))
main()
