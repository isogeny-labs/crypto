#!/usr/bin/env python3
"""Wave-7 L3 census: exact order-r subgroup coverage of degree-2 iota_T-function bases (and the named
E' Huff / Jacobi-intersection coordinates) against the x-frame null. Frozen instrument (PROTOCOL.md).

  python3 census.py --k 16 --t 4 5 6 7 8 --K 2000 --outdir ../data/k16

Per frame: a random Möbius map nu (a,b,c,d), values V = nu([0, 2^t)), base B = {P : f(P) in V}, O removed.
Counts (exact, see curves.census): covered_D (mixed-sign pair sums P_i - P_j), covered_full (all signed pair
sums +-P_i +-P_j), J_D, J_full (log-free selectors), n_points, n_fibres, plus arm-specific columns.
Seeds: frame i of (k, t, arm) is the i-th draw of random.Random(seed(k, t, arm)), seed = first 64 bits of
sha256("wave7-L3|k=<k>|t=<t>|arm=<arm>").
"""
import argparse, hashlib, json, os, random, sys, time
import numpy as np
import curves as cv
from desk_checks import fit_mobius, apply_mobius

ARMS_E = ['E.x', 'E.x_noself', 'E.iota_even', 'E.iota_T0', 'E.iota_odd']
ARMS_EP = ['Ep.x', 'Ep.iota_2E', 'Ep.iota_c01', 'Ep.iota_c10', 'Ep.iota_c11',
           'Ep.huff_x', 'Ep.huff_y', 'Ep.ji_s', 'Ep.ji_c', 'Ep.ji_d']
ALL_ARMS = ARMS_E + ARMS_EP
EP_CLASSES = [(0, 1), (1, 0), (1, 1)]          # E'(F_p)/2E'(F_p) minus the trivial class


def seed_of(k, t, arm):
    return int(hashlib.sha256(f'wave7-L3|k={k}|t={t}|arm={arm}'.encode()).hexdigest()[:16], 16)


def draw_T(G, rng, cls):
    """A random T (idx) in a class. E: 'even' (2S, not O / 2-torsion, S returned), 'odd'.
    E': (0,0) via 2S, or a nonzero class (k mod 2, eps); T never O or 2-torsion except when fixed."""
    while True:
        if cls in ('even', (0, 0)):
            iS = rng.randrange(1, G.n)
            iT = int(G.addi(iS, iS))
        elif cls == 'odd':
            iS, iT = None, 2 * rng.randrange(G.M // 2) + 1
        else:
            iS = None
            iT = int(G.enc(2 * rng.randrange(G.M // 2) + cls[0], cls[1]))
        P = G.point(iT)
        if P is not None and P[1] != 0:
            return iT, iS


def census_keepO(G, base, shifts):
    """census() for a base that may contain O (control comparison only)."""
    base = np.unique(np.asarray(base, dtype=np.int64))
    if 0 not in base:
        return cv.census(G, base, shifts)
    M, m2 = G.M, G.m2
    K, Ee = base % M, base // M
    dk = (K[:, None] - K[None, :]) % M
    de = Ee[:, None] ^ Ee[None, :]
    mD = (de == 0) & (dk % m2 == 0) & (dk != 0)
    setD = np.unique(dk[mD])
    sh = []
    for T in shifts:
        kT, eT = int(T) % M, int(T) // M
        parts = [setD]
        for sgn in (1, -1):
            kk = (dk.ravel() + sgn * kT) % M
            ok = ((de.ravel() ^ eT) == 0) & (kk % m2 == 0) & (kk != 0)
            parts.append(kk[ok])
        sh.append(int(np.unique(np.concatenate(parts)).size))
    return dict(covered_D=int(setD.size), covered_shift=sh)


def covered_D_noself(G, base):
    """Positive-control statistic: covered_D of an x-frame base with the within-fibre pairs (P, -P) removed."""
    base = np.unique(base[base != 0])
    M, m2 = G.M, G.m2
    K, Ee = base % M, base // M
    dk = (K[:, None] - K[None, :]) % M
    de = Ee[:, None] ^ Ee[None, :]
    partner = ((K[:, None] + K[None, :]) % M == 0) & (de == 0)
    mD = (de == 0) & (dk % m2 == 0) & (dk != 0) & ~partner
    return int(np.unique(dk[mD]).size)


def control_check(G, Xf, iS, iT, base, pairs, V, res):
    """Pointwise translated-frame check (DESK.md Theorem 1): B - S negation-closed, h_T = M o x o tau_{-S}
    on every base point, and the census equals the x-frame census on frame M^{-1}(V) exactly."""
    p = G.p
    Q = G.subi(base, iS)
    if set(int(x) for x in Q) != set(int(x) for x in G.neg(Q)):
        return 0
    xs = [(int(G.X[int(G.subi(i, iS))]), lam) for i, lam in pairs if int(G.subi(i, iS)) != 0]
    if len(set(x for x, _ in xs)) < 3:
        return 2                                  # too few fibres to fit M (reported, not a failure)
    Mf = fit_mobius(xs, p)
    if Mf is None or any(apply_mobius(Mf, x, p) != lam for x, lam in xs):
        return 0
    a, b, c, d = Mf
    Vx, withO = [], False
    for lam in V:
        den = (-c * lam + a) % p
        if den == 0:
            withO = True
            continue
        Vx.append((d * lam - b) * pow(den, -1, p) % p)
    xb, _ = Xf.base(Vx)
    if withO:
        xb = np.concatenate([xb, [0]])
    rx = census_keepO(G, xb, [iT])
    ok = rx['covered_D'] == res['covered_D'] and rx['covered_shift'][0] == res['covered_full']
    return 1 if ok else 0


def run_arm(B, arm, t, K, funcs):
    k = B['k']
    p = B['p']
    G = B['GE'] if arm.startswith('E.') else B['GEp']
    rng = random.Random(seed_of(k, t, arm))
    cols = {c: [] for c in ('covered_D', 'covered_full', 'J_D', 'J_full', 'n_points', 'n_fibres')}
    extra = {}
    for i in range(K):
        F = cv.random_mobius(rng, p)
        V = cv.mobius_values(F, t, p)
        shifts, iS, iT, pairs = [], None, None, None
        if arm in ('E.x', 'E.x_noself'):
            base, nf = funcs['E.x'].base(V)
            if arm == 'E.x':
                shifts = [draw_T(G, rng, 'odd')[0]]
        elif arm == 'Ep.x':
            base, nf = funcs['Ep.x'].base(V)
            shifts = [draw_T(G, rng, c)[0] for c in EP_CLASSES]
        elif arm in ('Ep.huff_x', 'Ep.huff_y', 'Ep.ji_s', 'Ep.ji_c', 'Ep.ji_d'):
            base, nf = funcs[arm].base(V)
        else:
            if arm == 'E.iota_T0':
                iT, iS = G.index_of((0, 0)), G.M // 4      # S = 2r g, 2S = 4r g = T0
            else:
                cls = {'E.iota_even': 'even', 'E.iota_odd': 'odd', 'Ep.iota_2E': (0, 0),
                       'Ep.iota_c01': (0, 1), 'Ep.iota_c10': (1, 0), 'Ep.iota_c11': (1, 1)}[arm]
                iT, iS = draw_T(G, rng, cls)
            base, nf, pairs = cv.slope_base(G, G.point(iT), V)
        res = cv.census(G, base, shifts)
        for c in ('covered_D', 'covered_full', 'J_D', 'J_full', 'n_points'):
            cols[c].append(res[c])
        cols['n_fibres'].append(nf)
        if shifts:
            for j, v in enumerate(res['covered_shift']):
                extra.setdefault(f'covered_shift{j}', []).append(v)
        if arm == 'E.x_noself':
            extra.setdefault('covered_D_noself', []).append(covered_D_noself(G, base))
        if arm in ('E.iota_even', 'E.iota_T0', 'Ep.iota_2E'):
            extra.setdefault('control_ok', []).append(control_check(G, funcs['E.x' if arm.startswith('E.') else 'Ep.x'],
                                                                    iS, iT, base, pairs, V, res))
    cols.update(extra)
    return {c: np.array(v, dtype=np.int64) for c, v in cols.items()}


def fixed_draws(B, t, funcs):
    """Single named frames (not random): frame_search's bits_lo0 and edwards (calibration), Montgomery y/x bits,
    and the identity frame y in [0, 2^t) for each E' coordinate."""
    p, A = B['p'], B['A']
    A3 = A * pow(3, -1, p) % p
    out = {}
    ys = range(1 << t)
    GE, GEp = B['GE'], B['GEp']
    V = [(y - A3) % p for y in ys]                                   # Weierstrass x = y  <=>  u = y - A/3
    out['E.bits_lo0'] = cv.census(GE, funcs['E.x'].base(V)[0])
    V = [(1 + y) * pow((1 - y) % p, -1, p) % p for y in ys if (1 - y) % p]
    out['E.edwards_bits'] = cv.census(GE, funcs['E.x'].base(V)[0])
    out['E.y_over_x_bits'] = cv.census(GE, cv.slope_base(GE, (0, 0), list(ys))[0])
    for name in ('Ep.x', 'Ep.huff_x', 'Ep.huff_y', 'Ep.ji_s', 'Ep.ji_c', 'Ep.ji_d'):
        out[name + '_bits'] = cv.census(GEp, funcs[name].base(list(ys))[0])
    for v in out.values():
        v.pop('covered_shift', None)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--k', type=int, required=True)
    ap.add_argument('--t', type=int, nargs='+', default=[4, 5, 6, 7, 8])
    ap.add_argument('--K', type=int, default=2000)
    ap.add_argument('--arms', nargs='+', default=ALL_ARMS)
    ap.add_argument('--outdir', required=True)
    a = ap.parse_args()
    os.makedirs(a.outdir, exist_ok=True)
    t0 = time.time()
    B = cv.build(a.k)
    funcs = {'E.x': cv.FixedFunc(cv.coord_x(B['GE'])), 'Ep.x': cv.FixedFunc(cv.coord_x(B['GEp']))}
    for w in 'xy':
        funcs['Ep.huff_' + w] = cv.FixedFunc(cv.coord_huff(B, w))
    for w in 'scd':
        funcs['Ep.ji_' + w] = cv.FixedFunc(cv.coord_ji(B, w))
    meta = dict(k=a.k, p=B['p'], A=B['A'], n=B['n'], r=B['r'], K=a.K, t=a.t, arms=a.arms,
                build_s=round(time.time() - t0, 2), timing={}, fixed={},
                shift_columns={'E.x': ['odd'], 'Ep.x': [str(c) for c in EP_CLASSES]})
    for t in a.t:
        meta['fixed'][str(t)] = fixed_draws(B, t, funcs)
    for arm in a.arms:
        data = {}
        for t in a.t:
            ts = time.time()
            res = run_arm(B, arm, t, a.K, funcs)
            for c, v in res.items():
                data[f't{t}_{c}'] = v.astype(np.int32)
            meta['timing'][f'{arm}|t={t}'] = round(time.time() - ts, 2)
            print(f'k={a.k} {arm} t={t} {meta["timing"][f"{arm}|t={t}"]}s', file=sys.stderr, flush=True)
        np.savez_compressed(os.path.join(a.outdir, f'{arm}.npz'), **data)
    meta['total_s'] = round(time.time() - t0, 2)
    import resource
    meta['maxrss_MB'] = round(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 1024, 1)
    meta['cpu_s'] = round(resource.getrusage(resource.RUSAGE_SELF).ru_utime + resource.getrusage(resource.RUSAGE_SELF).ru_stime, 2)
    json.dump(meta, open(os.path.join(a.outdir, 'meta.json'), 'w'), indent=1)


if __name__ == '__main__':
    main()
