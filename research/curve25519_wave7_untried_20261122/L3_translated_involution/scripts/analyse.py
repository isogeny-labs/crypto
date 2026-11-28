#!/usr/bin/env python3
"""Decide the predictions of PROTOCOL.md from the census data. Frozen with the protocol.

  python3 analyse.py --data ../data --out ../analysis            # full run
  python3 analyse.py --data ../smoke/data --out ../smoke/analysis --smoke

Estimators: means; ratio of means with a percentile bootstrap 95% interval (2000 resamples, unpaired,
rng seeded from the cell name); log-free selection = argmax J_D (ties: more points, then lower frame
index); KS distance for distribution equality. Verdicts use only the thresholds written in PROTOCOL.md.
"""
import argparse, glob, hashlib, json, math, os
import numpy as np
import model as mdl

FS_SUMMARY = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..',
                                           'curve25519_frame_search_20261103', 'analysis', 'summary.json'))
TOL = {4: 0.07, 5: 0.05, 6: 0.04, 7: 0.04, 8: 0.04}
NOHALF = ['E.iota_odd', 'Ep.iota_c01', 'Ep.iota_c10', 'Ep.iota_c11', 'Ep.huff_x', 'Ep.huff_y']
CONTROLS = ['E.iota_even', 'E.iota_T0', 'Ep.iota_2E']
JI = ['Ep.ji_s', 'Ep.ji_c', 'Ep.ji_d']
SHIFT_REF = {'E.iota_odd': ('E.x', 'covered_shift0', 'E.x|shift_odd'),
             'Ep.iota_c01': ('Ep.x', 'covered_shift0', 'Ep.x|shift_01'),
             'Ep.iota_c10': ('Ep.x', 'covered_shift1', 'Ep.x|shift_10'),
             'Ep.iota_c11': ('Ep.x', 'covered_shift2', 'Ep.x|shift_11')}


def null_of(arm):
    return 'E.x' if arm.startswith('E.') else 'Ep.x'


def rng_for(*parts):
    return np.random.default_rng(int(hashlib.sha256('|'.join(map(str, parts)).encode()).hexdigest()[:16], 16))


def ratio_ci(a, b, name, B=2000):
    a, b = np.asarray(a, float), np.asarray(b, float)
    g = rng_for('boot', name)
    ia = g.integers(0, a.size, (B, a.size))
    ib = g.integers(0, b.size, (B, b.size))
    rb = a[ia].mean(1) / b[ib].mean(1)
    return float(a.mean() / b.mean()), float(np.percentile(rb, 2.5)), float(np.percentile(rb, 97.5))


def ks_dist(a, b):
    a, b = np.sort(a), np.sort(b)
    xs = np.union1d(a, b)
    fa = np.searchsorted(a, xs, 'right') / a.size
    fb = np.searchsorted(b, xs, 'right') / b.size
    return float(np.abs(fa - fb).max())


def select(d, t):
    J, npts, cov = d[f't{t}_J_D'], d[f't{t}_n_points'], d[f't{t}_covered_D']
    order = np.lexsort((np.arange(J.size), -npts, -J))
    s = int(order[0])
    return dict(index=s, covered=int(cov[s]), gain_vs_mean=float(cov[s] / cov.mean()),
                oracle=int(cov.max()), sel_over_oracle=float(cov[s] / cov.max()))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--data', required=True)
    ap.add_argument('--out', required=True)
    ap.add_argument('--smoke', action='store_true')
    a = ap.parse_args()
    os.makedirs(a.out, exist_ok=True)
    D, META = {}, {}
    for kd in sorted(glob.glob(os.path.join(a.data, 'k*'))):
        k = int(os.path.basename(kd)[1:])
        META[k] = json.load(open(os.path.join(kd, 'meta.json')))
        for f in glob.glob(os.path.join(kd, '*.npz')):
            D[(k, os.path.basename(f)[:-4])] = dict(np.load(f))
    ks = sorted(META)
    ts = META[ks[0]]['t']
    model = mdl.table(ks=tuple(ks), ts=tuple(ts))
    fs = {(c['k'], c['t']): c for c in json.load(open(FS_SUMMARY))['cells']}
    S = dict(cells={}, verdicts={}, smoke=a.smoke)
    V = S['verdicts']

    def get(k, arm, t, col='covered_D'):
        return D[(k, arm)][f't{t}_{col}']

    def mrat(k, t, arm, ref):
        return model[f'{k}|{t}|{arm}']['D'] / model[f'{k}|{t}|{ref}']['D']

    # per-cell descriptive table
    for (k, arm), d in D.items():
        for t in ts:
            c = dict(mean_D=float(get(k, arm, t).mean()), sd_D=float(get(k, arm, t).std()),
                     mean_full=float(get(k, arm, t, 'covered_full').mean()),
                     mean_points=float(get(k, arm, t, 'n_points').mean()),
                     model_D=model[f'{k}|{t}|{arm}']['D'], model_full=model[f'{k}|{t}|{arm}']['full'])
            c.update({'sel_' + kk: v for kk, v in select(d, t).items()})
            S['cells'][f'{k}|{t}|{arm}'] = c

    def verdict(name, ok, n_ok, n_tot, detail):
        V[name] = dict(held=bool(ok), n_ok=n_ok, n_total=n_tot, detail=detail)

    # CAL1: exact reproduction of frame_search fixed frames
    det, nok, ntot = [], 0, 0
    for k in ks:
        for t in ts:
            if (k, t) not in fs:
                continue
            fx = META[k]['fixed'][str(t)]
            for mine, theirs in (('E.bits_lo0', 'bits_lo0'), ('E.edwards_bits', 'edwards')):
                ntot += 1
                ok = fx[mine]['covered_full'] == fs[(k, t)][theirs]
                nok += ok
                det.append((k, t, mine, fx[mine]['covered_full'], fs[(k, t)][theirs]))
    verdict('CAL1_exact_frame_search_fixed_frames', nok == ntot, nok, ntot, det)
    # CAL2: null mean vs frame_search Möbius mean
    det, nok, ntot = [], 0, 0
    for k in ks:
        for t in ts:
            if (k, t) not in fs:
                continue
            x = get(k, 'E.x', t)
            se = math.sqrt(x.var() / x.size + fs[(k, t)]['trans_sd'] ** 2 / 2000)
            ok = abs(x.mean() - fs[(k, t)]['mobius_mean']) <= 3 * se
            ntot += 1; nok += ok
            det.append((k, t, round(float(x.mean()), 2), fs[(k, t)]['mobius_mean'], round(se, 2)))
    verdict('CAL2_null_matches_frame_search_mobius_mean', nok >= ntot - 1, nok, ntot, det)
    # CAL3: model adequacy on both nulls
    det, nok, ntot = [], 0, 0
    for k in ks:
        for t in ts:
            for arm in ('E.x', 'Ep.x'):
                m = get(k, arm, t).mean(); pm = model[f'{k}|{t}|{arm}']['D']
                ok = abs(m / pm - 1) <= TOL[t]
                ntot += 1; nok += ok; det.append((k, t, arm, round(float(m / pm), 4)))
    verdict('CAL3_model_matches_nulls', nok == ntot, nok, ntot, det)
    # C1: pointwise translated-frame identity
    det, nok, ntot, n2 = [], 0, 0, 0
    for k in ks:
        for t in ts:
            for arm in CONTROLS:
                c = get(k, arm, t, 'control_ok')
                ntot += c.size; nok += int((c == 1).sum()); n2 += int((c == 2).sum())
                det.append((k, t, arm, int((c == 0).sum()), int((c == 2).sum())))
    verdict('C1_translated_frames_pointwise_identity', (ntot - nok - n2) == 0 and n2 <= 0.01 * ntot,
            nok, ntot, dict(cells=det, unfittable=n2))
    # C2: controls match their null in mean
    det, nok, ntot = [], 0, 0
    for k in ks:
        for t in ts:
            for arm in CONTROLS:
                r = get(k, arm, t).mean() / get(k, null_of(arm), t).mean()
                ok = abs(r - 1) <= TOL[t]; ntot += 1; nok += ok; det.append((k, t, arm, round(float(r), 4)))
    verdict('C2_controls_match_null', nok >= ntot - 1, nok, ntot, det)
    # AA: split-half A/A
    det, nok, ntot = [], 0, 0
    for k in ks:
        for t in ts:
            for arm in ('E.x', 'Ep.x'):
                x = get(k, arm, t)
                r, lo, hi = ratio_ci(x[0::2], x[1::2], f'AA|{k}|{t}|{arm}')
                ok = lo <= 1 <= hi; ntot += 1; nok += ok; det.append((k, t, arm, round(r, 4), round(lo, 4), round(hi, 4)))
    verdict('AA_split_half', nok >= ntot - 3, nok, ntot, det)
    # PC1: planted removal of the self-pair term is detected
    det, ok_dir, n_dir, ok_mod, n_mod = [], 0, 0, 0, 0
    for k in ks:
        for t in ts:
            r, lo, hi = ratio_ci(get(k, 'E.x_noself', t, 'covered_D_noself'), get(k, 'E.x', t), f'PC1|{k}|{t}')
            pm = mrat(k, t, 'E.x_noself', 'E.x')
            if t in (4, 5, 6):
                n_dir += 1; ok_dir += hi < 1
            n_mod += 1; ok_mod += abs(r - pm) <= TOL[t]
            det.append((k, t, round(r, 4), round(lo, 4), round(hi, 4), round(pm, 4)))
    verdict('PC1_planted_selfpair_removal_detected', ok_dir == n_dir and ok_mod >= n_mod - 1,
            ok_dir + ok_mod, n_dir + n_mod, det)
    # PC2: frame-search coset-selection gain reproduced
    det, nok, ntot = [], 0, 0
    for k in ks:
        for t in (5, 6, 7):
            if t not in ts:
                continue
            g = select(D[(k, 'E.x')], t)['gain_vs_mean']
            ok = g >= (2.0 if t == 5 else 1.3); ntot += 1; nok += ok; det.append((k, t, round(g, 3)))
    verdict('PC2_selection_gain_detected', nok == ntot, nok, ntot, det)
    # P1: no-half iota_T functions lose the self-pair term
    det, ok_dir, n_dir, ok_mod, n_mod = [], 0, 0, 0, 0
    for k in ks:
        for t in ts:
            for arm in NOHALF:
                r, lo, hi = ratio_ci(get(k, arm, t), get(k, null_of(arm), t), f'P1|{k}|{t}|{arm}')
                pm = mrat(k, t, arm, null_of(arm))
                if t in (4, 5):
                    n_dir += 1; ok_dir += hi < 1
                n_mod += 1; ok_mod += abs(r - pm) <= TOL[t]
                det.append((k, t, arm, round(r, 4), round(lo, 4), round(hi, 4), round(pm, 4)))
    verdict('P1_nohalf_below_null', ok_dir == n_dir and ok_mod >= n_mod - 5, ok_dir + ok_mod, n_dir + n_mod,
            dict(direction=f'{ok_dir}/{n_dir}', model=f'{ok_mod}/{n_mod}', cells=det))
    # P2: E' x-frames above E x-frames
    det, ok_dir, n_dir, ok_mod, n_mod = [], 0, 0, 0, 0
    for k in ks:
        for t in ts:
            r, lo, hi = ratio_ci(get(k, 'Ep.x', t), get(k, 'E.x', t), f'P2|{k}|{t}')
            pm = model[f'{k}|{t}|Ep.x']['D'] / model[f'{k}|{t}|E.x']['D']
            if t in (4, 5):
                n_dir += 1; ok_dir += lo > 1
            n_mod += 1; ok_mod += abs(r - pm) <= TOL[t]
            det.append((k, t, round(r, 4), round(lo, 4), round(hi, 4), round(pm, 4)))
    verdict('P2_Ep_xframes_above_E_xframes', ok_dir == n_dir and ok_mod >= n_mod - 1, ok_dir + ok_mod,
            n_dir + n_mod, dict(direction=f'{ok_dir}/{n_dir}', model=f'{ok_mod}/{n_mod}', cells=det))
    # P3: Jacobi-intersection (degree 4) coordinates halve coverage
    det, ok_dir, n_dir, ok_mod, n_mod = [], 0, 0, 0, 0
    for k in ks:
        for t in ts:
            for arm in JI:
                r, lo, hi = ratio_ci(get(k, arm, t), get(k, 'Ep.x', t), f'P3|{k}|{t}|{arm}')
                pm = mrat(k, t, arm, 'Ep.x')
                if t in (5, 6, 7):
                    n_dir += 1; ok_dir += hi < 0.75
                n_mod += 1; ok_mod += abs(r - pm) <= max(TOL[t], 0.06)
                det.append((k, t, arm, round(r, 4), round(lo, 4), round(hi, 4), round(pm, 4)))
    verdict('P3_JI_degree4_halves_coverage', ok_dir == n_dir and ok_mod >= n_mod - 5, ok_dir + ok_mod,
            n_dir + n_mod, dict(direction=f'{ok_dir}/{n_dir}', model=f'{ok_mod}/{n_mod}', cells=det))
    # P4: invariance (a) same class => same distribution, (b) no class effect
    det, nok, ntot = [], 0, 0
    for k in ks:
        for t in ts:
            if model[f'{k}|{t}|Ep.x']['D'] < 50:
                continue
            for x, y in (('Ep.huff_x', 'Ep.huff_y'), ('Ep.huff_x', 'Ep.iota_c01'), ('Ep.huff_y', 'Ep.iota_c01')):
                A_, B_ = get(k, x, t), get(k, y, t)
                r = A_.mean() / B_.mean(); dks = ks_dist(A_, B_)
                crit = 1.95 * math.sqrt((A_.size + B_.size) / (A_.size * B_.size))
                ok = 0.95 <= r <= 1.05 and dks <= crit
                ntot += 1; nok += ok; det.append((k, t, x, y, round(float(r), 4), round(dks, 4)))
    verdict('P4a_same_class_same_distribution', nok == ntot, nok, ntot, det)
    det, nok, ntot = [], 0, 0
    for k in ks:
        for t in ts:
            if model[f'{k}|{t}|Ep.x']['D'] < 50:
                continue
            for x, y in (('Ep.iota_c01', 'Ep.iota_c10'), ('Ep.iota_c01', 'Ep.iota_c11'), ('Ep.iota_c10', 'Ep.iota_c11')):
                r = get(k, x, t).mean() / get(k, y, t).mean()
                ok = 0.95 <= r <= 1.05; ntot += 1; nok += ok; det.append((k, t, x, y, round(float(r), 4)))
    verdict('P4b_no_class_effect', nok >= ntot - 2, nok, ntot, det)
    # P5: the T-shifted full target set against the x-null with the same shifts
    det, nok, ntot = [], 0, 0
    for k in ks:
        for t in ts:
            for arm, (ref, col, mref) in SHIFT_REF.items():
                r, lo, hi = ratio_ci(get(k, arm, t, 'covered_full'), get(k, ref, t, col), f'P5|{k}|{t}|{arm}')
                pm = model[f'{k}|{t}|{arm}']['full'] / model[f'{k}|{t}|{mref}']['full']
                ok = abs(r - pm) <= TOL[t]; ntot += 1; nok += ok
                fd = get(k, arm, t, 'covered_full').mean() / get(k, arm, t).mean()
                det.append((k, t, arm, round(r, 4), round(lo, 4), round(hi, 4), round(pm, 4), round(float(fd), 3)))
    verdict('P5_full_set_matches_shifted_null', nok >= ntot - 10, nok, ntot, det)
    # P6: no fatter selection tail for the no-half families
    det, oks = [], []
    for arm in NOHALF:
        rs = []
        for k in ks:
            for t in (5, 6, 7):
                if t not in ts:
                    continue
                s1, s0 = select(D[(k, arm)], t), select(D[(k, null_of(arm))], t)
                rs.append(s1['covered'] / s0['covered'])
                det.append((arm, k, t, s1['covered'], s0['covered'], round(rs[-1], 4),
                            round(s1['oracle'] / s0['oracle'], 4)))
        gm = float(np.exp(np.mean(np.log(rs))))
        oks.append(0.80 <= gm <= 1.05 and max(rs) <= 1.15)
        det.append((arm, 'geomean', round(gm, 4), 'max', round(max(rs), 4)))
    verdict('P6_no_fatter_selection_tail', all(oks), sum(oks), len(oks), det)
    # P7: E' selection against E selection
    rs, det = [], []
    for k in ks:
        for t in (5, 6, 7):
            if t not in ts:
                continue
            s1, s0 = select(D[(k, 'Ep.x')], t), select(D[(k, 'E.x')], t)
            rs.append(s1['covered'] / s0['covered']); det.append((k, t, s1['covered'], s0['covered'], round(rs[-1], 4)))
    gm = float(np.exp(np.mean(np.log(rs))))
    verdict('P7_Ep_selected_vs_E_selected', 1.00 <= gm <= 1.30, int(1.00 <= gm <= 1.30), 1, dict(geomean=gm, cells=det))
    json.dump(S, open(os.path.join(a.out, 'summary.json'), 'w'), indent=1, default=float)
    lines = ['# Verdicts' + (' (SMOKE, NOT DATA)' if a.smoke else ''), '']
    for name, v in V.items():
        lines.append(f'- {name}: {"held" if v["held"] else "FAILED"} ({v["n_ok"]}/{v["n_total"]}) [pcc:unverified]')
    open(os.path.join(a.out, 'VERDICTS.md'), 'w').write('\n'.join(lines) + '\n')
    print('\n'.join(lines))


if __name__ == '__main__':
    main()
