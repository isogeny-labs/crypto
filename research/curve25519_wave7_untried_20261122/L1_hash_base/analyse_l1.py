#!/usr/bin/env python3
"""L1 analysis.  usage: analyse_l1.py --in DIR --out DIR
Reads cell_k*_N*.npz written by run_l1.py, writes DIR/summary.json and DIR/RESULTS.md.
Estimator: verified-hit rate = sum(hits)/sum(targets) per arm and cell; ratios of rates;
95 % percentile intervals from 4000 bootstrap resamples of the bases (independent per arm; paired
for T0 vs blind, which share bases and targets).  Pooled = geometric mean over cells with the
per-cell bootstrap draws averaged in log space.  Thresholds are the frozen PROTOCOL.md section 6 values.
"""
import argparse, glob, json, math, os, sys
import numpy as np
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import l1lib as L

NB = 4000
W_BAND = (0.0, 0.1, 1.0)
COMPS = [  # name, numerator, denominator, kind
    ('hash/rand', 'hash', 'rand'), ('hash/bits_rand', 'hash', 'bits_rand'), ('bits_rand/rand', 'bits_rand', 'rand'),
    ('hash_proj/rand_proj', 'hash_proj', 'rand_proj'), ('rand_proj/rand', 'rand_proj', 'rand'),
    ('rand/rand2 (A/A)', 'rand', 'rand2'), ('hash/hash2 (A/A)', 'hash', 'hash2'),
    ('plant3/rand', 'plant3', 'rand'), ('plant5/rand', 'plant5', 'rand'), ('T0/blind', 't0', 'rand')]


def rate(x, t0=False):
    return x[:, 5 if t0 else 4].sum() / x[:, 3].sum()


def boot(num, den, rng, t0=False, exact=False):
    paired = t0
    if exact:
        f = lambda x, i: (x[i, 2 if t0 else 1]).mean(1)
        fn = lambda x: x[:, 2 if t0 else 1].mean()
        fd = lambda x: x[:, 1].mean()
    else:
        f = lambda x, i, col=None: x[i, 5 if t0 else 4].sum(1) / x[i, 3].sum(1)
        fn = lambda x: rate(x, t0)
        fd = lambda x: rate(x)
    i1 = rng.integers(0, len(num), (NB, len(num)))
    i2 = i1 if paired else rng.integers(0, len(den), (NB, len(den)))
    if exact:
        rn = num[i1, 2 if t0 else 1].mean(1); rd = den[i2, 1].mean(1)
    else:
        rn = num[i1, 5 if t0 else 4].sum(1) / num[i1, 3].sum(1); rd = den[i2, 4].sum(1) / den[i2, 3].sum(1)
    lr = np.log(rn / rd)
    return math.log(fn(num) / fd(den)), lr


def ci(lr):
    lo, hi = np.percentile(lr, [2.5, 97.5]); return float(math.exp(lo)), float(math.exp(hi))


def main():
    ap = argparse.ArgumentParser(); ap.add_argument('--in', dest='inp', required=True); ap.add_argument('--out', required=True)
    a = ap.parse_args(); os.makedirs(a.out, exist_ok=True)
    cells = []
    for f in sorted(glob.glob(f'{a.inp}/cell_k*_N*.npz'), key=lambda s: (int(s.split('_k')[1].split('_')[0]), int(s.split('_N')[1].split('.')[0]))):
        d = np.load(f); meta = json.loads(str(d['meta'])); cells.append((meta, {k: d[k] for k in d.files if k != 'meta'}))
    smoke = any(m['smoke'] for m, _ in cells)
    S = dict(smoke=smoke, cells=[], pooled={}, predictions={}, cost=[])
    pooled_lr = {c[0]: [] for c in COMPS}; pooled_pt = {c[0]: [] for c in COMPS}
    pooled_lr_x = {c[0]: [] for c in COMPS}; pooled_pt_x = {c[0]: [] for c in COMPS}
    per_cell_ci = {c[0]: [] for c in COMPS}; per_cell_pt = {c[0]: [] for c in COMPS}; per_cell_exact = {c[0]: [] for c in COMPS}
    design = {'plant3/rand': [], 'plant5/rand': []}
    zs = []; ver_fail = 0; ver_gap = 0; chk_fail = 0; pct = {'bits_lo0': [], 'hash_lit': []}
    ceil_viol = 0; proj_ceiling = []
    for meta, X in cells:
        k, N, r = meta['k'], meta['N'], meta['r']; rng = np.random.default_rng(L.seed('boot', k, N))
        cell = dict(k=k, N=N, B=meta['B'], r=r, m_u=meta['m_u'], m_p=meta['m_p'], ratios={}, arms={})
        chk_fail += meta['instrument_check_failures']
        for arm in ['bits_rand', 'rand', 'rand2', 'hash', 'hash2', 'rand_proj', 'hash_proj', 'plant3', 'plant5']:
            x = X[arm]; y = x[:, 1] / (r - 1)
            ver_fail += int(x[:, 7].sum())
            exp_ok = int(x[:, 4].sum() + (0 if arm != 'rand' else (x[:, 5] - x[:, 4]).sum()))
            ver_gap += int(x[:, 6].sum()) - exp_ok
            var = (x[:, 3] * y * (1 - y)).sum(); zs.append((k, N, arm, float((x[:, 4].sum() - (x[:, 3] * y).sum()) / math.sqrt(var))))
            if arm == 'rand':
                y0 = x[:, 2] / (r - 1); v0 = (x[:, 3] * y0 * (1 - y0)).sum()
                zs.append((k, N, 't0', float((x[:, 5].sum() - (x[:, 3] * y0).sum()) / math.sqrt(v0))))
            cell['arms'][arm] = dict(exact_rate=float(y.mean()), mc_rate=float(rate(x)), cv_exact=float(x[:, 1].std() / x[:, 1].mean()),
                                     mean_N=float(x[:, 0].mean()), hits=int(x[:, 4].sum()), verified=int(x[:, 6].sum()))
            if arm in ('rand', 'bits_rand', 'hash', 'hash2'):
                ceil_viol += int((x[:, 1] > 2 * N * N).sum())
        for arm in ('rand_proj', 'hash_proj'):
            proj_ceiling.append((k, N, arm, cell['arms'][arm]['exact_rate'] / (2 * N * N / (r - 1))))
        # single draws: percentile inside the matching random-base distribution
        e = X['bits_rand'][:, 1]; pct['bits_lo0'].append(float((e < X['bits_lo0'][0, 1]).mean() * 100 + (e == X['bits_lo0'][0, 1]).mean() * 50))
        e = X['hash'][:, 1]; pct['hash_lit'].append(float((e < e[0]).mean() * 100 + (e == e[0]).mean() * 50))
        for name, n_, d_ in COMPS:
            t0 = n_ == 't0'
            num = X['rand'] if t0 else X[n_]; den = X[d_]
            pt, lr = boot(num, den, rng, t0=t0); ptx, lrx = boot(num, den, rng, t0=t0, exact=True)
            cell['ratios'][name] = dict(mc=float(math.exp(pt)), mc_ci=ci(lr), exact=float(math.exp(ptx)), exact_ci=ci(lrx),
                                        half_width=float((ci(lr)[1] - ci(lr)[0]) / 2))
            pooled_lr[name].append(lr); pooled_pt[name].append(pt); pooled_lr_x[name].append(lrx); pooled_pt_x[name].append(ptx)
            per_cell_ci[name].append(ci(lr)); per_cell_pt[name].append(math.exp(pt)); per_cell_exact[name].append(math.exp(ptx))
            if name in design: design[name].append(float((X[n_][:, 0] ** 2).mean() / N ** 2))
        S['cells'].append(cell)
    nc = len(cells)
    for name in pooled_lr:
        lr = np.mean(pooled_lr[name], axis=0); S['pooled'][name] = dict(
            mc=float(math.exp(np.mean(pooled_pt[name]))), mc_ci=ci(lr),
            exact=float(math.exp(np.mean(pooled_pt_x[name]))), exact_ci=ci(np.mean(pooled_lr_x[name], axis=0)))

    P = S['pooled']; pc = per_cell_ci
    def pooled_null(nm, lo=0.98, hi=1.02, clo=0.97, chi=1.03):
        q = P[nm]; return lo <= q['mc'] <= hi and q['mc_ci'][0] >= clo and q['mc_ci'][1] <= chi
    def contains1(nm): return sum(c[0] <= 1 <= c[1] for c in pc[nm])
    def excl1(nm): return sum(not (c[0] <= 1 <= c[1]) for c in pc[nm])
    hw = [c['ratios']['hash/rand']['half_width'] for c in S['cells']]
    pr = S['predictions']
    pr['P1 hash/rand pooled in [0.98,1.02], CI inside [0.97,1.03]; >= 8/10 cell CIs contain 1'] = dict(
        value=P['hash/rand'], cells_containing_1=contains1('hash/rand'), cells=nc,
        holds=pooled_null('hash/rand') and contains1('hash/rand') >= math.ceil(0.8 * nc))
    pr['P1p power: hash/rand half-width <= 0.03 in >= 8/10 cells (else underpowered, null not claimable)'] = dict(
        half_widths=[round(h, 4) for h in hw], holds=sum(h <= 0.03 for h in hw) >= math.ceil(0.8 * nc))
    pr['P2 hash/bits_rand and bits_rand/rand pooled in [0.98,1.02], CI inside [0.97,1.03]'] = dict(
        hash_bits=P['hash/bits_rand'], bits_rand=P['bits_rand/rand'],
        holds=pooled_null('hash/bits_rand') and pooled_null('bits_rand/rand'))
    pr['P2b single draws (bits lo=0; literal theta hash base) inside the 1-99 percentile of their distribution in >= 8/10 cells'] = dict(
        bits_lo0_pct=pct['bits_lo0'], hash_lit_pct=pct['hash_lit'],
        holds=all(sum(1 <= v <= 99 for v in pct[a]) >= math.ceil(0.8 * nc) for a in pct))
    pr['P3 ceiling: no base above 2N^2/(r-1); projected arms in [0.78,1.0] of it; rand_proj/rand in [6.0,8.4] every cell; hash_proj/rand_proj pooled in [0.98,1.02]'] = dict(
        ceiling_violations=ceil_viol, proj_over_ceiling=[round(v[3], 3) for v in proj_ceiling],
        proj_over_unproj=[round(v, 2) for v in per_cell_exact['rand_proj/rand']], hash_proj_pooled=P['hash_proj/rand_proj'],
        holds=ceil_viol == 0 and all(0.78 <= v[3] <= 1.0 for v in proj_ceiling) and all(6.0 <= v <= 8.4 for v in per_cell_exact['rand_proj/rand'])
        and pooled_null('hash_proj/rand_proj'))
    pr['P4 positive control T0/blind exact in [1.80,2.20] every cell; MC CI excludes 1 in every cell'] = dict(
        exact=[round(v, 3) for v in per_cell_exact['T0/blind']], mc_pooled=P['T0/blind'], cells_excluding_1=excl1('T0/blind'),
        holds=all(1.8 <= v <= 2.2 for v in per_cell_exact['T0/blind']) and excl1('T0/blind') == nc)
    d3 = float(np.mean(design['plant3/rand'])); d5 = float(np.mean(design['plant5/rand']))
    pr['P5 planted: plant5/rand CI excludes 1 in >= 9/10 cells and pooled within 0.015 of design; plant3/rand pooled CI excludes 1, pooled within 0.01 of design, >= 6/10 cells exclude 1'] = dict(
        design=dict(plant3=d3, plant5=d5), pooled=dict(plant3=P['plant3/rand'], plant5=P['plant5/rand']),
        cells_excluding_1=dict(plant3=excl1('plant3/rand'), plant5=excl1('plant5/rand')),
        holds=excl1('plant5/rand') >= math.ceil(0.9 * nc) and abs(P['plant5/rand']['mc'] - d5) <= 0.015
        and P['plant3/rand']['mc_ci'][0] > 1 and abs(P['plant3/rand']['mc'] - d3) <= 0.01 and excl1('plant3/rand') >= math.ceil(0.6 * nc))
    pr['P6 A/A: rand/rand2 and hash/hash2 pooled in [0.985,1.015], CI inside [0.97,1.03]; >= 8/10 cell CIs contain 1'] = dict(
        rand=P['rand/rand2 (A/A)'], hash=P['hash/hash2 (A/A)'],
        cells_containing_1=dict(rand=contains1('rand/rand2 (A/A)'), hash=contains1('hash/hash2 (A/A)')),
        holds=pooled_null('rand/rand2 (A/A)', .985, 1.015) and pooled_null('hash/hash2 (A/A)', .985, 1.015)
        and contains1('rand/rand2 (A/A)') >= math.ceil(.8 * nc) and contains1('hash/hash2 (A/A)') >= math.ceil(.8 * nc))
    worst = max(zs, key=lambda t: abs(t[3]))
    pr['P7 instrument: 0 verification failures; every hit verified; 0 instrument-check failures; MC vs exact |z| <= 4 in every arm and cell'] = dict(
        verification_failures=ver_fail, unverified_hits=-ver_gap, instrument_check_failures=chk_fail, worst_z=worst,
        holds=ver_fail == 0 and ver_gap == 0 and chk_fail == 0 and abs(worst[3]) <= 4)

    # ---- costs (GAE = group-add equivalents); hash evaluations priced separately ----
    for meta, X in cells:
        N, p, r = meta['N'], meta['p'], meta['r']
        G = N * N; row = dict(k=meta['k'], N=N, group_ops_table=G, group_ops_project=3 * N,
                              hash_evals_build=p, hash_evals_valid_only=meta['hash_evals_build'], hash_evals_per_column=p / N, field_exps_bits_rand=2 * N,
                              hash_evals_probe_mode=N, rate_blind=float(rate(X['rand'])), by_w={})
        for w in W_BAND:
            row['by_w'][str(w)] = dict(hash=G + w * p, hash_proj=G + 3 * N + w * p, rand=G, bits=G, rand_proj=G + 3 * N,
                                       hash_over_rand=(G + w * p) / G,
                                       relations_per_target_hash=float(rate(X['hash'])))
        S['cost'].append(row)
    pr['P8 cost: hash arm priced total >= rand arm in every cell at w = 0, 0.1, 1 (equal at 0, strictly greater above); ranking holds only where the order holds across the band'] = dict(
        holds=all(c['by_w'][str(w)]['hash'] >= c['by_w'][str(w)]['rand'] for c in S['cost'] for w in W_BAND),
        order_across_band='tie at w = 0, rand cheaper at w = 0.1 and 1: no strict ranking across the band')
    json.dump(S, open(f'{a.out}/summary.json', 'w'), indent=1, default=float)

    # ---- markdown ----
    o = []
    if smoke: o.append('> SMOKE, NOT DATA: pipeline test on the smallest case; no statement below is a result.\n')
    o.append('# L1 hash-membership factor base: results table [pcc:unverified]\n')
    o.append('Ratios are verified-hit-rate ratios (MC), 95 % bootstrap intervals over bases; "exact" is the ratio of exact mean covered points.\n')
    o.append('| k | N | B | comparison | MC ratio | 95 % CI | exact ratio |\n|--:|--:|--:|:--|--:|:--|--:|')
    for c in S['cells']:
        for name, v in c['ratios'].items():
            o.append(f"| {c['k']} | {c['N']} | {c['B']} | {name} | {v['mc']:.4f} | [{v['mc_ci'][0]:.4f}, {v['mc_ci'][1]:.4f}] | {v['exact']:.4f} |")
    o.append('\n## Pooled over cells (geometric mean)\n\n| comparison | MC ratio | 95 % CI | exact ratio |\n|:--|--:|:--|--:|')
    for name, v in P.items(): o.append(f"| {name} | {v['mc']:.4f} | [{v['mc_ci'][0]:.4f}, {v['mc_ci'][1]:.4f}] | {v['exact']:.4f} |")
    o.append('\n## Predictions (thresholds frozen in PROTOCOL.md section 6)\n')
    for kx, v in pr.items(): o.append(f"- **{'HOLDS' if v['holds'] else 'FAILS'}**: {kx} [pcc:unverified]")
    o.append('\n## Cost, hash membership priced separately (GAE = group-add equivalents)\n')
    o.append('| k | N | table group ops (N^2) | hash evals to build base (p) | hash evals per column | hash/rand total at w=0 | w=0.1 | w=1 |\n|--:|--:|--:|--:|--:|--:|--:|--:|')
    for c in S['cost']:
        b = c['by_w']; o.append(f"| {c['k']} | {c['N']} | {c['group_ops_table']} | {c['hash_evals_build']} | {c['hash_evals_per_column']:.0f} | "
                                f"{b['0.0']['hash_over_rand']:.2f} | {b['0.1']['hash_over_rand']:.2f} | {b['1.0']['hash_over_rand']:.2f} |")
    o.append('\nTotals exclude the shared unpriced native work (Legendre/sqrt per column, matched across arms) and the T0 extra inversions; '
             'cost per relation = total / (Q * rate) for Q probed targets, rates in summary.json. No distance to rho is stated here: the generic '
             'baseline is 7.8x rho at k = 20 (curve25519_frame_search_20261103/README.md, imported).')
    open(f'{a.out}/RESULTS.md', 'w').write('\n'.join(o) + '\n')
    print('\n'.join(o[:3]), '\n...'); print('\n'.join(f"{'HOLDS' if v['holds'] else 'FAILS'}: {kx[:90]}" for kx, v in pr.items()))


if __name__ == '__main__':
    main()
