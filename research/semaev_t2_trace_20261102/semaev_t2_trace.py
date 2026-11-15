#!/usr/bin/env python3
"""Semaev 2015, t = 2 rows: first fall degree of the Weil descent of S_3(x1, x2, z).

research/semaev_t2_trace_20261102/PROTOCOL.md. Standard library only; exact arithmetic in
F_{2^n} = F_2[alpha]/(f) and in the Boolean ring. For each cell (n, m), k = ceil(n/m),
V = span{1, alpha, ..., alpha^(k-1)}, curve Y^2 + XY = X^3 + AX^2 + B so that
S_3(x1, x2, x3) = (x1 x2 + x1 x3 + x2 x3)^2 + x1 x2 x3 + B, and x3 = z a random constant.

Per instance:
  1. the n Boolean coordinate equations [S_3]_j in the 2k coordinates of x1, x2;
  2. the first fall degree d_ff (Semaev Sec. 4.4 definition, polynomial ring) for d = 1, 2;
  3. the trace certificate: sum_j Tr(z^-2 alpha^j) [S_3]_j == sum_i Tr(alpha^i)(a_i + b_i)
     + Tr(B z^-2);
  4. number of solutions (x1, x2) in V x V by direct evaluation;
  5. for systems without solutions, the least D in 2..5 with 1 in the Boolean Macaulay span
     {red(mu [S_3]_j) : mu multilinear, deg mu <= D - 2}.
"""
import random
import sys
import time

SEED = 20261102


# ---------------------------------------------------------------- F_2[x] and F_{2^n}

def pdeg(a):
    return a.bit_length() - 1


def pmod(a, f):
    df = pdeg(f)
    while a and pdeg(a) >= df:
        a ^= f << (pdeg(a) - df)
    return a


def pmul(a, b):
    r = 0
    while b:
        if b & 1:
            r ^= a
        a <<= 1
        b >>= 1
    return r


def pgcd(a, b):
    while b:
        a, b = b, pmod(a, b)
    return a


def ppowmod(a, e, f):
    r, a = 1, pmod(a, f)
    while e:
        if e & 1:
            r = pmod(pmul(r, a), f)
        a = pmod(pmul(a, a), f)
        e >>= 1
    return r


def prime_factors(n):
    out, p = set(), 2
    while p * p <= n:
        while n % p == 0:
            out.add(p)
            n //= p
        p += 1
    if n > 1:
        out.add(n)
    return out


def irreducible(f):
    """Rabin's test over F_2."""
    n = pdeg(f)
    x = 2
    if ppowmod(x, 1 << n, f) != pmod(x, f):
        return False
    for p in prime_factors(n):
        h = ppowmod(x, 1 << (n // p), f) ^ x
        if pgcd(f, pmod(h, f)) != 1:
            return False
    return True


def smallest_sparse_irreducible(n):
    for a in range(1, n):
        f = (1 << n) | (1 << a) | 1
        if irreducible(f):
            return f, f"trinomial x^{n}+x^{a}+1"
    best = None
    for a in range(3, n):
        for b in range(2, a):
            for c in range(1, b):
                f = (1 << n) | (1 << a) | (1 << b) | (1 << c) | 1
                if (best is None or f < best) and irreducible(f):
                    best = f
    return best, "pentanomial " + "+".join(f"x^{i}" for i in range(n, -1, -1) if best >> i & 1)


def random_irreducible(n, rng):
    while True:
        f = (1 << n) | rng.getrandbits(n) | 1
        if irreducible(f):
            return f, f"random 0x{f:x}"


class GF:
    def __init__(self, f):
        self.f, self.n = f, pdeg(f)

    def mul(self, a, b):
        return pmod(pmul(a, b), self.f)

    def pow(self, a, e):
        return ppowmod(a, e, self.f)

    def inv(self, a):
        return self.pow(a, (1 << self.n) - 2)

    def tr(self, a):
        t, s = 0, a
        for _ in range(self.n):
            t ^= s
            s = self.mul(s, s)
        assert t in (0, 1)
        return t


# ------------------------------------------------- Boolean polynomials over F_{2^n}

def bmul(F, p, q):
    out = {}
    for m1, c1 in p.items():
        for m2, c2 in q.items():
            m, c = m1 | m2, F.mul(c1, c2)
            out[m] = out.get(m, 0) ^ c
    return {m: c for m, c in out.items() if c}


def badd(*ps):
    out = {}
    for p in ps:
        for m, c in p.items():
            out[m] = out.get(m, 0) ^ c
    return {m: c for m, c in out.items() if c}


def bsquare(F, p):
    return {m: F.mul(c, c) for m, c in p.items()}  # Boolean ring: M^2 = M, char 2


def popcount(m):
    return bin(m).count("1")


def coordinate_equations(F, k, z, B):
    x1 = {1 << i: 1 << i for i in range(k)}
    x2 = {1 << (k + i): 1 << i for i in range(k)}
    zc = {0: z}
    x1x2 = bmul(F, x1, x2)
    P = badd(x1x2, bmul(F, zc, x1), bmul(F, zc, x2))
    S3 = badd(bsquare(F, P), bmul(F, zc, x1x2), {0: B} if B else {})
    coords = []
    for j in range(F.n):
        coords.append(frozenset(m for m, c in S3.items() if c >> j & 1))
    return coords, S3


# ------------------------------------------------- first fall degree (polynomial ring)

def fall_at(gens_exp, nv, d):
    """gens_exp: list of polynomials as frozensets of exponent tuples (polynomial ring over F_2).
    Fall at d: tuples (g_j) with deg(g_j f_j) <= d, the sum of degree < d, nonzero, and some
    product of degree d. K = tuples whose degree-d part of the sum vanishes."""
    def deg(p):
        return max((sum(e) for e in p), default=-1)
    def monos(k):
        out = [()]
        for _ in range(nv):
            out = [t + (e,) for t in out for e in range(k + 1)]
        return [t for t in out if sum(t) <= k]
    entries = []
    for f in gens_exp:
        df = deg(f)
        if df < 0 or df > d:
            continue
        for mu in monos(d - df):
            img = set()
            for t in f:
                img ^= {tuple(a + b for a, b in zip(t, mu))}
            entries.append((frozenset(img), sum(mu) + df))
    if not entries:
        return None
    topd = sorted({t for img, _ in entries for t in img if sum(t) == d})
    ri = {t: i for i, t in enumerate(topd)}
    pivots, basis = {}, []
    for c, (img, _) in enumerate(entries):
        v, comb = 0, 1 << c
        for t in img:
            if t in ri:
                v ^= 1 << ri[t]
        while v:
            low = v & -v
            if low in pivots:
                pv, pc = pivots[low]
                v ^= pv
                comb ^= pc
            else:
                pivots[low] = (v, comb)
                break
        if not v:
            basis.append(comb)
    not_z = not_w = False
    witness = None
    for comb in basis:
        s, top = set(), False
        idx = comb
        while idx:
            low = idx & -idx
            i = low.bit_length() - 1
            s ^= entries[i][0]
            top |= entries[i][1] == d
            idx ^= low
        not_z |= bool(s)
        not_w |= top
        if s and top and witness is None:
            witness = frozenset(s)
    return witness if (not_z and not_w) else None


def to_exp(poly_masks, nv):
    return frozenset(tuple((m >> i) & 1 for i in range(nv)) for m in poly_masks)


# ------------------------------------------------- Boolean Macaulay degree of 1

def macaulay_degree_of_one(coords, nv, dmax=5):
    """Returns (D, certificate_ok): the least D with 1 in the span, and whether the tracked
    combination of rows, recomputed from scratch, sums to exactly 1."""
    for D in range(2, dmax + 1):
        rows = []
        mus = [m for m in range(1 << nv) if popcount(m) <= D - 2]
        cols = {}
        pivots = {}
        def col(m):
            if m not in cols:
                cols[m] = len(cols)
            return cols[m]
        col(0)  # the constant monomial is column 0
        for f in coords:
            if not f or max(popcount(m) for m in f) > 2:
                pass
            for mu in mus:
                row_set = set()
                for t in f:
                    row_set ^= {t | mu}
                if not row_set or max(popcount(m) for m in row_set) > D:
                    continue
                v = 0
                for t in row_set:
                    v ^= 1 << col(t)
                rows.append(frozenset(row_set))
                comb = 1 << (len(rows) - 1)
                while v:
                    hi = v.bit_length() - 1
                    if hi in pivots:
                        pv, pc = pivots[hi]
                        v ^= pv
                        comb ^= pc
                    else:
                        pivots[hi] = (v, comb)
                        break
        # is the constant 1 (bit 0) in the span?
        v, comb = 1, 0
        while v:
            hi = v.bit_length() - 1
            if hi in pivots:
                pv, pc = pivots[hi]
                v ^= pv
                comb ^= pc
            else:
                break
        if v == 0:
            total = set()
            i = 0
            while comb:
                if comb & 1:
                    total ^= set(rows[i])
                comb >>= 1
                i += 1
            return D, total == {0}
    return None, False


# ------------------------------------------------- driver

CELLS = [(15, 4), (15, 5), (16, 4), (19, 3), (21, 3)]


def run(instances=20):
    rng = random.Random(SEED)
    all_ok = True
    print(f"# semaev_t2_trace.py seed {SEED}; cells (n, m) = {CELLS}; {instances} (z, B) per cell and field polynomial")
    for n, m in CELLS:
        k = -(-n // m)
        polys = [smallest_sparse_irreducible(n), random_irreducible(n, random.Random(SEED + n))]
        for f, fdesc in polys:
            F = GF(f)
            tr_V = [F.tr(1 << i) for i in range(k)]
            stats = {"ff2": 0, "ff1": 0, "nofall": 0, "cert": 0, "deg2all": 0, "solvable": 0, "D": {}}
            t0 = time.time()
            for _ in range(instances):
                z = 0
                while z == 0:
                    z = rng.getrandbits(n)
                B = rng.getrandbits(n)
                coords, S3 = coordinate_equations(F, k, z, B)
                degs = [max((popcount(t) for t in c), default=-1) for c in coords]
                stats["deg2all"] += all(d == 2 for d in degs)
                nv = 2 * k
                gens = [to_exp(c, nv) for c in coords if c]
                w1 = fall_at(gens, nv, 1)
                w2 = None if w1 else fall_at(gens, nv, 2)
                if w1:
                    stats["ff1"] += 1
                elif w2:
                    stats["ff2"] += 1
                else:
                    stats["nofall"] += 1
                lam = F.mul(F.inv(z), F.inv(z))
                combo = set()
                for j in range(n):
                    if F.tr(F.mul(lam, 1 << j)):
                        combo ^= set(coords[j])
                expect = set()
                for i in range(k):
                    if tr_V[i]:
                        expect ^= {1 << i, 1 << (k + i)}
                if F.tr(F.mul(B, lam)):
                    expect ^= {0}
                stats["cert"] += combo == expect
                # direct solution count over V x V
                sols = 0
                for a in range(1 << k):
                    for b in range(1 << k):
                        s = F.mul(a, b) ^ F.mul(z, a ^ b)
                        val = F.mul(s, s) ^ F.mul(z, F.mul(a, b)) ^ B
                        sols += val == 0
                if sols:
                    stats["solvable"] += 1
                else:
                    D, cert_ok = macaulay_degree_of_one(coords, nv)
                    stats["D"][D] = stats["D"].get(D, 0) + 1
                    stats["one_cert"] = stats.get("one_cert", 0) + int(cert_ok)
            ok = stats["ff2"] + stats["ff1"] == instances and stats["cert"] == instances
            all_ok &= ok
            print(f"CELL n={n} m={m} t=2 k={k} field {fdesc}: Tr on V basis {tr_V}; "
                  f"all coordinate equations of degree 2 in {stats['deg2all']}/{instances}; "
                  f"d_ff=1 in {stats['ff1']}, d_ff=2 in {stats['ff2']}, no fall <= 2 in {stats['nofall']}; "
                  f"trace certificate exact in {stats['cert']}/{instances}; "
                  f"solvable {stats['solvable']}; Macaulay degree of 1 (unsolvable) {dict(sorted(stats['D'].items(), key=lambda x: (x[0] is None, x[0] or 0)))} "
                  f"with a recomputed combination equal to 1 in {stats.get('one_cert', 0)}; "
                  f"{time.time()-t0:.1f}s -> {'PASS' if ok else 'CHECK'}")
            sys.stdout.flush()
    print(f"RESULT: first fall degree <= 2 with exact trace certificate on every instance -> {'PASS' if all_ok else 'NOT ALL'}")
    return all_ok


def selftest():
    # Field arithmetic sanity and the identity Tr(lambda u^2) = Tr(sqrt(lambda) u).
    rng = random.Random(1)
    f, _ = smallest_sparse_irreducible(15)
    F = GF(f)
    for _ in range(200):
        lam, u = rng.getrandbits(15), rng.getrandbits(15)
        sq = F.pow(lam, 1 << 14)  # sqrt in F_{2^15}
        assert F.mul(sq, sq) == lam
        assert F.tr(F.mul(lam, F.mul(u, u))) == F.tr(F.mul(sq, u))
    # Negative control: a combination that is NOT lambda = z^-2 must keep a quadratic part.
    k = 4
    z, B = rng.getrandbits(15) | 1, rng.getrandbits(15)
    coords, _ = coordinate_equations(F, k, z, B)
    lam = F.mul(F.inv(z), F.inv(z)) ^ 1
    combo = set()
    for j in range(15):
        if F.tr(F.mul(lam, 1 << j)):
            combo ^= set(coords[j])
    assert any(popcount(t) == 2 for t in combo), "negative control: quadratic part should survive"
    print("SELFTEST field arithmetic, trace/sqrt identity, negative control (lambda != z^-2 keeps degree 2) -> PASS")


if __name__ == "__main__":
    selftest()
    sys.exit(0 if run(int(sys.argv[1]) if len(sys.argv) > 1 else 20) else 1)
