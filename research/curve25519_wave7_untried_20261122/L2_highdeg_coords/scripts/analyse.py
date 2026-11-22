#!/usr/bin/env python3
"""L2: decide the predictions of PROTOCOL.md from data/ and write analysis/summary.json
and analysis/summary.md. Every verdict is mechanical; thresholds are the frozen ones."""
import argparse, glob, json, math, os, statistics
import numpy as np
import common as C_

HSIZE = {'xe': {16: 2, 18: 2, 20: 2}, 'xeye': {16: 4, 18: 2, 20: 2}}  # rational translation subgroup in the fibre
SYM_PRED = {  # (eps, j in Q = j r, sign) for the rational part; Q1 rational only at k = 16 (A + 2 square)
    'x': {(1, 0, '+'), (-1, 0, '+')},
    'v': {(1, 0, '+'), (-1, 0, '-')},
    'q': {(1, 0, '+'), (-1, 0, '-')},
    'uv': {(1, 0, '+'), (-1, 0, '-')},
    'jac6': {(1, 0, '+'), (-1, 0, '+')},
    'ye': {(1, 0, '+'), (-1, 0, '+'), (1, 4, '-'), (-1, 4, '-')},
    'xe': {(1, 0, '+'), (-1, 0, '-'), (1, 4, '-'), (-1, 4, '+')},
    'xeye': {(1, 0, '+'), (-1, 0, '-'), (1, 4, '+'), (-1, 4, '-')},
}
XEYE_K16_EXTRA = {(1, 2, '-'), (1, 6, '-'), (-1, 2, '+'), (-1, 6, '+')}
TOT_MAX = {'x': 4, 'ye': 6, 'xe': 8, 'v': 18, 'q': 18, 'xeye': 8, 'jac6': 36, 'uv': 50}


def boot_ratio(a, b, seed, B=2000):
    rng = np.random.default_rng(seed)
    ra = rng.integers(0, a.size, size=(B, a.size)); rb = rng.integers(0, b.size, size=(B, b.size))
    rs = a[ra].mean(axis=1) / b[rb].mean(axis=1)
    return float(np.percentile(rs, 2.5)), float(np.percentile(rs, 97.5))


def census(d):
    cells = []
    for fn in sorted(glob.glob(f'{d}/census_k*_*.npz')):
        base = os.path.basename(fn)[7:-4]
        k, arm, t = base.split('_')[0][1:], '_'.join(base.split('_')[1:-1]), base.split('_')[-1][1:]
        k, t = int(k), int(t)
        R = np.load(fn)['rows'].astype(np.float64)
        vals, pts, cols, cov, J, xm, xmJ, xvc, xv, xvJ = R.T
        sel = int(np.lexsort((-cols, -J))[0])
        selx = int(np.argmax(xmJ))
        c = dict(k=k, arm=arm, t=t, K=len(R), mean_cov=cov.mean(), mean_xm=xm.mean(), mean_xv=xv.mean(),
                 mean_cols=cols.mean(), mean_pts=pts.mean(), values=float(vals[0]),
                 R_col=cov.mean() / xm.mean() if xm.mean() else float('nan'),
                 R_val=cov.mean() / xv.mean() if xv.mean() else float('nan'),
                 pts_per_value=pts.mean() / vals[0] if vals[0] else None,
                 cols_per_value=cols.mean() / vals[0] if vals[0] else None,
                 sel_gain_f=cov[sel] / cov.mean() if cov.mean() else None,
                 sel_gain_xm=xm[selx] / xm.mean() if xm.mean() else None)
        c['R_col_ci'] = boot_ratio(cov, xm, C_.dseed('boot', k, arm, t, 'col'))
        c['R_val_ci'] = boot_ratio(cov, xv, C_.dseed('boot', k, arm, t, 'val')) if vals[0] else None
        cells.append(c)
    return cells


def decide(cells, sp, mc, meta):
    V = {}
    by = lambda arm: [c for c in cells if c['arm'] == arm]
    # PC1 replay
    pc = meta.get('PC1_replay', {})
    V['PC1_replay'] = bool(pc) and all(x['match'] for x in pc.values())
    pc1b = [c['sel_gain_f'] for c in by('xAA') if c['k'] == 20 and c['t'] in (4, 5)]
    V['PC1b_selection_gain_ge_1.5'] = bool(pc1b) and all(g >= 1.5 for g in pc1b)
    ev = by('even')
    V['PC2_even_in_1.6_2.4_all'] = bool(ev) and all(1.6 <= c['R_col'] <= 2.4 for c in ev)
    aa = by('xAA')
    V['NC1_AA_in_0.95_1.05_ge90pct'] = bool(aa) and sum(0.95 <= c['R_col'] <= 1.05 for c in aa) >= 0.9 * len(aa)
    rd = by('rand')
    V['NC2_rand_in_0.9_1.1_all'] = bool(rd) and all(0.9 <= c['R_col'] <= 1.1 for c in rd)
    ye = by('ye')
    V['NC3_ye_in_0.9_1.1_all'] = bool(ye) and all(0.9 <= c['R_col'] <= 1.1 for c in ye)
    # P1
    test = [c for c in cells if c['arm'] in ('v', 'q', 'uv', 'jac6') and c['mean_xm'] >= 50]
    inside = sum(0.85 <= c['R_col'] <= 1.15 for c in test)
    V['P1_inside'] = f'{inside}/{len(test)}'
    V['P1'] = bool(test) and inside >= 0.9 * len(test) and not any(c['R_col_ci'][0] > 1.15 for c in test)
    # P1x
    hx = [c for c in cells if c['arm'] in HSIZE and c['mean_xm'] >= 50]
    okx = [0.8 / HSIZE[c['arm']][c['k']] <= c['R_col'] <= 1.2 / HSIZE[c['arm']][c['k']] for c in hx]
    V['P1x_inside'] = f'{sum(okx)}/{len(okx)}'
    V['P1x'] = bool(hx) and all(okx)
    V['P1x_blind_cells_only'] = all(o for c, o in zip(hx, okx) if not (c['arm'] == 'xeye' and c['k'] == 16))
    # P2
    def rv_ok(c):
        a = c['arm']
        if a in ('v', 'q', 'uv'):
            return 3.0 <= c['R_val'] <= 5.0
        if a == 'jac6':
            return 0.75 <= c['R_val'] <= 1.33
        if a in HSIZE:
            m = 4 / HSIZE[a][c['k']]
            return 0.75 * m <= c['R_val'] <= 1.33 * m
        return True
    p2 = [c for c in cells if c['arm'] in ('v', 'q', 'uv', 'jac6', 'xe', 'xeye')]
    V['P2_inside'] = f'{sum(rv_ok(c) for c in p2)}/{len(p2)}'
    V['P2'] = bool(p2) and sum(rv_ok(c) for c in p2) >= 0.9 * len(p2)
    # P3 and P4 from sumpoly
    p3, p4fib, p4sym, p4deg, p4tot, p4ver = [], [], [], [], [], []
    for k, F in sp.items():
        k = int(k)
        for f, R in F.items():
            if f == 'facts':
                continue
            p3.append(0.99 <= R['points_per_value'] <= 1.01)
            want = C_.DESK[f]['deg'] if not (f == 'xeye' and k != 16) else 2
            p4fib.append(R['max_rational_fibre'] == want)
            got = {(s['eps'], int(s['Q_log'][:-1]), s['sign']) for s in R['symmetries_rational']}
            ws = SYM_PRED[f] | (XEYE_K16_EXTRA if (f == 'xeye' and k == 16) else set())
            p4sym.append(got == ws)
            pv = {'x': 2, 'ye': 2, 'xe': 4, 'v': 9, 'q': 9, 'xeye': 4, 'jac6': 18, 'uv': 25}[f]
            S = R['S']
            p4deg.append(S is not None and S['pervar_degree'] == pv and S['kernel_dim'] == 1)
            p4tot.append(S is not None and (S['total_degree'] == 4 if f == 'x' else S['total_degree'] <= TOT_MAX[f]))
            p4ver.append(S is not None and S['verify_zeros_vanish'] == S['verify_n'] and S['random_nonvanish'] >= 0.99 * S['verify_n'])
    V['P3_points_per_value'] = all(p3)
    V['P4_max_fibre'] = all(p4fib); V['P4_symmetries'] = all(p4sym); V['P4_pervar_degree'] = all(p4deg)
    V['P4_total_degree'] = all(p4tot); V['P4_verified'] = all(p4ver)
    V['PC3_S3'] = all(F['x'].get('PC3_S3_proportional', False) for F in sp.values() if 'x' in F)
    # P5 from macaulay
    if mc:
        recs = [r for r in mc if 'e' in r]
        def e_pred(f, T):
            n = 2 * T
            if f in ('v', 'q', 'uv', 'jac6'):
                return n
            if f in ('xe', 'xeye'):
                return 8 if T >= 4 else n
            return 4
        V['P5a_e'] = all(r['e'] == e_pred(r['f'], r['T']) for r in recs)
        V['P5a_mismatch'] = [(r['k'], r['f'], r['T'], r['e']) for r in recs if r['e'] != e_pred(r['f'], r['T'])][:20]
        m2 = [r for r in recs if 'M2' in r and r['M2']['mode'] == 'ref']
        V['P5b_dense_Dref_n'] = all(r['M2']['D_ref'] == r['n'] for r in m2 if r['f'] in ('v', 'q', 'uv', 'jac6'))
        xs = {T: [r for r in m2 if r['f'] == 'x' and r['T'] == T] for T in (4, 5)}
        V['P5c_x_T5_le_n-2'] = bool(xs[5]) and sum(r['M2']['D_ref'] is not None and r['M2']['D_ref'] <= 8 for r in xs[5]) >= 0.9 * len(xs[5])
        V['P5c_x_T4_le_6'] = bool(xs[4]) and sum(r['M2']['D_ref'] is not None and r['M2']['D_ref'] <= 6 for r in xs[4]) >= 0.9 * len(xs[4])
        med = {}
        for r in m2:
            med.setdefault((r['k'], r['f'], r['T']), []).append(r['M2']['hist'][-1]['muladds'])
        med = {kk: statistics.median(v) for kk, v in med.items()}
        V['M2_median_muladds'] = {f'{k}/{f}/T{T}': v for (k, f, T), v in sorted(med.items())}
        ratios = {}
        for k in (16, 18, 20):
            for f in ('v', 'q'):
                if (k, f, 5) in med and (k, 'x', 5) in med:
                    ratios[f'{k}/{f}'] = med[(k, f, 5)] / med[(k, 'x', 5)]
        V['P5d_ratios_T5'] = ratios
        V['P5d'] = bool(ratios) and all(x >= 3 for x in ratios.values())
        dmed = lambda f, k: statistics.median([r['M2']['D_ref'] for r in m2 if r['f'] == f and r['T'] == 5 and r['k'] == k and r['M2']['D_ref']] or [float('nan')])
        V['P5e_ye_eq_x'] = all(abs(dmed('ye', k) - dmed('x', k)) <= 1 for k in (16, 18, 20))
        V['P5f_xe_xeye_above_x'] = all(dmed(f, k) >= dmed('x', k) + 1 for f in ('xe', 'xeye') for k in (16, 18, 20))
        m1 = [r for r in recs if 'M1' in r]
        V['P5g_M1_full'] = bool(m1) and sum(r['M1']['d_mult'] == r['n'] for r in m1) >= 0.9 * len(m1)
        V['consistency_census_vs_system'] = all(r['census_agrees'] for r in recs)
        V['M2_rank_check'] = all(r['M2']['rank_at_n_check'] in (True, None) for r in recs if 'M2' in r)
        V['C1_terms_x'] = all(r['terms'] == {4: 121, 5: 256, 6: 484}[r['T']] for r in recs if r['f'] == 'x')
        # cost per relation (M2 median per target / coverage probability of the same base)
        V['note'] = 'cost per relation needs the census coverage of the [0,2^t) base: written by macaulay.py runs as census_covered fraction'
    return V


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--data', default='data')
    ap.add_argument('--out', default='analysis')
    a = ap.parse_args()
    cells = census(a.data)
    sp = json.load(open(f'{a.data}/sumpoly.json')) if os.path.exists(f'{a.data}/sumpoly.json') else {}
    mc = json.load(open(f'{a.data}/macaulay.json')) if os.path.exists(f'{a.data}/macaulay.json') else []
    meta = json.load(open(f'{a.data}/census_meta.json')) if os.path.exists(f'{a.data}/census_meta.json') else {}
    V = decide(cells, sp, mc, meta)
    json.dump(dict(verdicts=V, cells=cells), open(f'{a.out}/summary.json', 'w'), indent=1, default=float)
    with open(f'{a.out}/summary.md', 'w') as fh:
        fh.write('# L2 summary (mechanical; every sentence [pcc:unverified])\n\n')
        for kk, v in V.items():
            fh.write(f'- {kk}: {v}\n')
        fh.write('\n| k | arm | t | cols | cov f | cov xm | R_col [95% CI] | R_val | pts/value | sel gain f / xm |\n|--:|:--|--:|--:|--:|--:|:--|--:|--:|:--|\n')
        for c in sorted(cells, key=lambda c: (c['arm'], c['k'], c['t'])):
            fh.write(f"| {c['k']} | {c['arm']} | {c['t']} | {c['mean_cols']:.1f} | {c['mean_cov']:.1f} | {c['mean_xm']:.1f} | "
                     f"{c['R_col']:.3f} [{c['R_col_ci'][0]:.3f}, {c['R_col_ci'][1]:.3f}] | {c['R_val']:.2f} | "
                     f"{(c['pts_per_value'] or 0):.3f} | {(c['sel_gain_f'] or 0):.2f} / {(c['sel_gain_xm'] or 0):.2f} |\n")
    print(json.dumps(V, indent=1, default=float))


if __name__ == '__main__':
    main()
