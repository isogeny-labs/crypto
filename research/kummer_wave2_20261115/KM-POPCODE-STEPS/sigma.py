"""KM-POPCODE-STEPS: nu_d(s), sigma_W(d) for primitive low-popcount multiples of f on the tower.
Python + numpy, dense linear algebra mod p. float64 BLAS is exact because p < 2^16 for the main
runs (partial sums < 2^53); small cells at p = 786433 are also exact (k*p^2 < 2^53 for k <= 256).
See PROTOCOL.md. [pcc:unverified]"""
import os
os.environ.setdefault("OMP_NUM_THREADS", "1"); os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
import numpy as np, itertools, time, json, sys

def log(*a):
    print(time.strftime("%H:%M:%S"), *a, flush=True)

# ---------------- linear algebra mod p ----------------
def inv_unit_lower(L, p):
    k = L.shape[0]
    inv = np.zeros((k, k), np.int64)
    for i in range(k):
        inv[i, i] = 1
        if i:
            inv[i, :i] = (-(L[i, :i] @ inv[:i, :i])) % p
    return inv

def inv_upper(U, p):
    k = U.shape[0]
    d = np.array([pow(int(U[i, i]) % p, -1, p) for i in range(k)], np.int64)
    Up = (U * d[:, None]) % p                      # unit upper
    Li = inv_unit_lower(np.ascontiguousarray(Up.T), p)
    return (Li.T * d[None, :]) % p                 # U^{-1} = U'^{-1} diag(d^{-1}) ... see test

def nullspace(M, p, b=256, rows_chunk=1024):
    """Right kernel of M (m x n int32, destroyed). Returns (free, K) with K n x nfree int32,
    K[free[q], q] = 1 (RREF-style coordinates)."""
    m, n = M.shape
    r = 0; piv = []; free = []
    c0 = 0
    while c0 < n:
        if r >= m:
            free.extend(range(c0, n)); break
        c1 = min(n, c0 + b); w = c1 - c0
        panel = M[r:, c0:c1].astype(np.int64)
        L = np.zeros((m - r, w), np.int64)
        k = 0; pcols = []
        for j in range(w):
            if k >= m - r:
                free.append(c0 + j); continue
            col = panel[k:, j] % p
            nz = np.flatnonzero(col)
            if nz.size == 0:
                free.append(c0 + j); continue
            i0 = int(nz[0]) + k
            if i0 != k:
                panel[[k, i0]] = panel[[i0, k]]
                L[[k, i0]] = L[[i0, k]]
                M[[r + k, r + i0]] = M[[r + i0, r + k]]
            prow = panel[k] % p
            panel[k] = prow
            inv = pow(int(prow[j]), -1, p)
            cvec = (panel[k + 1:, j] % p) * inv % p
            L[k + 1:, k] = cvec
            if cvec.size:
                panel[k + 1:, j:] -= cvec[:, None] * prow[None, j:]
            pcols.append(c0 + j); k += 1
        if k > 0:
            M[r:r + k, c0:c1] = (panel[:k] % p).astype(np.int32)
            M[r + k:, c0:c1] = 0
            if c1 < n:
                Lm = L[:k, :k].copy()
                Lm[np.arange(k), np.arange(k)] = 1
                Linv = inv_unit_lower(Lm, p).astype(np.float64)
                T1 = M[r:r + k, c1:].astype(np.float64)
                U12 = (Linv @ T1) % p
                M[r:r + k, c1:] = U12.astype(np.int32)
                L21 = (L[k:, :k] % p).astype(np.float64)
                rr = r + k
                for i in range(0, m - rr, rows_chunk):
                    i2 = min(m - rr, i + rows_chunk)
                    blk = M[rr + i:rr + i2, c1:].astype(np.float64) - L21[i:i2] @ U12
                    M[rr + i:rr + i2, c1:] = (blk % p).astype(np.int32)
            piv.extend(pcols); r += k
        del panel, L
        c0 = c1
    piv = np.array(piv, np.int64); free = np.array(free, np.int64)
    nf = len(free)
    K = np.zeros((n, nf), np.int32)
    if nf:
        K[free, np.arange(nf)] = 1
    if r and nf:
        X = np.zeros((r, nf), np.float64)
        bb = 256
        starts = list(range(0, r, bb))[::-1]
        for i0 in starts:
            i1 = min(r, i0 + bb)
            rows = M[i0:i1]
            rhs = rows[:, free].astype(np.float64)
            if i1 < r:
                rhs = rhs - rows[:, piv[i1:]].astype(np.float64) @ X[i1:]
            D = rows[:, piv[i0:i1]].astype(np.int64)
            D = np.triu(D)
            Dinv = inv_upper(D, p).astype(np.float64)
            X[i0:i1] = (Dinv @ (rhs % p)) % p
        K[piv, :] = ((-X) % p).astype(np.int32)
    return free, K

def rank_rows(A, p):
    """rank of A (rows x cols, int32), A destroyed."""
    free, _ = nullspace(A, p)
    return A.shape[1] - len(free)

# ---------------- monomials ----------------
class Level:
    def __init__(self, s, d):
        self.s, self.d = s, d
        bits = 2 * s
        ws = []
        for k in range(d + 1):
            for comb in itertools.combinations(range(bits), k):
                w = 0
                for c in comb: w |= 1 << c
                ws.append(w)
        w = np.array(ws, np.int64)
        mask = (1 << s) - 1
        self.a = w & mask; self.b = w >> s
        self.n = len(w)
        self.pop = np.array([bin(int(x)).count("1") for x in w], np.int64)
        self.key = (self.a << 16) | self.b
        self.order = np.argsort(self.key); self.skey = self.key[self.order]
        # parent / bit source for value recursion
        pa = self.a.copy(); pb = self.b.copy(); src = np.zeros(self.n, np.int64)
        for i in range(self.n):
            a, b = int(self.a[i]), int(self.b[i])
            if a:
                low = a & -a; pa[i] = a ^ low; src[i] = low.bit_length() - 1
            elif b:
                low = b & -b; pb[i] = b ^ low; src[i] = s + low.bit_length() - 1
        self.parent = self.index_of((pa << 16) | pb)
        self.src = src
        self.popstart = np.searchsorted(self.pop, np.arange(d + 2))  # w is ordered by popcount
    def index_of(self, keys):
        pos = np.searchsorted(self.skey, keys)
        posc = np.minimum(pos, self.n - 1)
        ok = (pos < self.n) & (self.skey[posc] == keys)
        return np.where(ok, self.order[posc], -1)

def eval_chunk(lev, X, Y, p):
    """values (len(X) x n) int64 of all level monomials at points (X,Y)."""
    s = lev.s; c = len(X)
    BP = np.zeros((2 * s, c), np.int64)
    bx = X % p; by = Y % p
    for j in range(s):
        BP[j] = bx; BP[s + j] = by
        bx = bx * bx % p; by = by * by % p
    V = np.empty((lev.n, c), np.int64)
    V[0] = 1
    for k in range(1, lev.d + 1):
        lo, hi = lev.popstart[k], lev.popstart[k + 1]
        if lo >= hi: continue
        V[lo:hi] = V[lev.parent[lo:hi]] * BP[lev.src[lo:hi]] % p
    return V.T

def eval_matrix(lev, X, Y, p, out=None):
    n = lev.n
    chunk = max(64, int(25e6 // max(n, 1)))
    if out is None: out = np.empty((len(X), n), np.int32)
    for i in range(0, len(X), chunk):
        out[i:i + chunk] = eval_chunk(lev, X[i:i + chunk], Y[i:i + chunk], p).astype(np.int32)
    return out

# ---------------- curves ----------------
def modinv_arr(a, p):
    res = np.ones_like(a); base = a % p; e = p - 2
    while e:
        if e & 1: res = res * base % p
        base = base * base % p; e >>= 1
    return res

def curve_points(W, coef, p, need, rng, kind="irr"):
    """list of distinct F_p-points (X,Y), randomly ordered, at least `need` if available."""
    xs = np.arange(p, dtype=np.int64)
    if kind == "lines":   # (1+z1)(1+z2): lines z1=-1 and z2=-1, interleaved
        t = rng.permutation(p).astype(np.int64)
        X = np.empty(2 * p, np.int64); Y = np.empty(2 * p, np.int64)
        X[0::2] = p - 1; Y[0::2] = t
        X[1::2] = t;     Y[1::2] = p - 1
        # drop duplicate (-1,-1) beyond first occurrence is harmless (kept distinct count slightly lower)
        keep = np.ones(2 * p, bool)
        dup = np.flatnonzero((X == p - 1) & (Y == p - 1))
        keep[dup[1:]] = False
        return X[keep], Y[keep]
    g = []  # g[b] = sum_a c[a][b] x^a
    for b in range(W + 1):
        v = np.zeros(p, np.int64)
        for a in range(W, -1, -1):
            v = (v * xs + int(coef[a][b])) % p
        g.append(v)
    PX = []; PY = []
    if W == 1:
        ok = g[1] != 0
        y = (-g[0] * modinv_arr(np.where(ok, g[1], 1), p)) % p
        PX.append(xs[ok]); PY.append(y[ok])
    elif W == 2:
        sq = np.full(p, -1, np.int64); sq[(xs * xs) % p] = xs
        ok = g[2] != 0
        disc = (g[1] * g[1] - 4 * g[2] * g[0]) % p
        r = sq[disc]; ok &= r >= 0
        i2a = modinv_arr(np.where(ok, 2 * g[2] % p, 1), p)
        for sgn in (1, -1):
            y = ((-g[1] + sgn * r) % p) * i2a % p
            PX.append(xs[ok]); PY.append(y[ok])
        # r == 0 gives the same root twice: dedupe later
    else:
        got = 0
        for y in range(p):
            val = g[W]
            for b in range(W - 1, -1, -1):
                val = (val * y + g[b]) % p
            hit = xs[val == 0]
            if hit.size:
                PX.append(hit); PY.append(np.full(hit.size, y, np.int64)); got += hit.size
            if got >= need + 100: break
    X = np.concatenate(PX); Y = np.concatenate(PY)
    key = np.unique(X * p + Y)
    perm = rng.permutation(len(key))
    key = key[perm]
    return key // p, key % p

def need_points(W, s, kind): return (1 << (s + 1)) if kind == "lines" else W * (1 << (s + 1))

# ---------------- ladder ----------------
class Ladder:
    def __init__(self, W, coef, p, D, smax, rng, kind="irr", tag="", out=None, margin=200, verify=True):
        self.W, self.coef, self.p, self.D, self.smax, self.kind = W, coef, p, D, smax, kind
        self.rng = rng; self.tag = tag; self.out = out; self.margin = margin; self.verify = verify
        need = need_points(W, smax, kind)
        self.X, self.Y = curve_points(W, coef, p, need, rng, kind)
        log(tag, "points", len(self.X), "need(smax)", need)
        if len(self.X) < need: raise RuntimeError("not enough points %d < %d" % (len(self.X), need))
        self.lev = {}; self.ker = {}
        self.rows = []
    def level(self, d, s):
        if (d, s) not in self.lev: self.lev[(d, s)] = Level(s, d)
        return self.lev[(d, s)]
    def kernel(self, d, s):
        p = self.p; lev = self.level(d, s); n = lev.n
        Fc = need_points(self.W, s, self.kind)
        if n + self.margin < Fc:
            m = n + self.margin; sub = self.rng.choice(Fc, m, replace=False); sub.sort()
        else:
            m = Fc; sub = np.arange(Fc)
        M = eval_matrix(lev, self.X[sub], self.Y[sub], p)
        free, K = nullspace(M, p)
        del M
        ok = True
        if self.verify and len(free):
            Kf = K.astype(np.float64)
            chunk = max(64, int(25e6 // n))
            for i in range(0, Fc, chunk):
                A = eval_chunk(lev, self.X[i:min(Fc, i + chunk)], self.Y[i:min(Fc, i + chunk)], p).astype(np.float64)
                if np.any((A @ Kf) % p != 0): ok = False; break
        return free, K, m, Fc, ok
    def run(self):
        p = self.p
        for s in range(1, self.smax + 1):
            for d in range(1, self.D + 1):
                t0 = time.time()
                lev = self.level(d, s)
                free, K, m, Fc, ok = self.kernel(d, s)
                k = len(free)
                self.ker[(d, s)] = (free, K)
                rkE = None; nu = None
                if d >= 2 and k:
                    rkE = self.rankE(d, s)
                    nu = k - rkE
                elif d >= 2:
                    rkE = 0; nu = 0
                row = dict(tag=self.tag, W=self.W, p=p, d=d, s=s, n=lev.n, rows=m, Fc=Fc,
                           dimM=k, rankE=rkE, nu=nu, verified=ok, sec=round(time.time() - t0, 1))
                self.rows.append(row)
                log(json.dumps(row))
                if self.out:
                    with open(self.out, "a") as fh: fh.write(json.dumps(row) + "\n")
                # drop stale
                self.ker.pop((d, s - 2), None); self.lev.pop((d, s - 2), None)
    def rankE(self, d, s):
        p = self.p
        lev = self.level(d, s); free, K = self.ker[(d, s)]; k = len(free)
        cols = []
        # part 1: M_d(s-1)
        if s >= 2 and (d, s - 1) in self.ker:
            lp = self.level(d, s - 1); fp, Kp = self.ker[(d, s - 1)]
            if len(fp):
                mp = lp.index_of(lev.key[free])        # level-s free col -> row in level s-1 (or -1)
                # need the reverse: coordinates of embedded M_d(s-1) vectors at free columns of level s
                emb = lev.index_of(lp.key)             # level s-1 row -> level s index
                inv = np.full(lev.n, -1, np.int64); inv[emb] = np.arange(lp.n)
                src = inv[free]
                C = np.zeros((k, len(fp)), np.int32)
                good = src >= 0
                C[good] = Kp[src[good]]
                cols.append(C)
        # part 2: y * (M_{d-1}(s) cap no-carry)
        if (d - 1, s) in self.ker:
            l2 = self.level(d - 1, s); f2, K2 = self.ker[(d - 1, s)]; k2 = len(f2)
            if k2:
                for var in (0, 1):
                    ex = l2.a if var == 0 else l2.b
                    for j in range(s):
                        carry = (ex + (1 << j)) >= (1 << s)
                        comp = np.flatnonzero(carry)
                        if comp.size:
                            A = np.ascontiguousarray(K2[comp, :])
                            wf, Wm = nullspace(A, p)
                            if len(wf) == 0: continue
                            V2 = ((K2.astype(np.float64) @ Wm.astype(np.float64)) % p).astype(np.int32)
                        else:
                            V2 = K2
                        na = l2.a + ((1 << j) if var == 0 else 0)
                        nb = l2.b + ((1 << j) if var == 1 else 0)
                        okm = ~carry
                        tgt = np.full(l2.n, -1, np.int64)
                        tgt[okm] = lev.index_of(((na[okm]) << 16) | nb[okm])
                        assert np.all(tgt[okm] >= 0)
                        inv = np.full(lev.n, -1, np.int64); inv[tgt[okm]] = np.flatnonzero(okm)
                        src = inv[free]
                        C = np.zeros((k, V2.shape[1]), np.int32)
                        good = src >= 0
                        C[good] = V2[src[good]]
                        cols.append(C)
        if not cols: return 0
        G = np.ascontiguousarray(np.concatenate(cols, axis=1).T)   # gens x k
        # drop zero rows cheaply
        G = G[np.any(G != 0, axis=1)]
        if G.shape[0] == 0: return 0
        return rank_rows(G, p)
