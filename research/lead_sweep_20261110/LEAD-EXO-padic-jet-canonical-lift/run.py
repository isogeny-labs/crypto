#!/usr/bin/env python3
"""Toy test: do non-homomorphic Z/p^2 sections of E(F_p) carry linear scalar information? stdlib only."""
import sys, json, random, math
SMOKE = "--smoke" in sys.argv

def isprime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True

def order(p, a, b):
    s = 1
    for x in range(p):
        r = (x*x*x + a*x + b) % p
        if r == 0: continue
        s += 2 if pow(r, (p-1)//2, p) == 1 else 0
    return s + 0  # count: 1 (inf) + 2 per QR + 1 per root handled below

def order_exact(p, a, b):
    s = 1
    for x in range(p):
        r = (x*x*x + a*x + b) % p
        if r == 0: s += 1
        elif pow(r, (p-1)//2, p) == 1: s += 2
    return s

def find_curve(p, rng):
    while True:
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        N = order_exact(p, a, b)
        if N != p and isprime(N): return a, b, N

def sqrt_mod(r, p):  # p = 3 mod 4 enforced
    return pow(r, (p+1)//4, p)

def affine_mults(p, a, b, N, rng):
    # find generator
    while True:
        x = rng.randrange(p); r = (x**3 + a*x + b) % p
        if r and pow(r, (p-1)//2, p) == 1: G = (x, sqrt_mod(r, p)); break
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if (P[1]+Q[1]) % p == 0: return None
            l = (3*P[0]*P[0]+a)*pow(2*P[1], -1, p) % p
        else:
            l = (Q[1]-P[1])*pow(Q[0]-P[0], -1, p) % p
        x3 = (l*l-P[0]-Q[0]) % p
        return (x3, (l*(P[0]-x3)-P[1]) % p)
    pts = []; P = G
    for k in range(1, N):
        pts.append(P); P = add(P, G)
    return pts

def jac_dbl(P, a, m):
    X, Y, Z = P
    if Z % m == 0 and False: return P
    S = 4*X*Y*Y % m; M = (3*X*X + a*pow(Z, 4, m)) % m
    X3 = (M*M - 2*S) % m
    return (X3, (M*(S-X3) - 8*pow(Y, 4, m)) % m, 2*Y*Z % m)

def jac_add(P, Q, a, m):
    X1, Y1, Z1 = P; X2, Y2, Z2 = Q
    U1 = X1*Z2*Z2 % m; U2 = X2*Z1*Z1 % m
    S1 = Y1*pow(Z2, 3, m) % m; S2 = Y2*pow(Z1, 3, m) % m
    H = (U2-U1) % m; R = (S2-S1) % m
    if H == 0 and R == 0: return jac_dbl(P, a, m)
    H2 = H*H % m; H3 = H*H2 % m
    X3 = (R*R - H3 - 2*U1*H2) % m
    return (X3, (R*(U1*H2-X3) - S1*H3) % m, H*Z1*Z2 % m)

def smul(k, P, a, m):
    R = None
    for bit in bin(k)[2:]:
        if R is not None: R = jac_dbl(R, a, m)
        if bit == '1': R = P if R is None else jac_add(R, P, a, m)
    return R

def lift(P, p, a, b, variant):
    x, y = P; m = p*p
    if variant == 1: x = x + p*((x*x+3) % p)
    f = (x**3 + a*x + b) % m
    # Hensel: y~ = y + p*h, y~^2 = f  => 2y h = (f - y^2)/p mod p
    h = ((f - y*y) % m)//p * pow(2*y, -1, p) % p if (f-y*y) % p == 0 else None
    return (x % m, (y + p*h) % m, 1)

def jet_digit(P, p, a, b, N, variant):
    m = p*p
    Q = smul(N, lift(P, p, a, b, variant), a, m)
    X, Y, Z = Q
    assert Z % p == 0 and Y % p != 0
    z = (-X*Z*pow(Y, -1, m)) % m
    return (z//p) % p

def mode_stat(t, p):
    cnt = {}
    for i in range(len(t)-1):
        d = (t[i+1]-t[i]) % p; cnt[d] = cnt.get(d, 0) + 1
    return max(cnt.values())/(len(t)-1)

def null_stats(t, p, rng, n):
    out = []
    for _ in range(n):
        s = t[:]; rng.shuffle(s); out.append(mode_stat(s, p))
    return out

def zscore(v, xs):
    mu = sum(xs)/len(xs); sd = math.sqrt(sum((x-mu)**2 for x in xs)/(len(xs)-1)) or 1e-12
    return (v-mu)/sd

def cell(p, seed, nshuf, variant):
    rng = random.Random(seed)
    a, b, N = find_curve(p, rng)
    pts = affine_mults(p, a, b, N, rng)
    t = [jet_digit(P, p, a, b, N, variant) for P in pts]
    real = mode_stat(t, p)
    nul = null_stats(t, p, random.Random(seed+1), nshuf)
    aa = null_stats(t, p, random.Random(seed+2), nshuf)   # A/A: independent shuffle batch vs batch 1
    zr = zscore(real, nul)
    z_aa = zscore(sum(aa)/len(aa), nul)
    # positive control: leaky section t = c*k + noise-free
    c = rng.randrange(1, p); tl = [(c*k) % p for k in range(1, N)]
    pos = mode_stat(tl, p)
    return dict(p=p, a=a, b=b, N=N, variant=variant, real_mode=real, null_mean=sum(nul)/len(nul),
                null_max=max(nul), z_real=zr, z_aa_batchmean=z_aa, pos_control_mode=pos,
                distinct_t=len(set(t)), t_all_zero=all(v == 0 for v in t))

def main():
    # p = 3 mod 4 primes near 2^k (group order ~ p)
    def nextp(n):
        while not (isprime(n) and n % 4 == 3): n += 1
        return n
    sizes = [(nextp(101), 1, 20)] if SMOKE else [(nextp(2**11), 4, 200), (nextp(2**13), 3, 100), (nextp(2**15), 1, 40)]
    cells = []
    for p, ncurves, nshuf in sizes:
        for ci in range(ncurves):
            for variant in (0, 1):
                cells.append(cell(p, 20261110+1000*ci+p, nshuf, variant))
    zs = [c["z_real"] for c in cells]
    pos_ok = all(c["pos_control_mode"] == 1.0 for c in cells)
    aa_ok = all(abs(c["z_aa_batchmean"]) < 1.0 for c in cells) if not SMOKE else True
    zthr = 4.0
    if not pos_ok or not aa_ok: verdict = "inconclusive"
    elif max(zs) > zthr: verdict = "falsified"
    else: verdict = "consistent_with_prediction"
    print(json.dumps(dict(lead="LEAD-EXO-padic-jet-canonical-lift", smoke=SMOKE,
        statistic="mode frequency of first differences of formal-log digit t(kG) along k, z vs shuffle null",
        z_threshold=zthr, max_z=max(zs), positive_control_ok=pos_ok, aa_ok=aa_ok,
        base_seed=20261110, cells=cells, verdict=verdict)))
main()
