"""mllc dataset generator: exact prime-field short-Weierstrass curves of prime order
near 2^m, points Q=kP for random k, plus the control encodings. numpy int64, p < 2^26.
Development/analysis code; every number it prints is [pcc:unverified]."""
import numpy as np, math, random

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def prime_near(m, rng):
    while True:
        p = (1 << m) - rng.randrange(1, 1 << (m-3))
        if p % 2 and is_prime(p): return p

def count_points(p, a, b):
    """#E(F_p) by Legendre sum with a squares table (exact)."""
    sq = np.zeros(p, dtype=np.int8); sq[(np.arange(p, dtype=np.int64)**2) % p] = 1
    x = np.arange(p, dtype=np.int64)
    f = (x*x % p * x + a*x + b) % p
    chi = np.where(f == 0, 0, np.where(sq[f] == 1, 1, -1))
    return p + 1 + int(chi.sum())

def ec_add(P, Q, a, p):  # python exact reference; None = infinity
    if P is None: return Q
    if Q is None: return P
    (x1,y1),(x2,y2) = P,Q
    if x1 == x2:
        if (y1+y2) % p == 0: return None
        l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
    else:
        l = (y2-y1) * pow(x2-x1, -1, p) % p
    x3 = (l*l - x1 - x2) % p
    return (x3, (l*(x1-x3) - y1) % p)

def ec_mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = ec_add(R, P, a, p)
        P = ec_add(P, P, a, p); k >>= 1
    return R

def make_curve(m, seed):
    """prime-order curve, p ~ 2^m. Returns dict(p,a,b,N,P)."""
    rng = random.Random(seed)
    p = prime_near(m, rng)
    while True:
        a, b = rng.randrange(p), rng.randrange(1, p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        N = count_points(p, a, b)
        if is_prime(N) and N != p:      # N == p is anomalous: excluded
            break
    while True:
        x = rng.randrange(p); r = (x**3 + a*x + b) % p
        if pow(r, (p-1)//2, p) == 1:
            y = pow(r, (p+1)//4, p) if p % 4 == 3 else None
            if y is None:  # Tonelli-Shanks fallback
                y = _sqrt(r, p)
            P = (x, y)
            assert ec_mul(N, P, a, p) is None and P is not None
            return dict(p=p, a=a, b=b, N=N, P=P)

def _sqrt(n, p):
    q, s = p-1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    M, c, t, R = s, pow(z, q, p), pow(n, q, p), pow(n, (q+1)//2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1: tt = tt*tt % p; i += 1
        b = pow(c, 1 << (M-i-1), p); M, c = i, b*b % p; t, R = t*c % p, R*b % p
    return R

def _vinv(x, p):  # vectorised Fermat inverse, int64, p < 2^26
    r = np.ones_like(x); b = x % p; e = p-2
    while e:
        if e & 1: r = r*b % p
        b = b*b % p; e >>= 1
    return r

def points_for(curve, ks):
    """Q = kP for int array ks in [1,N-1], by two exact tables + one vectorised add."""
    p, a, N, P = curve['p'], curve['a'], curve['N'], curve['P']
    s = math.isqrt(N) + 1
    def table(base, n):
        T = [None]*n; R = None
        for i in range(n):
            T[i] = R; R = ec_add(R, base, a, p)
        return T
    lo = table(P, s); hi = table(ec_mul(s, P, a, p), s + 1)
    def arr(T):
        inf = np.array([t is None for t in T]); xs = np.array([0 if t is None else t[0] for t in T], dtype=np.int64)
        ys = np.array([0 if t is None else t[1] for t in T], dtype=np.int64); return inf, xs, ys
    li, lx, ly = arr(lo); hi_i, hx, hy = arr(hi)
    ks = np.asarray(ks, dtype=np.int64); i, j = ks // s, ks % s
    x1, y1, i1 = hx[i], hy[i], hi_i[i]; x2, y2, i2 = lx[j], ly[j], li[j]
    dx = (x2 - x1) % p; same = (dx == 0) & ~i1 & ~i2
    dbl = same & (y1 == y2)
    num = np.where(dbl, (3*x1*x1 + a) % p, (y2 - y1) % p)
    den = np.where(dbl, 2*y1 % p, dx); den = np.where(den == 0, 1, den)
    l = num * _vinv(den, p) % p
    x3 = (l*l - x1 - x2) % p; y3 = (l*(x1 - x3) - y1) % p
    x3 = np.where(i1, x2, np.where(i2, x1, x3)); y3 = np.where(i1, y2, np.where(i2, y1, y3))
    assert not (same & ~dbl).any(), "infinity result for k in [1,N-1]"
    return x3, y3

def on_curve(curve, x, y):
    p = curve['p']; return bool((((y*y - x*x % p * x - curve['a']*x - curve['b']) % p) == 0).all())

def bits(v, m):
    v = np.asarray(v, dtype=np.int64); return ((v[:, None] >> np.arange(m)) & 1).astype(np.float32)

def validate(curve, n=500, seed=1):
    """exact checks: group order, table arithmetic == python double-and-add, points on curve."""
    rng = np.random.default_rng(seed); N = curve['N']
    ks = rng.integers(1, N, n); x, y = points_for(curve, ks)
    assert on_curve(curve, x, y)
    for k, xx, yy in zip(ks[:200], x[:200], y[:200]):
        assert ec_mul(int(k), curve['P'], curve['a'], curve['p']) == (int(xx), int(yy))
    assert ec_mul(N, curve['P'], curve['a'], curve['p']) is None
    return True

# ---- controls -------------------------------------------------------------
def labels(ks, N, kind):
    ks = np.asarray(ks, dtype=np.int64)
    return (ks & 1) if kind == 'lsb' else (ks >= (N+1)//2).astype(np.int64)

def height(num, den, N):
    return f"{num}/{den}"

def safe_prime_group(m, seed):
    """F_p^*, p=2q+1 safe prime, N=p-1 even cyclic; returns (p, g)."""
    rng = random.Random(seed)
    while True:
        q = (1 << (m-1)) - rng.randrange(1, 1 << (m-4))
        if q % 2 and is_prime(q) and is_prime(2*q+1):
            p = 2*q+1
            for g in range(2, p):
                if pow(g, 2, p) != 1 and pow(g, q, p) != 1: return p, g

def fp_star_points(p, g, ks):
    # table method, exact
    N = p-1; s = math.isqrt(N)+1
    lo = [pow(g, j, p) for j in range(s)]; gs = pow(g, s, p); hi = [pow(gs, i, p) for i in range(s+1)]
    ks = np.asarray(ks, dtype=np.int64)
    return (np.array(hi, dtype=np.int64)[ks//s] * np.array(lo, dtype=np.int64)[ks % s]) % p
