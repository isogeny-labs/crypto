"""BKK (mixed volume) root bound of each arm's square subsystem: Semaev part + base equations for
x_1..x_{m-1}. Segments [0, L e_i] factor out: MV = L^(m-1) * MV(projections onto (x_m, aux)).
Also the Bezout bound L^(m-1) * prod(total degrees)."""
import sys, itertools, json
sys.path.insert(0, '/dev/shm/pylib'); sys.path.insert(0, '/dev/shm/bkk/code')
import numpy as np
from scipy.spatial import ConvexHull, QhullError
from bkklib import *

def vol(points, k):
    P = np.array(sorted(set(points)), dtype=float)
    if k == 1:
        return float(P[:, 0].max() - P[:, 0].min())
    if len(P) <= k or np.linalg.matrix_rank(P - P[0]) < k:
        return 0.0
    try:
        return float(ConvexHull(P).volume)
    except QhullError:
        return 0.0

def mink(A, B):
    return list({tuple(a + b for a, b in zip(x, y)) for x in A for y in B})

def hull_pts(P, k):
    P = sorted(set(P))
    if k == 1 or len(P) <= k + 1:
        return P
    arr = np.array(P, dtype=float)
    if np.linalg.matrix_rank(arr - arr[0]) < k:
        return P
    h = ConvexHull(arr)
    return [P[i] for i in h.vertices]

def mixed_volume(polys, k):
    tot = 0.0
    for r in range(1, k + 1):
        for S in itertools.combinations(range(k), r):
            acc = [tuple([0] * k)]
            for i in S:
                acc = hull_pts(mink(acc, polys[i]), k)
            tot += (-1) ** (k - r) * vol(acc, k)
    return tot  # normalised so that MV(simplex,...,simplex) = 1 (Bernstein count)

def arm_bkk(arm, m, inst):
    eqs, order = semaev_part(arm, m, inst['p'], inst['a'], inst['b'], inst['xR'])
    keep = [v for v in order if v not in [f'x{i}' for i in range(1, m)]]  # x_m + aux
    k = len(keep)
    assert k == len(eqs), (arm, keep, len(eqs))
    polys = []
    tdeg = []
    for d, vs in eqs:
        pts = []
        for mono in d:
            e = dict(zip(vs, mono))
            pts.append(tuple(e.get(v, 0) for v in keep))
        polys.append(hull_pts(pts, k))
        tdeg.append(max(sum(mm) for mm in d))
    mv = mixed_volume(polys, k)
    if k == 1:
        mv = vol(polys[0], 1)
    L = inst['L']
    bez = 1
    for t in tdeg: bez *= t
    return dict(arm=arm, m=m, nvars=len(order), k=k, MV_proj=round(mv, 6), BKK=round(mv) * L ** (m - 1),
                Bezout=bez * L ** (m - 1), tdeg=tdeg, L=L)

if __name__ == '__main__':
    out = []
    for m in (3, 4):
        inst = make_instance(m, 12, 0)
        for arm in ARMS[m]:
            r = arm_bkk(arm, m, inst)
            out.append(r); print(json.dumps(r), flush=True)
    # BKK pick per m
    for m in (3, 4):
        rows = [r for r in out if r['m'] == m and r['arm'] != 'D']
        order = ARMS[m]
        best = min(rows, key=lambda r: (r['BKK'], r['nvars'], order.index(r['arm'])))
        print(json.dumps({'m': m, 'BKK_pick': best['arm'], 'all_alt_BKK_equal_to_D':
                          all(r['BKK'] == [x for x in out if x['m'] == m and x['arm'] == 'D'][0]['BKK'] for r in rows)}))
