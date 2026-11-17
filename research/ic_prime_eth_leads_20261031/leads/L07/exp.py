#!/usr/bin/env python3
"""L07: second moment of X(R) = #{decompositions of target R as sum of m base points}, exact over all R in Z_r.
usage: exp.py BITS REPS OUT.jsonl [workers]
Points are k*G, k in Z_r (r prime = group order); X over all targets R = FFT cyclic convolution over Z_r.
"""
import sys, json, math, time
import numpy as np
import scipy.fft as sf

bits = int(sys.argv[1]); reps = int(sys.argv[2]); outp = sys.argv[3]
W = int(sys.argv[4]) if len(sys.argv) > 4 else 1
import os
REPOFF=int(os.environ.get('REPOFF','0')); ARMS=os.environ.get('ARMS','window,window_y,affine,sparse5,dense,interval,random').split(','); CUR=os.environ.get('CURVES')
here = __import__('os').path.dirname(__import__('os').path.abspath(__file__))

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

curves = [l.split() for l in open(f'{here}/curves.txt')]
curves = [(i+1, *map(int, l)) for i, l in enumerate(curves)]
curves = [c for c in curves if c[1] == bits and (CUR is None or str(c[0]) in CUR.split(','))]

def popparity(v):
    v = v.astype(np.uint64)
    for s in (32,16,8,4,2,1): v ^= v >> np.uint64(s)
    return (v & np.uint64(1)).astype(bool)

# ---- membership masks over k in Z_r ----
def m_window(c, p, r, n, rng):
    w = max(1, round(n*p/r)); o = int(rng.integers(0, p))
    x = c // 2
    return ((x - o) % p) < w
def m_window_y(c, p, r, n, rng):
    # x-window of double width, one y-parity only: same size, NOT closed under negation
    w = max(1, round(2*n*p/r)); o = int(rng.integers(0, p)); par = int(rng.integers(0, 2))
    return ((((c // 2) - o) % p) < w) & ((c & 1) == par)
def m_affine(c, p, r, n, rng):
    S = max(1, round(r/n))
    while True:
        q = int(rng.integers(4*p, 8*p))
        if isprime(q): break
    a = int(rng.integers(1, q)); b = int(rng.integers(0, q)); t = int(rng.integers(0, S))
    return (((a*c + b) % q) % S) == t
def m_parity(c, nb, n, r, rng, weight):
    k = max(1, round(math.log2(r/n)))
    ok = np.ones(len(c), bool)
    for _ in range(k):
        if weight:
            pos = rng.choice(nb, size=weight, replace=False)
            mask = int(sum(1 << int(i) for i in pos))
        else:
            mask = 0
            while mask == 0: mask = int(rng.integers(0, 1 << nb))
        rhs = bool(rng.integers(0, 2))
        ok &= (popparity(c & mask) == rhs)
    return ok
def m_random(r, n, rng):
    mk = np.zeros(r, bool); mk[rng.choice(r, size=n, replace=False)] = True; return mk
def m_interval(r, n, rng):
    o = int(rng.integers(0, r)); mk = np.zeros(r, bool); mk[(o + np.arange(n)) % r] = True; return mk

def stats(X):
    X = np.rint(X)[1:]   # exclude target R=0 (negation-closed bases give X(0)=n)
    E = X.mean(); V = X.var()
    return dict(E=float(E), Var=float(V), P1=float((X > 0).mean()), P2=float((X >= 2).mean()))

def conv_indep(masks, r):
    F = None
    for mk in masks:
        f = sf.rfft(mk.astype(np.float64), workers=W)
        F = f if F is None else F*f
    return sf.irfft(F, n=r, workers=W)

def conv_shared(mk, r, m):
    idx = np.nonzero(mk)[0].astype(np.int64)
    Fs = []
    for j in range(1, m+1):
        g = np.zeros(r, np.float64); g[(idx*j) % r] = 1.0
        Fs.append(sf.rfft(g, workers=W))
    F1 = Fs[0]
    Xo = sf.irfft(F1**m, n=r, workers=W)
    if m == 2:   e = (F1**2 - Fs[1])/2
    elif m == 3: e = (F1**3 - 3*F1*Fs[1] + 2*Fs[2])/6
    elif m == 4: e = (F1**4 - 6*F1**2*Fs[1] + 3*Fs[1]**2 + 8*F1*Fs[2] - 6*Fs[3])/24
    return Xo, sf.irfft(e, n=r, workers=W)

rows = open(outp, 'a')
t0 = time.time()
for (ci, b, p, a, bb, r, gx, gy) in curves:
    tab = np.fromfile(f'{here}/tab_{b}_{ci}.bin', dtype=np.uint32).reshape(-1, 2).astype(np.int64)
    assert len(tab) == r
    c = 2*tab[:, 0] + (tab[:, 1] & 1)          # point code < 2p, distinguishes P and -P
    nb = bits + 1
    for rep in range(REPOFF, REPOFF+reps):
        for m in (2, 3, 4):
            for Enom in (1.0, 0.25):
                K = round((m-1)*math.log2(r) - math.log2(Enom))
                ks_ind = [K//m + (1 if i < K % m else 0) for i in range(m)]
                k_sh = round(K/m)
                for mode in ('indep', 'shared'):
                    ks = ks_ind if mode == 'indep' else [k_sh]*m
                    nom = [r/2**k for k in ks]
                    seed = np.random.SeedSequence([ci, rep, m, int(Enom*4), 0 if mode == 'indep' else 1])
                    rng = np.random.default_rng(seed)
                    def arm_masks(arm):
                        out = []
                        for i in range(1 if mode == 'shared' else m):
                            n = nom[i]
                            if arm == 'window': mk = m_window(c, p, r, n, rng)
                            elif arm == 'window_y': mk = m_window_y(c, p, r, n, rng)
                            elif arm == 'affine': mk = m_affine(c, p, r, n, rng)
                            elif arm == 'sparse5': mk = m_parity(c, nb, n, r, rng, 5)
                            elif arm == 'dense': mk = m_parity(c, nb, n, r, rng, 0)
                            elif arm == 'interval': mk = m_interval(r, round(n), rng)
                            elif arm in ('random', 'random2'): mk = m_random(r, max(1, round(n)), rng)
                            if arm not in ('interval',): mk = mk.copy(); mk[0] = False
                            out.append(mk)
                        return out*m if mode == 'shared' else out
                    def run(masks):
                        sizes = [int(x.sum()) for x in masks]
                        if mode == 'indep':
                            st = stats(conv_indep(masks, r)); st['sizes'] = sizes; return st
                        Xo, Xu = conv_shared(masks[0], r, m)
                        st = stats(Xu); so = stats(Xo)
                        st.update({'ord_'+k: v for k, v in so.items()}); st['sizes'] = sizes[:1]; return st
                    for arm in ARMS:
                        A = arm_masks(arm)
                        sa = run(A)
                        # paired random null: same realised sizes
                        if arm == 'random':
                            Bm = arm_masks('random2'); ref = 'random2_nominal'
                        else:
                            Bm = [m_random(r, s, rng) for s in sa['sizes']]
                            if mode == 'shared': Bm = Bm*m
                            ref = 'random_same_sizes'
                        sb = run(Bm)
                        rows.write(json.dumps(dict(bits=b, p=p, r=r, curve=ci, rep=rep, m=m, Enom=Enom, mode=mode, arm=arm,
                                                   ref=ref, arm_stats=sa, ref_stats=sb)) + '\n')
                    rows.flush()
        print(f'curve {ci} rep {rep} done t={time.time()-t0:.0f}s', flush=True)
