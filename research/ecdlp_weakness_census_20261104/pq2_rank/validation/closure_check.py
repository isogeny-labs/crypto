#!/usr/bin/env python3
"""Validator's independent re-derivation of the pq2_rank quantities.

Written from scratch by the validator; it imports nothing from the producer
(src/main.rs, log_to_json.py, crosscheck/). Inputs are a pq2_rank run JSON
(instance parameters a, b, modulus, basis, x_R and the producer's numbers) and
the PQ2_DUMP files of the same run (descended equations and Z).

Per instance it computes, all over F_2:

1. Field and curve sanity: the modulus is irreducible of degree n (Rabin test),
   b != 0, the basis of V has rank n', x_R is the abscissa of a curve point, and
   S3(x(P), x(Q), x(P+Q)) = 0 on random point pairs, with its own group law.
2. The Weil descent, without the producer's coefficient formulas: S3(x1, x2, x_R)
   is evaluated at all 2^N assignments (x1 = sum a_i v_i, x2 = sum b_j v_j) and
   each of the n output bits is turned into its algebraic normal form by the
   Moebius transform. That ANF must equal the dumped equation, monomial for
   monomial. Z is read off the same evaluation (all n bits zero) and must equal
   the dumped Z.
3. k = N + 1 - rank of the evaluation matrix [1 | z] over z in Z (the dimension
   of the affine functions vanishing on Z), and affdim = rank - 1.
4. rho = rank of the bilinear (a_i b_j) parts of the n equations.
5. One-shot Macaulay low_d = dim(span{m * g : deg(m g) <= d} cap P_{<=1}) for
   d = 2, 3 (and 4 where requested), g over the equations and x_v^2 + x_v.
6. The degree-d closure: the smallest F_2-subspace of polynomials of degree <= d
   (in F_2[x_1..x_N], monomials not reduced by x^2 = x) that contains the
   equations and x_v^2 + x_v and is closed under f -> x_v f whenever
   deg(x_v f) <= d. Computed by worklist saturation: every vector inserted into
   an echelon basis (distinct leading monomials, monomials indexed by degree so
   that the leading monomial carries the degree) is multiplied by every variable
   if its degree is <= d - 1, and each product is reduced and inserted if new.
   Reported: dim of its degree-<=1 part, its total dimension, whether 1 is in it.
   Invariants checked: every basis vector vanishes on every z in Z (members of
   the ideal must), the degree-<=1 part does not exceed k, and a closedness pass
   re-multiplies basis vectors (all of them at d = 3, a seeded sample at d = 4)
   and checks each product reduces to zero.

Usage:
  closure_check.py --run-json RUN.json --dump-dir DIR --out OUT.json
                   [--iter4-max-n M] [--low4-max-n M] [--only n:i,...]
"""
from __future__ import annotations

import argparse
import itertools
import json
import random
import sys
import time
from collections import deque
from pathlib import Path

import numpy as np

# ---------------------------------------------------------------------------
# F_2[x] arithmetic on Python ints (for the modulus test)


def p_deg(a: int) -> int:
    return a.bit_length() - 1


def p_mod(a: int, m: int) -> int:
    dm = p_deg(m)
    while a and p_deg(a) >= dm:
        a ^= m << (p_deg(a) - dm)
    return a


def p_mul(a: int, b: int) -> int:
    r = 0
    while b:
        if b & 1:
            r ^= a
        a <<= 1
        b >>= 1
    return r


def p_gcd(a: int, b: int) -> int:
    while b:
        a, b = b, p_mod(a, b)
    return a


def p_x_pow2k(k: int, m: int) -> int:
    """x^(2^k) mod m."""
    r = 2
    for _ in range(k):
        r = p_mod(p_mul(r, r), m)
    return r


def prime_factors(n: int) -> list[int]:
    out, p = [], 2
    while p * p <= n:
        if n % p == 0:
            out.append(p)
            while n % p == 0:
                n //= p
        p += 1
    if n > 1:
        out.append(n)
    return out


def rabin_irreducible(f: int, n: int) -> bool:
    if p_deg(f) != n:
        return False
    if p_x_pow2k(n, f) != p_mod(2, f):
        return False
    for p in prime_factors(n):
        h = p_x_pow2k(n // p, f) ^ 2
        if p_gcd(f, p_mod(h, f)) != 1:
            return False
    return True


# ---------------------------------------------------------------------------
# GF(2^n): scalar and numpy-vectorised


class Field:
    def __init__(self, n: int, f: int):
        self.n, self.f = n, f

    def mul(self, a: int, b: int) -> int:
        return p_mod(p_mul(a, b), self.f)

    def pow(self, a: int, e: int) -> int:
        r = 1
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            e >>= 1
        return r

    def inv(self, a: int) -> int:
        assert a
        return self.pow(a, (1 << self.n) - 2)

    def vmul(self, a: np.ndarray, b) -> np.ndarray:
        """Elementwise product of uint64 arrays (or array and scalar); n <= 31."""
        n = self.n
        a = np.asarray(a, dtype=np.uint64)
        b = np.broadcast_to(np.asarray(b, dtype=np.uint64), a.shape)
        r = np.zeros(a.shape, dtype=np.uint64)
        one = np.uint64(1)
        for i in range(n):
            bit = (b >> np.uint64(i)) & one
            r ^= (a << np.uint64(i)) * bit
        for i in range(2 * n - 2, n - 1, -1):
            bit = (r >> np.uint64(i)) & one
            r ^= np.uint64(self.f << (i - n)) * bit
        return r


def solve_linear_gf2(cols: list[int], target: int, nbits: int):
    """Solve sum_j y_j cols[j] = target over F_2; cols are nbits-bit ints."""
    rows = []  # (vector, combination)
    for j, c in enumerate(cols):
        rows.append([c, 1 << j])
    piv = {}
    for r in rows:
        v, comb = r
        while v:
            t = v.bit_length() - 1
            if t in piv:
                v ^= piv[t][0]
                comb ^= piv[t][1]
            else:
                piv[t] = (v, comb)
                break
    v, comb = target, 0
    while v:
        t = v.bit_length() - 1
        if t not in piv:
            return None
        v ^= piv[t][0]
        comb ^= piv[t][1]
    return comb


class BinCurve:
    """y^2 + x y = x^3 + a x^2 + b over GF(2^n)."""

    def __init__(self, F: Field, a: int, b: int):
        self.F, self.a, self.b = F, a, b

    def on_curve(self, P) -> bool:
        if P is None:
            return True
        F = self.F
        x, y = P
        return F.mul(y, y) ^ F.mul(x, y) == F.mul(F.mul(x, x), x) ^ F.mul(self.a, F.mul(x, x)) ^ self.b

    def lift(self, x: int):
        F = self.F
        rhs = F.mul(F.mul(x, x), x) ^ F.mul(self.a, F.mul(x, x)) ^ self.b
        # y -> y^2 + x y is F_2-linear; images of the basis elements 2^j
        cols = [F.mul(1 << j, 1 << j) ^ F.mul(x, 1 << j) for j in range(F.n)]
        y = solve_linear_gf2(cols, rhs, F.n)
        if y is None:
            return None
        P = (x, y)
        assert self.on_curve(P)
        return P

    def add(self, P, Q):
        F = self.F
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if y1 ^ y2 == x1 or x1 == 0:  # Q = -P (also P = -P at x = 0)
                return None
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.mul(lam, lam) ^ lam ^ self.a
            y3 = F.mul(x1, x1) ^ F.mul(lam ^ 1, x3)
        else:
            lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
            x3 = F.mul(lam, lam) ^ lam ^ x1 ^ x2 ^ self.a
            y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
        R = (x3, y3)
        assert self.on_curve(R), "group law produced an off-curve point"
        return R

    def s3(self, x1: int, x2: int, x3: int) -> int:
        F = self.F
        s = F.mul(x1, x2) ^ F.mul(x1, x3) ^ F.mul(x2, x3)
        return F.mul(s, s) ^ F.mul(F.mul(x1, x2), x3) ^ self.b


def sum_basis(basis: list[int], assign: int, offset: int) -> int:
    x = 0
    for i, v in enumerate(basis):
        if (assign >> (offset + i)) & 1:
            x ^= v
    return x


def gf2_rank(vecs) -> int:
    piv = {}
    for v in vecs:
        while v:
            t = v.bit_length() - 1
            if t in piv:
                v ^= piv[t]
            else:
                piv[t] = v
                break
    return len(piv)


# ---------------------------------------------------------------------------
# Weil descent by evaluation and Moebius transform


def descent_by_evaluation(F: Field, b: int, basis: list[int], xr: int, chunk_bits: int = 20):
    """S3(x1(a), x2(b), x_R) at every assignment index a | b << n', evaluated in
    chunks of consecutive indices; returns Z and the ANF support of each bit."""
    npr = len(basis)
    N = 2 * npr
    V = np.zeros(1, dtype=np.uint64)
    for v in basis:  # V[c] = sum_i c_i v_i, index bit i <-> basis vector i
        V = np.concatenate([V, V ^ np.uint64(v)])
    S = np.empty(1 << N, dtype=np.uint32 if F.n <= 32 else np.uint64)
    step = 1 << max(npr, min(chunk_bits, N))
    for lo in range(0, 1 << N, step):
        idx = np.arange(lo, lo + step, dtype=np.uint64)
        X1 = V[(idx & np.uint64((1 << npr) - 1)).astype(np.int64)]
        X2 = V[(idx >> np.uint64(npr)).astype(np.int64)]
        x1x2 = F.vmul(X1, X2)
        sv = x1x2 ^ F.vmul(X1, xr) ^ F.vmul(X2, xr)
        S[lo:lo + step] = F.vmul(sv, sv) ^ F.vmul(x1x2, xr) ^ np.uint64(b)
    Z = [int(i) for i in np.flatnonzero(S == 0)]
    anf = []
    for l in range(F.n):
        t = ((S >> l) & 1).astype(np.uint8)
        for i in range(N):
            t = t.reshape(-1, 2, 1 << i)
            t[:, 1, :] ^= t[:, 0, :]
            t = t.reshape(-1)
        anf.append(set(int(m) for m in np.flatnonzero(t)))
    return Z, anf


# ---------------------------------------------------------------------------
# Polynomials of degree <= d in N variables as bitmask ints


class Monomials:
    def __init__(self, N: int, d: int):
        self.N, self.d = N, d
        self.monos: list[tuple] = []
        self.deg: list[int] = []
        for e in range(d + 1):
            for m in itertools.combinations_with_replacement(range(N), e):
                self.monos.append(m)
                self.deg.append(e)
        self.index = {m: i for i, m in enumerate(self.monos)}
        self.ncols = len(self.monos)
        self.nbytes = (self.ncols + 7) // 8
        self.low_limit = 1 + N  # indices < low_limit have degree <= 1
        # mt[v][i] = index of x_v * monos[i] (or -1 if degree would exceed d)
        mt = np.full((N, self.ncols), -1, dtype=np.int64)
        for i, m in enumerate(self.monos):
            if len(m) < d:
                for v in range(N):
                    mt[v, i] = self.index[tuple(sorted(m + (v,)))]
        self.mt = mt

    def from_indices(self, idx, distinct: bool = False) -> int:
        arr = np.zeros(self.nbytes * 8, dtype=np.uint8)
        if distinct:  # x_v * (monomials) is injective, so no cancellation
            arr[np.asarray(idx, dtype=np.int64)] = 1
        else:  # F_2 sum: repeated monomials cancel
            np.bitwise_xor.at(arr, np.asarray(idx, dtype=np.int64), 1)
        return int.from_bytes(np.packbits(arr, bitorder="little").tobytes(), "little")

    def to_indices(self, f: int) -> np.ndarray:
        raw = np.frombuffer(f.to_bytes(self.nbytes, "little"), dtype=np.uint8)
        return np.flatnonzero(np.unpackbits(raw, bitorder="little"))

    def poly(self, terms) -> int:
        return self.from_indices([self.index[tuple(sorted(t))] for t in terms])

    def degree(self, f: int) -> int:
        return self.deg[f.bit_length() - 1]

    def eval_mask(self, z: int) -> int:
        """Bitmask of the monomials that evaluate to 1 at the 0/1 point z."""
        idx = [i for i, m in enumerate(self.monos) if all((z >> v) & 1 for v in m)]
        return self.from_indices(idx)


class Echelon:
    def __init__(self):
        self.piv: dict[int, int] = {}

    def reduce(self, v: int) -> int:
        piv = self.piv
        while v:
            p = piv.get(v.bit_length() - 1)
            if p is None:
                return v
            v ^= p
        return 0

    def insert(self, v: int) -> int:
        r = self.reduce(v)
        if r:
            self.piv[r.bit_length() - 1] = r
        return r


def generators(M: Monomials, eqs: list[list[list[int]]]) -> list[int]:
    gens = []
    for e in eqs:
        p = M.poly([tuple(t) for t in e])
        if p:
            gens.append(p)
    for v in range(M.N):
        gens.append(M.poly([(v, v), (v,)]))
    return gens


def one_shot_low(M: Monomials, gens: list[int]) -> tuple[int, int]:
    E = Echelon()
    mult = {e: list(itertools.combinations_with_replacement(range(M.N), e)) for e in range(M.d + 1)}
    for g in gens:
        terms = [M.monos[i] for i in M.to_indices(g)]
        gd = M.degree(g)
        for e in range(M.d - gd + 1):
            for m in mult[e]:
                E.insert(M.poly([t + m for t in terms]))
    low = sum(1 for p in E.piv if p < M.low_limit)
    return low, len(E.piv)


def closure(M: Monomials, gens: list[int]):
    E = Echelon()
    queue = deque()
    for g in gens:
        r = E.insert(g)
        if r:
            queue.append(r)
    products = 0
    while queue:
        f = queue.popleft()
        if M.degree(f) >= M.d:
            continue
        idx = M.to_indices(f)
        for v in range(M.N):
            prod_idx = M.mt[v][idx]
            assert (prod_idx >= 0).all()
            products += 1
            r = E.insert(M.from_indices(prod_idx, distinct=True))
            if r:
                queue.append(r)
    low = sum(1 for p in E.piv if p < M.low_limit)
    return E, low, products


def closedness_pass(M: Monomials, E: Echelon, sample: int | None, seed: int) -> tuple[int, int]:
    cand = [f for f in E.piv.values() if M.degree(f) < M.d]
    pairs = [(f, v) for f in cand for v in range(M.N)]
    if sample is not None and sample < len(pairs):
        pairs = random.Random(seed).sample(pairs, sample)
    bad = 0
    for f, v in pairs:
        if E.reduce(M.from_indices(M.mt[v][M.to_indices(f)], distinct=True)):
            bad += 1
    return len(pairs), bad


# ---------------------------------------------------------------------------


def d_lin_label(k, low2, low3, low4):
    if low2 >= k:
        return "2"
    if low3 >= k:
        return "3"
    if low4 is None:
        return ">3"
    return "4" if low4 >= k else ">4"


def d_iter_label(k, low2, it3, it4):
    if low2 >= k:
        return "2"
    if it3 is not None and it3 >= k:
        return "3"
    if it4 is not None and it4 >= k:
        return "4"
    if it4 is not None:
        return ">4"
    if it3 is not None:
        return ">3"
    return "?"


def descent_by_interpolation(F: Field, C, basis: list[int], xr: int, rnd, n_random: int = 500):
    """For N too large to brute-force. x1, x2 are F_2-linear in (a, b) and S3 is a product
    of such forms, so each output bit has Boolean degree <= 2 and is fixed by its values at
    the points of Hamming weight <= 2; interpolate there (Moebius on the weight-<=2 cube),
    then check the interpolant against S3 at n_random random assignments."""
    npr = len(basis)
    N = 2 * npr

    def s3_at(assign: int) -> int:
        x1 = x2 = 0
        for i in range(npr):
            if (assign >> i) & 1:
                x1 ^= basis[i]
            if (assign >> (npr + i)) & 1:
                x2 ^= basis[i]
        return C.s3(x1, x2, xr)

    f0 = s3_at(0)
    f1 = [s3_at(1 << i) for i in range(N)]
    coef = {0: f0}
    for i in range(N):
        coef[1 << i] = f1[i] ^ f0
    for i in range(N):
        for j in range(i + 1, N):
            coef[(1 << i) | (1 << j)] = s3_at((1 << i) | (1 << j)) ^ f1[i] ^ f1[j] ^ f0
    anf = [set(m for m, c in coef.items() if (c >> l) & 1) for l in range(F.n)]

    def interp(assign: int) -> int:
        v = 0
        for m, c in coef.items():
            if m & assign == m:
                v ^= c
        return v
    agree = all(interp(a) == s3_at(a) for a in (rnd.getrandbits(N) for _ in range(n_random)))
    return anf, agree


def check_instance(row: dict, dump: dict, iter4: bool, low4: bool, iter3: bool, seed: int,
                   expect: dict | None = None, interp: bool = False) -> dict:
    t0 = time.time()
    n, npr, N = row["n"], row["nprime"], row["N"]
    out: dict = {"n": n, "instance": row["instance"], "N": N, "checks": {}, "mine": {}, "rust": {}}
    ck = out["checks"]
    f = int(row["modulus"], 16)
    a, b, xr = int(row["a"], 16), int(row["b"], 16), int(row["x_R"], 16)
    basis = [int(v, 16) for v in row["basis"]]
    F = Field(n, f)
    ck["dump_n_N_match"] = dump["n"] == n and dump["N"] == N == 2 * npr and npr == (n + 1) // 2
    ck["modulus_irreducible_deg_n"] = rabin_irreducible(f, n)
    ck["b_nonzero"] = b != 0
    ck["basis_rank_nprime"] = len(basis) == npr and gf2_rank(basis) == npr
    C = BinCurve(F, a, b)
    ck["x_R_is_abscissa"] = C.lift(xr) is not None
    # S3 on true sums with the validator's own group law
    rnd = random.Random(seed ^ (n << 8) ^ row["instance"])
    ok, tested = True, 0
    while tested < 4:
        P = C.lift(rnd.randrange(1, 1 << n))
        Q = C.lift(rnd.randrange(1, 1 << n))
        if P is None or Q is None:
            continue
        R = C.add(P, Q)
        if R is None or P[0] == Q[0]:
            continue
        ok &= C.s3(P[0], Q[0], R[0]) == 0
        tested += 1
    ck["S3_vanishes_on_true_sums"] = ok
    # descent by evaluation (brute force), or by exact interpolation for large N
    if interp:
        anf, agree = descent_by_interpolation(F, C, basis, xr, rnd)
        ck["degree_le_2_interpolant_matches_S3_at_500_random_points"] = agree
        Z = list(dump["Z"])  # completeness is not brute-forced here (see msolve quotient dimension)
        ev_ok = all(C.s3(sum_basis(basis, z, 0), sum_basis(basis, z, len(basis)), xr) == 0 for z in Z)
        ck["dumped_Z_are_zeros_of_S3"] = ev_ok
    else:
        Z, anf = descent_by_evaluation(F, b, basis, xr)
    dumped = []
    sq_free = True
    for e in dump["eqs"]:
        s: set[int] = set()
        for t in e:
            if len(set(t)) != len(t):
                sq_free = False
            m = 0
            for v in t:
                m |= 1 << v
            s ^= {m}
        dumped.append(s)
    ck["dumped_eqs_square_free"] = sq_free
    ck["descent_anf_equals_dump"] = len(dumped) == n and all(x == y for x, y in zip(anf, dumped))
    if not interp:
        ck["Z_bruteforce_equals_dump"] = sorted(Z) == sorted(dump["Z"])
    # k, affdim
    ev = [1 | (z << 1) for z in Z]
    r = gf2_rank(ev)
    k = N + 1 - r
    me = out["mine"]
    me.update(Z_size=len(Z), affdim=r - 1, k=k)
    # rho
    quad = []
    shape_ok = True
    for s in anf:
        v = 0
        for m in s:
            w = bin(m).count("1")
            if w == 2:
                i = (m & ((1 << npr) - 1)).bit_length() - 1
                j = (m >> npr).bit_length() - 1
                if i < 0 or j < 0 or (m & ((1 << npr) - 1)).bit_count() != 1:
                    shape_ok = False
                    continue
                v |= 1 << (i * npr + j)
            elif w > 2:
                shape_ok = False
        quad.append(v)
    ck["descent_bilinear_shape"] = shape_ok
    me["rho"] = gf2_rank(quad)
    # one-shot and iterated
    timings = {}
    for d in (2, 3, 4):
        if d == 4 and not low4:
            me["low4"] = None
            continue
        t = time.time()
        M = Monomials(N, d)
        low, _ = one_shot_low(M, generators(M, dump["eqs"]))
        me[f"low{d}"] = low
        timings[f"low{d}_s"] = round(time.time() - t, 2)
    zmasks = None
    for d, want in ((2, True), (3, iter3), (4, iter4)):
        if not want:
            me[f"iter{d}"] = None
            continue
        t = time.time()
        M = Monomials(N, d)
        E, low, products = closure(M, generators(M, dump["eqs"]))
        me[f"iter{d}"] = low
        me[f"closure{d}_dim"] = len(E.piv)
        me[f"closure{d}_contains_1"] = 0 in E.piv
        me[f"closure{d}_products"] = products
        zmasks = [M.eval_mask(z) for z in Z]
        vanish = all((fv & zm).bit_count() % 2 == 0 for fv in E.piv.values() for zm in zmasks)
        ck[f"closure{d}_vanishes_on_Z"] = vanish
        ck[f"closure{d}_low_le_k"] = low <= k
        tested, bad = closedness_pass(M, E, None if d <= 3 else 400, seed + d)
        ck[f"closure{d}_closed_pass"] = bad == 0
        me[f"closure{d}_closedness_products_tested"] = tested
        timings[f"iter{d}_s"] = round(time.time() - t, 2)
    me["d_lin"] = d_lin_label(k, me["low2"], me["low3"], me["low4"])
    me["d_iter"] = d_iter_label(k, me["low2"], me["iter3"], me["iter4"])  # producer's convention
    # README definition (least d with iter_d >= k), testing degree 2 with the degree-2 closure
    me["d_iter_by_closure"] = d_iter_label(k, me["iter2"], me["iter3"], me["iter4"])
    # producer's values
    keys = ["Z_size", "affdim", "k", "rho", "low2", "low3", "low4", "iter3", "iter4", "d_lin", "d_iter"]
    src = row if expect is None else expect
    if expect is not None:
        ck["expect_row_same_instance"] = all(expect.get(kk) == row.get(kk) for kk in
                                             ("n", "nprime", "N", "a", "b", "modulus", "basis", "x_R"))
    rust = {kk: src.get(kk) for kk in keys}
    out["rust"] = rust
    mism = [kk for kk in keys if rust[kk] != me.get(kk)]
    out["value_mismatches"] = mism
    out["failed_checks"] = [c for c, v in ck.items() if v is not True]
    out["timings"] = timings
    out["seconds"] = round(time.time() - t0, 2)
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-json", required=True)
    ap.add_argument("--dump-dir", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--iter3-max-n", type=int, default=99)
    ap.add_argument("--iter4-max-n", type=int, default=0)
    ap.add_argument("--low4-max-n", type=int, default=0)
    ap.add_argument("--only", default="")
    ap.add_argument("--seed", type=int, default=20261105)
    ap.add_argument("--interp-above-N", type=int, default=99,
                    help="use exact weight-<=2 interpolation instead of 2^N brute force when N exceeds this")
    ap.add_argument("--expect-json", default="",
                    help="compare against this producer JSON (same instances, matched by n and instance)")
    args = ap.parse_args()
    rows = json.loads(Path(args.run_json).read_text())["rows"]
    expect = {}
    if args.expect_json:
        expect = {(r["n"], r["instance"]): r for r in json.loads(Path(args.expect_json).read_text())["rows"]}
    only = {tuple(int(x) for x in s.split(":")) for s in args.only.split(",") if s}
    results = []
    for row in rows:
        if only and (row["n"], row["instance"]) not in only:
            continue
        dpath = Path(args.dump_dir) / f"n{row['n']}_i{row['instance']}.json"
        dump = json.loads(dpath.read_text())
        res = check_instance(row, dump, iter4=row["n"] <= args.iter4_max_n,
                             low4=row["n"] <= args.low4_max_n, iter3=row["n"] <= args.iter3_max_n,
                             seed=args.seed, interp=row["N"] > args.interp_above_N,
                             expect=expect.get((row["n"], row["instance"]), {"missing": True}) if expect else None)
        res["dump"] = str(dpath)
        results.append(res)
        status = "OK" if not res["value_mismatches"] and not res["failed_checks"] else "MISMATCH"
        print(f"n={row['n']:2} i={row['instance']} {status} mine={res['mine']} "
              f"mism={res['value_mismatches']} failed={res['failed_checks']} {res['seconds']} s", flush=True)
    summary = {
        "instances": len(results),
        "instances_all_values_match": sum(1 for r in results if not r["value_mismatches"]),
        "instances_all_checks_pass": sum(1 for r in results if not r["failed_checks"]),
        "compared_values": sum(sum(1 for v in r["rust"].values()) for r in results),
        "value_mismatches": sum(len(r["value_mismatches"]) for r in results),
    }
    Path(args.out).write_text(json.dumps({"schema": "validator.closure-check/v1", "run_json": args.run_json,
                                          "dump_dir": args.dump_dir, "args": vars(args), "summary": summary,
                                          "results": results}, indent=1))
    print(json.dumps(summary))
    return 0 if summary["value_mismatches"] == 0 and summary["instances_all_checks_pass"] == len(results) else 1


if __name__ == "__main__":
    sys.exit(main())
