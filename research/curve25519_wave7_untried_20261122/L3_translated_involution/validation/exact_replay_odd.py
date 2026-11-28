#!/usr/bin/env python3
"""Frame-exact replay of recorded k=16 E.iota_odd (and E' class arms) cells. The study's generator convention
(random.Random(1), x = randrange(p), its square-root branch for p = 5 mod 8, y sign by random() < 0.5) is re-implemented
here only to map the recorded T labels to points; h_T bases (pointwise slopes, not line-cubic solving) and coverage
are the validator's own."""
import hashlib, json, random, sys
import numpy as np
from vcore import *
ROOT = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L3_translated_involution'
p, A, n, r, E = build(16)
G = Group(E, n, r, False)
al, be = (A + 2) % p, (A - 2) % p
Ep = Curve(p, -(al + be), al * be, 0)
Gp = Group(Ep, n, r, True)
LL = np.arange(n)

def sqrt58(a):
    a %= p
    if a == 0: return 0
    if pow(a, (p - 1) // 2, p) != 1: return None
    x = pow(a, (p + 3) // 8, p)
    if x * x % p != a: x = x * pow(2, (p - 1) // 4, p) % p
    return x

def their_gen(C, M, noncyc_two=None):
    rng = random.Random(1)
    while True:
        while True:
            x = rng.randrange(p); y = sqrt58(C.f(x))
            if y is not None and y != 0:
                g = (x, y if rng.random() < 0.5 else (-y) % p); break
        if C.mul(M, g) is None and all(C.mul(M // q, g) is not None for q in factor(M)):
            return g

gE = their_gen(E, n)
gP = their_gen(Ep, n // 2)
# their h on E': first 2-torsion in [(0,0),(al,0),(be,0)] different from (M/2) g
gm2 = Ep.mul(n // 4, gP)
hP = [P for P in [(0, 0), (al, 0), (be, 0)] if P != gm2][0]

def h_table(Gr, C, LT):
    xT, yT = Gr.pt(LT)
    X, Y = Gr.X, Gr.Y
    den = (X - xT) % p
    v = (Y + yT) % p * ninv(np.where(den == 0, 1, den), p) % p
    v[int(Gr.neg(LT))] = (3 * xT * xT + 2 * C.a2 * xT + C.a4) * pow((-2 * yT) % p, -1, p) % p
    v[LT] = -1; v[0] = -1
    return v

def census(Gr, B):
    B = np.unique(B[B != 0]); K, Ee = Gr.ke(B); M, m2 = Gr.M, Gr.m2
    dk = (K[:, None] - K[None, :]) % M; same = Ee[:, None] == Ee[None, :]
    Dset = np.unique(dk[same & (dk % m2 == 0) & (dk != 0)])
    sk = (K[:, None] + K[None, :]) % M
    Sset = np.unique(sk[same & (sk % m2 == 0) & (sk != 0)])
    return int(Dset.size), int(np.unique(np.concatenate([Dset, Sset, (-Sset) % M])).size), int(B.size)

def seed_of(k, t, arm):
    return int(hashlib.sha256(f'wave7-L3|k={k}|t={t}|arm={arm}'.encode()).hexdigest()[:16], 16)

out = {}
for arm in ('E.iota_odd', 'Ep.iota_c01'):
    for t in (4, 5, 6, 7, 8):
        rng = random.Random(seed_of(16, t, arm))
        rec = np.load(f'{ROOT}/data/k16/{arm}.npz')
        RD, RF, RN = (rec[f't{t}_{c}'] for c in ('covered_D', 'covered_full', 'n_points'))
        match, bad = 0, None
        for i in range(2000):
            while True:
                F = tuple(rng.randrange(p) for _ in range(4))
                if (F[0] * F[3] - F[1] * F[2]) % p: break
            a, b, c, d = F
            V = np.array(sorted(set((a*y+b) * pow((c*y+d) % p, -1, p) % p for y in range(1 << t) if (c*y+d) % p)), dtype=np.int64)
            if arm == 'E.iota_odd':
                kT = 2 * rng.randrange(n // 2) + 1
                T = E.mul(kT, gE); LT = G.label(T); Gr, C = G, E
            else:
                while True:
                    kT = 2 * rng.randrange(n // 4) + 0
                    T = Ep.add(Ep.mul(kT, gP), hP)
                    if T is not None and T[1] != 0: break
                LT = Gp.label(T); Gr, C = Gp, Ep
            got = census(Gr, LL[np.isin(h_table(Gr, C, LT), V)])
            ok = got == (int(RD[i]), int(RF[i]), int(RN[i]))
            match += ok
            if not ok and bad is None: bad = (i, got, (int(RD[i]), int(RF[i]), int(RN[i])))
        out[f'16|{t}|{arm}'] = dict(frames=2000, exact_match=match, first_mismatch=bad)
        print(arm, t, match, '/ 2000', bad, flush=True)
json.dump(out, open(sys.argv[1], 'w'), indent=1)
