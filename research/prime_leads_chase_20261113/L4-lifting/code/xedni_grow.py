"""Q3 toy: growing-rank plane-cubic xedni (k = 2..8 lifted points, base O = (0:1:0)).

usage: xedni_grow.py ARM BITS K TRIALS SEED OUT.jsonl     ARM in {H, B, P}
H: honest fresh-mask lifts; B: box bound on E(F_p) (no lift); P: planted P3 = P1 (+) P2 over Q.
"""
import sys, json, random, time
from math import gcd
from itertools import product
from fpylll import IntegerMatrix, LLL

L1 = (1 << 61) - 1          # Mersenne prime
L2 = 2305843009213693921    # prime < 2^61 (checked at import)

# monomials: x^3, x^2y, xy^2, y^3, x^2z, xyz, y^2z, xz^2, yz^2, z^3
EXPS = [(3, 0, 0), (2, 1, 0), (1, 2, 0), (0, 3, 0), (2, 0, 1), (1, 1, 1), (0, 2, 1), (1, 0, 2), (0, 1, 2), (0, 0, 3)]


def is_prime(n):
    if n < 2:
        return False
    for sp in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % sp == 0:
            return n == sp
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


assert is_prime(L1) and is_prime(L2)


class Cubic:
    """Plane cubic F(x,y,z)=sum c_m x^a y^b z^c over Z (mod=None) or Z/mod."""

    def __init__(self, c, mod=None):
        self.mod = mod
        self.c = [ci % mod for ci in c] if mod else list(c)

    def _r(self, v):
        return v % self.mod if self.mod else v

    def F(self, P):
        x, y, z = P
        s = 0
        for ci, (a, b, e) in zip(self.c, EXPS):
            if ci:
                s += ci * x ** a * y ** b * z ** e
        return self._r(s)

    def grad(self, P):
        x, y, z = P
        g = [0, 0, 0]
        for ci, (a, b, e) in zip(self.c, EXPS):
            if not ci:
                continue
            if a:
                g[0] += ci * a * x ** (a - 1) * y ** b * z ** e
            if b:
                g[1] += ci * b * x ** a * y ** (b - 1) * z ** e
            if e:
                g[2] += ci * e * x ** a * y ** b * z ** (e - 1)
        return [self._r(t) for t in g]

    def norm(self, P):
        if self.mod:
            m = self.mod
            for i in (2, 1, 0):
                if P[i] % m:
                    inv = pow(P[i], -1, m)
                    return tuple(t * inv % m for t in P)
            raise ZeroDivisionError('zero point')
        g = gcd(gcd(P[0], P[1]), P[2])
        if g == 0:
            raise ZeroDivisionError('zero point')
        P = tuple(t // g for t in P)
        for t in (P[2], P[1], P[0]):
            if t:
                return P if t > 0 else tuple(-u for u in P)
        return P

    def eq(self, A, B):
        cr = (A[1] * B[2] - A[2] * B[1], A[2] * B[0] - A[0] * B[2], A[0] * B[1] - A[1] * B[0])
        return all(self._r(t) == 0 for t in cr)

    def third(self, A, B):
        if self.eq(A, B):
            g = self.grad(A)
            if all(t == 0 for t in g):
                raise ValueError('singular')
            for e in ((1, 0, 0), (0, 1, 0), (0, 0, 1)):
                Bp = (g[1] * e[2] - g[2] * e[1], g[2] * e[0] - g[0] * e[2], g[0] * e[1] - g[1] * e[0])
                Bp = tuple(self._r(t) for t in Bp)
                if any(Bp) and not self.eq(Bp, A):
                    break
            fb = self.F(Bp)
            gb = self.grad(Bp)
            pa = self._r(sum(a * b for a, b in zip(A, gb)))
            C = tuple(self._r(fb * a - pa * b) for a, b in zip(A, Bp))
        else:
            gA, gB = self.grad(A), self.grad(B)
            u = self._r(sum(a * b for a, b in zip(A, gB)))  # <A, gradF(B)>
            v = self._r(sum(a * b for a, b in zip(B, gA)))  # <B, gradF(A)>
            if u == 0 and v == 0:
                raise ValueError('line in curve')
            C = tuple(self._r(u * a - v * b) for a, b in zip(A, B))
        return self.norm(C)


class Group:
    def __init__(self, cub, O):
        self.C = cub
        self.O = cub.norm(O)
        self.S = cub.third(self.O, self.O)

    def add(self, A, B):
        return self.C.third(self.O, self.C.third(A, B))

    def neg(self, A):
        return self.C.third(A, self.S)

    def mul(self, n, A):
        if n < 0:
            n, A = -n, self.neg(A)
        R = self.O
        while n:
            if n & 1:
                R = self.add(R, A)
            A = self.add(A, A)
            n >>= 1
        return R

    def is_O(self, A):
        return self.C.eq(A, self.O)


def relations_box(G, pts, cbox=2):
    """All n in {-cbox..cbox}^k \\ 0 (up to sign) with sum n_i P_i = O, via meet in the middle."""
    k = len(pts)
    rng = range(-cbox, cbox + 1)
    mult = [{m: G.mul(m, P) for m in rng} for P in pts]
    a = (k + 1) // 2
    left = {}

    def sums(idx):
        out = [((), G.O)]
        for i in idx:
            out = [(n + (m,), G.add(S, mult[i][m]) if m else S) for (n, S) in out for m in rng]
        return out
    for n, S in sums(range(a)):
        left.setdefault(S, []).append(n)
    rels = []
    for n, S in sums(range(a, k)):
        T = G.neg(S)
        for nl in left.get(T, []):
            v = nl + n
            if any(v):
                first = next(t for t in v if t)
                if first > 0:
                    rels.append(v)
    return rels


# ---------------- finite field curve -----------------

def legendre_count(A, B, p):
    import numpy as np
    x = np.arange(p, dtype=np.int64)
    f = (x * x % p * x + A * x + B) % p
    # Euler criterion, vectorised square-and-multiply
    e = (p - 1) // 2
    r = np.ones(p, dtype=np.int64)
    b = f.copy()
    while e:
        if e & 1:
            r = r * b % p
        b = b * b % p
        e >>= 1
    n_sq = int(np.sum(r == 1)); n_zero = int(np.sum(f == 0))
    return 1 + n_zero + 2 * n_sq


def make_curve(bits, rng):
    while True:
        p = rng.randrange(1 << (bits - 1), 1 << bits)
        if not is_prime(p) or p % 4 != 3:
            continue
        for _ in range(200):
            A, B = rng.randrange(p), rng.randrange(p)
            if (4 * A ** 3 + 27 * B * B) % p == 0:
                continue
            N = legendre_count(A, B, p)
            if is_prime(N) and N != p:
                # random point
                while True:
                    x = rng.randrange(p)
                    f = (x ** 3 + A * x + B) % p
                    if pow(f, (p - 1) // 2, p) == 1:
                        y = pow(f, (p + 1) // 4, p)
                        return p, A, B, N, (x, y, 1)


def cE(A, B):
    c = [0] * 10
    c[0] = -1; c[6] = 1; c[7] = -A; c[9] = -B
    return c


def fit_cubic(pts, p, A, B):
    """Integer cubic through O=(0:1:0) and pts (affine ints), congruent to lambda*E mod p, small."""
    ce = cE(A, B)
    idx = [0, 1, 2, 4, 5, 6, 7, 8, 9]  # drop y^3
    ce9 = [ce[i] for i in idx]
    rows9 = [[x ** 3, x * x * y, x * y * y, x * x, x * y, y * y, x, y, 1] for (x, y) in pts]
    k = len(pts)
    W = 1 << (p.bit_length() * (3 * k + 3) + 64)
    gens = [ce9] + [[p if j == i else 0 for j in range(9)] for i in range(9)]
    Bm = []
    for g in gens:
        tail = [W * sum(r[j] * g[j] for j in range(9)) for r in rows9]
        Bm.append(g + tail)
    M = IntegerMatrix.from_matrix(Bm)
    LLL.reduction(M)
    best = None
    for i in range(M.nrows):
        row = [M[i, j] for j in range(9 + k)]
        if any(row[9:]) or not any(row[:9]):
            continue
        c9 = row[:9]
        # lambda: c9 == lam*ce9 mod p ; ce9[5] = 1 (y^2 z)
        lam = c9[5] % p
        if lam == 0:
            continue
        if any((c9[j] - lam * ce9[j]) % p for j in range(9)):
            continue
        nrm = sum(t * t for t in c9)
        if best is None or nrm < best[0]:
            best = (nrm, c9)
    if best is None:
        return None
    c9 = best[1]
    c = [0] * 10
    for j, i in enumerate(idx):
        c[i] = c9[j]
    return c


def run_trial(arm, p, A, B, q, P0, Q0, xlog, k, rng, cbox=2):
    Ep = Group(Cubic(cE(A, B), p), (0, 1, 0))
    s = [rng.randrange(q) for _ in range(k)]
    t = [rng.randrange(q) for _ in range(k)]
    R = [Ep.add(Ep.mul(s[i], P0), Ep.mul(t[i], Q0)) for i in range(k)]
    if arm == 'P' and k >= 3:
        s[2] = (s[0] + s[1]) % q; t[2] = (t[0] + t[1]) % q
        R[2] = Ep.add(R[0], R[1])
    rec = {'k': k}
    if any(r[2] == 0 for r in R):
        rec['status'] = 'R_is_O'; return rec
    def cl(v):
        return v - p if v > p // 2 else v
    Rn = [Ep.C.norm(r) for r in R]
    lifts = [(cl(r[0]), cl(r[1])) for r in Rn]
    if arm == 'B':
        rels = relations_box(Ep, Rn, cbox)
        classes = {}
        for i, r in enumerate(Rn):
            classes.setdefault(r, []).append(i)
        rec.update(classify(rels, list(classes.values()), s, t, q, xlog))
        rec['status'] = 'ok'
        return rec
    distinct = []
    for L in lifts:
        if L not in distinct:
            distinct.append(L)
    fitpts = [L for i, L in enumerate(distinct)] if arm == 'H' else [L for L in distinct if L != lifts[2]]
    c = fit_cubic(fitpts, p, A, B)
    if c is None:
        rec['status'] = 'nofit'; return rec
    rec['coef_bits'] = max(abs(v).bit_length() for v in c)
    CQ = Cubic(c)
    GQ = Group(CQ, (0, 1, 0))
    Pq = [(x, y, 1) for (x, y) in lifts]
    try:
        if arm == 'P':
            Pq[2] = GQ.add(Pq[0], Pq[1])
            rec['P3_bits'] = max(abs(v).bit_length() for v in Pq[2])
        if not all(CQ.F(P) == 0 for P in Pq):
            rec['status'] = 'offcurve'; return rec
        G1 = Group(Cubic(c, L1), (0, 1, 0))
        P1 = [G1.C.norm(P) for P in Pq]
        rels = relations_box(G1, P1, cbox)
    except (ValueError, ZeroDivisionError) as e:
        rec['status'] = 'degenerate:' + str(e); return rec
    # confirm mod L2 and exactly over Q (first few)
    G2 = Group(Cubic(c, L2), (0, 1, 0))
    P2 = [G2.C.norm(P) for P in Pq]
    conf = []
    nexact = 0
    for n in rels:
        S = G2.O
        for ni, Pi in zip(n, P2):
            if ni:
                S = G2.add(S, G2.mul(ni, Pi))
        if not G2.is_O(S):
            continue
        conf.append(n)
    exact_ok = None
    if conf:
        exact_ok = True
        for n in conf[:3]:
            S = GQ.O
            for ni, Pi in zip(n, Pq):
                if ni:
                    S = GQ.add(S, GQ.mul(ni, Pi))
            nexact += 1
            exact_ok = exact_ok and GQ.is_O(S)
    # reduced consistency: every Q relation must hold in E(F_p)
    modp_ok = True
    for n in conf:
        S = Ep.O
        for ni, Ri in zip(n, Rn):
            if ni:
                S = Ep.add(S, Ep.mul(ni, Ri))
        modp_ok = modp_ok and Ep.is_O(S)
    classes = {}
    for i, P in enumerate(Pq):
        classes.setdefault(CQ.norm(P), []).append(i)
    rec.update(classify(conf, list(classes.values()), s, t, q, xlog))
    rec.update({'status': 'ok', 'n_rel_l1': len(rels), 'n_rel_conf': len(conf), 'exact_checked': nexact,
                'exact_ok': exact_ok, 'modp_ok': modp_ok, 'n_classes': len(classes)})
    if arm == 'P':
        target = tuple([1, 1, -1] + [0] * (k - 3))
        rec['planted_found'] = target in [tuple(n) for n in conf]
    return rec


def classify(rels, classes, s, t, q, xlog):
    triv = tors = nontriv = useful = correct = 0
    examples = []
    for n in rels:
        csum = [sum(n[i] for i in cls) for cls in classes]
        nz = sum(1 for v in csum if v)
        if nz == 0:
            triv += 1; continue
        if nz == 1:
            tors += 1; continue
        nontriv += 1
        den = sum(ni * ti for ni, ti in zip(n, t)) % q
        if den:
            useful += 1
            xr = (-sum(ni * si for ni, si in zip(n, s)) * pow(den, -1, q)) % q
            if xr == xlog:
                correct += 1
        if len(examples) < 3:
            examples.append(list(n))
    return {'triv': triv, 'tors': tors, 'nontriv': nontriv, 'useful': useful, 'log_correct': correct,
            'examples': examples, 'any_nontriv': nontriv > 0}


def main():
    arm, bits, k, trials, seed, outp = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5]), sys.argv[6]
    rng = random.Random(seed * 1000003 + bits * 101 + k)
    fo = open(outp, 'a')
    ncurve = max(1, trials // 25)
    t0 = time.time()
    done = 0
    for ci in range(ncurve):
        p, A, B, q, P0 = make_curve(bits, rng)
        Ep = Group(Cubic(cE(A, B), p), (0, 1, 0))
        xlog = rng.randrange(1, q)
        Q0 = Ep.mul(xlog, P0)
        per = trials // ncurve + (1 if ci < trials % ncurve else 0)
        for _ in range(per):
            rec = run_trial(arm, p, A, B, q, P0, Q0, xlog, k, rng)
            rec.update({'arm': arm, 'bits': bits, 'p': p, 'q': q, 'seed': seed})
            fo.write(json.dumps(rec) + '\n')
            done += 1
    fo.flush()
    print(arm, bits, k, done, 'trials', round(time.time() - t0, 1), 's', flush=True)


if __name__ == '__main__':
    main()
