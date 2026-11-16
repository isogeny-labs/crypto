#!/usr/bin/env python3
"""HKY closure over R_eps = F_p[z1,z2]/(z_i^K - eps), K=2^t, popcount degree. numpy only."""
import sys, json, time, hashlib, itertools
import numpy as np

pc = lambda k: bin(k).count("1")

def primroot(p):
    fs = []; n = p - 1; d = 2
    while d * d <= n:
        if n % d == 0:
            fs.append(d)
            while n % d == 0: n //= d
        d += 1
    if n > 1: fs.append(n)
    for h in range(2, p):
        if all(pow(h, (p - 1) // q, p) != 1 for q in fs): return h

def rng(p, draw, tr, tag):
    h = hashlib.sha256(f"kmad|{tag}|{p}|{draw}|{tr}".encode()).digest()
    return [int.from_bytes(h[i:i+4], "little") % p for i in range(0, 20, 4)]

def S3_nf(p, a, b, g, xR):
    """normal form coefficients f[(i,j)] of S3(g z1, g z2, xR) in z1^i z2^j, i,j<=2"""
    f = {}
    def add(i, j, c): f[(i, j)] = (f.get((i, j), 0) + c) % p
    X = xR
    g2, g3, g4 = g*g % p, g**3 % p, g**4 % p
    add(2, 0, g2*X*X); add(0, 2, g2*X*X); add(1, 1, -2*g2*X*X)
    add(2, 1, -2*X*g3); add(1, 2, -2*X*g3)
    add(1, 0, -2*X*a*g); add(0, 1, -2*X*a*g); add(0, 0, -4*X*b)
    add(2, 2, g4); add(1, 1, -2*a*g2); add(0, 0, a*a)
    add(1, 0, -4*b*g); add(0, 1, -4*b*g)
    return f

def box1(p, c):
    return {(0,0): c[0] % p, (1,0): c[1] % p, (0,1): c[2] % p, (1,1): c[3] % p}

def grid_vals(p, f, t):
    K = 1 << t
    om = pow(primroot(p), (p - 1) // K, p)
    pw = np.array([pow(om, i, p) for i in range(K)], dtype=np.int64)
    val = np.zeros((K, K), dtype=np.int64)
    for (i, j), c in f.items():
        val = (val + c * np.outer(pw[(np.arange(K) * i) % K], pw[(np.arange(K) * j) % K])) % p
    return val

def no_zero_on_grid(p, f, t):
    return not (grid_vals(p, f, t) == 0).any()

class Closure:
    def __init__(self, p, t, eps, f, c):
        self.p, self.t, self.eps, self.c = p, t, eps % p, c
        K = self.K = 1 << t
        ks = [(a, b) for a in range(K) for b in range(K) if pc(a) + pc(b) <= c]
        ks.sort(key=lambda ab: (-(pc(ab[0]) + pc(ab[1])), ab))
        self.cols = ks; self.N = len(ks)
        self.idx = {k: i for i, k in enumerate(ks)}
        self.deg = np.array([pc(a) + pc(b) for a, b in ks])
        self.const = self.idx[(0, 0)]
        self.vmaps = []
        for v in (0, 1):
            for j in range(t):
                src, tgt, fac = [], [], []
                for i, (a, b) in enumerate(ks):
                    if self.deg[i] >= c: continue
                    e = (a, b)[v] + (1 << j)
                    w = 0
                    if e >= K: e -= K; w = 1
                    if w and self.eps == 0: continue
                    k2 = (e, b) if v == 0 else (a, e)
                    src.append(i); tgt.append(self.idx[k2]); fac.append(self.eps if w else 1)
                self.vmaps.append((np.array(src, dtype=np.int64), np.array(tgt, dtype=np.int64), np.array(fac, dtype=np.float64)))
        v0 = np.zeros(self.N)
        for (i, j), co in f.items():
            v0[self.idx[(i, j)]] = co % p
        self.f0 = v0

    def run(self):
        p, N = self.p, self.N
        B = np.zeros((0, N)); piv = np.zeros(0, dtype=np.int64)
        B, piv, queue = self._reduce_add(self.f0[None, :], B, piv)
        while len(queue):
            qdeg = self.deg[np.argmax(queue != 0, axis=1)]
            queue = queue[qdeg < self.c]
            if not len(queue): break
            cand = np.zeros((len(queue) * len(self.vmaps), N)); r = 0
            for (src, tgt, fac) in self.vmaps:
                cand[r:r+len(queue), tgt] = np.mod(queue[:, src] * fac, p)
                r += len(queue)
            cand = cand[cand.any(axis=1)]
            if not len(cand): break
            B, piv, queue = self._chunked(cand, B, piv)
        self.B, self.piv = B, piv
        return self

    def _chunked(self, cand, B, piv, ch=192):
        outs = []
        for s in range(0, len(cand), ch):
            B, piv, nr = self._reduce_add(cand[s:s+ch], B, piv)
            if len(nr): outs.append(nr)
        return B, piv, (np.vstack(outs) if outs else np.zeros((0, self.N)))

    def _reduce_add(self, X, B, piv):
        p = self.p
        if len(piv):
            X = np.mod(X - np.mod(X[:, piv] @ B, p), p)
        R = X[X.any(axis=1)].copy()
        acc = []; accq = []
        for i in range(len(R)):
            row = R[i]
            if not row.any(): continue
            q = int(np.argmax(row != 0))
            row = np.mod(row * pow(int(row[q]), p - 2, p), p)
            R[i] = row
            col = R[:, q].copy(); col[i] = 0
            nz = np.nonzero(col)[0]
            if len(nz):
                R[nz] = np.mod(R[nz] - np.outer(col[nz], row), p)
            acc.append(i); accq.append(q)
        if not acc:
            return B, piv, np.zeros((0, self.N))
        Bn = R[acc]; qn = np.array(accq, dtype=np.int64)
        if len(piv):
            B = np.mod(B - np.mod(B[:, qn] @ Bn, p), p)
        B = np.vstack([B, Bn]); piv = np.concatenate([piv, qn])
        return B, piv, Bn

    def has_one(self): return bool((self.piv == self.const).any())
    def dim(self): return len(self.piv)
    def dim_le(self, d): return int((self.deg[self.piv] <= d).sum())

def dF(p, t, eps, f, cmax, tlimit=1e9):
    prev_dim = None; ffd = None; rows = []; t0 = time.time()
    for c in range(2, cmax + 1):
        C = Closure(p, t, eps, f, c).run()
        d, dle = C.dim(), C.dim_le(c - 1)
        rows.append(dict(c=c, dim=d, dim_le_cm1=dle, cols=C.N, one=C.has_one(), s=round(time.time() - t0, 2)))
        if ffd is None and c >= 3 and prev_dim is not None and dle > prev_dim: ffd = c
        prev_dim = d
        if C.has_one():
            return dict(dF=c, ffd=ffd, censored=False, rows=rows)
        if time.time() - t0 > tlimit: return dict(dF=None, ffd=ffd, censored=True, rows=rows, note="time")
    return dict(dF=None, ffd=ffd, censored=True, rows=rows)

if __name__ == "__main__":
    p = 40961
    for tr in range(6):
        a, b, g, xR, _ = rng(p, 0, tr, "inst")
        f = S3_nf(p, a, b, g, xR)
        print(tr, [no_zero_on_grid(p, f, t) for t in (2,3,4)])
    a, b, g, xR, _ = rng(p, 0, 0, "inst")
    f = S3_nf(p, a, b, g, xR)
    for t in (2, 3, 4, 5):
        t0=time.time()
        print(t, no_zero_on_grid(p, f, t), dF(p, t, 1, f, 7)["dF"], dF(p, t, 0, f, 7)["dF"], time.time()-t0)
