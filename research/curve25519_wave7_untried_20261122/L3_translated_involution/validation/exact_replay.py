#!/usr/bin/env python3
"""Frame-exact replay of recorded k=16 cells: the protocol's seed rule (sha256 'wave7-L3|k=|t=|arm=', random.Random)
and draw order (nu = 4 x randrange(p), redrawn if singular; then the arm's T draws) are mirrored; bases and coverage
are computed with the validator's own code (vcore), not the study's."""
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
desk = json.load(open(f'{ROOT}/data/desk_k16.json'))['huff_params']   # recorded a, b (sign convention only)


def huff_table(a, b, e0, ea):
    X, Y = Gp.X, Gp.Y
    u = (X - e0) % p
    val = b * ((u + a * a) % p) % p * ninv(np.where(Y == 0, 1, Y), p) % p
    val = np.where(Y == 0, -1, val)
    for L in np.nonzero((Y == 0) & (X >= 0))[0]:
        if int(u[L]) == (-a * a) % p:
            val[L] = 0
    val[0] = -1
    return val


es = [0, al, be]
e0 = [e for e in es if all(leg(e - o, p) == 1 for o in es if o != e)][0]
ea = 0 if e0 != 0 else min(al, be)
a_own, b_own = sqrtp(e0 - ea, p), sqrtp(e0 - [e for e in es if e not in (e0, ea)][0], p)
assert {a_own, p - a_own} >= {desk['a']} or a_own in (desk['a'], p - desk['a'])
VH = huff_table(desk['a'], desk['b'], e0, ea)
VX = G.X.copy(); VX[0] = -1
VXp = Gp.X.copy(); VXp[0] = -1


def census(Gr, B):
    B = np.unique(B[B != 0])
    K, Ee = Gr.ke(B)
    M, m2 = Gr.M, Gr.m2
    dk = (K[:, None] - K[None, :]) % M
    same = Ee[:, None] == Ee[None, :]
    Dset = np.unique(dk[same & (dk % m2 == 0) & (dk != 0)])
    sk = (K[:, None] + K[None, :]) % M
    Sset = np.unique(sk[same & (sk % m2 == 0) & (sk != 0)])
    return int(Dset.size), int(np.unique(np.concatenate([Dset, Sset, (-Sset) % M])).size), int(B.size)


def seed_of(k, t, arm):
    return int(hashlib.sha256(f'wave7-L3|k={k}|t={t}|arm={arm}'.encode()).hexdigest()[:16], 16)


out = {}
for arm in ('Ep.huff_x', 'E.x', 'Ep.x'):
    for t in (4, 5, 6, 7, 8):
        rng = random.Random(seed_of(16, t, arm))
        rec = np.load(f'{ROOT}/data/k16/{arm}.npz')
        RD, RF, RN = (rec[f't{t}_{c}'] for c in ('covered_D', 'covered_full', 'n_points'))
        match = 0
        first_bad = None
        for i in range(2000):
            while True:
                F = tuple(rng.randrange(p) for _ in range(4))
                if (F[0] * F[3] - F[1] * F[2]) % p:
                    break
            a, b, c, d = F
            V = np.array(sorted(set((a * y + b) * pow((c * y + d) % p, -1, p) % p for y in range(1 << t) if (c * y + d) % p)),
                         dtype=np.int64)
            if arm == 'E.x':
                rng.randrange(n // 2)                    # the odd shift T draw (1 call; odd T is never 2-torsion)
                cD, cF, cN = census(G, LL[np.isin(VX, V)])
            elif arm == 'Ep.x':
                for _ in range(3):
                    rng.randrange((n // 2) // 2)         # one T per nonzero class (retry only if 2-torsion: rare)
                cD, cF, cN = census(Gp, LL[np.isin(VXp, V)])
            else:
                cD, cF, cN = census(Gp, LL[np.isin(VH, V)])
            ok = (cD, cF, cN) == (int(RD[i]), int(RF[i]), int(RN[i]))
            match += ok
            if not ok and first_bad is None:
                first_bad = (i, (cD, cF, cN), (int(RD[i]), int(RF[i]), int(RN[i])))
        out[f'16|{t}|{arm}'] = dict(frames=2000, exact_match=match, first_mismatch=first_bad,
                                    recorded_mean_D=float(RD.mean()))
        print(arm, t, match, '/ 2000', first_bad, flush=True)
json.dump(out, open(sys.argv[1], 'w'), indent=1)
