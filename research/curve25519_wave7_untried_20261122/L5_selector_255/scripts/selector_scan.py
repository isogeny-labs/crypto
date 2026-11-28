#!/usr/bin/env python3
"""Frame-selector census vs t on a family miniature (PROTOCOL.md, arm F).

For each t: N random translation frames (1, L, 0, 1), L uniform in [0, p) from a derived seed, plus the
fixed frame bits_lo0 (L = 0). Per frame: L, cols c, folded class counts m0..m4, J, S1, covered (exact,
only when t <= tcov), and a 2-torsion flag. Also an iid surrogate per frame (negative/random control):
c_s ~ Binomial(2^t, rho_hat), (m_s) ~ Multinomial(c_s; 1/8, 1/4, 1/4, 1/4, 1/8), J_s from the counts.
Counted selector cost: [r]P by double-and-add, instrumented, on a derived sample of base points per t,
with the label checked against log mod 8.
Output: one compressed .npz per (curve, t) plus a JSON cell summary line on stdout.
"""
import argparse, json, os, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from l5common import (CURVES, derived_seed, build, frame_logs_translate, class_counts, J_from_counts,
                      S1_from_counts, J_exact, covered_exact)

PROBS = np.array([1 / 8, 1 / 4, 1 / 4, 1 / 4, 1 / 8])   # folded classes 0, {1,7}, {2,6}, {3,5}, 4

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--curve', required=True, choices=list(CURVES))
    ap.add_argument('--t', type=int, nargs='+', required=True)
    ap.add_argument('--N', type=int, default=4000)
    ap.add_argument('--tcov', type=int, default=10, help='compute exact covered only for t <= tcov')
    ap.add_argument('--cost-sample', type=int, default=200)
    ap.add_argument('--outdir', required=True)
    a = ap.parse_args()
    k, p, A, n = CURVES[a.curve]
    os.makedirs(a.outdir, exist_ok=True)
    t0 = time.time()
    logu, r, g, add, mul = build(p, A, n)
    A3 = A * pow(3, -1, p) % p
    print(json.dumps(dict(event='log_table', curve=a.curve, r=r, seconds=round(time.time() - t0, 1))), flush=True)
    # E[8] labels for the counted cost check: T = [r]g has order 8; [r]P = [log P mod 8] T
    T = mul(r, g); E8 = {}
    Q = None
    for i in range(8):
        E8[Q] = i; Q = add(Q, T)
    for t in a.t:
        tc = time.time()
        rng = np.random.default_rng(derived_seed('frames', a.curve, t))
        Ls = np.concatenate([[0], rng.integers(0, p, size=a.N)]).astype(np.int64)   # index 0 = bits_lo0
        rows = np.zeros((a.N + 1, 10), dtype=np.int64)   # L c m0 m1 m2 m3 m4 J S1 covered
        tors = np.zeros(a.N + 1, dtype=np.int8)
        for i, L in enumerate(Ls):
            logs = frame_logs_translate(p, A3, logu, int(L), t)
            m = class_counts(logs)
            if np.any(logs == n // 2):            # 2-torsion column: use the exact numpy J
                tors[i] = 1; J = J_exact(logs, n)
            else:
                J = J_from_counts(m)
            cov = covered_exact(logs, n) if t <= a.tcov else -1
            rows[i] = [L, len(logs), *m, J, S1_from_counts(m), cov]
        # iid surrogate (random control)
        srng = np.random.default_rng(derived_seed('surrogate', a.curve, t))
        rho = rows[1:, 1].mean() / (1 << t)
        cs = srng.binomial(1 << t, rho, size=a.N + 1)
        ms = np.array([srng.multinomial(c, PROBS) for c in cs])
        Js = np.array([J_from_counts(mm) for mm in ms])
        # counted selector cost on a derived sample of base points
        crng = np.random.default_rng(derived_seed('cost', a.curve, t))
        ops_list, ok = [], 0
        for fi in crng.integers(1, a.N + 1, size=a.cost_sample):
            logs = frame_logs_translate(p, A3, logu, int(Ls[fi]), t)
            if len(logs) == 0: continue
            lg = int(logs[crng.integers(0, len(logs))])
            P = mul(lg, g)
            cnt = [0]
            def add_c(X, Y):
                if X is not None and Y is not None: cnt[0] += 1
                return add(X, Y)
            R = None; B = P; kk = r
            while kk:                              # double-and-add, counted
                if kk & 1: R = add_c(R, B)
                kk >>= 1
                if kk: B = add_c(B, B)
            ops_list.append(cnt[0]); ok += int(E8.get(R, -1) == lg % 8)
        np.savez_compressed(os.path.join(a.outdir, f'{a.curve}_t{t}.npz'), rows=rows, tors=tors,
                            surrogate_counts=ms.astype(np.int32), surrogate_J=Js, rho=rho)
        print(json.dumps(dict(event='cell', curve=a.curve, k=k, p=p, A=A, r=r, t=t, N=a.N,
                              mean_c=float(rows[1:, 1].mean()), mean_J=float(rows[1:, 7].mean()),
                              mean_cov=float(rows[1:, 9].mean()), bits_lo0=rows[0].tolist(),
                              torsion_frames=int(tors.sum()), rho=rho,
                              cost_points=len(ops_list), cost_group_ops_per_point=float(np.mean(ops_list)),
                              cost_label_matches=ok, log2r=float(np.log2(r)),
                              legendre_per_frame=1 << t, seconds=round(time.time() - tc, 1))), flush=True)

if __name__ == '__main__':
    main()
