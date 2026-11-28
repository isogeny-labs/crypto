#!/usr/bin/env python3
"""Validator's independent reduced census replay at k = 16 (own frames, own bases, own coverage counting).
Arms: E.x (null, with one odd-class T shift), E.iota_odd (no-half), Ep.huff_x, Ep.x; plus E.iota_even with an
own pointwise Theorem-1 check. Compares means with the recorded npz cells."""
import hashlib, json, math, sys, time
import numpy as np
from vcore import *

KP = int(sys.argv[2]) if len(sys.argv) > 2 else 1000
import os
SALT = os.environ.get('SALT', '')
ARMS_RUN = os.environ.get('ARMS', 'E.x,E.iota_odd,Ep.huff_x,Ep.x').split(',')
TS_RUN = [int(x) for x in os.environ.get('TS', '4,5,6,7,8').split(',')]
DO_CTRL = os.environ.get('CTRL', '1') == '1'
ROOT = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L3_translated_involution'
p, A, n, r, E = build(16)
G = Group(E, n, r, False)
al, be = (A + 2) % p, (A - 2) % p
Ep = Curve(p, -(al + be), al * be, 0)
Gp = Group(Ep, n, r, True)
LL = np.arange(n)


def tables_huff():
    es = [0, al, be]
    e0 = [e for e in es if all(leg(e - o, p) == 1 for o in es if o != e)][0]
    ea, eb = [e for e in es if e != e0]
    a, b = sqrtp(e0 - ea, p), sqrtp(e0 - eb, p)
    X, Y = Gp.X, Gp.Y
    u = (X - e0) % p
    val = b * ((u + a * a) % p) % p * ninv(np.where(Y == 0, 1, Y), p) % p
    val = np.where(Y == 0, -1, val)
    for L in np.nonzero((Y == 0) & (X >= 0))[0]:
        if int(u[L]) == (-a * a) % p:
            val[L] = 0
    val[0] = -1
    return val


VX = G.X.copy(); VX[0] = -1
VXp = Gp.X.copy(); VXp[0] = -1
VH = tables_huff()


def h_table(Gr, C, LT):
    """h_T(P) = slope of the line through -T and P, for every label; -1 at the poles O and T."""
    xT, yT = Gr.pt(LT)
    X, Y = Gr.X, Gr.Y
    den = (X - xT) % p
    v = (Y + yT) % p * ninv(np.where(den == 0, 1, den), p) % p
    Ln = int(Gr.neg(LT))
    v[Ln] = (3 * xT * xT + 2 * C.a2 * xT + C.a4) * pow((-2 * yT) % p, -1, p) % p
    v[LT] = -1
    v[0] = -1
    return v


def frameV(g, t):
    while True:
        a, b, c, d = (int(z) for z in g.integers(0, p, 4))
        if (a * d - b * c) % p:
            break
    ys = np.arange(1 << t, dtype=np.int64)
    den = (c * ys + d) % p
    ok = den != 0
    return np.unique((a * ys[ok] + b) % p * ninv(den[ok], p) % p), (a, b, c, d)


def census(Gr, B, shift=None):
    B = np.unique(B[B != 0])
    K, Ee = Gr.ke(B)
    M, m2 = Gr.M, Gr.m2
    dk = (K[:, None] - K[None, :]) % M
    same = Ee[:, None] == Ee[None, :]
    Dset = np.unique(dk[same & (dk % m2 == 0) & (dk != 0)])
    sk = (K[:, None] + K[None, :]) % M
    Sset = np.unique(sk[same & (sk % m2 == 0) & (sk != 0)])
    full = np.unique(np.concatenate([Dset, Sset, (-Sset) % M]))
    out = dict(D=int(Dset.size), full=int(full.size), npts=int(B.size))
    if shift is not None:
        kT, eT = Gr.ke(shift)
        de = Ee[:, None] ^ Ee[None, :]
        parts = [Dset]
        for s in (1, -1):
            kk = (dk + s * int(kT)) % M
            m = ((de ^ int(eT)) == 0) & (kk % m2 == 0) & (kk != 0)
            parts.append(kk[m])
        out['shift'] = int(np.unique(np.concatenate(parts)).size)
    return out


def pointwise_D(Gr, C, B):
    """Log-free covered_D by curve arithmetic: distinct R = P_i - P_j with [r]R = O, R != O."""
    pts = [Gr.pt(L) for L in np.unique(B[B != 0])]
    S = set()
    for P in pts:
        for Q in pts:
            R = C.sub(P, Q)
            if R is not None and R not in S and C.mul(r, R) is None:
                S.add(R)
    return len(S)


def seed(name):
    return int(hashlib.blake2b(('validator-L3-replay|' + SALT + name).encode(), digest_size=8).hexdigest(), 16)


res = {}
spot = dict(n=0, ok=0)
t0 = time.time()
for t in TS_RUN:
    for arm in ARMS_RUN:
        g = np.random.Generator(np.random.PCG64(seed(f'{arm}|{t}')))
        cols = {c: [] for c in ('D', 'full', 'npts', 'shift')}
        for i in range(KP):
            V, F = frameV(g, t)
            if arm == 'E.x':
                B = LL[np.isin(VX, V)]
                LT = 2 * int(g.integers(0, n // 2)) + 1
                c = census(G, B, LT)
                Gr, C = G, E
            elif arm == 'E.iota_odd':
                LT = 2 * int(g.integers(0, n // 2)) + 1
                B = LL[np.isin(h_table(G, E, LT), V)]
                c = census(G, B)
                Gr, C = G, E
            elif arm == 'Ep.huff_x':
                B = LL[np.isin(VH, V)]
                c = census(Gp, B)
                Gr, C = Gp, Ep
            else:
                B = LL[np.isin(VXp, V)]
                c = census(Gp, B)
                Gr, C = Gp, Ep
            for kk, v in c.items():
                cols[kk].append(v)
            if t in (4, 5) and i < 6:
                spot['n'] += 1
                spot['ok'] += pointwise_D(Gr, C, B) == c['D']
        rec = np.load(f'{ROOT}/data/k16/{arm}.npz')
        cell = {}
        for kk, rk in (('D', 'covered_D'), ('full', 'covered_full'), ('npts', 'n_points'), ('shift', 'covered_shift0')):
            if not cols[kk]:
                continue
            a = np.array(cols[kk], float)
            b = rec[f't{t}_{rk}'].astype(float)
            se = math.sqrt(a.var(ddof=1) / a.size + b.var(ddof=1) / b.size)
            cell[kk] = dict(mine=round(a.mean(), 3), recorded=round(b.mean(), 3), z=round((a.mean() - b.mean()) / se, 2),
                            ratio=round(a.mean() / b.mean(), 4))
        res[f'16|{t}|{arm}'] = cell
        print(t, arm, {k_: (v['mine'], v['recorded'], v['z']) for k_, v in cell.items()}, round(time.time() - t0, 1), flush=True)

# control arm E.iota_even: own pointwise Theorem-1 check per frame (fit on 3 curve points, verify on ALL points)
ctrl = dict(frames=0, ok=0, viol=[])
for t in ((4, 5, 6) if DO_CTRL else ()):
    g = np.random.Generator(np.random.PCG64(seed(f'E.iota_even|{t}')))
    Ds = []
    for i in range(300):
        V, F = frameV(g, t)
        while True:
            Ls = int(g.integers(1, n))
            LT = int(G.add(Ls, Ls))
            if LT != 0 and G.pt(LT)[1] != 0:
                break
        hv = h_table(G, E, LT)
        Q = G.sub(LL, Ls)                         # label of P - S
        xq = G.X[Q]                               # x(P - S); -1 when P = S
        ok_pts = (hv >= 0) & (xq >= 0)
        idx = np.nonzero(ok_pts)[0]
        pr, seen = [], set()
        for L in idx[:50]:
            if int(xq[L]) not in seen:
                seen.add(int(xq[L]))
                pr.append((int(xq[L]), int(hv[L])))
            if len(pr) == 3:
                break
        mu = fit_mobius(pr, p)
        a_, b_, c_, d_ = mu
        den = (c_ * xq[idx] + d_) % p
        pred = np.where(den == 0, -2, (a_ * xq[idx] + b_) % p * ninv(np.where(den == 0, 1, den), p) % p)
        okfit = bool(np.all(pred == hv[idx]))
        B = LL[np.isin(hv, V)]
        Bs = G.sub(B, Ls)
        negc = set(Bs.tolist()) == set(G.neg(Bs).tolist())
        mi = mobius_inv(mu, p)
        Vx = []
        withO = False
        for v in V.tolist():
            u = mobius(mi, v, p)
            if u is None:
                withO = True
            else:
                Vx.append(u)
        Bx = LL[np.isin(VX, np.array(Vx, dtype=np.int64))]
        if withO:
            Bx = np.concatenate([Bx, [0]])
        c1 = census(G, B)
        # x-frame side may contain O (label 0): count with O kept
        K = np.unique(Bx)
        dk = (K[:, None] - K[None, :]) % n
        Dx = np.unique(dk[(dk % 8 == 0) & (dk != 0)])
        parts = [Dx]
        for s in (1, -1):
            kk = (dk + s * LT) % n
            parts.append(kk[(kk % 8 == 0) & (kk != 0)])
        shx = int(np.unique(np.concatenate(parts)).size)
        ok = okfit and negc and c1['D'] == Dx.size and c1['full'] == shx
        ctrl['frames'] += 1
        ctrl['ok'] += ok
        if not ok:
            ctrl['viol'].append((t, i, okfit, negc, c1['D'], int(Dx.size), c1['full'], shx))
        Ds.append(c1['D'])
    rec = np.load(f'{ROOT}/data/k16/E.iota_even.npz')[f't{t}_covered_D'].astype(float)
    a = np.array(Ds, float)
    se = math.sqrt(a.var(ddof=1) / a.size + rec.var(ddof=1) / rec.size)
    res[f'16|{t}|E.iota_even'] = dict(D=dict(mine=round(a.mean(), 3), recorded=round(rec.mean(), 3),
                                             z=round((a.mean() - rec.mean()) / se, 2)))
    print(t, 'E.iota_even', res[f'16|{t}|E.iota_even'], ctrl['ok'], '/', ctrl['frames'], flush=True)

zs = [v['z'] for cell in res.values() for v in cell.values()]
out = dict(K_per_cell=KP, cells=res, spot_pointwise=spot, control_theorem1=ctrl, max_abs_z=max(abs(z) for z in zs),
           n_z=len(zs), n_abs_z_gt_2=sum(abs(z) > 2 for z in zs), n_abs_z_gt_3=sum(abs(z) > 3 for z in zs),
           seconds=round(time.time() - t0, 1))
json.dump(out, open(sys.argv[1], 'w'), indent=1, default=lambda o: o.item() if hasattr(o, 'item') else str(o))
print(json.dumps({k: v for k, v in out.items() if k != 'cells'}, indent=1, default=lambda o: o.item() if hasattr(o, 'item') else str(o)))
