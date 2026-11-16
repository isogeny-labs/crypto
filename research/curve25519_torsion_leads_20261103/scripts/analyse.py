#!/usr/bin/env python3
"""Decide Q1-Q4 of PROTOCOL.md (frozen 012d945af9) from data/k*.json."""
import json, glob, numpy as np
cells = []; Q1n = Q2ok = 0; Q2all = True; Q3a = True; Q3b = []; Q4n = 0; Q4tot = 0
for f in sorted(glob.glob('data/k*.json'), key=lambda s: int(s.split('k')[-1][:-5])):
    d = json.load(open(f)); k = d['k']
    for c in d['cells']:
        t, R = c['t'], c['rows']
        # frame-only: argmax J_0, then cols, then lowest L
        fo = sorted(R, key=lambda x: (-x['J'][0], -x['cols'], x['L']))[0]
        cov_fo = fo['N'][0]
        # joint: argmax over (frame, c) of J_c, ties cols, lowest L, lowest c
        cand = [(-x['J'][cc], -x['cols'], x['L'], cc, x['N'][cc]) for x in R for cc in range(8)]
        jt = sorted(cand)[0]; cov_jt = jt[4]; c_jt = jt[3]
        oracle = max(x['N'][cc] for x in R for cc in range(8))
        g1 = cov_jt / cov_fo; Q1n += g1 >= 1.10
        q2 = cov_jt / oracle; Q2all &= q2 >= 0.9
        # T2
        odd = sum(x['t2_odd'] for x in R); Q3a &= odd == 0
        eff2 = np.mean([x['t2_N0'] / x['t2_cols'] ** 2 for x in R if x['t2_cols']])
        eff1 = np.mean([x['N'][0] / x['cols'] ** 2 for x in R if x['cols']])
        r3 = eff2 / eff1
        if t >= 5: Q3b.append((k, t, r3))
        t2_cols_mean = np.mean([x['t2_cols'] for x in R]); cols_mean = np.mean([x['cols'] for x in R])
        t2_cov_mean = np.mean([x['t2_N0'] for x in R]); cov_mean = np.mean([x['N'][0] for x in R])
        cell = dict(k=k, t=t, cov_frame_only=cov_fo, cov_joint=cov_jt, joint_coset=c_jt, gain_T1=g1,
                    oracle_joint=oracle, joint_over_oracle=q2, cov_mean_plain=cov_mean,
                    t2_odd_points=odd, t2_eff_ratio=r3, t2_cols_mean=t2_cols_mean, cols_mean=cols_mean,
                    t2_cov_mean=t2_cov_mean, t2_best_cov=max(x['t2_N0'] for x in R))
        if t <= 6:
            R3 = [x for x in R if 'm3_cov' in x]
            sel3 = sorted(R3, key=lambda x: (-x['J3'], -x['cols'], x['L']))[0]
            sel2 = sorted(R3, key=lambda x: (-x['J'][0], -x['cols'], x['L']))[0]
            g3 = sel3['m3_cov'] / np.mean([x['m3_cov'] for x in R3])
            g2 = sel2['N'][0] / np.mean([x['N'][0] for x in R3])
            Q4tot += 1; Q4n += g3 > g2
            cell.update(m3_gain=g3, m2_gain_same300=g2, m3_sel_cov=sel3['m3_cov'], m3_mean=float(np.mean([x['m3_cov'] for x in R3])))
        cells.append(cell)
Q3b_ok = all(1.4 <= r <= 2.1 for _, _, r in Q3b)
summ = dict(Q1=Q1n >= 12, Q1_count=int(Q1n), Q2=bool(Q2all), Q3a=bool(Q3a), Q3b=Q3b_ok,
            Q3b_fail=[x for x in Q3b if not 1.4 <= x[2] <= 2.1], Q4=Q4n >= 12, Q4_count=f'{Q4n}/{Q4tot}', cells=cells)
json.dump(summ, open('analysis/summary.json', 'w'), indent=1, default=float)
print({k: v for k, v in summ.items() if k != 'cells'})
print(' k  t | fo   joint(c)  T1gain  /oracle | plain_mean  T2 eff  T2cols/cols  T2cov_mean | m3gain m2gain')
for c in cells:
    print(f"{c['k']:2d} {c['t']} | {c['cov_frame_only']:5d} {c['cov_joint']:5d}({c['joint_coset']}) {c['gain_T1']:5.2f} {c['joint_over_oracle']:5.2f} | {c['cov_mean_plain']:7.1f} {c['t2_eff_ratio']:6.2f} {c['t2_cols_mean']/c['cols_mean']:6.2f} {c['t2_cov_mean']:8.1f} |"
          + (f" {c['m3_gain']:5.2f} {c['m2_gain_same300']:5.2f}" if 'm3_gain' in c else ''))
