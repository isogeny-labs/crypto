#!/usr/bin/env python3
"""Gaudry-style index calculus on E/F_{p^2} versus matched Pollard rho, counted in F_p ops.

F_{p^2} = F_p[t]/(t^2+1), p = 3 mod 4. E: y^2 = x^3 + a x + b, a, b random in F_{p^2},
#E(F_{p^2}) = r prime (found by baby-step giant-step in the Hasse interval).

Index calculus (Weil descent, n = 2):
  factor base: points with x in F_p (vertex = x; canonical y fixed per vertex).
  target R = alpha P + beta Q from an r-adding walk (1 point addition per trial).
  decomposition R = s1 F1 + s2 F2: S3(X1, X2, x(R)) = 0 in symmetric variables
  s = X1 + X2, m = X1 X2 in F_p. The two F_p components E0, E1 of S3 are conics;
  E1 is linear in m, so m = N(s)/D(s) and E0 * D^2 is a quartic in s. Roots over F_p
  by gcd(s^p - s, F) and Cantor-Zassenhaus; then X1, X2 from s, m; both must have
  f(X) a square in F_{p^2}; signs from one point addition.
  linear algebra: every relation has weight <= 2, so it is an edge in a graph on
  factor-base x values; a union-find with signed offsets is exact elimination mod r
  (weighted union-find, the double-large-prime graph of Gaudry-Thome-Theriault-Diem).
  Modes, replayed on the same stream of relations:
     2lp      : every relation kept (all of F_p is the vertex set) -- the natural method
     full:B   : only relations with both x < B kept (plain Gaudry with a smaller base)
     1lp:B    : relations with at least one x < B kept (single large prime)
IC targets: the same 20 multipliers, chosen by an independent RNG each step (so the walk cannot
cycle); a target whose x was already seen is skipped (counted as a trial; its addition is paid)
so the IC arm never uses a rho collision.
Rho: r-adding walk (20 multipliers, point-dependent), store every point, match x (so +-R collide).

Cost unit: F_p multiplications (squarings counted as multiplications), inversions counted
separately (I). Additions/subtractions, hash/dict and union-find bookkeeping are not counted;
mod-r scalar arithmetic of the solver is counted separately (Zr). The walk setup (20 scalar
multiplications for the multipliers, the start point) is identical in both arms and not
counted. Legendre symbols and square roots are exponentiations, counted multiplication by
multiplication.
"""
import json, math, random, sys, time

C = [0, 0, 0]  # F_p mults, F_p inversions, Z_r mults
p = 0


def mul(a, b):
    C[0] += 1
    return a * b % p


def inv(a):
    C[1] += 1
    return pow(a, p - 2, p)


def powc(a, e):
    """square-and-multiply, counted"""
    r = 1
    first = True
    for bit in bin(e)[2:]:
        if not first:
            r = r * r % p; C[0] += 1
        if bit == '1':
            if first:
                r = a % p
            else:
                r = r * a % p; C[0] += 1
        first = False
    return r


def legendre(a):
    a %= p
    if a == 0:
        return 0
    return 1 if powc(a, (p - 1) // 2) == 1 else -1


def sqrt_fp(a):
    """p = 3 mod 4; returns None if not a square"""
    a %= p
    if a == 0:
        return 0
    r = powc(a, (p + 1) // 4)
    C[0] += 1
    if r * r % p != a:
        return None
    return r

# ---------------- F_{p^2} -------------------------------------------------------------

def f2mul(x, y):
    a, b = x; c, d = y
    C[0] += 3
    ac = a * c; bd = b * d
    return ((ac - bd) % p, ((a + b) * (c + d) - ac - bd) % p)


def f2sqr(x):
    a, b = x
    C[0] += 2
    return ((a + b) * (a - b) % p, 2 * a * b % p)


def f2inv(x):
    a, b = x
    C[0] += 4; C[1] += 1
    n = pow((a * a + b * b) % p, p - 2, p)
    return (a * n % p, -b * n % p)


def f2mulfp(x, c):
    C[0] += 2
    return (x[0] * c % p, x[1] * c % p)


def f2add(x, y):
    return ((x[0] + y[0]) % p, (x[1] + y[1]) % p)


def f2sub(x, y):
    return ((x[0] - y[0]) % p, (x[1] - y[1]) % p)


def f2neg(x):
    return ((-x[0]) % p, (-x[1]) % p)


def f2sqrt(x):
    """square root in F_{p^2}, p = 3 mod 4, t^2 = -1; None if non-square"""
    a, b = x
    if b == 0:
        r = sqrt_fp(a)
        if r is not None:
            return (r, 0)
        r = sqrt_fp(-a)
        return (0, r)
    C[0] += 2
    n = sqrt_fp(a * a + b * b)
    if n is None:
        return None
    for nn in (n, -n):
        h = (a + nn) * ((p + 1) // 2) % p
        C[0] += 1
        u = sqrt_fp(h)
        if u is not None and u != 0:
            C[0] += 1
            v = b * inv(2 * u % p) % p
            return (u, v)
    return None

O = None


def padd(P, Q, a):
    if P is None:
        return Q
    if Q is None:
        return P
    if P[0] == Q[0]:
        if (P[1][0] + Q[1][0]) % p == 0 and (P[1][1] + Q[1][1]) % p == 0:
            return None
        # doubling
        x2 = f2sqr(P[0])
        num = f2add(f2add(f2add(x2, x2), x2), a)
        lam = f2mul(num, f2inv(f2add(P[1], P[1])))
    else:
        lam = f2mul(f2sub(Q[1], P[1]), f2inv(f2sub(Q[0], P[0])))
    x3 = f2sub(f2sub(f2sqr(lam), P[0]), Q[0])
    y3 = f2sub(f2mul(lam, f2sub(P[0], x3)), P[1])
    return (x3, y3)


def pneg(P):
    return None if P is None else (P[0], f2neg(P[1]))


def smul(k, P, a):
    R = None
    for bit in bin(k)[2:]:
        R = padd(R, R, a)
        if bit == '1':
            R = padd(R, P, a)
    return R

# ---------------- curve generation (uncounted) ----------------------------------------

def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
    for a_ in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a_, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def rand_point(a, b, rng):
    while True:
        x = (rng.randrange(p), rng.randrange(p))
        rhs = f2add(f2add(f2mul(f2sqr(x), x), f2mul(a, x)), b)
        y = f2sqrt(rhs)
        if y is not None:
            return (x, y)


def find_curve(rng):
    """random curve over F_{p^2} with prime #E; BSGS in [q+1-2p, q+1+2p]"""
    q = p * p
    while True:
        a = (rng.randrange(p), rng.randrange(p)); b = (rng.randrange(p), rng.randrange(p))
        if a[1] == 0 and b[1] == 0:
            continue
        # discriminant
        a3 = f2mul(f2sqr(a), a); b2 = f2sqr(b)
        if (4 * a3[0] + 27 * b2[0]) % p == 0 and (4 * a3[1] + 27 * b2[1]) % p == 0:
            continue
        P = rand_point(a, b, rng)
        L = q + 1 - 2 * p
        W = 4 * p + 1
        m = int(math.isqrt(W)) + 1
        baby = {}
        R = None
        for j in range(m):
            baby.setdefault(None if R is None else (R[0], R[1]), j)
            R = padd(R, P, a)
        mP = R  # m P
        G = smul(L, P, a)
        negmP = pneg(mP)
        found = None
        for i in range(m + 1):
            # G = (L + i m) P;  G == -jP -> N = L+im+j ; G == jP -> N = L+im-j
            for key, sgn in (((None if G is None else (G[0], f2neg(G[1]))), 1), (G, -1)):
                if key in baby:
                    N = L + i * m + sgn * baby[key]
                    if L <= N <= q + 1 + 2 * p and is_prime(N):
                        found = N
                        break
            if found:
                break
            G = padd(G, mP, a)
        if found is None:
            continue
        # recheck
        if smul(found, P, a) is not None:
            continue
        return a, b, found, P

# ---------------- polynomials over F_p (counted) --------------------------------------
# list of coefficients, low degree first

def ptrim(f):
    while f and f[-1] == 0:
        f.pop()
    return f


def pmul(f, g):
    if not f or not g:
        return []
    r = [0] * (len(f) + len(g) - 1)
    for i, x in enumerate(f):
        if x == 0:
            continue
        for j, y in enumerate(g):
            r[i + j] += x * y
    C[0] += len(f) * len(g)
    return ptrim([c % p for c in r])


def psqr(f):
    n = len(f)
    if n == 0:
        return []
    r = [0] * (2 * n - 1)
    for i in range(n):
        r[2 * i] += f[i] * f[i]
        for j in range(i + 1, n):
            r[i + j] += 2 * f[i] * f[j]
    C[0] += n * (n + 1) // 2
    return ptrim([c % p for c in r])


def pmod_monic(f, g):
    """f mod g, g monic"""
    f = f[:]
    dg = len(g) - 1
    while len(f) - 1 >= dg and f:
        c = f[-1]
        sh = len(f) - 1 - dg
        if c:
            for i in range(dg):
                f[sh + i] = (f[sh + i] - c * g[i]) % p
            C[0] += dg
        f.pop()
        ptrim(f)
    return f


def pmonic(f):
    c = f[-1]
    if c == 1:
        return f
    ci = inv(c)
    C[0] += len(f) - 1
    return [x * ci % p for x in f[:-1]] + [1]


def pgcd(f, g):
    f = ptrim(f[:]); g = ptrim(g[:])
    while g:
        g = pmonic(g)
        f, g = g, pmod_monic(f, g)
    return pmonic(f) if f else f


def ppowmod(base, e, g):
    r = [1]
    first = True
    for bit in bin(e)[2:]:
        if not first:
            r = pmod_monic(psqr(r), g)
        if bit == '1':
            r = base[:] if first else pmod_monic(pmul(r, base), g)
        first = False
    return r


def proots(f, rng):
    """distinct roots in F_p of f (deg <= 4)"""
    f = pmonic(ptrim(f[:]))
    if len(f) <= 1:
        return []
    h = ppowmod([0, 1], p, f)          # s^p mod f
    h = h + [0] * (2 - len(h)) if len(h) < 2 else h
    h = h[:]
    h[1] = (h[1] - 1) % p
    g = pgcd(f, ptrim(h))
    return split_roots(g, rng)


def split_roots(g, rng):
    d = len(g) - 1
    if d <= 0:
        return []
    if d == 1:
        return [(-g[0]) % p]
    if d == 2:
        C[0] += 1
        disc = (g[1] * g[1] - 4 * g[0]) % p
        r = sqrt_fp(disc)
        i2 = (p + 1) // 2
        C[0] += 2
        return [((-g[1] + r) * i2) % p, ((-g[1] - r) * i2) % p]
    while True:
        dl = rng.randrange(p)
        h = ppowmod([dl, 1], (p - 1) // 2, g)
        h = h[:] if h else [0]
        h[0] = (h[0] - 1) % p
        u = pgcd(g, ptrim(h))
        du = len(u) - 1
        if 0 < du < d:
            # g / u
            v = pdivexact(g, u)
            return split_roots(u, rng) + split_roots(v, rng)


def pdivexact(f, g):
    """f / g, g monic, exact"""
    f = f[:]
    dg = len(g) - 1
    q = [0] * (len(f) - dg)
    while len(f) - 1 >= dg:
        c = f[-1]
        sh = len(f) - 1 - dg
        q[sh] = c
        for i in range(dg):
            f[sh + i] = (f[sh + i] - c * g[i]) % p
        C[0] += dg
        f.pop()
    return q

# ---------------- Z_r solver: union-find with signed offsets --------------------------

class Solver:
    """F_v = sig * F_parent + (A, B), (A, B) meaning A P + B Q, mod r."""

    def __init__(self, r):
        self.r = r
        self.par = {}
        self.rooted = {}

    def find(self, v):
        if v not in self.par:
            self.par[v] = (v, 1, 0, 0)
            return v, 1, 0, 0
        path = []
        u = v
        while True:
            pu, s, A, B = self.par[u]
            if pu == u:
                break
            path.append(u)
            u = pu
        root = u
        # compress
        r = self.r
        for w in reversed(path):
            pw, s, A, B = self.par[w]
            if pw == root:
                continue
            _, s2, A2, B2 = self.par[pw]
            C[2] += 2
            self.par[w] = (root, s * s2, (s * A2 + A) % r, (s * B2 + B) % r)
        _, s, A, B = self.par[v] if v != root else (v, 1, 0, 0)
        return root, s, A, B

    def add(self, i, si, j, sj, Ra, Rb):
        """relation si F_i + sj F_j = Ra P + Rb Q. Returns (A, B) with A P + B Q = 0, or None."""
        r = self.r
        ri, ci, Ai, Bi = self.find(i)
        rj, cj, Aj, Bj = self.find(j)
        a_i = si * ci; a_j = sj * cj
        # a_i F_ri + a_j F_rj = M
        MA = (Ra - si * Ai - sj * Aj) % r
        MB = (Rb - si * Bi - sj * Bj) % r
        if ri != rj:
            Ki = self.rooted.get(ri); Kj = self.rooted.get(rj)
            if Ki is not None and Kj is not None:
                return ((a_i * Ki[0] + a_j * Kj[0] - MA) % r, (a_i * Ki[1] + a_j * Kj[1] - MB) % r)
            # F_ri = a_i * M - a_i a_j F_rj
            self.par[ri] = (rj, -a_i * a_j, a_i * MA % r, a_i * MB % r)
            if Ki is not None:  # determine F_rj:  Ki = -a_i a_j F_rj + a_i M  ->  F_rj = -a_i a_j (Ki - a_i M)
                s = -a_i * a_j
                self.rooted[rj] = (s * (Ki[0] - a_i * MA) % r, s * (Ki[1] - a_i * MB) % r)
                del self.rooted[ri]
            return None
        c = a_i + a_j
        K = self.rooted.get(ri)
        if K is not None:
            return ((c * K[0] - MA) % r, (c * K[1] - MB) % r)
        if c == 0:
            return (MA, MB)
        ic = pow(c, -1, r); C[2] += 3
        self.rooted[ri] = (MA * ic % r, MB * ic % r)
        return None

# ---------------- the walk -----------------------------------------------------------

class Walk:
    def __init__(self, P, Q, a, r, rng, nm=20):
        self.a = a; self.r = r
        self.mult = []
        for _ in range(nm):
            c = rng.randrange(r); d = rng.randrange(r)
            self.mult.append((padd(smul(c, P, a), smul(d, Q, a), a), c, d))
        c = rng.randrange(r); d = rng.randrange(r)
        self.R = padd(smul(c, P, a), smul(d, Q, a), a); self.c = c; self.d = d
        self.nm = nm

    def step(self, rng=None):
        R = self.R
        if rng is not None:  # IC: multiplier chosen independently of the point (no cycling)
            h = rng.randrange(self.nm)
        else:
            h = (R[0][0] * 7 + R[0][1] * 13 + R[1][0]) % self.nm
        M, c, d = self.mult[h]
        self.R = padd(R, M, self.a)
        self.c = (self.c + c) % self.r; self.d = (self.d + d) % self.r
        return self.R


def solve_eq(A, B, r):
    if B % r == 0:
        return None
    return (-A) * pow(B, -1, r) % r

# ---------------- rho ----------------------------------------------------------------

def rho_store_neg(P, Q, a, r, w, rng):
    C[0] = C[1] = C[2] = 0
    W = Walk(P, Q, a, r, rng)
    C[0] = C[1] = C[2] = 0
    seen = {}
    steps = 0
    t0 = time.time()
    while True:
        R = W.step(); steps += 1
        if R is None:
            continue
        x = R[0]
        if x in seen:
            y, c0, d0 = seen[x]
            if y == R[1]:
                A, B = (W.c - c0) % r, (W.d - d0) % r
            else:
                A, B = (W.c + c0) % r, (W.d + d0) % r
            ww = solve_eq(A, B, r)
            if ww is not None:
                assert ww == w, "rho wrong log"
                return dict(steps=steps, M=C[0], I=C[1], Zr=C[2], sec=time.time() - t0)
        else:
            seen[x] = (R[1], W.c, W.d)

# ---------------- index calculus -------------------------------------------------------

def canon_y(y):
    ny = f2neg(y)
    return y if y <= ny else ny


def decompose(R, a, b, aa, rng):
    """return list of (X1, s1, X2, s2) with R = s1 F_X1 + s2 F_X2 (F_X canonical)."""
    xR = R[0]
    u, v = xR
    if v == 0:
        return []
    X2 = f2sqr(xR)
    axR = f2mul(a, xR); bxR = f2mul(b, xR)
    # coefficients (F_{p^2})
    cm = f2sub(f2neg(f2add(f2add(X2, X2), f2add(X2, X2))), f2add(a, a))
    csm = f2neg(f2add(xR, xR))
    css = X2
    cs = f2sub(f2neg(f2add(axR, axR)), f2add(f2add(b, b), f2add(b, b)))
    c0 = f2sub(aa, f2add(f2add(bxR, bxR), f2add(bxR, bxR)))
    # E1 (imag parts): m*(csm1 s + cm1) + css1 s^2 + cs1 s + c01 = 0
    D = ptrim([cm[1], csm[1]])
    Nn = ptrim([(-c0[1]) % p, (-cs[1]) % p, (-css[1]) % p])
    if len(D) < 1:
        return []
    # E0 * D^2 = Nn^2 + (cm0 + csm0 s) Nn D + (css0 s^2 + cs0 s + c00) D^2
    t1 = psqr(Nn)
    t2 = pmul(pmul(ptrim([cm[0], csm[0]]), Nn), D)
    t3 = pmul(ptrim([c0[0], cs[0], css[0]]), psqr(D))
    n = max(len(t1), len(t2), len(t3))
    F = ptrim([((t1[i] if i < len(t1) else 0) + (t2[i] if i < len(t2) else 0) + (t3[i] if i < len(t3) else 0)) % p for i in range(n)])
    if len(F) <= 1:
        return []
    out = []
    i2 = (p + 1) // 2
    for s in proots(F, rng):
        # m = Nn(s)/D(s)
        Ds = (D[0] + (D[1] * s if len(D) > 1 else 0)) % p
        C[0] += 1
        if Ds == 0:
            continue
        Ns = (Nn[0] + Nn[1] * s + Nn[2] * s * s) % p if len(Nn) == 3 else (sum(c * pow(s, k, p) for k, c in enumerate(Nn)) % p)
        C[0] += 3
        m = mul(Ns, inv(Ds))
        disc = (mul(s, s) - 4 * m) % p
        rt = sqrt_fp(disc)
        if rt is None:
            continue
        X1 = mul(s + rt, i2); Xb = mul(s - rt, i2)
        pts = []
        for X in (X1, Xb):
            x2 = mul(X, X); x3 = mul(x2, X)
            fx = f2add(f2add((x3, 0), f2mulfp(a, X)), b)
            C[0] += 2
            nrm = (fx[0] * fx[0] + fx[1] * fx[1]) % p
            if legendre(nrm) != 1:
                break
            pts.append(fx)
        if len(pts) < 2:
            continue
        P1 = ((X1, 0), canon_y(f2sqrt(pts[0])))
        P2 = ((Xb, 0), canon_y(f2sqrt(pts[1])))
        S = padd(P1, P2, a)
        if S is not None and S[0] == xR:
            s1 = 1 if S[1] == R[1] else -1
            out.append((X1, s1, Xb, s1))
        else:
            D2 = padd(P1, pneg(P2), a)
            assert D2 is not None and D2[0] == xR, "decomposition inconsistent"
            s1 = 1 if D2[1] == R[1] else -1
            out.append((X1, s1, Xb, -s1))
    return out


def ic_run(P, Q, a, b, r, w, rng, modes, cap_factor=8.0):
    """one stream of trials; each mode replays the same relations through its own solver."""
    C[0] = C[1] = C[2] = 0
    W = Walk(P, Q, a, r, rng)
    aa = f2sqr(a)
    C[0] = C[1] = C[2] = 0
    solvers = {m: Solver(r) for m in modes}
    done = {}
    trials = 0; rels = 0
    kept = {m: 0 for m in modes}
    t0 = time.time()
    first_trials = None
    seenx = set()
    dup = 0
    while len(done) < len(modes):
        R = W.step(rng); trials += 1
        if R is None:
            continue
        if R[0] in seenx:   # a repeated target would be a rho collision, not index calculus: skip it
            dup += 1
            continue
        seenx.add(R[0])
        for (X1, s1, X2, s2) in decompose(R, a, b, aa, rng):
            rels += 1
            for m in modes:
                if m in done:
                    continue
                kind, B = m
                nsmall = (X1 < B) + (X2 < B)
                need = {'2lp': 0, '1lp': 1, 'full': 2}[kind]
                if nsmall < need:
                    continue
                kept[m] += 1
                zr0 = C[2]
                eq = solvers[m].add(X1, s1, X2, s2, W.c, W.d)
                if eq is not None:
                    ww = solve_eq(eq[0], eq[1], r)
                    if ww is not None:
                        assert ww == w, "IC wrong log"
                        done[m] = dict(trials=trials, rels_seen=rels, rels_kept=kept[m], M=C[0], I=C[1],
                                       Zr_total=C[2], sec=time.time() - t0, vertices=len(solvers[m].par), dup_skipped=dup)
        if first_trials is None and done:
            first_trials = trials
        if first_trials is not None and trials > cap_factor * first_trials:
            break
    for m in modes:
        if m not in done:
            done[m] = dict(censored=True, trials=trials, M=C[0], I=C[1])
    return done


def main():
    global p
    p = int(sys.argv[1]); seed = int(sys.argv[2]); ntarg = int(sys.argv[3]); nrho = int(sys.argv[4])
    out = sys.argv[5]
    assert p % 4 == 3 and is_prime(p)
    rng = random.Random(seed)
    t0 = time.time()
    a, b, r, _ = find_curve(rng)
    G = rand_point(a, b, rng)
    tgen = time.time() - t0
    modes = [('2lp', p), ('full', p // 2), ('1lp', p // 2), ('1lp', p // 4)]
    res = dict(p=p, seed=seed, a=a, b=b, r=r, log2r=math.log2(r), gen_sec=tgen, targets=[])
    for k in range(ntarg):
        w = rng.randrange(1, r)
        Q = smul(w, G, a)
        rho = [rho_store_neg(G, Q, a, r, w, rng) for _ in range(nrho)]
        ic = ic_run(G, Q, a, b, r, w, rng, modes)
        res['targets'].append(dict(w=w, rho=rho, ic={f"{m[0]}:{m[1]}": v for m, v in ic.items()}))
        print(p, seed, k, 'rho steps', [x['steps'] for x in rho], 'ic', {f"{m[0]}:{m[1]}": (v.get('trials'), v.get('M')) for m, v in ic.items()}, flush=True)
    with open(out, 'w') as fh:
        json.dump(res, fh, indent=1)


if __name__ == '__main__':
    main()
