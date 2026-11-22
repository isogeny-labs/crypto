#!/usr/bin/env python3
"""L2 part (a): exact m = 2 coverage census of f-frame bases against x-frames.

For each frame of arm f: V = [L, L + 2^t) mod p, L uniform (derived seed),
base = {P : f(P) in V}, classes = base folded by +-.
Per frame we record:
  f  : values 2^t, points |base|, cols, covered, J
  xm : a Weierstrass-x frame with the SAME number of cols (first `cols` x-values
       on the curve from a uniform start L'), covered, J        (per-column null)
  xv : a Weierstrass-x frame with the SAME |V| = 2^t, cols, covered  (per-value null)
Arms:
  test     v q uv xeye jac6              (degree >= 3 chart functions)
  neg ctrl ye xe (degree 2), rand (uniform random classes), xAA (x-frames, |V| = 2^(t+1))
  pos ctrl even: x-frame restricted to P in 2E (even log), cols matched
covered = exact distinct R != O in E[r] with R = +-P_i +-P_j (i = j allowed),
the frame-search instrument (common.coverage).
"""
import argparse, json, sys, time
import numpy as np
import common as C_

ARMS_TEST = ['v', 'q', 'uv', 'xeye', 'jac6']
ARMS_CTRL = ['ye', 'xe', 'rand', 'xAA', 'even']


def xframe_first(XT, L, c, n, even=False, logsx=None):
    """classes of the first c x-values (cyclically from L) carrying points
    (even=True: only x-values whose points have even log)."""
    if not even:
        xs = XT.first_values(L, c)
    else:
        i = np.searchsorted(XT.ev_uvals, L % XT.p)
        xs = XT.ev_uvals[(i + np.arange(c)) % XT.ev_uvals.size]
    # one class per x value
    idx = np.searchsorted(XT.val, xs)
    return C_.fold(XT.log[idx], n)


def run_cell(C, tables, arm, t, K, XT):
    n, p = C.n, C.p
    rng = np.random.default_rng(C_.dseed(C.k, arm, t))
    rows = np.zeros((K, 10), dtype=np.int64)  # values pts cols cov J | xm_cov xm_J | xv_cols xv_cov xv_J
    for i in range(K):
        L = int(rng.integers(p)); Lm = int(rng.integers(p)); Lv = int(rng.integers(p))
        if arm == 'rand':
            c = int(round(2 ** t))
            cls = set()
            while len(cls) < c:
                l = int(rng.integers(1, n))
                if l != n // 2:  # skip T0 (no subgroup sums of interest)
                    cls.add(min(l, n - l))
            cls = np.array(sorted(cls)); pts = 2 * c; vals = 0
        elif arm == 'xAA':
            lg = XT.interval(L, 2 ** (t + 1)); cls = C_.fold(lg, n); pts = lg.size; vals = 2 ** (t + 1)
        elif arm == 'even':
            c = int(round(2 ** t))
            cls = xframe_first(XT, L, c, n, even=True); pts = 2 * c; vals = 0
        else:
            T = tables[arm]
            lg = T.interval(L, 2 ** t); cls = C_.fold(lg, n); pts = lg.size; vals = 2 ** t
        cov, cols, J = C_.coverage(cls, n)
        xm = xframe_first(XT, Lm, cols, n) if cols else np.array([], dtype=np.int64)
        xm_cov, _, xm_J = C_.coverage(xm, n)
        xv = C_.fold(XT.interval(Lv, 2 ** t), n)
        xv_cov, xv_cols, xv_J = C_.coverage(xv, n)
        rows[i] = (vals, pts, cols, cov, J, xm_cov, xm_J, xv_cols, xv_cov, xv_J)
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--k', type=int, nargs='+', default=[16, 18, 20])
    ap.add_argument('--t', type=int, nargs='+', default=[4, 5, 6, 7])
    ap.add_argument('--arms', nargs='+', default=ARMS_TEST + ARMS_CTRL)
    ap.add_argument('--K', type=int, default=2000)
    ap.add_argument('--out', required=True, help='output directory')
    ap.add_argument('--pc1', action='store_true', help='run positive control PC1 (frame-search replay, k = 20)')
    a = ap.parse_args()
    t0 = time.time()
    meta = {}
    for k in a.k:
        C = C_.Curve(k)
        XT = C_.FTable(C, 'x')
        XT.ev_uvals = np.unique(XT.val[(XT.log % 2) == 0])
        tables = {f: C_.FTable(C, f) for f in a.arms if f in C_.FUNCS}
        meta[k] = dict(facts=C.facts, p=C.p, A=C.A, n=C.n, r=C.r)
        print(f'k={k} tables built {time.time()-t0:.1f}s', file=sys.stderr)
        if a.pc1 and k == 20:
            # Frame-search replay: Weierstrass x in [L, L + 2^t), published exact counts.
            pc = {}
            for (L, t, want) in [(0, 5, 42), (0, 6, 162), (976503, 5, 198), (908172, 6, 536)]:
                cov, cols, J = C_.coverage(C_.fold(XT.interval(L, 2 ** t), C.n), C.n)
                pc[f'L{L}_t{t}'] = dict(covered=cov, cols=cols, J=J, published=want, match=cov == want)
            meta['PC1_replay'] = pc
            print('PC1', pc, file=sys.stderr)
        for arm in a.arms:
            for t in a.t:
                rows = run_cell(C, tables, arm, t, a.K, XT)
                np.savez_compressed(f'{a.out}/census_k{k}_{arm}_t{t}.npz', rows=rows.astype(np.int32),
                                    seed=C_.dseed(k, arm, t))
                print(f'k={k} {arm} t={t} done {time.time()-t0:.0f}s mean cov f {rows[:,3].mean():.1f} xm {rows[:,5].mean():.1f}', file=sys.stderr)
    json.dump(meta, open(f'{a.out}/census_meta.json', 'w'), indent=1, default=int)


if __name__ == '__main__':
    main()
