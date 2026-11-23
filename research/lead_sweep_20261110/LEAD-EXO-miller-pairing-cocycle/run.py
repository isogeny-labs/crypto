#!/usr/bin/env python3
"""Preregistered toy: is U(k)=f_{k,P}(Q) (Miller value) a hash of k? stdlib only."""
import sys, json, math, random, cmath

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def factor(n):
    f, i = [], 2
    while i * i <= n:
        while n % i == 0: f.append(i); n //= i
        i += 1
    if n > 1: f.append(n)
    return sorted(set(f))

def find_curve(p, rng):
    sq = [0] * p
    for y in range(p): sq[y * y % p] += 1
    for _ in range(4000):
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a**3 + 27 * b * b) % p == 0: continue
        N = 1 + sum(sq[(x**3 + a * x + b) % p] for x in range(p))
        if is_prime(N) and N > 100 and math.gcd(N, p - 1) == 1 and N != p:
            return a, b, N
    return None

def sqrtmod(n, p):
    n %= p
    if n == 0: return 0
    if pow(n, (p - 1) // 2, p) != 1: return None
    for y in range(p):
        if y * y % p == n: return y

def add(P, R, a, p):
    if P is None: return R
    if R is None: return P
    if P[0] == R[0] and (P[1] + R[1]) % p == 0: return None
    if P == R: m = (3 * P[0]**2 + a) * pow(2 * P[1], -1, p) % p
    else: m = (R[1] - P[1]) * pow(R[0] - P[0], -1, p) % p
    x = (m * m - P[0] - R[0]) % p
    return (x, (m * (P[0] - x) - P[1]) % p)

def lv(A, B, Q, a, p):
    """line through A,B over vertical at A+B, at Q; returns (num, den) or None if degenerate."""
    S = add(A, B, a, p)
    if A is not None and B is not None and A[0] == B[0] and (A[1] + B[1]) % p == 0:
        return ((Q[0] - A[0]) % p, 1)
    if A == B: m = (3 * A[0]**2 + a) * pow(2 * A[1], -1, p) % p
    else: m = (B[1] - A[1]) * pow(B[0] - A[0], -1, p) % p
    num = (Q[1] - A[1] - m * (Q[0] - A[0])) % p
    den = (Q[0] - S[0]) % p
    return (num, den)

def miller(k, P, Q, a, p):
    f, R = 1, P
    for bit in bin(k)[3:]:
        t = lv(R, R, Q, a, p)
        if t[0] == 0 or t[1] == 0: return None
        f = f * f % p * t[0] % p * pow(t[1], -1, p) % p
        R = add(R, R, a, p)
        if bit == '1':
            t = lv(R, P, Q, a, p)
            if t[0] == 0 or t[1] == 0: return None
            f = f * t[0] % p * pow(t[1], -1, p) % p
            R = add(R, P, a, p)
    return f

def build(p, seed, nk):
    rng = random.Random(seed)
    c = find_curve(p, rng)
    if c is None: return None
    a, b, N = c
    while True:
        x = rng.randrange(p)
        y = sqrtmod(x**3 + a * x + b, p)
        if y: P = (x, y); break
    while True:
        x = rng.randrange(p)
        y = sqrtmod(x**3 + a * x + b, p)
        if y and (x, y) != P: Q = (x, y); break
    d = [q for q in factor(p - 1) if q >= 3]
    d = d[0] if d else None
    if d is None: return None
    e = (p - 1) // d
    ks, U = [], []
    for k in range(2, nk + 2):
        u = miller(k, P, Q, a, p)
        if u: ks.append(k); U.append(u)
    cls = [pow(u, e, p) for u in U]
    # map d-th roots of unity to Z_d via a generator of the order-d subgroup
    gen = None
    for g in range(2, p):
        h = pow(g, e, p)
        if h != 1: gen = h; break
    tab = {pow(gen, j, p): j for j in range(d)}
    return dict(a=a, b=b, N=N, P=P, Q=Q, d=d, ks=ks, cls=[tab[c] for c in cls], U=U, rng=rng)

def spectral(ks, cls, d, N, T, rng, extra=None):
    """max over T random frequencies t of |S_t|^2/n for S_t=sum zeta_d^cls * e(k t/N); Exp(1) null per t."""
    n = len(ks)
    z = [cmath.exp(2j * math.pi * c / d) for c in cls]
    w = [cmath.exp(2j * math.pi * i / N) for i in range(N)]
    best = 0.0
    for i in range(T):
        t = extra if (extra is not None and i == 0) else rng.randrange(1, N)
        s = sum(zi * w[(k * t) % N] for zi, k in zip(z, ks))
        best = max(best, abs(s)**2 / n)
    return best

def chisq_p(cls, d):
    n = len(cls); cnt = [0] * d
    for c in cls: cnt[c] += 1
    x = sum((c - n / d)**2 / (n / d) for c in cnt)
    # Wilson-Hilferty normal approx upper tail
    df = d - 1
    zz = ((x / df)**(1 / 3) - (1 - 2 / (9 * df))) / math.sqrt(2 / (9 * df))
    return 0.5 * math.erfc(zz / math.sqrt(2)), x

def pmax(m, T):
    return 1 - (1 - math.exp(-m))**T

def main():
    smoke = '--smoke' in sys.argv
    primes_in = [1031] if smoke else [4099, 12289, 32771]
    nk = 300 if smoke else 6000
    T = 40 if smoke else 400
    ALPHA = 1e-3
    out = dict(lead="LEAD-EXO-miller-pairing-cocycle", smoke=smoke, seeds=[], configs=[], alpha=ALPHA, T=T, nk=nk)
    real_ok, ctrl_ok, aa_ok, n_cfg = True, True, True, 0
    for p0 in primes_in:
        p = p0
        while True:
            while not is_prime(p): p += 1
            seed = 20261110 + p
            c = build(p, seed, nk)
            if c is not None: break
            p += 1
        n_cfg += 1
        out['seeds'].append(seed)
        d, N, ks, cls = c['d'], c['N'], c['ks'], c['cls']
        rng = random.Random(seed + 1)
        real = spectral(ks, cls, d, N, T, rng); p_real = pmax(real, T)
        pc, chi = chisq_p(cls, d)
        # homomorphism-defect: U(a+b)==U(a)U(b) on random pairs
        Ud = dict(zip(ks, c['U'])); pr = c['p'] if 'p' in c else p
        hits = tot = 0
        for _ in range(2000 if not smoke else 200):
            x, y = rng.choice(ks), rng.choice(ks)
            if x + y in Ud:
                tot += 1; hits += (Ud[x + y] == Ud[x] * Ud[y] % p)
        # null (random classes) and A/A (second independent null)
        r1 = [rng.randrange(d) for _ in ks]; r2 = [rng.randrange(d) for _ in ks]
        nul = spectral(ks, r1, d, N, T, rng); aa = spectral(ks, r2, d, N, T, rng)
        # positive control: planted linear phase in k with 40% noise
        t0 = rng.randrange(1, N)
        pl = [((round(d * ((k * t0) % N) / N)) % d) if rng.random() < 0.4 else rng.randrange(d) for k in ks]
        pos = spectral(ks, pl, d, N, T, rng, extra=N - t0)
        # positive control 2: genuine homomorphism k -> g^k class should have hom-rate 1 (detector check)
        ok_real = p_real > ALPHA and pc > ALPHA and hits <= max(2, tot // 100)
        ok_ctrl = pmax(pos, T) < 1e-6
        ok_aa = pmax(nul, T) > ALPHA and pmax(aa, T) > ALPHA
        real_ok &= ok_real; ctrl_ok &= ok_ctrl; aa_ok &= ok_aa
        out['configs'].append(dict(p=p, N=N, d=d, a=c['a'], b=c['b'], n_used=len(ks),
            real_maxstat=round(real, 3), real_p=p_real, chisq=round(chi, 2), chisq_p=pc,
            hom_hits=hits, hom_pairs=tot, null_maxstat=round(nul, 3), aa_maxstat=round(aa, 3),
            planted_maxstat=round(pos, 3), planted_p=pmax(pos, T)))
    out['controls_pass'] = bool(ctrl_ok and aa_ok)
    if not out['controls_pass']: out['verdict'] = 'inconclusive'
    elif real_ok: out['verdict'] = 'consistent_with_prediction'
    else: out['verdict'] = 'falsified'
    print(json.dumps(out)); return 0

if __name__ == '__main__':
    sys.exit(main())
