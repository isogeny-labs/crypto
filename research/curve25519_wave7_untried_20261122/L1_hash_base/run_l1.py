#!/usr/bin/env python3
"""Run L1 cells.  usage: run_l1.py --out DIR [--k 16 18 ...] [--B n] [--sizes 0 1] [--smoke]
One npz per (k, N): arm -> int64 [bases, 8] with columns
  N_actual, exact_points, exact_points_with_T0 (-1 unless arm rand), m, hits, hits_with_T0, verified_ok, verified_fail
Bases per cell B = max(512, ceil(1.5 * 2 * (1.96 * cv(N) / 0.03)^2)), cv(N) = CV_A / sqrt(N)  (CV_A frozen in PROTOCOL.md).
"""
import argparse, json, math, os, sys, time, random
import numpy as np
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import l1lib as L

CV_A = 0.7      # frozen in PROTOCOL.md section 7: smoke CV*sqrt(N) = 0.41-0.68 at N = 18, 36 (smoke, not data); 0.7 is the upper bound
ARMS_B = ['bits_rand', 'rand', 'rand2', 'hash', 'hash2', 'rand_proj', 'hash_proj', 'plant3', 'plant5']


def bases_for(N):
    return max(512, math.ceil(1.5 * 2 * (1.96 * CV_A / math.sqrt(N) / 0.03) ** 2))


def run_cell(cv, N, B, outdir, smoke):
    k, r, p = cv.k, cv.r, cv.p
    t0 = time.time()
    m_u = math.ceil(L.HITS_PER_BASE * 4 * r / N ** 2)                  # unprojected: yield ~ N^2/(4r)
    m_p = math.ceil(L.HITS_PER_BASE * (r - 1) / (2 * N ** 2))          # projected: yield ~ 2N^2/(r-1)
    chk = cv.instrument_check(random.Random(L.seed('check', k, N)))
    cap = (B + 2) * N + 64
    ord1, h1 = cv.hash_order('key1', cap); ord2, _ = cv.hash_order('key2', cap)
    th = int(h1[N - 1])                                                # literal theta of the b = 0 hash base
    rows = {a: [] for a in ARMS_B}; rows['bits_lo0'] = []
    t_hash = time.time() - t0
    jr = lambda b: np.random.default_rng(L.seed('targets', k, N, b)).integers(1, r, size=max(m_u, m_p))
    R = lambda *a: np.random.default_rng(L.seed(*a))
    rows['bits_lo0'].append(cv.eval_base(cv.b_bits_lo0(N), jr(-1), m_u))
    for b in range(B):
        j = jr(b)
        Lo = int(R('bitsL', k, N, b).integers(0, p))
        rows['bits_rand'].append(cv.eval_base(cv.b_bits_rand(N, Lo), j, m_u))
        rows['rand'].append(cv.eval_base(cv.b_rand(N, R('rand', k, N, b)), j, m_u, want_t0=True))
        rows['rand2'].append(cv.eval_base(cv.b_rand(N, R('rand2', k, N, b)), j, m_u))
        rows['hash'].append(cv.eval_base(cv.b_hash(ord1, N, b), j, m_u))
        rows['hash2'].append(cv.eval_base(cv.b_hash(ord2, N, b), j, m_u))
        rows['rand_proj'].append(cv.eval_base(cv.b_rand_proj(N, R('randp', k, N, b)), j, m_p))
        rows['hash_proj'].append(cv.eval_base(cv.b_hash_proj(ord1, N, b), j, m_p))
        for nm, f in L.PLANT.items():
            rg = R(nm, k, N, b); x = f * N; Np = int(x) + int(rg.random() < x - int(x))
            rows[nm].append(cv.eval_base(cv.b_rand(Np, rg), j, m_u))
        if smoke and b % 20 == 0:
            print(f'  base {b}/{B} {time.time()-t0:.1f}s', file=sys.stderr)
    meta = dict(k=k, N=N, B=B, p=p, A=cv.A, order=cv.n, r=r, m_u=m_u, m_p=m_p, theta_b0=th,
                instrument_check_failures=chk, hash_pass_s=t_hash, wall_s=time.time() - t0,
                smoke=bool(smoke), cv_A=CV_A, hash_evals_build=int(cv.M), p_scan=p)
    os.makedirs(outdir, exist_ok=True)
    np.savez_compressed(f'{outdir}/cell_k{k}_N{N}.npz', meta=json.dumps(meta),
                        **{a: np.array(v, dtype=np.int64) for a, v in rows.items()})
    print(json.dumps(meta), file=sys.stderr)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', required=True); ap.add_argument('--k', type=int, nargs='+')
    ap.add_argument('--B', type=int); ap.add_argument('--sizes', type=int, nargs='+', help='indices into C_SIZES')
    ap.add_argument('--smoke', action='store_true')
    a = ap.parse_args()
    for k in a.k or sorted(L.CURVES):
        t = time.time(); cv = L.Curve(k)
        print(f'k={k} table {time.time()-t:.1f}s, valid columns {cv.M}', file=sys.stderr)
        for ci in (a.sizes if a.sizes is not None else range(len(L.C_SIZES))):
            N = max(2, round(L.C_SIZES[ci] * math.sqrt(cv.r)))
            run_cell(cv, N, a.B or bases_for(N), a.out, a.smoke)


if __name__ == '__main__':
    main()
