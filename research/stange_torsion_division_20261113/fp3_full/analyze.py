#!/usr/bin/env python3
"""Summarize runs/*.json: full cost by phase, gap to matched store-all +- rho, fitted slopes with
intervals, parity extrapolation, plots. Writes summary.json, analysis.txt (stdout), gap_fit.{png,svg,pdf}."""
import glob, json, math, os, random, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))
WI = 3.0          # F_p inversion priced at 3 M (as in ext_field)
PHASES = ['walk', 'build', 'f4', 'multmatrix', 'wiedemann', 'rootfind', 'eigen', 'post']
MODES = ['all', '2lp:B', '1lp:B']
TQ = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306, 9: 2.262, 10: 2.228,
      15: 2.131, 20: 2.086, 25: 2.060, 30: 2.042, 40: 2.021, 60: 2.000}


def tq(df):
    ks = sorted(TQ)
    for k in ks:
        if df <= k:
            return TQ[k]
    return 1.96


def ols(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs); sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    b = sxy / sxx; a = my - b * mx
    res = [y - a - b * x for x, y in zip(xs, ys)]
    s2 = sum(r * r for r in res) / max(1, n - 2)
    se = math.sqrt(s2 / sxx)
    return a, b, se, n


def load():
    out = {}
    for f in sorted(glob.glob(os.path.join(HERE, 'runs', '*.json'))):
        r = json.load(open(f))
        out.setdefault(r['p'], []).append(r)
    return out


def meq_phases(ph):
    return {k: v[0] + WI * v[1] for k, v in ph.items()}


def curve_record(c, zrw):
    """per-curve means over targets"""
    rho = [x['M'] + WI * x['I'] for t in c['targets'] for x in t['rho']]
    rho_steps = [x['steps'] for t in c['targets'] for x in t['rho']]
    rec = dict(p=c['p'], seed=c['seed'], log2r=c['log2r'], rho_meq=st.mean(rho), rho_steps=st.mean(rho_steps),
               rho_runs=len(rho), modes={})
    for m in MODES:
        vals = [t['ic'][m] for t in c['targets']]
        if any(v.get('censored') for v in vals):
            rec['modes'][m] = dict(censored=sum(1 for v in vals if v.get('censored')), n=len(vals))
            continue
        ph = []
        for v in vals:
            e = meq_phases(v['phases'])
            e['la'] = zrw * v['Zr']
            e['la_wied_est'] = zrw * v['Zr_wiedemann_est']
            ph.append(e)
        mean_ph = {k: st.mean(e[k] for e in ph) for k in ph[0]}
        tot = sum(mean_ph[k] for k in PHASES + ['la'])
        rec['modes'][m] = dict(total=tot, phases=mean_ph, trials=st.mean(v['trials'] for v in vals),
                               rels=st.mean(v['rels_kept'] for v in vals), ncols=st.mean(v['ncols'] for v in vals),
                               solver_calls=st.mean(v['solver_calls'] for v in vals),
                               gap=tot / rec['rho_meq'], sec=st.mean(v['sec'] for v in vals))
    stat = {}
    for t in c['targets']:
        for k, v in t['decomposition']['status'].items():
            stat[k] = stat.get(k, 0) + v
    rec['status'] = stat
    rec['rho_sec'] = st.mean(x['sec'] for t in c['targets'] for x in t['rho'])
    return rec


def main():
    data = load()
    out = dict(unit='F_p multiplications, I = 3 M; Z_r mult priced 1 M (primary) and 9 M (sensitivity)', sizes=[])
    lines = []
    P = print
    for zrw, tag in ((1.0, 'Zr=1M'), (9.0, 'Zr=9M')):
        curves = [curve_record(c, zrw) for p in sorted(data) for c in data[p]]
        out['curves_' + tag] = curves
    curves = out['curves_Zr=1M']
    ps = sorted(data)
    P('# Full counted n = 3 IC vs store-all +- rho (unit: F_p mult; I = 3M; Z_r mult = 1M)\n')
    hdr = '| p | log2 r | curves/targets | rho steps | rho M-eq | IC trials (all) | rels | M-eq per trial | IC M-eq (all) | gap all | gap 2lp:B | gap 1lp:B | gap all, Zr=9M |'
    P(hdr); P('|' + '---|' * hdr.count('|')[:-1] if False else '|' + '---|' * (hdr.count('|') - 1))
    c9 = {(c['p'], c['seed']): c for c in out['curves_Zr=9M']}
    for p in ps:
        cs = [c for c in curves if c['p'] == p]
        row = dict(p=p, log2r=st.mean(c['log2r'] for c in cs), curves=len(cs),
                   targets=sum(len(c2['targets']) for c2 in data[p]), rho_steps=st.mean(c['rho_steps'] for c in cs),
                   rho_meq=st.mean(c['rho_meq'] for c in cs))
        for m in MODES:
            ok = [c for c in cs if 'total' in c['modes'][m]]
            cens = sum(c['modes'][m].get('censored', 0) for c in cs)
            if len(ok) == len(cs):
                row[m] = dict(total=st.mean(c['modes'][m]['total'] for c in cs),
                              gap=st.mean(c['modes'][m]['total'] for c in cs) / row['rho_meq'],
                              gap_curves=[c['modes'][m]['gap'] for c in cs],
                              trials=st.mean(c['modes'][m]['trials'] for c in cs),
                              rels=st.mean(c['modes'][m]['rels'] for c in cs),
                              phases={k: st.mean(c['modes'][m]['phases'][k] for c in cs) for k in cs[0]['modes'][m]['phases']},
                              sec=st.mean(c['modes'][m]['sec'] for c in cs))
            else:
                row[m] = dict(censored_targets=cens)
        row['gap_all_Zr9'] = st.mean(c9[(c['p'], c['seed'])]['modes']['all']['total'] for c in cs) / row['rho_meq']
        row['status'] = {}
        for c in cs:
            for k, v in c['status'].items():
                row['status'][k] = row['status'].get(k, 0) + v
        row['rho_sec'] = st.mean(c['rho_sec'] for c in cs)
        out['sizes'].append(row)
        a = row['all']
        g = lambda m: ('%.0f' % row[m]['gap']) if 'gap' in row[m] else 'censored %d' % row[m]['censored_targets']
        P(f"| {p} | {row['log2r']:.1f} | {row['curves']}/{row['targets']} | {row['rho_steps']:.0f} | {row['rho_meq']:.3g} | "
          f"{a['trials']:.0f} | {a['rels']:.0f} | {a['total'] / a['trials']:.3g} | {a['total']:.3g} | {g('all')} | {g('2lp:B')} | {g('1lp:B')} | {row['gap_all_Zr9']:.0f} |")
    P('\n## Breakdown, mode all (share of IC M-eq; per-trial M-eq)\n')
    hdr = '| p | ' + ' | '.join(PHASES + ['la']) + ' | decomposition per solver call | la (Wiedemann est.) |'
    P(hdr); P('|' + '---|' * (hdr.count('|') - 1))
    for row in out['sizes']:
        a = row['all']; ph = a['phases']; tot = a['total']
        calls = st.mean(c2['ic']['all']['solver_calls'] for c in data[row['p']] for c2 in c['targets'])
        dec = sum(ph[k] for k in ['build', 'f4', 'multmatrix', 'wiedemann', 'rootfind', 'eigen', 'post'])
        a['decomp_per_call'] = dec / calls
        P(f"| {row['p']} | " + ' | '.join('%.1f%%' % (100 * ph[k] / tot) for k in PHASES + ['la']) +
          f" | {dec / calls:.3g} | {ph['la_wied_est']:.3g} ({100 * ph['la_wied_est'] / tot:.2f}%) |")
    # ---- fits
    P('\n## Fits (log2 of quantity vs log2 r; OLS over curves; 95% t intervals)\n')
    fits = {}

    def fit(name, pts):
        xs = [x for x, _ in pts]; ys = [math.log2(y) for _, y in pts]
        a, b, se, n = ols(xs, ys)
        fits[name] = dict(intercept=a, slope=b, se=se, n=n, ci95=[b - tq(n - 2) * se, b + tq(n - 2) * se])
        P(f"- {name}: slope {b:+.3f} (95% CI {fits[name]['ci95'][0]:+.3f} .. {fits[name]['ci95'][1]:+.3f}), n = {n}")
    for m in MODES:
        pts = [(c['log2r'], c['modes'][m]['gap']) for c in curves if 'gap' in c['modes'][m]]
        fit('gap ' + m, pts)
    fit('gap all, Zr=9M', [(c['log2r'], c['modes']['all']['gap']) for c in out['curves_Zr=9M']])
    fit('rho M-eq', [(c['log2r'], c['rho_meq']) for c in curves])
    fit('IC all M-eq', [(c['log2r'], c['modes']['all']['total']) for c in curves])
    fit('IC all trials', [(c['log2r'], c['modes']['all']['trials']) for c in curves])
    for k in PHASES + ['la']:
        fit('IC all phase ' + k, [(c['log2r'], max(1.0, c['modes']['all']['phases'][k])) for c in curves])
    fit('decomposition M-eq per trial (all)', [(c['log2r'], sum(c['modes']['all']['phases'][k] for k in PHASES if k != 'walk') / c['modes']['all']['trials']) for c in curves])
    # fit on the larger half only (sizes p >= 251)
    big = [c for c in curves if c['p'] >= 251]
    fit('gap all, p >= 251 only', [(c['log2r'], c['modes']['all']['gap']) for c in big])
    out['fits'] = fits
    # ---- parity extrapolation with bootstrap over curves (resampled within each size)
    P('\n## Parity extrapolation of the gap fit (mode all)\n')
    rng = random.Random(12345)
    bysize = {}
    for c in curves:
        bysize.setdefault(c['p'], []).append((c['log2r'], math.log2(c['modes']['all']['gap'])))
    res = {}
    for name, sel in (('all sizes', ps), ('p >= 251', [p for p in ps if p >= 251])):
        pts = [x for p in sel for x in bysize[p]]
        a, b, se, n = ols([x for x, _ in pts], [y for _, y in pts])
        x0 = -a / b if b < 0 else float('inf')
        boots = []; slopes = []
        for _ in range(4000):
            bp = [rng.choice(bysize[p]) for p in sel for _ in bysize[p]]
            if len(set(x for x, _ in bp)) < 2:
                continue
            aa, bb, _, _ = ols([x for x, _ in bp], [y for _, y in bp])
            slopes.append(bb)
            boots.append(-aa / bb if bb < 0 else float('inf'))
        boots.sort(); slopes.sort()
        q = lambda L, f: L[int(f * (len(L) - 1))]
        frac_nonneg = sum(1 for s in slopes if s >= 0) / len(slopes)
        res[name] = dict(slope=b, intercept=a, parity_log2r=x0, boot_slope_95=[q(slopes, .025), q(slopes, .975)],
                         boot_parity_median=q(boots, .5), boot_parity_95=[q(boots, .025), q(boots, .975)],
                         frac_boot_slope_nonneg=frac_nonneg)
        P(f"- {name}: gap(log2) = {a:.2f} {b:+.4f} * log2 r; parity at log2 r = {x0:.0f}; bootstrap slope 95% "
          f"[{q(slopes, .025):+.4f}, {q(slopes, .975):+.4f}]; parity 95% [{q(boots, .025):.0f}, {q(boots, .975):.0f}] "
          f"(median {q(boots, .5):.0f}); share of bootstrap slopes >= 0: {frac_nonneg:.3f}")
    out['parity'] = res
    # ---- component model: decomposition/trial ~ const, trials ~ p, LA growth
    json.dump(out, open(os.path.join(HERE, 'summary.json'), 'w'), indent=1)
    plot(out, curves, fits, res)


def plot(out, curves, fits, res):
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    fig, ax = plt.subplots(1, 2, figsize=(12, 4.6))
    xs = [c['log2r'] for c in curves]
    for m, col in zip(MODES, ['#2a6fdb', '#d95f02', '#7570b3']):
        pts = [(c['log2r'], c['modes'][m]['gap']) for c in curves if 'gap' in c['modes'][m]]
        ax[0].scatter([x for x, _ in pts], [y for _, y in pts], s=14, color=col, alpha=.7, label=f'gap {m} (per curve)')
    f = fits['gap all']
    xx = [min(xs) - 1, max(xs) + 1]
    ax[0].plot(xx, [2 ** (f['intercept'] + f['slope'] * x) for x in xx], color='#2a6fdb', lw=1.5,
               label=f"OLS all: slope {f['slope']:+.3f} (95% CI {f['ci95'][0]:+.3f}..{f['ci95'][1]:+.3f})")
    ax[0].axhline(1, color='k', lw=.8, ls=':')
    ax[0].set_yscale('log', base=2); ax[0].set_xlabel('log2 r'); ax[0].set_ylabel('IC M-eq / rho M-eq')
    ax[0].set_title('Fully counted E/F_{p^3} IC vs store-all +- rho'); ax[0].legend(fontsize=7)
    ax[0].grid(alpha=.3)
    # stacked breakdown per size, mode all
    sizes = out['sizes']
    xs2 = list(range(len(sizes)))
    bottom = [0] * len(sizes)
    cols = ['#999999', '#a6cee3', '#1f78b4', '#b2df8a', '#33a02c', '#fb9a99', '#e31a1c', '#fdbf6f', '#ff7f00']
    for k, col in zip(['walk', 'build', 'f4', 'multmatrix', 'wiedemann', 'rootfind', 'eigen', 'post', 'la'], cols):
        v = [s['all']['phases'][k] / s['all']['total'] for s in sizes]
        ax[1].bar(xs2, v, bottom=bottom, color=col, label=k)
        bottom = [b + x for b, x in zip(bottom, v)]
    ax[1].set_xticks(xs2); ax[1].set_xticklabels([f"{s['p']}\n{s['log2r']:.0f}b" for s in sizes], fontsize=8)
    ax[1].set_ylabel('share of IC M-eq (mode all)'); ax[1].set_title('Cost breakdown by phase')
    ax[1].legend(fontsize=7, ncol=3, loc='lower right')
    fig.tight_layout()
    for ext in ('png', 'svg', 'pdf'):
        fig.savefig(os.path.join(HERE, 'gap_fit.' + ext), dpi=130)


if __name__ == '__main__':
    main()
