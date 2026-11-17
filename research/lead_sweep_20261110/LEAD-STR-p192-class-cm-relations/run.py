import json, random, sys

def legendre_count(p, b):
    sq = [0]*p
    for y in range(p): sq[y*y % p] += 1
    return 1 + sum(sq[(x*x*x + b) % p] for x in range(p))

def isprime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True

def add(P, Q, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        m = 3*x1*x1 * pow(2*y1, -1, p) % p
    else:
        m = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (m*m - x1 - x2) % p
    return (x3, (m*(x1 - x3) - y1) % p)

def mul(k, P, p):
    R = None
    while k:
        if k & 1: R = add(R, P, p)
        P = add(P, P, p); k >>= 1
    return R

def sqrt_mod(a, p):
    for _ in range(200):
        pass
    return pow(a, (p+1)//4, p) if p % 4 == 3 else None

def rand_point(p, b, rng):
    while True:
        x = rng.randrange(p); r = (x**3 + b) % p
        if pow(r, (p-1)//2, p) == 1 and r:
            # Tonelli via brute-safe: use Cipolla-free approach
            y = tsq(r, p)
            return (x, y)

def tsq(a, p):
    q, s = p-1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q+1)//2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        bb = pow(c, 1 << (m-i-1), p)
        m, c, t, r = i, bb*bb % p, t*bb*bb % p, r*bb % p
    return r

def smooth(n, B=7):
    n = abs(n)
    if n == 0: return False
    for q in (2, 3, 5, 7):
        while n % q == 0: n //= q
    return n == 1

def curves(seed, count, plo, phi):
    rng = random.Random(seed); out = []; tries = 0
    while len(out) < count and tries < 200000:
        tries += 1
        p = rng.randrange(plo, phi) | 1
        if p % 3 != 1 or not isprime(p): continue
        b = rng.randrange(1, p)
        N = legendre_count(p, b)
        if N % 3 == 1 and isprime(N) and N > 100: out.append((p, b, N))
    return out

def test_curve(p, b, N, rng, nq, ab):
    om = 1
    g = 2
    while om == 1:
        om = pow(g, (p-1)//3, p); g += 1
    zeta = lambda Q: None if Q is None else (om*Q[0] % p, Q[1])
    P0 = rand_point(p, b, rng)
    Z = zeta(P0); R = None; lam = None
    for k in range(1, N):
        R = add(R, P0, p)
        if R == Z: lam = k; break
    if lam is None: return dict(ok=0, tot=1, bad_neg=0, neg_tot=1, nsm=0, root=False)
    root = (lam*lam + lam + 1) % N == 0
    lam2 = lam*lam % N
    ok = tot = bneg = ntot = nsm = 0
    for _ in range(nq):
        Q = rand_point(p, b, rng)
        tot += 1; ok += zeta(Q) == mul(lam, Q, p)
        ntot += 1; bneg += zeta(Q) == mul(lam2, Q, p)   # wrong scalar (conjugate root)
        for a in range(-ab, ab+1):
            for c in range(1, ab+1):
                if smooth(a*a - a*c + c*c):
                    nsm += 1; tot += 1
                    lhs = add(mul(a % N, Q, p), mul(c, zeta(Q), p), p)
                    ok += lhs == mul((a + c*lam) % N, Q, p)
    return dict(ok=ok, tot=tot, bad_neg=bneg, neg_tot=ntot, nsm=nsm, root=root)

def arm(seed, count, plo, phi, nq, ab):
    rng = random.Random(seed*7+1); cs = curves(seed, count, plo, phi)
    r = [test_curve(*c, rng, nq, ab) for c in cs]
    ok = sum(x['ok'] for x in r); tot = sum(x['tot'] for x in r)
    return dict(curves=len(cs), ok=ok, tot=tot, frac=ok/tot if tot else None,
                neg_false_match=sum(x['bad_neg'] for x in r), neg_tot=sum(x['neg_tot'] for x in r),
                smooth_elems=sum(x['nsm'] for x in r), all_roots=all(x['root'] for x in r),
                orders=[c[2] for c in cs])

def main():
    smoke = '--smoke' in sys.argv
    if smoke: cnt, plo, phi, nq, ab = 2, 600, 2000, 3, 6
    else: cnt, plo, phi, nq, ab = 12, 2**12, 2**17, 8, 12
    A = arm(1001, cnt, plo, phi, nq, ab)       # CM arm (real j=0 curves)
    B = arm(2002, cnt, plo, phi, nq, ab)       # A/A: independent seed set, same instrument
    # positive control: instrument must detect a deliberately wrong claim (conjugate scalar) -> neg arm
    pos_detect = A['neg_tot'] > 0 and A['neg_false_match'] < 0.05*A['neg_tot']
    if A['tot'] == 0 or B['tot'] == 0 or not pos_detect: v = 'inconclusive'
    elif A['ok'] == A['tot'] and B['ok'] == B['tot'] and A['all_roots'] and B['all_roots']: v = 'consistent_with_prediction'
    else: v = 'falsified'
    print(json.dumps(dict(lead='LEAD-STR-p192-class-cm-relations', smoke=smoke,
        statistic='fraction of smooth-norm (7-smooth) elements a+b*zeta of Z[zeta3] acting on prime-order E(F_p), j=0, as the scalar a+b*lambda mod N',
        threshold='falsified if any mismatch; consistent if 1.0 in both arms and wrong-scalar control matches <5%',
        seeds=[1001, 2002], params=dict(curves=cnt, p_range=[plo, phi], queries=nq, coeff_bound=ab),
        arm_cm=A, arm_AA=B, negative_control='conjugate-root scalar lambda^2 (must fail)',
        positive_control='scalar action with lambda recovered by exhaustive search, root of x^2+x+1 mod N',
        verdict=v)))
main()
