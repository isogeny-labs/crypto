#!/usr/bin/env python3
"""PC3 and the bootstrap calibration (PROTOCOL.md).

k = 20, t in {5, 6}: P independent pools of K = 2000 fresh translation frames (derived seeds, disjoint from
the main census seeds). Per pool: the realised frame-search gain (selected covered / pool-mean covered, the
published statistic). Per pair of pools (N = 4000): the U-statistic gain at K = 2000 on J and on covered, whose
spread across pairs calibrates the bootstrap SE of the main census (same N and K).
Usage: pc3_pools.py <out.json> [P]"""
import json, os, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from l5common import CURVES, derived_seed, build, frame_logs_translate, class_counts, J_from_counts, J_exact, covered_exact, gain

def main():
    out = sys.argv[1]; P = int(sys.argv[2]) if len(sys.argv) > 2 else 80; K = 2000
    k, p, A, n = CURVES['k20']; logu, r, *_ = build(p, A, n); A3 = A * pow(3, -1, p) % p
    res = dict(K=K, pools=P, cells=[]); t0 = time.time()
    for t in (5, 6):
        rng = np.random.default_rng(derived_seed('pc3pools', t))
        pools = []
        for q in range(P):
            Ls = rng.integers(0, p, size=K)
            rows = []
            for L in Ls:
                logs = frame_logs_translate(p, A3, logu, int(L), t)
                J = J_exact(logs, n) if np.any(logs == n // 2) else J_from_counts(class_counts(logs))
                rows.append((int(L), len(logs), J, covered_exact(logs, n)))
            pools.append(np.array(rows, dtype=np.int64))
        realised = []
        for R in pools:
            o = np.lexsort((R[:, 0], -R[:, 1], -R[:, 2]))      # argmax J, then cols, then lowest L
            realised.append(R[o[0], 3] / R[:, 3].mean())
        ustat_J, ustat_cov = [], []
        for q in range(0, P - 1, 2):
            R = np.vstack([pools[q], pools[q + 1]])
            keys = [R[:, 2], R[:, 1], -R[:, 0]]
            ustat_J.append(gain(R[:, 2], keys, K)); ustat_cov.append(gain(R[:, 3], keys, K))
        res['cells'].append(dict(t=t, realised=[float(x) for x in realised],
                                 realised_lo95=float(np.percentile(realised, 2.5)), realised_hi95=float(np.percentile(realised, 97.5)),
                                 ustat_J=[float(x) for x in ustat_J], ustat_cov=[float(x) for x in ustat_cov],
                                 ustat_J_sd=float(np.std(ustat_J, ddof=1)), ustat_cov_sd=float(np.std(ustat_cov, ddof=1))))
    res['seconds'] = round(time.time() - t0, 1)
    json.dump(res, open(out, 'w'), indent=1)
    print(json.dumps({'seconds': res['seconds']}))

if __name__ == '__main__':
    main()
