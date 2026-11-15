#!/usr/bin/env python3
"""Exact degree-c polynomial-calculus (HKY) closure of a PKM Kummer chain system, in the
quotient ring R[u] = F_p[(Z/2^t)^m][u_0..u_{m-3}]  (tower equations reduced away exactly:
y_{i,0}^{e} -> the square-free monomial with bits of e mod 2^t).

V_c = smallest subspace of R[u]_{<=c} containing the reduced generators of degree <= c and
closed under  w in V_c, deg w <= c-1  ==>  NF(v*w) in V_c  for every variable v.
1 in V_c  iff a degree-c PC refutation exists (any selection strategy; F4 with max degree c
included).  Prints JSON: system, c, dim R[u]_{<=c}, dim V_c, one_in_closure.

Usage: closure.py SYS.ms m t c   (p < 2^16 required: exact float64 BLAS products)
"""
import itertools
import json
import os
import sys
import time

os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
import numpy as np  # noqa: E402

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import msparse as MP  # noqa: E402


def popcount(x):
    return bin(x).count("1")


def u_monos(k, maxdeg):
    out = []
    for d in range(maxdeg + 1):
        for comb in itertools.combinations_with_replacement(range(k), d):
            e = [0] * k
            for i in comb:
                e[i] += 1
            out.append(tuple(e))
    return out


class Ring:
    def __init__(self, m, t, c):
        self.m, self.t, self.c, self.T = m, t, c, 1 << t
        k = m - 2
        umon = {d: [] for d in range(c + 1)}
        for b in u_monos(k, c):
            umon[sum(b)].append(b)
        monos = []
        for kk in itertools.product(range(self.T), repeat=m):
            dk = sum(popcount(x) for x in kk)
            if dk > c:
                continue
            for db in range(c - dk + 1):
                for b in umon[db]:
                    monos.append((dk + db, kk, b))
        monos.sort(key=lambda z: (-z[0], z[1], z[2]))
        self.monos = [(kk, b) for _, kk, b in monos]
        self.deg = np.array([z[0] for z in monos], dtype=np.int64)
        self.index = {mo: i for i, mo in enumerate(self.monos)}
        self.d = len(self.monos)
        self.const = self.index[(tuple([0] * m), tuple([0] * k))]
        # multiplication tables for columns of degree <= c-1
        self.low = np.nonzero(self.deg <= c - 1)[0]
        self.mul = []
        for i in range(m):
            for j in range(t):
                tab = np.empty(len(self.low), dtype=np.int64)
                for n, col in enumerate(self.low):
                    kk, b = self.monos[col]
                    kk2 = list(kk)
                    kk2[i] = (kk2[i] + (1 << j)) % self.T
                    tab[n] = self.index[(tuple(kk2), b)]
                self.mul.append(tab)
        for q in range(k):
            tab = np.empty(len(self.low), dtype=np.int64)
            for n, col in enumerate(self.low):
                kk, b = self.monos[col]
                b2 = list(b)
                b2[q] += 1
                tab[n] = self.index[(kk, tuple(b2))]
            self.mul.append(tab)


def reduce_generators(path, m, t, ring, p):
    names, pp, polys = MP.read_ms(path)
    assert pp == p
    T = 1 << t
    gens = []
    for P in polys:
        red = {}
        for e, cf in P.items():
            kk = [0] * m
            b = [0] * (m - 2)
            for vi, ex in enumerate(e):
                if not ex:
                    continue
                nm = names[vi]
                if nm.startswith("y"):
                    i, j = map(int, nm[1:].split("_"))
                    kk[i] = (kk[i] + ex * (1 << j)) % T
                else:
                    b[int(nm[1:])] += ex
            key = (tuple(kk), tuple(b))
            red[key] = (red.get(key, 0) + cf) % p
        red = {k_: v for k_, v in red.items() if v}
        if not red:
            continue  # tower equations reduce to 0
        deg = max(sum(popcount(x) for x in k_[0]) + sum(k_[1]) for k_ in red)
        if deg > ring.c:
            continue
        row = np.zeros(ring.d, dtype=np.int64)
        for k_, v in red.items():
            row[ring.index[k_]] = v
        gens.append(row)
    return gens


def rref_small(M, p):
    M = M % p
    rows, cols = M.shape
    piv = []
    r = 0
    for c in range(cols):
        if r == rows:
            break
        nz = np.nonzero(M[r:, c])[0]
        if len(nz) == 0:
            continue
        i = r + nz[0]
        if i != r:
            M[[r, i]] = M[[i, r]]
        inv = pow(int(M[r, c]), p - 2, p)
        M[r] = (M[r] * inv) % p
        f = M[:, c].copy()
        f[r] = 0
        nzr = np.nonzero(f)[0]
        if len(nzr):
            M[nzr] = (M[nzr] - np.outer(f[nzr], M[r])) % p
        piv.append(c)
        r += 1
    return M[:r], piv


class Echelon:
    """RREF basis kept as float64 (exact: entries < p < 2^16, inner sums < 2^53)"""

    def __init__(self, d, p):
        self.d, self.p = d, p
        self.B = np.zeros((0, d), dtype=np.float64)
        self.piv = []

    def reduce(self, M):
        M = np.asarray(M, dtype=np.float64) % self.p
        if len(self.piv):
            M = (M - np.fmod(M[:, self.piv] @ self.B, self.p)) % self.p
        return M

    def insert(self, M):
        """insert rows of M; return the new RREF rows (int64) that enlarged the space"""
        R = self.reduce(M)
        nzrows = np.nonzero(np.any(R != 0, axis=1))[0]
        if len(nzrows) == 0:
            return np.zeros((0, self.d), dtype=np.int64)
        Rn, pn = rref_small(R[nzrows].astype(np.int64), self.p)
        if len(pn) == 0:
            return Rn
        Bn = Rn.astype(np.float64)
        if len(self.piv):
            self.B = (self.B - np.fmod(self.B[:, pn] @ Bn, self.p)) % self.p
        self.B = np.vstack([self.B, Bn])
        self.piv = list(self.piv) + list(pn)
        return Rn


def closure(path, m, t, c, p, chunk=1500, tlimit=3600):
    t0 = time.time()
    ring = Ring(m, t, c)
    gens = reduce_generators(path, m, t, ring, p)
    V = Echelon(ring.d, p)
    if gens:
        V.insert(np.array(gens))
    done = Echelon(ring.d, p)
    lowmask = ring.deg <= c - 1
    it = 0
    while True:
        it += 1
        if ring.const in V.piv:
            break
        if time.time() - t0 > tlimit:
            return dict(system=os.path.basename(path), c=c, dimR=ring.d, dimV=len(V.piv), one_in_closure=None,
                        status="time_limit", iterations=it, seconds=round(time.time() - t0, 1))
        pivdeg = ring.deg[V.piv] if len(V.piv) else np.array([], dtype=np.int64)
        lowrows = V.B[np.nonzero(pivdeg <= c - 1)[0]] if len(V.piv) else np.zeros((0, ring.d))
        if len(lowrows) == 0:
            break
        newrows = done.insert(lowrows)
        if len(newrows) == 0:
            break
        W = newrows[:, ring.low]
        stop = False
        for tab in ring.mul:  # products generated chunk-wise (memory)
            for s in range(0, len(W), chunk):
                Wb = W[s:s + chunk]
                P = np.zeros((len(Wb), ring.d), dtype=np.int64)
                P[:, tab] = Wb
                V.insert(P)
                if ring.const in V.piv:
                    stop = True
                    break
            if stop:
                break
    return dict(system=os.path.basename(path), c=c, dimR=ring.d, dimV=len(V.piv),
                one_in_closure=bool(ring.const in V.piv), status="complete", iterations=it,
                seconds=round(time.time() - t0, 1))


def main():
    path, m, t, c = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
    names, p, _ = MP.read_ms(path)
    assert p < 1 << 16
    print(json.dumps(closure(path, m, t, c, p)), flush=True)


if __name__ == "__main__":
    main()
