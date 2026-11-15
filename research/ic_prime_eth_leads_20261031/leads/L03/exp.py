#!/usr/bin/env python3
"""L03: weight ladder of sparse F2-parity hashed bases vs Bochum affine hash vs dense F2-affine vs window vs random null.
usage: exp.py BITS SEEDS_PER_CURVE OUT.jsonl   env: CURVES=i,j  SEED0=0  FAMS=a,b  MS=2,3
X(R) for all R in Z_r by FFT over the dlog index (k -> kG). Independent slots (Bochum: slot-specific bases)."""
import sys, json, math, time, os
import numpy as np
import scipy.fft as sf
bits = int(sys.argv[1]); nseeds = int(sys.argv[2]); outp = sys.argv[3]
SEED0 = int(os.environ.get('SEED0', '0')); CUR = os.environ.get('CURVES')
FAMS = os.environ.get('FAMS', 'bochum,dense,sp3,sp5,sp7,reg3,reg5,sp3c,sp5c,reg3c,window,random').split(',')
MS = [int(x) for x in os.environ.get('MS', '2,3').split(',')]
here = os.path.dirname(os.path.abspath(__file__))
def isprime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d = n-1; s = 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True
curves = [(i+1, *map(int, l.split())) for i, l in enumerate(open(f'{here}/curves.txt'))]
curves = [c for c in curves if c[1] == bits and (CUR is None or str(c[0]) in CUR.split(','))]
def popparity(v):
    v = v.astype(np.uint64)
    for s in (32,16,8,4,2,1): v ^= v >> np.uint64(s)
    return (v & np.uint64(1)).astype(bool)
# ---- membership masks over k in Z_r; c = enc = 2x + (y&1) < 2p (the Bochum encoding) ----
def m_bochum(c, p, r, n, rng):
    # exact Bochum form: s prime in [2p,4p], alpha nonzero, beta; member iff (alpha*enc+beta) mod s < tau
    while True:
        s = int(rng.integers(2*p, 4*p+1))
        if isprime(s): break
    al = int(rng.integers(1, s)); be = int(rng.integers(0, s)); tau = max(1, round(s*n/r))
    return ((al*c + be) % s) < tau
def nrows(r, n): return max(1, round(math.log2(r/n)))
def rows_sparse(nb, k, w, rng):
    out = []
    for _ in range(k):
        pos = rng.choice(nb, size=w, replace=False)
        out.append(int(sum(1 << int(i) for i in pos)))
    return out
def rows_regular(nb, k, w, rng):
    # biregular Tanner graph: every column appears in ~k*w/nb rows (configuration model), no repeated column in a row
    for _ in range(200):
        stubs = np.resize(np.arange(nb), k*w); rng.shuffle(stubs); R = stubs.reshape(k, w)
        if all(len(set(row)) == w for row in R):
            return [int(sum(1 << int(i) for i in row)) for row in R]
    return rows_sparse(nb, k, w, rng)
def rows_dense(nb, k, rng):
    out = []
    for _ in range(k):
        mk = 0
        while mk == 0: mk = int(rng.integers(0, 1 << nb))
        out.append(mk)
    return out
def m_parity(c, rows, rng, plant=None):
    # plant=None: i.i.d. uniform RHS (system may be inconsistent on the curve -> empty base);
    # plant=code: RHS taken from a random curve point's code, so that point is always a member (consistent system)
    ok = np.ones(len(c), bool)
    for mk in rows:
        rhs = bool(rng.integers(0, 2)) if plant is None else bool(bin(int(plant) & mk).count('1') & 1)
        ok &= (popparity(c & mk) == rhs)
    return ok
def m_window(c, p, r, n, rng):
    w = max(1, round(n*p/r)); o = int(rng.integers(0, p))
    return (((c // 2) - o) % p) < w
def m_random(r, n, rng):
    mk = np.zeros(r, bool)
    if n > 0: mk[rng.choice(r, size=n, replace=False)] = True
    return mk
def make(fam, c, p, r, n, nb, rng):
    k = nrows(r, n)
    if fam == 'bochum': return m_bochum(c, p, r, n, rng)
    if fam == 'dense': return m_parity(c, rows_dense(nb, k, rng), rng)
    if fam.endswith('c') and fam != 'random':
        w = fam.rstrip('c'); plant = int(c[1 + int(rng.integers(0, len(c)-1))])
        rows = rows_regular(nb, k, int(w[3:]), rng) if w.startswith('reg') else rows_sparse(nb, k, int(w[2:]), rng)
        return m_parity(c, rows, rng, plant)
    if fam.startswith('sp'): return m_parity(c, rows_sparse(nb, k, int(fam[2:]), rng), rng)
    if fam.startswith('reg'): return m_parity(c, rows_regular(nb, k, int(fam[3:]), rng), rng)
    if fam == 'window': return m_window(c, p, r, n, rng)
    if fam in ('random', 'random2'): return m_random(r, max(1, round(n)), rng)
    raise ValueError(fam)
def conv(masks, r):
    F = None
    for mk in masks:
        f = sf.rfft(mk.astype(np.float64)); F = f if F is None else F*f
    return sf.irfft(F, n=r)
def stats(Xf):
    X = np.rint(Xf)[1:]  # exclude R=0
    E = float(X.mean()); V = float(X.var())
    return dict(E=E, Var=V, P1=float((X > 0).mean()), P2=float((X >= 2).mean()), F2=(V+E*E)/(E*E) if E > 0 else None)
rows = open(outp, 'a'); t0 = time.time()
for (ci, b, p, a, bb, r, gx, gy) in curves:
    tab = np.fromfile(f'{here}/tab_{b}_{ci}.bin', dtype=np.uint32).reshape(-1, 2).astype(np.int64)
    assert len(tab) == r
    c = 2*tab[:, 0] + (tab[:, 1] & 1); nb = bits + 1
    for sd in range(SEED0, SEED0+nseeds):
        for m in MS:
            K = round((m-1)*math.log2(r))
            ks = [K//m + (1 if i < K % m else 0) for i in range(m)]
            rng = np.random.default_rng(np.random.SeedSequence([ci, sd, m]))
            for fam in FAMS:
                ms_ = []
                for i in range(m):
                    mk = make(fam, c, p, r, r/2**ks[i], nb, rng).copy(); mk[0] = False; ms_.append(mk)
                sizes = [int(x.sum()) for x in ms_]
                Xf = conv(ms_, r)
                cons = abs(float(np.rint(Xf).sum()) - float(np.prod([float(s) for s in sizes]))) < 0.5
                # planted: choose random members, sum their dlogs, X(R) must be >= 1
                idx = [rng.choice(np.nonzero(x)[0]) if x.any() else 0 for x in ms_]
                planted = (not all(x.any() for x in ms_)) or bool(np.rint(Xf[int(sum(int(v) for v in idx)) % r]) >= 1)
                sa = stats(Xf)
                nm = [m_random(r, s, rng) for s in sizes]
                ref = 'random2_nominal' if fam == 'random' else 'random_same_sizes'
                if fam == 'random': nm = [make('random2', c, p, r, r/2**ks[i], nb, rng) for i in range(m)]
                sb = stats(conv(nm, r))
                rows.write(json.dumps(dict(bits=b, p=p, r=r, curve=ci, seed=sd, m=m, fam=fam, sizes=sizes, cons=bool(cons),
                                           planted=bool(planted), a=sa, ref=sb)) + '\n')
            rows.flush()
        if sd % 10 == 0: print(f'curve {ci} seed {sd} done t={time.time()-t0:.0f}s', flush=True)
