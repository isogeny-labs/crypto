#!/usr/bin/env python3
"""Validator recomputation for g3_affine_twist_20261121 (independent of fpeval.py,
smith.py, run.py, census.py; reads only the stored formulas/*.json and the data rows).

Own pieces:
  * F_{p^d} = F_p[x]/(g), g a *validator-chosen* random irreducible (Rabin test),
    multiplication by numpy convolution + reduction matrix (producer: schoolbook in a
    quadratic tower over F_p[z]/(f)).
  * roots of f in F_{p^28} by equal-degree splitting over the F_{p^7} subfield
    (producer: z, z^p, ... in F_p[z]/(f)); orbit length 7 checked (=> f irreducible).
  * fourth roots by Tonelli-Shanks twice, each multiplied by a random 4th root of unity;
    zeta8 = a random primitive 8th root (any of the 4 Galois conjugates).
  * relabelling sigma: own brute force over S_8, a random member of the coset.
  * S_L evaluated from the stored formula terms; must be a constant (in F_p).
  * main arm: #J(F_p) from own point counts over F_p, F_p^2, F_p^3 (vectorised), Newton.
  * control: Smith maps by exhaustive search over P^1 x P^1 (normalisation g(a1)=g(b1)=inf,
    g(a2)=g(b2)=0 with a random order of the four pairs), fibre norm by the Sylvester
    resultant Res(N - t0 D, F) (producer: linear solve + quadratic, det of mult. map).
Decision recomputed as chi_p(-1) * chi(c) * chi(S_L) and compared with the stored rows.

Usage: python3 recompute_fp.py STUDY_DIR OUT.jsonl [SEED]
"""
import itertools
import json
import os
import random
import sys
import time

import numpy as np


# ======================================================================= F_p polys
def ptrim(a):
    a = list(a)
    while a and a[-1] == 0:
        a.pop()
    return a


def pdivmod_p(a, b, p):
    a = ptrim(a)
    b = ptrim(b)
    inv = pow(b[-1], p - 2, p)
    q = [0] * max(len(a) - len(b) + 1, 1)
    while len(a) >= len(b):
        c = a[-1] * inv % p
        sh = len(a) - len(b)
        q[sh] = c
        for i, y in enumerate(b):
            a[sh + i] = (a[sh + i] - c * y) % p
        a = ptrim(a)
    return q, a


def pgcd_p(a, b, p):
    a, b = ptrim(a), ptrim(b)
    while b:
        _, r = pdivmod_p(a, b, p)
        a, b = b, r
    return a


# ======================================================================= F_{p^d}
class GF:
    def __init__(self, p, g):
        self.p = p
        self.g = [x % p for x in g]
        self.d = d = len(g) - 1
        assert self.g[-1] == 1
        self.q = p ** d
        R = np.zeros((max(d - 1, 1), d), dtype=np.int64)
        cur = [(-c) % p for c in self.g[:d]]
        for k in range(d - 1):
            R[k] = cur
            top = cur[-1]
            sh = [0] + cur[:-1]
            cur = [(sh[i] - top * self.g[i]) % p for i in range(d)]
        self.R = R
        self.one = self.c(1)
        self.zero = self.c(0)

    def c(self, x):
        v = np.zeros(self.d, dtype=np.int64)
        v[0] = x % self.p
        return v

    def mul(self, a, b):
        if self.d == 1:
            return np.array([a[0] * b[0] % self.p], dtype=np.int64)
        cc = np.convolve(a, b)
        return (cc[:self.d] + cc[self.d:] @ self.R) % self.p

    def add(self, a, b):
        return (a + b) % self.p

    def sub(self, a, b):
        return (a - b) % self.p

    def neg(self, a):
        return (-a) % self.p

    def pow(self, a, e):
        r = self.one
        base = a
        while e:
            if e & 1:
                r = self.mul(r, base)
            base = self.mul(base, base)
            e >>= 1
        return r

    def inv(self, a):
        assert not self.is_zero(a)
        return self.pow(a, self.q - 2)

    def is_zero(self, a):
        return not a.any()

    def eq(self, a, b):
        return bool((a == b).all())

    def is_const(self, a):
        return not a[1:].any()

    def rand(self, rng):
        return np.array([rng.randrange(self.p) for _ in range(self.d)], dtype=np.int64)


def rabin_irreducible(p, g):
    d = len(g) - 1
    F = GF(p, g)
    x = np.zeros(d, dtype=np.int64)
    x[1 % d] = 1 if d > 1 else 0
    if d == 1:
        return True
    xp = [x]
    for _ in range(d):
        xp.append(F.pow(xp[-1], p))
    if not F.eq(xp[d], x):
        return False
    primes = [r for r in range(2, d + 1) if d % r == 0 and all(r % s for s in range(2, r))]
    for r in primes:
        h = [int(v) for v in F.sub(xp[d // r], x)]
        if len(ptrim(pgcd_p(h, g, p))) != 1:
            return False
    return True


def random_irreducible(p, d, rng):
    while True:
        g = [rng.randrange(p) for _ in range(d)] + [1]
        if g[0] and rabin_irreducible(p, g):
            return g


def v2(n):
    s = 0
    while n % 2 == 0:
        n //= 2
        s += 1
    return s


class Field:
    """F_q with sqrt, fourth roots, zeta8."""

    def __init__(self, p, d, rng):
        self.F = F = GF(p, random_irreducible(p, d, rng))
        self.p, self.q = p, F.q
        assert (F.q - 1) % 8 == 0
        mone = F.c(-1)
        while True:
            a = F.rand(rng)
            if not F.is_zero(a) and F.eq(F.pow(a, (F.q - 1) // 2), mone):
                self.nonres = a
                break
        while True:
            a = F.rand(rng)
            if F.is_zero(a):
                continue
            z = F.pow(a, (F.q - 1) // 8)
            if F.eq(F.pow(z, 4), mone):
                self.z8 = z
                break
        i4 = F.mul(self.z8, self.z8)
        self.units4 = [F.one, i4, F.neg(F.one), F.neg(i4)]
        self.s = v2(F.q - 1)
        self.m = (F.q - 1) >> self.s
        self.zm = F.pow(self.nonres, self.m)

    def sqrt(self, a):
        F = self.F
        if F.is_zero(a):
            return a
        x = F.pow(a, (self.m + 1) // 2)
        t = F.pow(a, self.m)
        z = self.zm
        M = self.s
        while not F.eq(t, F.one):
            i, tt = 0, t
            while not F.eq(tt, F.one):
                tt = F.mul(tt, tt)
                i += 1
                if i >= M:
                    return None
            b = z
            for _ in range(M - i - 1):
                b = F.mul(b, b)
            x = F.mul(x, b)
            z = F.mul(b, b)
            t = F.mul(t, z)
            M = i
        assert F.eq(F.mul(x, x), a)
        return x

    def fourth_root(self, a, rng):
        F = self.F
        s = self.sqrt(a)
        r = self.sqrt(s)
        if r is None:
            r = self.sqrt(F.neg(s))
        assert r is not None
        r2 = F.mul(r, r)
        assert F.eq(F.mul(r2, r2), a)
        return F.mul(r, self.units4[rng.randrange(4)])


# ======================================================================= polys over F_q
def qp_trim(F, a):
    a = list(a)
    while a and F.is_zero(a[-1]):
        a.pop()
    return a


def qp_mul(F, a, b):
    if not a or not b:
        return []
    r = [F.zero] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if F.is_zero(x):
            continue
        for j, y in enumerate(b):
            r[i + j] = F.add(r[i + j], F.mul(x, y))
    return qp_trim(F, r)


def qp_mod(F, a, m):
    """m monic."""
    a = qp_trim(F, a)
    dm = len(m) - 1
    while len(a) - 1 >= dm:
        c = a[-1]
        sh = len(a) - 1 - dm
        for j in range(dm + 1):
            a[sh + j] = F.sub(a[sh + j], F.mul(c, m[j]))
        a = qp_trim(F, a)
    return a


def qp_monic(F, a):
    a = qp_trim(F, a)
    inv = F.inv(a[-1])
    return [F.mul(x, inv) for x in a]


def qp_gcd(F, a, b):
    a, b = qp_trim(F, a), qp_trim(F, b)
    while b:
        b = qp_monic(F, b)
        a, b = b, qp_mod(F, a, b)
    return qp_monic(F, a) if a else a


def qp_powmod(F, a, e, m):
    r = [F.one]
    base = qp_mod(F, a, m)
    while e:
        if e & 1:
            r = qp_mod(F, qp_mul(F, r, base), m)
        base = qp_mod(F, qp_mul(F, base, base), m)
        e >>= 1
    return r


def frob(F, a, k):
    return F.pow(a, F.p ** k)


def one_root_in_subfield(Fd, poly, sub_deg, rng):
    """A root of poly (monic, F_q coefficients, splitting over F_{p^sub_deg} inside F_q)."""
    F = Fd.F
    d = F.d
    m = poly
    while len(m) > 2:
        r = F.rand(rng)
        delta = r
        for k in range(1, d // sub_deg):
            delta = F.add(delta, frob(F, r, k * sub_deg))
        h = qp_powmod(F, [delta, F.one], (F.p ** sub_deg - 1) // 2, m)
        h = qp_trim(F, h + [F.zero])
        if not h:
            continue
        h[0] = F.sub(h[0], F.one)
        g = qp_gcd(F, m, h)
        if 1 < len(g) < len(m):
            m = g if len(g) <= len(m) - len(g) + 1 else qp_monic(F, qp_div_exact(F, m, g))
    return F.neg(m[0])


def qp_div_exact(F, a, b):
    """a / b for monic b dividing a."""
    a = qp_trim(F, a)
    db = len(b) - 1
    q = [F.zero] * (len(a) - db)
    while len(a) - 1 >= db:
        c = a[-1]
        sh = len(a) - 1 - db
        q[sh] = c
        for j in range(db + 1):
            a[sh + j] = F.sub(a[sh + j], F.mul(c, b[j]))
        a = qp_trim(F, a)
    assert not a
    return q


# ======================================================================= Lagrangians
def norm(s):
    s = frozenset(s)
    c = frozenset(range(8)) - s
    return min(s, c, key=lambda t: (len(t), sorted(t)))


def lkey(subsets):
    return frozenset(norm(s) for s in subsets)


_SIGMA_CACHE = {}


def sigmas_for(formula_L, target_key):
    ck = (lkey(formula_L), target_key)
    if ck not in _SIGMA_CACHE:
        out = []
        for sig in itertools.permutations(range(8)):
            if lkey([[sig[i] for i in s] for s in formula_L]) == target_key:
                out.append(sig)
        _SIGMA_CACHE[ck] = out
    return _SIGMA_CACHE[ck]


# ======================================================================= S_L
def eval_SL(Fd, formula, PAIRS, es, rng):
    F = Fd.F
    rho = []
    for (i, j) in PAIRS:
        rho.append(Fd.fourth_root(F.sub(es[j], es[i]), rng))
    sq = [F.mul(r, r) for r in rho]
    zp = [F.one]
    for _ in range(7):
        zp.append(F.mul(zp[-1], Fd.z8))
    tot = F.one
    for fac in formula:
        acc = F.zero
        for k, v in fac["terms"]:
            t = zp[k % 8]
            for idx, e in enumerate(v):
                if e == 0:
                    continue
                if e == 1:
                    t = F.mul(t, rho[idx])
                elif e == 2:
                    t = F.mul(t, sq[idx])
                else:
                    t = F.mul(t, F.pow(rho[idx], e))
            acc = F.add(acc, t)
        tot = F.mul(tot, acc)
    return tot


def leg(x, p):
    x %= p
    if x == 0:
        return 0
    return 1 if pow(x, (p - 1) // 2, p) == 1 else -1


# ======================================================================= point counts
def vec_field_elems(p, k, rng):
    if k == 1:
        return None, np.arange(p, dtype=np.int64).reshape(p, 1)
    g = random_irreducible(p, k, rng)
    E = np.array(list(itertools.product(range(p), repeat=k)), dtype=np.int64)
    return GF(p, g), E


def vmul(F, A, B):
    p = F.p
    if F is None:
        return A * B % p
    d = F.d
    C = np.zeros((A.shape[0], 2 * d - 1), dtype=np.int64)
    for i in range(d):
        for j in range(d):
            C[:, i + j] += A[:, i] * B[:, j]
    C %= p
    return (C[:, :d] + C[:, d:] @ F.R) % p


def vpow(F, A, e, p):
    d = A.shape[1]
    R = np.zeros_like(A)
    R[:, 0] = 1
    while e:
        if e & 1:
            R = vmul(F, R, A) if F is not None else R * A % p
        A = vmul(F, A, A) if F is not None else A * A % p
        e >>= 1
    return R


def count_points(f, p, k, rng):
    F, E = vec_field_elems(p, k, rng)
    q = p ** k
    acc = np.zeros_like(E)
    acc[:, 0] = f[-1]
    for cf in reversed(f[:-1]):
        acc = vmul(F, acc, E) if F is not None else acc * E % p
        acc[:, 0] = (acc[:, 0] + cf) % p
    ch = vpow(F, acc, (q - 1) // 2, p)
    zero = ~acc.any(axis=1)
    one = (ch[:, 0] == 1) & ~ch[:, 1:].any(axis=1) if k > 1 else (ch[:, 0] == 1)
    chi = np.where(zero, 0, np.where(one, 1, -1))
    return q + 1 + int(chi.sum())


def jac_order(f, p, rng):
    N = [count_points(f, p, k, rng) for k in (1, 2, 3)]
    S = [p ** k + 1 - N[k - 1] for k in (1, 2, 3)]
    e1 = S[0]
    e2 = (e1 * S[0] - S[1]) // 2
    e3 = (e2 * S[0] - e1 * S[1] + S[2]) // 3
    P = [p ** 3, -p * p * e1, p * e2, -e3, e2, -e1, 1]
    return N, P, sum(P)


def is_prime(n):
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True


# ======================================================================= Smith (control)
def pm(a, b, p):
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            r[i + j] = (r[i + j] + x * y) % p
    return r


def pev(a, x, p):
    v = 0
    for c in reversed(a):
        v = (v * x + c) % p
    return v


def det_mod(M, p):
    M = [r[:] for r in M]
    n = len(M)
    det = 1
    for c in range(n):
        piv = next((i for i in range(c, n) if M[i][c] % p), None)
        if piv is None:
            return 0
        if piv != c:
            M[c], M[piv] = M[piv], M[c]
            det = -det
        det = det * M[c][c] % p
        inv = pow(M[c][c], p - 2, p)
        for i in range(c + 1, n):
            f = M[i][c] * inv % p
            if f:
                M[i] = [(x - f * y) % p for x, y in zip(M[i], M[c])]
    return det % p


def resultant(A, B, p):
    """Sylvester resultant of A, B (low->high coefficient lists, exact degrees)."""
    A, B = ptrim(A), ptrim(B)
    m, n = len(A) - 1, len(B) - 1
    N = m + n
    rows = []
    for i in range(n):
        row = [0] * N
        for k, c in enumerate(reversed(A)):
            row[i + k] = c
        rows.append(row)
    for i in range(m):
        row = [0] * N
        for k, c in enumerate(reversed(B)):
            row[i + k] = c
        rows.append(row)
    return det_mod(rows, p)


def smith_independent(u, c, pairs, p, rng):
    """All degree-3 maps g with g(a_k)=g(b_k) (one per fibration) and fibre-norm symbols."""
    pv = [(u[a], u[b]) for a, b in pairs]
    order = list(range(4))
    rng.shuffle(order)
    (a1, b1), (a2, b2), (a3, b3), (a4, b4) = [pv[k] for k in order]
    F = [c % p]
    for x in u:
        F = pm(F, [(-x) % p, 1], p)
    q2 = pm([(-a2) % p, 1], [(-b2) % p, 1], p)
    q1 = pm([(-a1) % p, 1], [(-b1) % p, 1], p)
    lin = [(1, b) for b in range(p)] + [(0, 1)]  # alpha x + beta  as (alpha, beta)
    maps = []
    for (al, be) in lin:
        N = pm(q2, [be, al], p)
        for (ga, de) in lin:
            D = pm(q1, [de, ga], p)
            if max(len(ptrim(N)), len(ptrim(D))) != 4:
                continue
            ok = all((pev(N, a, p) * pev(D, b, p) - pev(N, b, p) * pev(D, a, p)) % p == 0
                     for a, b in ((a3, b3), (a4, b4)))
            if not ok:
                continue
            if len(ptrim(pgcd_p(N, D, p))) != 1:
                continue
            maps.append((N, D))
    syms = []
    for N, D in maps:
        s = set()
        for t0 in range(p):
            G = ptrim([(x - t0 * y) % p for x, y in zip(N + [0] * 4, D + [0] * 4)])
            if len(G) != 4:
                continue
            dG = ptrim([(i * G[i]) % p for i in range(1, 4)])
            if len(ptrim(pgcd_p(G, dG, p))) != 1:
                continue
            r = resultant(G, F, p)
            if r == 0:
                continue
            s.add(leg(r, p))
        syms.append(sorted(s))
    return maps, syms


# ======================================================================= driver
def load_formula(study, name):
    j = json.load(open(os.path.join(study, "formulas", name)))
    return j["formula"], [tuple(x) for x in j["pairs"]], j["report"]


AFFINE_S = None


def main(study, outpath, seed=20261121):
    rng = random.Random(seed)
    t_start = time.time()
    fA, PAIRS, rA = load_formula(study, "f_001000.json")
    fP, PAIRS0, rP = load_formula(study, "f_000000.json")
    assert PAIRS == PAIRS0 == [(i, j) for i in range(8) for j in range(i + 1, 8)]
    reports = {}
    for fn in sorted(os.listdir(os.path.join(study, "formulas"))):
        reports[fn] = load_formula(study, fn)
    affine_names = sorted(n for n, v in reports.items() if v[2]["kind"] == "affine")
    pair_names = sorted(n for n, v in reports.items() if v[2]["kind"] == "pair")
    unused_aff = [n for n in affine_names if n not in ("f_001000.json", "f_110111.json")]
    unused_pair = [n for n in pair_names if n != "f_000000.json"]

    main_rows = [json.loads(l) for l in open(os.path.join(study, "data", "main.jsonl"))]
    ctl_rows = [json.loads(l) for l in open(os.path.join(study, "data", "control.jsonl"))]
    primes = sorted({r["p"] for r in main_rows})
    out = open(outpath, "w")
    summary = {"main": [], "control": []}

    # ---------------- main-arm sample: 6 random codomains per prime + 1 degenerate per prime
    for p in primes:
        rows = [r for r in main_rows if r["p"] == p]
        nondeg = [r for r in rows if r["evals"][0]["val"] not in (0, None)]
        deg = [r for r in rows if r["evals"][0]["val"] == 0]
        sample = rng.sample(nondeg, 6) + (rng.sample(deg, 1) if deg else [])
        Fd = Field(p, 28, random.Random(rng.random()))
        F = Fd.F
        for r in sample:
            t0 = time.time()
            f = r["f"]
            fpoly = [F.c(x) for x in f]
            alpha = one_root_in_subfield(Fd, fpoly, 7, rng)
            roots = [alpha]
            for _ in range(6):
                roots.append(F.pow(roots[-1], p))
            assert F.eq(F.pow(roots[-1], p), alpha), "orbit not 7"
            assert len({tuple(x) for x in roots}) == 7
            for x in roots:
                acc = F.c(f[-1])
                for cf in reversed(f[:-1]):
                    acc = F.add(F.mul(acc, x), F.c(cf))
                assert F.is_zero(acc)
            N, P, nJ = jac_order(f, p, rng)
            target = lkey(r["L_subsets"])
            res = {"p": p, "f": f, "L": r["L"], "nJ_data": r["nJ"], "nJ_mine": nJ, "nJ_prime": is_prime(nJ),
                   "P_match": P == r["P"], "N1_match": N[0] == r["N1"], "evals": []}
            for e in r["evals"]:
                t = e["t"]
                us = [F.inv(F.sub(x, F.c(t))) for x in roots] + [F.zero]
                ft = sum(cf * pow(t, i, p) for i, cf in enumerate(f)) % p
                vals = []
                for fname in ("f_001000.json", rng.choice(unused_aff)):
                    fm, _, rp = reports[fname]
                    sigs = sigmas_for(rp["L_subsets"], target)
                    sig = rng.choice(sigs)
                    v = eval_SL(Fd, fm, PAIRS, [us[sig[i]] for i in range(8)], rng)
                    vals.append((fname, int(v[0]) if F.is_const(v) else None, len(sigs)))
                mine = vals[0][1]
                sign_mine = None if mine is None else leg(-1, p) * leg(ft, p) * leg(mine, p)
                sign_data = None if e["val"] is None else leg(-1, p) * leg(e["c"], p) * leg(e["val"], p)
                res["evals"].append({"t": t, "c_data": e["c"], "c_mine": ft, "val_data": e["val"],
                                     "val_mine": mine, "val_other_affine": vals[1], "coset_size": vals[0][2],
                                     "sign_data": sign_data, "sign_mine": sign_mine,
                                     "val_equal": mine == e["val"],
                                     "other_formula_equal": vals[1][1] == mine})
            res["sec"] = round(time.time() - t0, 1)
            out.write(json.dumps(res) + "\n")
            out.flush()
            summary["main"].append(res)
            print("main", p, r["L"], [(x["val_data"], x["val_mine"], x["val_other_affine"][1]) for x in res["evals"]],
                  "nJ", nJ, res["nJ_prime"], res["P_match"], res["sec"], "s", flush=True)

    # ---------------- control sample: 5 compared + 1 S_L=0 + 1 no-map kernel per prime
    for p in primes:
        rows = [r for r in ctl_rows if r["p"] == p and "smith" not in r]
        comp = [r for r in rows if r["n_maps"] > 0 and r["vals"][0] not in (0, None)
                and len({s for ss in r["smith_symbols"] for s in ss}) == 1]
        zer = [r for r in rows if r["n_maps"] > 0 and r["vals"][0] == 0]
        nom = [r for r in rows if r["n_maps"] == 0]
        sample = rng.sample(comp, 5) + rng.sample(zer, 1) + rng.sample(nom, 1)
        Fd = Field(p, 4, random.Random(rng.random()))
        F = Fd.F
        for r in sample:
            t0 = time.time()
            u, c, pairs = r["u"], r["c"], [tuple(x) for x in r["pairs"]]
            target = frozenset(norm(s) for s in
                               [list(a) for a in pairs] +
                               [list(pairs[0]) + list(pairs[1]), list(pairs[0]) + list(pairs[2]),
                                list(pairs[0]) + list(pairs[3])])
            assert len(target) == 7
            es_all = [F.c(x) for x in u]
            vals = []
            for fname in ("f_000000.json", rng.choice(unused_pair)):
                fm, _, rp = reports[fname]
                sigs = sigmas_for(rp["L_subsets"], target)
                sig = rng.choice(sigs)
                v = eval_SL(Fd, fm, PAIRS, [es_all[sig[i]] for i in range(8)], rng)
                vals.append((fname, int(v[0]) if F.is_const(v) else None))
            maps, syms = smith_independent(u, c, pairs, p, rng)
            flat = {s for ss in syms for s in ss}
            mine = vals[0][1]
            pred = None if not mine else leg(-1, p) * leg(c, p) * leg(mine, p)
            dsyms = {s for ss in r["smith_symbols"] for s in ss}
            res = {"p": p, "curve": r["curve"], "pairs": pairs, "val_data": r["vals"][0], "val_mine": mine,
                   "val_other_pair_formula": vals[1], "n_maps_data": r["n_maps"], "n_maps_mine": len(maps),
                   "smith_data": sorted(dsyms), "smith_mine": sorted(flat),
                   "pred_mine": pred,
                   "val_equal": mine == r["vals"][0],
                   "other_formula_equal": vals[1][1] == mine,
                   "smith_equal": sorted(dsyms) == sorted(flat),
                   "agree_mine": (pred == next(iter(flat))) if (pred and len(flat) == 1) else None,
                   "sec": round(time.time() - t0, 1)}
            out.write(json.dumps(res) + "\n")
            out.flush()
            summary["control"].append(res)
            print("ctl", p, pairs, r["vals"][0], mine, vals[1][1], "maps", r["n_maps"], len(maps),
                  "smith", sorted(dsyms), sorted(flat), "agree", res["agree_mine"], flush=True)

    # ---------------- tallies
    ev = [e for r in summary["main"] for e in r["evals"]]
    tal = {
        "main_codomains": len(summary["main"]),
        "main_evals": len(ev),
        "main_val_equal": sum(e["val_equal"] for e in ev),
        "main_sign_equal": sum(e["sign_mine"] == e["sign_data"] for e in ev),
        "main_c_equal": sum(e["c_mine"] == e["c_data"] for e in ev),
        "main_other_affine_formula_equal": sum(e["other_formula_equal"] for e in ev),
        "main_nJ_equal": sum(r["nJ_mine"] == r["nJ_data"] for r in summary["main"]),
        "main_nJ_prime": sum(r["nJ_prime"] for r in summary["main"]),
        "main_P_equal": sum(r["P_match"] for r in summary["main"]),
        "control_kernels": len(summary["control"]),
        "control_val_equal": sum(r["val_equal"] for r in summary["control"]),
        "control_other_pair_formula_equal": sum(r["other_formula_equal"] for r in summary["control"]),
        "control_nmaps_equal": sum(r["n_maps_data"] == r["n_maps_mine"] for r in summary["control"]),
        "control_smith_equal": sum(r["smith_equal"] for r in summary["control"]),
        "control_agree_mine_true": sum(r["agree_mine"] is True for r in summary["control"]),
        "control_agree_mine_false": sum(r["agree_mine"] is False for r in summary["control"]),
        "seconds": round(time.time() - t_start, 1),
        "seed": seed,
    }
    out.write(json.dumps({"tally": tal}) + "\n")
    out.close()
    print(json.dumps(tal, indent=1))


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2], int(sys.argv[3]) if len(sys.argv) > 3 else 20261121)
