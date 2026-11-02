import sys, itertools, random, os
os.environ.setdefault("OMP_NUM_THREADS","3"); os.environ.setdefault("OPENBLAS_NUM_THREADS","3")
import numpy as np
import importlib.util as _u
_sp=_u.spec_from_file_location("lat2common","/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/MP-lattices-2/common.py"); _m=_u.module_from_spec(_sp); _sp.loader.exec_module(_m)
S3_specialised=_m.S3_specialised; S4_specialised=_m.S4_specialised

def curve(p, rng):
    while True:
        a, b, xR = rng.randrange(p), rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b**2) % p: return a, b, xR

def fpoly(m, p, rng):
    a, b, xR = curve(p, rng)
    f = S3_specialised(a, b, xR, p) if m == 2 else S4_specialised(a, b, xR, p)
    return f, (a, b, xR)

def evalf(f, pt, q):
    s = 0
    for e, c in f.items():
        t = c
        for x, k in zip(pt, e): t = t * pow(x, k, q) % q
        s += t
    return s % q

def sample_U(f, m, p, l, N, rng):
    """N random points of U mod p^l: (a mod p) in V(F_p), arbitrary lift."""
    q = p**l; pts = []
    while len(pts) < N:
        pre = [rng.randrange(p) for _ in range(m-1)]
        roots = [X for X in range(p) if evalf(f, pre+[X], p) == 0]
        if not roots: continue
        r = rng.choice(roots)
        base = pre + [r]
        pts.append([(x + p*rng.randrange(p**(l-1))) % q for x in base])
    return pts

def eval_matrix(M, pts, q):
    P = np.array(pts, dtype=np.int64); m = P.shape[1]; D = max(max(s) for s in M)
    pw = np.ones((P.shape[0], m, D+1), dtype=np.int64)
    for k in range(1, D+1): pw[:, :, k] = pw[:, :, k-1] * P % q
    A = np.ones((P.shape[0], len(M)), dtype=np.int64)
    for j, s in enumerate(M):
        col = np.ones(P.shape[0], dtype=np.int64)
        for i in range(m): col = col * pw[:, i, s[i]] % q
        A[:, j] = col
    return A

def val_matrix(A, p, l):
    V = np.full(A.shape, l, dtype=np.int64); B = A.copy(); nz = B != 0
    V[nz] = 0
    cur = nz.copy(); 
    for k in range(1, l):
        cur = cur & (B % p == 0); B = np.where(cur, B // p, B); V[cur] = k
    return V

def gain(A, p, l):
    """log_p [ker : p^l Z^n] for ker = {c in Z^n : A c = 0 mod p^l}. Valuation-pivoted SNF over Z/p^l."""
    q = p**l; A = A % q; n = A.shape[1]; e = 0
    while A.size:
        V = val_matrix(A, p, l)
        idx = np.argmin(V); r, c = divmod(int(idx), A.shape[1]); v = int(V[r, c])
        if v >= l: break
        pv = p**v; u = int(A[r, c]) // pv; ui = pow(u, -1, q)
        t = (A[:, c] // pv) * ui % q
        A = (A - np.outer(t, A[r]) % q) % q
        A = np.delete(np.delete(A, r, 0), c, 1)
        e += l - v
    return l*n - e

def ksum(M, d, l):
    return sum(min(l, min(si//d for si in s)) for s in M)

def degsum(M): return sum(sum(s) for s in M)
