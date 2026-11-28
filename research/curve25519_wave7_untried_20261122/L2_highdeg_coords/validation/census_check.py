"""Validator: recompute every census ratio, interval and verdict from the raw rows.
Independent of scripts/analyse.py. Rows: vals pts cols cov J | xm_cov xm_J | xv_cols xv_cov xv_J.
Thresholds are read from PROTOCOL.md section 5/6 (typed here by the validator)."""
import glob, hashlib, json, os, sys
import numpy as np

LEAD = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L2_highdeg_coords'
H = {('xe', 16): 2, ('xe', 18): 2, ('xe', 20): 2, ('xeye', 16): 4, ('xeye', 18): 2, ('xeye', 20): 2}


def dseed(*parts):
    s = 'wave7-L2|' + '|'.join(str(x) for x in parts)
    return int(hashlib.sha256(s.encode()).hexdigest()[:12], 16)


def load():
    cells = {}
    for fn in sorted(glob.glob(f'{LEAD}/data/census_k*_*_t*.npz')):
        b = os.path.basename(fn)[:-4].split('_')
        k, arm, t = int(b[1][1:]), b[2], int(b[3][1:])
        z = np.load(fn)
        rows = z['rows']
        cells[(k, arm, t)] = dict(rows=rows, seed=int(z['seed']), seed_ok=int(z['seed']) == dseed(k, arm, t))
    return cells


def json_mirror_ok(k, arm, t, rows):
    base = f'{LEAD}/data_json/data/census_k{k}_{arm}_t{t}'
    parts = sorted(glob.glob(base + '.json')) or sorted(glob.glob(base + '.part*.json'))
    data = []
    for pf in parts:
        d = json.load(open(pf))
        a = d['arrays']['rows']
        data.extend(a['data'])
    arr = np.array(data, dtype=np.int64)
    return arr.shape == rows.shape and np.array_equal(arr, rows.astype(np.int64))


def stats(R, rng, B=2000):
    R = R.astype(np.float64)
    vals, pts, cols, cov, J, xm, xmJ, xvc, xv, xvJ = R.T
    K = len(R)
    out = dict(K=K, mean_cov=cov.mean(), mean_xm=xm.mean(), mean_xv=xv.mean(), mean_cols=cols.mean(),
               values=float(vals[0]))
    out['R_col'] = cov.mean() / xm.mean()
    out['R_val'] = cov.mean() / xv.mean() if vals[0] else None
    out['pts_per_value'] = pts.mean() / vals[0] if vals[0] else None
    # selection gain: argmax J, ties -> more cols, then first index
    best = max(range(K), key=lambda i: (J[i], cols[i], -i))
    out['sel_gain_f'] = cov[best] / cov.mean()
    bx = max(range(K), key=lambda i: (xmJ[i], -i))
    out['sel_gain_xm'] = xm[bx] / xm.mean()
    # independent-arm percentile bootstrap (PROTOCOL section 4)
    ia = rng.integers(0, K, size=(B, K)); ib = rng.integers(0, K, size=(B, K))
    rc = cov[ia].mean(1) / xm[ib].mean(1)
    out['R_col_ci'] = [float(np.percentile(rc, 2.5)), float(np.percentile(rc, 97.5))]
    if vals[0]:
        ic = rng.integers(0, K, size=(B, K)); iv = rng.integers(0, K, size=(B, K))
        rv = cov[ic].mean(1) / xv[iv].mean(1)
        out['R_val_ci'] = [float(np.percentile(rv, 2.5)), float(np.percentile(rv, 97.5))]
    # paired bootstrap (same frame index for f and its xm), beside the protocol's estimator
    ip = rng.integers(0, K, size=(B, K))
    rp = cov[ip].mean(1) / xm[ip].mean(1)
    out['R_col_paired_ci'] = [float(np.percentile(rp, 2.5)), float(np.percentile(rp, 97.5))]
    return out


def verdicts(S):
    V = {}
    arm = lambda a: [c for c in S.values() if c['arm'] == a]
    V['PC1b'] = [round(c['sel_gain_f'], 4) for c in sorted(arm('xAA'), key=lambda c: c['t']) if c['k'] == 20 and c['t'] in (4, 5)]
    V['PC1b_pass'] = all(g >= 1.5 for g in V['PC1b'])
    V['PC2_range'] = [min(c['R_col'] for c in arm('even')), max(c['R_col'] for c in arm('even'))]
    V['PC2_pass'] = all(1.6 <= c['R_col'] <= 2.4 for c in arm('even'))
    aa = arm('xAA')
    V['NC1_inside'] = f"{sum(0.95 <= c['R_col'] <= 1.05 for c in aa)}/{len(aa)}"
    V['NC1_pass'] = sum(0.95 <= c['R_col'] <= 1.05 for c in aa) >= 0.9 * len(aa)
    V['NC2_range'] = [min(c['R_col'] for c in arm('rand')), max(c['R_col'] for c in arm('rand'))]
    V['NC2_pass'] = all(0.9 <= c['R_col'] <= 1.1 for c in arm('rand'))
    V['NC3_range'] = [min(c['R_col'] for c in arm('ye')), max(c['R_col'] for c in arm('ye'))]
    V['NC3_census_pass'] = all(0.9 <= c['R_col'] <= 1.1 for c in arm('ye'))
    test = [c for c in S.values() if c['arm'] in ('v', 'q', 'uv', 'jac6')]
    qual = [c for c in test if c['mean_xm'] >= 50]
    ins = sum(0.85 <= c['R_col'] <= 1.15 for c in qual)
    V['P1_qualifying'] = len(qual); V['P1_inside'] = f'{ins}/{len(qual)}'
    V['P1_excluded_cells'] = sorted([(c['k'], c['arm'], c['t'], round(c['mean_xm'], 1)) for c in test if c['mean_xm'] < 50])
    V['P1_range_qualifying'] = [min(c['R_col'] for c in qual), max(c['R_col'] for c in qual)]
    V['P1_range_all48'] = [min(c['R_col'] for c in test), max(c['R_col'] for c in test)]
    V['P1_max_ci_lo'] = max(c['R_col_ci'][0] for c in qual)
    V['P1_pass'] = ins >= 0.9 * len(qual) and V['P1_max_ci_lo'] <= 1.15
    # P1x: every cell (no xm filter in PROTOCOL)
    hx = [c for c in S.values() if c['arm'] in ('xe', 'xeye')]
    fails = []
    for c in hx:
        h = H[(c['arm'], c['k'])]
        if not (0.8 / h <= c['R_col'] <= 1.2 / h):
            fails.append(dict(k=c['k'], arm=c['arm'], t=c['t'], R_col=c['R_col'], ci=c['R_col_ci'], band=[0.8 / h, 1.2 / h]))
    V['P1x_inside'] = f'{len(hx) - len(fails)}/{len(hx)}'; V['P1x_fails'] = fails; V['P1x_pass'] = not fails
    V['P1x_blind_only_pass'] = not [f for f in fails if not (f['arm'] == 'xeye' and f['k'] == 16)]
    for a in ('xe', 'xeye'):
        for ks in ((16,), (18, 20)):
            cs = [c['R_col'] for c in hx if c['arm'] == a and c['k'] in ks]
            V[f'P1x_range_{a}_{ks}'] = [min(cs), max(cs)]
    # P2
    def rv_ok(c):
        a = c['arm']
        if a in ('v', 'q', 'uv'):
            return 3.0 <= c['R_val'] <= 5.0
        if a == 'jac6':
            return 0.75 <= c['R_val'] <= 1.33
        m = 4 / H[(a, c['k'])]
        return 0.75 * m <= c['R_val'] <= 1.33 * m
    p2 = [c for c in S.values() if c['arm'] in ('v', 'q', 'uv', 'jac6', 'xe', 'xeye')]
    V['P2_inside'] = f'{sum(rv_ok(c) for c in p2)}/{len(p2)}'
    V['P2_pass'] = sum(rv_ok(c) for c in p2) >= 0.9 * len(p2)
    V['P2_out'] = [(c['k'], c['arm'], c['t'], c['R_val']) for c in p2 if not rv_ok(c)]
    for a in ('v', 'q', 'uv', 'jac6', 'xe', 'ye'):
        cs = [c['R_val'] for c in S.values() if c['arm'] == a]
        V[f'R_val_range_{a}'] = [min(cs), max(cs)]
    for ks in ((16,), (18, 20)):
        cs = [c['R_val'] for c in S.values() if c['arm'] == 'xeye' and c['k'] in ks]
        V[f'R_val_range_xeye_{ks}'] = [min(cs), max(cs)]
    for a in ('v', 'q', 'uv', 'jac6', 'ye', 'xAA', 'rand'):
        cs = [c['R_col'] for c in S.values() if c['arm'] == a]
        V[f'R_col_range_{a}'] = [min(cs), max(cs)]
    # lead-positive census criterion
    pos = []
    for a in ('v', 'q', 'uv', 'jac6', 'xe', 'xeye'):
        cs = [c for c in S.values() if c['arm'] == a]
        hits = sum(c['R_col'] >= 1.5 and c['R_col_ci'][0] > 1.15 for c in cs)
        if hits >= 2 / 3 * len(cs):
            pos.append(a)
    V['lead_positive_census'] = pos
    V['max_R_col_test_arms'] = max(c['R_col'] for c in S.values() if c['arm'] in ('v', 'q', 'uv', 'jac6', 'xe', 'xeye'))
    # selection gains (descriptive)
    ta = [c for c in S.values() if c['arm'] in ('v', 'q', 'uv', 'jac6', 'xe', 'xeye', 'ye')]
    V['sel_gain_f_range_fn_arms'] = [min(c['sel_gain_f'] for c in ta), max(c['sel_gain_f'] for c in ta)]
    V['sel_gain_xm_range_fn_arms'] = [min(c['sel_gain_xm'] for c in ta), max(c['sel_gain_xm'] for c in ta)]
    al = list(S.values())
    V['sel_gain_f_range_all_arms'] = [min(c['sel_gain_f'] for c in al), max(c['sel_gain_f'] for c in al)]
    V['sel_gain_xm_range_all_arms'] = [min(c['sel_gain_xm'] for c in al), max(c['sel_gain_xm'] for c in al)]
    return V


def run(cells, drop, seed):
    rng = np.random.default_rng(seed)
    S = {}
    for key, c in sorted(cells.items()):
        k, arm, t = key
        R = c['rows']
        lo = drop.get(key, 0)
        if lo is None:
            continue
        R = R[lo:]
        s = stats(R, rng)
        s.update(k=k, arm=arm, t=t)
        S[key] = s
    return S


def main():
    cells = load()
    out = {'n_cells': len(cells), 'seeds_match_rule': all(c['seed_ok'] for c in cells.values())}
    out['json_mirrors_equal'] = all(json_mirror_ok(*key, c['rows']) for key, c in cells.items())
    # smoke overlap: first 20 rows of k = 16, t = 4
    ov = {}
    for fn in sorted(glob.glob(f'{LEAD}/smoke/census_k16_*_t4.npz')):
        arm = os.path.basename(fn).split('_')[2]
        sm = np.load(fn)['rows']
        d = cells[(16, arm, 4)]['rows']
        ov[arm] = dict(smoke_rows=int(len(sm)), equal_first=bool(np.array_equal(sm, d[:len(sm)])),
                       smoke_seed=int(np.load(fn)['seed']), data_seed=cells[(16, arm, 4)]['seed'])
    out['smoke_overlap_k16_t4'] = ov
    # pre-freeze K = 50 check (PROTOCOL section 0) against the first 50 data rows of k = 16, t = 5
    pf = {'v': (262.8, 255.4), 'xeye': (77.9, 279.0), 'even': (507.6, 259.4), 'rand': (260.3, 256.3), 'xAA': (272.6, 266.8)}
    out['prefreeze_K50_k16_t5'] = {a: dict(first50_cov=float(cells[(16, a, 5)]['rows'][:50, 3].mean()),
                                           first50_xm=float(cells[(16, a, 5)]['rows'][:50, 5].mean()),
                                           protocol_s0=pf[a]) for a in pf}
    base = run(cells, {}, 20261122)
    out['verdicts_full'] = verdicts(base)
    # compare with producer's summary.json cells
    prod = {(c['k'], c['arm'], c['t']): c for c in json.load(open(f'{LEAD}/analysis/summary.json'))['cells']}
    dif = dict(R_col=0, R_val=0, mean_cov=0, sel_f=0, sel_xm=0, ci_lo=0, ci_hi=0)
    for key, s in base.items():
        p = prod[key]
        dif['R_col'] = max(dif['R_col'], abs(s['R_col'] - p['R_col']))
        dif['mean_cov'] = max(dif['mean_cov'], abs(s['mean_cov'] - p['mean_cov']))
        if s['R_val'] is not None:
            dif['R_val'] = max(dif['R_val'], abs(s['R_val'] - p['R_val']))
        dif['sel_f'] = max(dif['sel_f'], abs(s['sel_gain_f'] - p['sel_gain_f']))
        dif['sel_xm'] = max(dif['sel_xm'], abs(s['sel_gain_xm'] - p['sel_gain_xm']))
        dif['ci_lo'] = max(dif['ci_lo'], abs(s['R_col_ci'][0] - p['R_col_ci'][0]))
        dif['ci_hi'] = max(dif['ci_hi'], abs(s['R_col_ci'][1] - p['R_col_ci'][1]))
    out['max_abs_diff_vs_producer_summary'] = dif
    key = (16, 'xeye', 7)
    out['P1x_failing_cell'] = dict(R_col=base[key]['R_col'], ci=base[key]['R_col_ci'], paired_ci=base[key]['R_col_paired_ci'],
                                   mean_cov=base[key]['mean_cov'], mean_xm=base[key]['mean_xm'],
                                   producer=dict(R_col=prod[key]['R_col'], ci=prod[key]['R_col_ci']), bound=1.2 / 4)
    # sensitivity: drop overlapping rows / seen cell
    smoke_drop = {(16, a, 4): 20 for a in ov}
    pre_drop = {(16, a, 5): 50 for a in pf}
    pre_cell = {(16, a, 5): None for a in pf}
    sens = {}
    for name, drop in (('drop_smoke20_k16t4', smoke_drop), ('drop_prefreeze50_k16t5', pre_drop),
                       ('drop_both', {**smoke_drop, **pre_drop}), ('without_seen_cell_k16t5', pre_cell),
                       ('without_seen_cell_and_smoke20', {**smoke_drop, **pre_cell})):
        S = run(cells, drop, 20261123)
        V = verdicts(S)
        sens[name] = {kk: V[kk] for kk in ('P1_inside', 'P1_pass', 'P1x_inside', 'P1x_pass', 'P1x_blind_only_pass', 'P1x_fails',
                                          'P2_inside', 'P2_pass', 'PC2_pass', 'NC1_inside', 'NC1_pass', 'NC2_pass', 'NC3_census_pass',
                                          'lead_positive_census', 'PC2_range', 'NC2_range')}
    out['sensitivity'] = sens
    out['cells'] = {f'{k}/{a}/{t}': {kk: v for kk, v in s.items() if kk not in ('k', 'arm', 't')} for (k, a, t), s in base.items()}
    json.dump(out, open('census_check.json', 'w'), indent=1, default=float)
    V = out['verdicts_full']
    print(json.dumps({kk: out[kk] for kk in out if kk not in ('cells',)}, indent=1, default=float)[:20000])


if __name__ == '__main__':
    main()
