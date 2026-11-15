#!/usr/bin/env python3
"""Track B: 2-torsion and symmetry folds for Gaudry index calculus on E/F_{p^2}.

Imports the unchanged copy of ext_field/ic_fp2.py (ic_fp2_copy.py) for F_{p^2} arithmetic,
point addition, the union-find solver, the walk and store-all +-rho.

Two arms, each with its own matched store-all +-rho on the same curves:

  base  : prime-order curves, the ext_field decomposition (x in F_p, quartic in s,
          gcd(s^p - s, F)), 2lp mode. Instrumented so that the same run also gives
              - shift : multiplying by s (or s + d) inside s^p mod F costs a shift and the
                        reduction only (the pmul by [0,1] is not charged);
              - win   : every F_p exponentiation (Legendre, square roots) costed by the best
                        sliding-window chain instead of binary square-and-multiply;
              - jac   : every Legendre symbol priced as one Jacobi computation = 1 I.
  tors  : curves y^2 = x^3 + a x + b over F_{p^2} with a rational 2-torsion point
          T = (x0, 0), f'(x0) = c^2 a square, #E = 4 r, r prime. Factor base: points with
          x = x0 + c t, t in F_p. The line is closed under P -> P + T (t -> 1/t), so the
          class of P is {+-P, +-(P+T)} and the vertex is sigma = t + 1/t (FGHR, m = 2).
          The descended system is linear in (S, P, D) = (s1+s2, s1 s2, (t1-1/t1)(t2-1/t2))
          with D^2 = (P+4)^2 - 4 S^2: one F_p quadratic, then square roots
          (no gcd(s^p - s, .)). Two solvers on the same relation stream:
              fold   : vertex sigma (columns of 4 points), relation on [4]-images;
              nofold : vertex (sigma, branch) (columns of 2 points) -- the same oracle
                       without the T column fold.

Cost unit: F_p multiplications M (squarings = M), inversions I counted separately;
priced I = 3M and I = 1.5 log2(p) M. Legendre calls (count, and the M they spent) and the
binary-vs-window exponentiation difference are recorded so every pricing variant is
recomputed from the same run.
"""
import json, math, random, sys, time
import ic_fp2_copy as B

C = B.C
X = dict(nL=0, ML=0, win_save=0, shift_save=0, lk=0, table_M=0)   # extra counters
VAR = 'naive'      # naive | memo | table  (how F_p Legendre symbols and square roots are done)
SQ = None          # square-root table for VAR == 'table'

# ---------------- instrumentation --------------------------------------------------------
_win_cache = {}


def window_cost(e):
    """multiplications (squarings included) of the best left-to-right sliding-window chain."""
    if e in _win_cache:
        return _win_cache[e]
    bits = bin(e)[2:]
    best = None
    for w in range(1, 7):
        pre = 0 if w == 1 else 1 + (2 ** (w - 1) - 1)  # a^2, then a^3, a^5, ..., a^(2^w-1)
        i = 0; n = len(bits); cost = 0; first = True
        while i < n:
            if bits[i] == '0':
                if not first:
                    cost += 1
                i += 1
                continue
            j = min(n, i + w)
            while bits[j - 1] == '0':
                j -= 1
            L = j - i
            if first:
                first = False
            else:
                cost += L + 1   # L squarings + 1 multiply
            i = j
        tot = cost + pre
        if best is None or tot < best:
            best = tot
    _win_cache[e] = best
    return best


def binary_cost(e):
    return (e.bit_length() - 1) + (bin(e).count('1') - 1)


_powc_orig = B.powc


def powc(a, e):
    X['win_save'] += binary_cost(e) - window_cost(e)
    return _powc_orig(a, e)


B.powc = powc


def legendre(a):
    X['nL'] += 1
    m0 = C[0]
    a %= B.p
    if a == 0:
        r = 0
    else:
        r = 1 if powc(a, (B.p - 1) // 2) == 1 else -1
    X['ML'] += C[0] - m0
    return r


B.legendre = legendre

_sqrt_orig = B.sqrt_fp


def build_table():
    """SQ[z] = smallest square root of z (or -1): (p-1)/2 counted squarings."""
    global SQ
    p = B.p
    SQ = [-1] * p
    SQ[0] = 0
    for u in range(1, (p - 1) // 2 + 1):
        SQ[u * u % p] = u
    C[0] += (p - 1) // 2
    X['table_M'] += (p - 1) // 2


def sqrt_tab(a):
    X['lk'] += 1
    v = SQ[a % B.p]
    return None if v < 0 else v


def legendre_tab(a):
    X['nL'] += 1; X['lk'] += 1
    a %= B.p
    if a == 0:
        return 0
    return 1 if SQ[a] > 0 else -1


def set_variant(v):
    global VAR
    VAR = v
    if v == 'table':
        B.sqrt_fp = sqrt_tab; B.legendre = legendre_tab
    else:
        B.sqrt_fp = _sqrt_orig; B.legendre = legendre


def ppowmod(base, e, g):
    """copy of the original; records the pmul cost that a shift would avoid when base = s + d."""
    r = [1]
    first = True
    for bit in bin(e)[2:]:
        if not first:
            r = B.pmod_monic(B.psqr(r), g)
        if bit == '1':
            if first:
                r = base[:]
            else:
                if len(base) == 2 and base[1] == 1:
                    X['shift_save'] += len(r) * 2 - (len(r) if base[0] else 0)
                r = B.pmod_monic(B.pmul(r, base), g)
        first = False
    return r


B.ppowmod = ppowmod


def snapshot():
    return dict(M=C[0], I=C[1], Zr=C[2], nL=X['nL'], ML=X['ML'], win_save=X['win_save'],
                shift_save=X['shift_save'], lk=X['lk'], table_M=X['table_M'])


def reset():
    C[0] = C[1] = C[2] = 0
    for k in X:
        X[k] = 0

# ---------------- uncounted F_{p^2} helpers (setup only) --------------------------------

def um(x, y):
    p = B.p
    return ((x[0] * y[0] - x[1] * y[1]) % p, (x[0] * y[1] + x[1] * y[0]) % p)


def ua(x, y):
    return ((x[0] + y[0]) % B.p, (x[1] + y[1]) % B.p)


def us(x, k):
    return ((x[0] * k) % B.p, (x[1] * k) % B.p)


ZERO = (0, 0); ONE = (1, 0)

# bivariate polys in t1, t2 with F_{p^2} coefficients: dict (i, j) -> coef


def bp_add(f, g):
    h = dict(f)
    for k, v in g.items():
        h[k] = ua(h.get(k, ZERO), v)
    return {k: v for k, v in h.items() if v != ZERO}


def bp_mul(f, g):
    h = {}
    for (i, j), u in f.items():
        for (k, l), v in g.items():
            h[(i + k, j + l)] = ua(h.get((i + k, j + l), ZERO), um(u, v))
    return {k: v for k, v in h.items() if v != ZERO}


def bp_sc(f, c):
    return {k: um(v, c) for k, v in f.items() if um(v, c) != ZERO}


def s3_coeffs(a, b, x0, c):
    """S3(x0 + c t1, x0 + c t2, x3) = A(t) x3^2 + Bc(t) x3 + Cc(t); return (A, Bc, Cc)."""
    x1 = {(0, 0): x0, (1, 0): c}
    x2 = {(0, 0): x0, (0, 1): c}
    m1 = {(0, 0): us(ONE, B.p - 1)}
    d = bp_add(x1, bp_sc(x2, us(ONE, B.p - 1)))
    A = bp_mul(d, d)
    s = bp_add(x1, x2)
    pr = bp_mul(x1, x2)
    Bc = bp_sc(bp_add(bp_mul(s, bp_add(pr, {(0, 0): a})), {(0, 0): us(b, 2)}), us(ONE, B.p - 2))
    t = bp_add(pr, {(0, 0): us(a, B.p - 1)})
    Cc = bp_add(bp_mul(t, t), bp_sc(s, us(b, B.p - 4)))
    return A, Bc, Cc

# ---------------- torsion-family curves -------------------------------------------------

def find_tors_curve(rng):
    p = B.p
    q = p * p
    tries = 0
    while True:
        tries += 1
        x0 = (rng.randrange(p), rng.randrange(p)); a = (rng.randrange(p), rng.randrange(p))
        b = B.f2neg(ua(um(um(x0, x0), x0), um(a, x0)))
        if a[1] == 0 and b[1] == 0:
            continue
        a3 = um(um(a, a), a); b2 = um(b, b)
        if ua(us(a3, 4), us(b2, 27)) == ZERO:
            continue
        fp = ua(us(um(x0, x0), 3), a)
        if fp == ZERO:
            continue
        c = B.f2sqrt(fp)
        if c is None:
            continue
        P = B.rand_point(a, b, rng)
        L = q + 1 - 2 * p
        W = 4 * p + 1
        m = int(math.isqrt(W)) + 1
        baby = {}
        R = None
        for j in range(m):
            if R is not None:
                baby.setdefault(R, j)
            R = B.padd(R, P, a)
        mP = R
        G = B.smul(L, P, a)
        N = None
        for i in range(m + 1):
            if G is None:
                N = L + i * m; break
            if G in baby:
                N = L + i * m - baby[G]; break
            nG = B.pneg(G)
            if nG in baby:
                N = L + i * m + baby[nG]; break
            G = B.padd(G, mP, a)
        if N is None or N % 4 or not B.is_prime(N // 4):
            continue
        if B.smul(N, P, a) is not None:
            continue
        r = N // 4
        while True:
            G = B.smul(4, B.rand_point(a, b, rng), a)
            if G is not None:
                break
        assert B.smul(r, G, a) is None
        return a, b, x0, c, r, G, tries

# ---------------- symmetric oracle --------------------------------------------------------

class TorsOracle:
    def __init__(self, a, b, x0, c):
        p = B.p
        self.a, self.b, self.x0, self.c = a, b, x0, c
        A, Bc, Cc = s3_coeffs(a, b, x0, c)
        # normalise the equation by c^-2 (setup): the x3^2 coefficients become -2, 0, 1, 0
        c2 = um(c, c); n = (c2[0] * c2[0] + c2[1] * c2[1]) % p
        ni = pow(n, p - 2, p); c2i = ((c2[0] * ni) % p, (-c2[1] * ni) % p)
        A, Bc, Cc = (bp_sc(A, c2i), bp_sc(Bc, c2i), bp_sc(Cc, c2i))
        # symmetry checks: g_ij = g_ji = g_{2-i,2-j}
        for poly in (A, Bc, Cc):
            for (i, j), v in poly.items():
                assert i <= 2 and j <= 2
                assert poly.get((j, i), ZERO) == v
                assert poly.get((2 - i, 2 - j), ZERO) == v, "T-symmetry fails"
        self.g = {}
        for key in ((1, 1), (0, 1), (0, 0), (0, 2)):
            self.g[key] = (A.get(key, ZERO), Bc.get(key, ZERO), Cc.get(key, ZERO))
        self.x03 = us(x0, 3)
        self.T = (x0, (0, 0))
        self.memo = {}
        self.valid = {}
        self.i2 = (p + 1) // 2

    def geval(self, key, xR, xR2):
        A, Bc, Cc = self.g[key]
        v = Cc
        if A != ZERO:
            if A[1] == 0 and A[0] in (1, 2, B.p - 1, B.p - 2):   # small integer: additions only
                v = B.f2add(v, us(xR2, A[0]))
            else:
                v = B.f2add(v, B.f2mul(A, xR2))
        if Bc != ZERO:
            v = B.f2add(v, B.f2mul(Bc, xR))
        return v

    def vertex(self, s):
        """memoised canonical data for sigma: (t_c, delta_c, w).  Counted once per sigma."""
        v = self.memo.get(s)
        if v is not None:
            return v
        p = B.p
        if VAR != 'naive' and s in self.valid:
            dl = self.valid[s]
        else:
            dl = B.sqrt_fp((B.mul(s, s) - 4) % p)
        dl = min(dl, p - dl)
        tc = B.mul((s + dl) % p, self.i2)
        w = B.f2sqrt(B.f2add(B.f2mulfp(self.c, s), self.x03))
        w = B.canon_y(w)
        v = (tc, dl, w)
        self.memo[s] = v
        return v

    def point(self, t, w):
        ct = B.f2mulfp(self.c, t)
        return (B.f2add(self.x0, ct), B.f2mul(ct, w))

    def sigma_ok(self, s):
        p = B.p
        if VAR != 'naive':
            X['lk'] += 1
            if s in self.valid:
                return self.valid[s] is not None
            ok = None
            if s not in (2, p - 2):
                dl = B.sqrt_fp((B.mul(s, s) - 4) % p)
                if dl:
                    u = B.f2add(B.f2mulfp(self.c, s), self.x03)
                    if u != ZERO:
                        C[0] += 2
                        if B.legendre((u[0] * u[0] + u[1] * u[1]) % p) == 1:
                            ok = dl
            self.valid[s] = ok
            return ok is not None
        if s in self.memo:
            return True
        if s in (2, p - 2):
            return False
        if B.legendre((B.mul(s, s) - 4) % p) != 1:
            return False
        u = B.f2add(B.f2mulfp(self.c, s), self.x03)
        if u == ZERO:
            return False
        C[0] += 2
        return B.legendre((u[0] * u[0] + u[1] * u[1]) % p) == 1

    def decompose(self, R):
        """list of (s1, e1, br1, s2, e2, br2): R = e1 P(s1, br1) + e2 P(s2, br2) exactly,
        P(s, 'c') = P(t_c), P(s, 'f') = P(1/t_c) = -(P(t_c) + T)."""
        p = B.p
        xR = R[0]
        xR2 = B.f2sqr(xR)
        g11 = self.geval((1, 1), xR, xR2); g01 = self.geval((0, 1), xR, xR2)
        g00 = self.geval((0, 0), xR, xR2); g02 = self.geval((0, 2), xR, xR2)
        k = B.f2add(g11, g11); aS = B.f2add(g01, g01)
        bP = B.f2add(g00, g02); dD = B.f2sub(g00, g02)
        k0, k1 = k; a0, a1 = aS; b0, b1 = bP; d0, d1 = dD
        det = (B.mul(a0, b1) - B.mul(a1, b0)) % p
        if det == 0:
            return []
        s0 = (B.mul(k1, b0) - B.mul(k0, b1)) % p
        s1 = (B.mul(d1, b0) - B.mul(d0, b1)) % p
        p0 = (B.mul(a1, k0) - B.mul(a0, k1)) % p
        p1 = (B.mul(a1, d0) - B.mul(a0, d1)) % p
        q0 = (p0 + 4 * det) % p
        QA = (B.mul(p1, p1) - 4 * B.mul(s1, s1) - B.mul(det, det)) % p
        QB = (B.mul(q0, p1) - 4 * B.mul(s0, s1)) % p     # half the linear coefficient
        QC = (B.mul(q0, q0) - 4 * B.mul(s0, s0)) % p
        if QA == 0:
            if QB == 0:
                return []
            Ds = [B.mul((-QC) % p, B.inv(2 * QB % p))]
        else:
            disc = (B.mul(QB, QB) - B.mul(QA, QC)) % p
            if VAR == 'naive' and B.legendre(disc) == -1:
                return []
            rt = B.sqrt_fp(disc)
            if rt is None:
                return []
            iA = B.inv(QA)
            Ds = [B.mul((-QB + rt) % p, iA)] if rt == 0 else \
                 [B.mul((-QB + rt) % p, iA), B.mul((-QB - rt) % p, iA)]
        idet = B.inv(det)
        out = []
        for D in Ds:
            S = B.mul((s0 + B.mul(s1, D)) % p, idet)
            P = B.mul((p0 + B.mul(p1, D)) % p, idet)
            dsc = (B.mul(S, S) - 4 * P) % p
            if VAR == 'naive' and B.legendre(dsc) == -1:
                continue
            r2 = B.sqrt_fp(dsc)
            if r2 is None:
                continue
            sg1 = B.mul((S + r2) % p, self.i2); sg2 = B.mul((S - r2) % p, self.i2)
            if sg2 < sg1:          # canonical order, independent of which square root came back
                sg1, sg2 = sg2, sg1
            if not (self.sigma_ok(sg1) and self.sigma_ok(sg2)):
                continue
            tc1, dl1, w1 = self.vertex(sg1)
            tc2, dl2, w2 = self.vertex(sg2)
            # choose t1 = t_c(sg1); then delta2 = D / dl1 decides the branch of t2
            br2 = 'c' if B.mul(dl1, dl2) == D else 'f'
            if br2 == 'f':
                assert B.mul(dl1, (p - dl2) % p) == D
            t2 = tc2 if br2 == 'c' else (sg2 - tc2) % p
            P1 = self.point(tc1, w1); P2 = self.point(t2, w2)
            Ssum = B.padd(P1, P2, self.a)
            if Ssum is not None and Ssum[0] == xR:
                e1 = 1 if Ssum[1] == R[1] else -1; e2 = e1
            else:
                Dd = B.padd(P1, B.pneg(P2), self.a)
                assert Dd is not None and Dd[0] == xR, "tors decomposition inconsistent"
                e1 = 1 if Dd[1] == R[1] else -1; e2 = -e1
            out.append((sg1, e1, 'c', sg2, e2, br2))
        return out


def tors_ic_run(G, Q, a, r, w, orc, rng, cap_factor=8.0):
    reset()
    W = B.Walk(G, Q, a, r, rng)
    reset()
    orc.memo = {}; orc.valid = {}
    if VAR == 'table':
        build_table()
    solvers = {'fold': B.Solver(r), 'nofold': B.Solver(r)}
    done = {}
    trials = 0; rels = 0; dup = 0
    seenx = set()
    t0 = time.time(); first = None
    while len(done) < 2:
        R = W.step(rng); trials += 1
        if R is None:
            continue
        if R[0] in seenx:
            dup += 1
            continue
        seenx.add(R[0])
        for (s1, e1, b1, s2, e2, b2) in orc.decompose(R):
            rels += 1
            Ra = 4 * W.c % r; Rb = 4 * W.d % r     # [4]R = e1 [4]P1 + e2 [4]P2
            for mode, S in solvers.items():
                if mode in done:
                    continue
                if mode == 'fold':
                    # [4]P(s,'f') = -[4]P(s,'c')
                    eq = S.add(s1, e1, s2, e2 if b2 == 'c' else -e2, Ra, Rb)
                else:
                    eq = S.add((s1, b1), e1, (s2, b2), e2, Ra, Rb)
                if eq is not None:
                    ww = B.solve_eq(eq[0], eq[1], r)
                    if ww is not None:
                        assert ww == w, "tors IC wrong log"
                        d = snapshot(); d['lk'] += trials + 2 * rels
                        d.update(trials=trials, rels=rels, vertices=len(S.par),
                                                 memo=len(orc.memo), dup_skipped=dup,
                                                 sec=time.time() - t0)
                        done[mode] = d
        if first is None and done:
            first = trials
        if first is not None and trials > cap_factor * first:
            break
    for mode in solvers:
        if mode not in done:
            d = snapshot(); d.update(censored=True, trials=trials)
            done[mode] = d
    return done


def base_ic_run(G, Q, a, b, r, w, rng):
    reset()
    if VAR == 'table':
        build_table()
    res = B.ic_run(G, Q, a, b, r, w, rng, [('2lp', B.p)])
    d = res[('2lp', B.p)]
    d.update(nL=X['nL'], ML=X['ML'], win_save=X['win_save'], shift_save=X['shift_save'],
             table_M=X['table_M'], lk=X['lk'] + d['trials'] + 2 * d['rels_kept'])
    return d


def rho(G, Q, a, r, w, rng):
    reset()
    d = B.rho_store_neg(G, Q, a, r, w, rng)
    d.update(nL=X['nL'], ML=X['ML'], win_save=X['win_save'], shift_save=X['shift_save'],
             lk=d['steps'], table_M=0)
    return d


def main():
    arm = sys.argv[1]
    var = 'naive'
    if ':' in arm:
        arm, var = arm.split(':')
    set_variant('naive')   # curve generation always uses the exponentiation square roots
    p = int(sys.argv[2]); seed = int(sys.argv[3])
    ntarg = int(sys.argv[4]); nrho = int(sys.argv[5]); out = sys.argv[6]
    B.p = p
    assert p % 4 == 3 and B.is_prime(p)
    rng = random.Random(seed) if arm == "base" else random.Random(1000003 * seed + 7)   # base: ext_field curves
    t0 = time.time()
    if arm == 'base':
        a, b, r, _ = B.find_curve(rng)
        G = B.rand_point(a, b, rng)
        info = dict(a=a, b=b)
    else:
        a, b, x0, c, r, G, tries = find_tors_curve(rng)
        orc = TorsOracle(a, b, x0, c)
        info = dict(a=a, b=b, x0=x0, c=c, curve_tries=tries)
    set_variant(var)
    reset()
    res = dict(arm=arm, variant=var, p=p, seed=seed, r=r, log2r=math.log2(r), gen_sec=time.time() - t0,
               targets=[], **info)
    for k in range(ntarg):
        w = rng.randrange(1, r)
        Q = B.smul(w, G, a)
        rh = [rho(G, Q, a, r, w, rng) for _ in range(nrho)]
        if arm == 'base':
            ic = {'2lp': base_ic_run(G, Q, a, b, r, w, rng)}
        else:
            ic = tors_ic_run(G, Q, a, r, w, orc, rng)
        res['targets'].append(dict(w=w, rho=rh, ic=ic))
        print(arm, var, p, seed, k, 'rho', [x['steps'] for x in rh],
              'ic', {m: (v['trials'], v['M']) for m, v in ic.items()}, flush=True)
    with open(out, 'w') as fh:
        json.dump(res, fh, indent=0)


if __name__ == '__main__':
    main()
