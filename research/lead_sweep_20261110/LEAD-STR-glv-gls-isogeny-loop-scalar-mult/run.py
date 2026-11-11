#!/usr/bin/env python3
"""LEAD-STR-glv-gls-isogeny-loop-scalar-mult toy protocol. stdlib only."""
import sys, json, random, math

SMOKE = "--smoke" in sys.argv
BITS = 11 if SMOKE else 17
NK = 20 if SMOKE else 300      # scalars per curve for op counting
NSEEDS = 4 if SMOKE else 20    # birthday seeds per arm
SEED = 20261110

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def find_curve(rng):
    while True:
        p = rng.randrange(1 << BITS, 1 << (BITS + 1))
        if p % 3 != 1 or not isprime(p): continue
        sq = {}
        for y in range(p): sq.setdefault(y * y % p, y)
        for b in range(1, 40):
            n = 1
            for x in range(p):
                v = (x ** 3 + b) % p
                if v == 0: n += 1
                elif v in sq: n += 2
            if isprime(n) and n != p:
                return p, b, n, sq

class Curve:
    def __init__(s, p, b): s.p, s.b, s.ops = p, b, 0
    def add(s, P, Q):
        p = s.p
        if P is None: return Q
        if Q is None: return P
        s.ops += 1
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            l = 3 * P[0] * P[0] * pow(2 * P[1], -1, p) % p
        else:
            l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
        x = (l * l - P[0] - Q[0]) % p
        return (x, (l * (P[0] - x) - P[1]) % p)
    def neg(s, P): return None if P is None else (P[0], (-P[1]) % s.p)
    def mul_binary(s, k, P):
        R = None
        for c in bin(k)[2:]:
            R = s.add(R, R)
            if c == '1': R = s.add(R, P)
        return R
    def mul_shamir(s, a, b, P, Q):
        # a*P + b*Q, signs absorbed by free negation
        if a < 0: a, P = -a, s.neg(P)
        if b < 0: b, Q = -b, s.neg(Q)
        PQ = s.add(P, Q); s.ops_pre = 1
        R = None
        for i in range(max(a.bit_length(), b.bit_length()) - 1, -1, -1):
            R = s.add(R, R)
            d = ((a >> i) & 1, (b >> i) & 1)
            if d == (1, 0): R = s.add(R, P)
            elif d == (0, 1): R = s.add(R, Q)
            elif d == (1, 1): R = s.add(R, PQ)
        return R

def lam_beta(C, N, sq, rng):
    p = C.p
    # find generator point
    while True:
        x = rng.randrange(p); v = (x ** 3 + C.b) % p
        if v in sq and v != 0: P = (x, sq[v]); break
    beta = None
    while beta is None:
        g = pow(rng.randrange(2, p), (p - 1) // 3, p)
        if g != 1: beta = g
    r = (-1 + math.isqrt(-3 % N)) if False else None
    # lambda: root of l^2+l+1 mod N (N==1 mod 3 here)
    lams = [l for l in _roots(N)]
    for l in lams:
        for bt in (beta, beta * beta % p):
            if C.mul_binary(l, P) == (bt * P[0] % p, P[1]):
                return P, l, bt
    raise RuntimeError("no eigenvalue")

def _roots(N):
    # square root of -3 mod prime N via Tonelli-Shanks
    a = (-3) % N
    if pow(a, (N - 1) // 2, N) != 1: return []
    q, s = N - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (N - 1) // 2, N) != N - 1: z += 1
    m, c, t, r = s, pow(z, q, N), pow(a, q, N), pow(a, (q + 1) // 2, N)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2 * t2 % N; i += 1
        bb = pow(c, 1 << (m - i - 1), N)
        m, c, t, r = i, bb * bb % N, t * bb * bb % N, r * bb % N
    inv2 = pow(2, -1, N)
    return [(-1 + r) * inv2 % N, (-1 - r) * inv2 % N]

def gauss(N, lam):
    u, v = (N, 0), (-lam, 1)
    dot = lambda a, b: a[0] * b[0] + a[1] * b[1]
    while True:
        if dot(u, u) < dot(v, v): u, v = v, u
        m = round(dot(u, v) / dot(v, v))
        if m == 0: return u, v
        u = (u[0] - m * v[0], u[1] - m * v[1])

def decompose(k, N, lam, basis):
    (a1, b1), (a2, b2) = basis
    det = a1 * b2 - a2 * b1
    c1 = round(k * b2 / det); c2 = round(-k * b1 / det)
    k1 = k - c1 * a1 - c2 * a2; k2 = -c1 * b1 - c2 * b2
    assert (k1 + k2 * lam - k) % N == 0
    return k1, k2

def op_experiment(rng, C, N, P, lam, beta, wrong_lam):
    basis = gauss(N, lam)
    phiP = (beta * P[0] % C.p, P[1])
    tot_b = tot_g = 0; ok = True
    for _ in range(NK):
        k = rng.randrange(1, N)
        C.ops = 0; R1 = C.mul_binary(k, P); tb = C.ops
        k1, k2 = decompose(k, N, lam, basis)
        C.ops = 0; R2 = C.mul_shamir(k1, k2, P, phiP); tg = C.ops + 0
        ok &= (R1 == R2); tot_b += tb; tot_g += tg
    # negative control: wrong eigenvalue must break correctness
    bad = 0
    mu = (wrong_lam if wrong_lam % N not in (lam, N - 1 - lam) else (wrong_lam + 7)) % N
    bb = gauss(N, mu)
    for _ in range(20):
        k = rng.randrange(1, N)
        k1, k2 = decompose(k, N, mu, bb)
        R = C.mul_shamir(k1, k2, P, phiP)
        bad += (R != C.mul_binary(k, P))
    return tot_b / tot_g, ok, bad

def canon(C, P, beta):
    # orbit under Aut of order 6: x -> beta^i x, y -> +-y
    if P is None: return None
    p = C.p; xs = []
    x = P[0]
    for _ in range(3):
        for y in (P[1], (-P[1]) % p): xs.append((x, y))
        x = x * beta % p
    return min(xs)

def birthday(rng_seed, C, N, P, beta, use_canon):
    rng = random.Random(rng_seed)
    # sampling uniformly random group elements via precomputed table walk is
    # replaced by direct multiples k*P with k uniform: draws needed to first repeat.
    seen = set(); n = 0
    # cheap generation: random k, point via binary mult would be slow; use
    # incremental table of all multiples (N small) built once per call site.
    return None

def build_table(C, N, P):
    T = [None] * N; R = None
    for i in range(1, N):
        R = C.add(R, P); T[i] = R
    return T

def draws_to_collision(T, N, C, beta, use_canon, seed, cache):
    rng = random.Random(seed); seen = set(); n = 0
    while True:
        k = rng.randrange(1, N); n += 1
        key = cache[k] if use_canon else k
        if key in seen: return n
        seen.add(key)

def main():
    rng = random.Random(SEED)
    p, b, N, sq = find_curve(rng)
    C = Curve(p, b)
    P, lam, beta = lam_beta(C, N, sq, rng)
    r_ops, ok, bad = op_experiment(random.Random(SEED + 1), C, N, P, lam, beta, 12345)
    T = build_table(C, N, P)
    # orbit canonical labels: map k -> min index of orbit via lambda action on scalars
    # (x->beta x corresponds to scalar lam or lam^2; negation to -k)
    lam2 = lam * lam % N
    cache = [0] * N
    for k in range(1, N):
        cache[k] = min(k, (-k) % N, k * lam % N, (-k * lam) % N, k * lam2 % N, (-k * lam2) % N)
    plain = [draws_to_collision(T, N, C, beta, False, 1000 + s, cache) for s in range(NSEEDS)]
    plain2 = [draws_to_collision(T, N, C, beta, False, 2000 + s, cache) for s in range(NSEEDS)]
    orb = [draws_to_collision(T, N, C, beta, True, 3000 + s, cache) for s in range(NSEEDS)]
    m = lambda v: sum(v) / len(v)
    ratio_orb = m(orb) / m(plain)
    aa = m(plain) / m(plain2)
    # positive control for the orbit detector: negation-only canonical label (expect ~1/sqrt2)
    neg = [draws_to_collision(T, N, C, beta, True, 4000 + s, [min(k, (-k) % N) for k in range(N)]) for s in range(NSEEDS)]
    ratio_neg = m(neg) / m(plain)
    lo, hi = (1.3, 2.1) if SMOKE else (1.45, 2.0)
    ops_ok = lo <= r_ops <= hi and ok and bad >= 15
    aa_ok = (0.5 if SMOKE else 0.8) <= aa <= (2.0 if SMOKE else 1.25)
    ceil = 1 / math.sqrt(6)
    orb_ok = abs(ratio_orb - ceil) <= (0.25 if SMOKE else 0.08)
    if not aa_ok: verdict = "inconclusive"
    elif ops_ok and orb_ok: verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps({
        "lead": "LEAD-STR-glv-gls-isogeny-loop-scalar-mult", "smoke": SMOKE, "seed": SEED,
        "params": {"p": p, "b": b, "N": N, "lambda": lam, "beta": beta, "NK": NK, "NSEEDS": NSEEDS},
        "statistic": {"binary_over_glv_ops_ratio": r_ops, "glv_correct": ok,
                      "orbit_over_plain_draws_ratio": ratio_orb, "sqrt6_inverse": ceil},
        "controls": {"negative_wrong_lambda_failures_of_20": bad,
                     "aa_plain_ratio": aa, "positive_negation_only_ratio": ratio_neg,
                     "positive_expected": 1 / math.sqrt(2)},
        "thresholds": {"ops_ratio": [lo, hi], "orbit_tol": 0.25 if SMOKE else 0.08},
        "verdict": verdict}))

main()
