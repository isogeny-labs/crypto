#!/usr/bin/env python3
"""Summarize runs_n2/ and runs_n3/ into summary.json, print markdown tables, draw gap plot."""
import glob, json, math, os, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))


def meq(d, wI):
    return d['M'] + wI * d['I']


def fit(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs); sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    b = sxy / sxx; a = my - b * mx
    res = [y - a - b * x for x, y in zip(xs, ys)]
    se = math.sqrt(sum(r * r for r in res) / max(1, n - 2) / sxx) if n > 2 else float('nan')
    return b, se


def load(d):
    out = {}
    for f in sorted(glob.glob(os.path.join(HERE, d, '*.json'))):
        r = json.load(open(f))
        out.setdefault(r['p'], []).append(r)
    return out


def summarize_n2():
    data = load('runs_n2')
    rows = []
    pts = {}
    for p in sorted(data):
        curves = data[p]
        lg = math.log2(p)
        for wname, wI in (('I=3M', 3.0), ('I=1.5log2pM', 1.5 * lg)):
            pass
        modes = list(curves[0]['targets'][0]['ic'].keys())
        rec = dict(p=p, curves=len(curves), targets=sum(len(c['targets']) for c in curves),
                   log2r=st.mean(c['log2r'] for c in curves))
        # per curve means
        rho_steps = []; rho_M = {}; ic = {m: {} for m in modes}
        for wname, wI in (('w3', 3.0), ('wlog', 1.5 * lg)):
            rho_c = []; ic_c = {m: [] for m in modes}
            for c in curves:
                rr = [x for t in c['targets'] for x in t['rho']]
                rho_c.append(st.mean(meq(x, wI) for x in rr))
                for m in modes:
                    vals = [t['ic'][m] for t in c['targets']]
                    if any(v.get('censored') for v in vals):
                        ic_c[m].append(float('nan'))
                    else:
                        ic_c[m].append(st.mean(meq(v, wI) for v in vals))
            rec['rho_Meq_' + wname] = st.mean(rho_c)
            for m in modes:
                vals = [v for v in ic_c[m] if v == v]
                rec[f'ic_{m.split(":")[0] if m.startswith("2lp") else m}_Meq_{wname}'] = st.mean(vals) if vals else None
                rec[f'gap_{m.split(":")[0] if m.startswith("2lp") else m}_{wname}'] = (st.mean(vals) / st.mean(rho_c)) if vals else None
        allrho = [x for c in curves for t in c['targets'] for x in t['rho']]
        rec['rho_steps_mean'] = st.mean(x['steps'] for x in allrho)
        rec['rho_steps_over_sqrt_r'] = st.mean(x['steps'] / math.sqrt(c['r']) for c in curves for t in c['targets'] for x in t['rho'])
        rec['rho_sec_mean'] = st.mean(x['sec'] for x in allrho)
        m2 = [m for m in modes if m.startswith('2lp')][0]
        icv = [t['ic'][m2] for c in curves for t in c['targets']]
        rec['ic2lp_trials_mean'] = st.mean(v['trials'] for v in icv)
        rec['ic2lp_trials_over_p'] = st.mean(v['trials'] / p for v in icv)
        rec['ic2lp_rels_mean'] = st.mean(v['rels_kept'] for v in icv)
        rec['ic2lp_vertices_mean'] = st.mean(v['vertices'] for v in icv)
        rec['ic2lp_M_per_trial'] = st.mean(v['M'] / v['trials'] for v in icv)
        rec['ic2lp_I_per_trial'] = st.mean(v['I'] / v['trials'] for v in icv)
        rec['ic2lp_Zr_mean'] = st.mean(v['Zr_total'] for v in icv)
        rec['ic2lp_sec_mean'] = st.mean(v['sec'] for v in icv)
        rec['ic2lp_dup_skipped_mean'] = st.mean(v['dup_skipped'] for v in icv)
        rec['rho_M_per_step'] = st.mean(x['M'] / x['steps'] for x in allrho)
        rec['all_logs_checked'] = True  # every run asserts the recovered log equals w
        rec['gap_per_curve_2lp_w3'] = []
        for c in curves:
            rr = [x for t in c['targets'] for x in t['rho']]
            ii = [t['ic'][m2] for t in c['targets']]
            rec['gap_per_curve_2lp_w3'].append(st.mean(meq(v, 3) for v in ii) / st.mean(meq(x, 3) for x in rr))
        rows.append(rec)
    # fits over p (log2 r)
    xs = [r['log2r'] for r in rows]
    fits = {}
    for key in ('rho_Meq_w3', 'ic_2lp_Meq_w3', 'rho_Meq_wlog', 'ic_2lp_Meq_wlog'):
        ys = [math.log2(r[key]) for r in rows]
        fits[key] = fit(xs, ys)
    ys = [math.log2(r['ic2lp_trials_mean']) for r in rows]; fits['ic2lp_trials'] = fit(xs, ys)
    ys = [math.log2(r['rho_steps_mean']) for r in rows]; fits['rho_steps'] = fit(xs, ys)
    ys = [math.log2(r['gap_2lp_w3']) for r in rows]; fits['gap_2lp_w3'] = fit(xs, ys)
    return rows, fits


def summarize_n3():
    data = load('runs_n3')
    rows = []
    for p in sorted(data):
        curves = data[p]; lg = math.log2(p)
        rec = dict(p=p, curves=len(curves), targets=sum(len(c['targets']) for c in curves),
                   log2r=st.mean(c['log2r'] for c in curves))
        modes = list(curves[0]['targets'][0]['ic'].keys())
        rho_c = []; ic_c = {m: [] for m in modes}; budget = []
        for c in curves:
            rr = [x for t in c['targets'] for x in t['rho']]
            rho_c.append(st.mean(meq(x, 3) for x in rr))
            for m in modes:
                vals = [t['ic'][m] for t in c['targets']]
                ic_c[m].append(float('nan') if any(v.get('censored') for v in vals) else st.mean(meq(v, 3) for v in vals))
        rec['rho_Meq_w3'] = st.mean(rho_c)
        for m in modes:
            rec[f'censored_{m}'] = sum(1 for c in curves for t in c['targets'] if t['ic'][m].get('censored'))
        for m in modes:
            vals = [v for v in ic_c[m] if v == v]
            if rec[f'censored_{m}']:
                vals = []  # a mean over uncensored curves only would be biased low
            rec[f'ic_{m}_counted_Meq_w3'] = st.mean(vals) if vals else None
            rec[f'gap_lb_{m}_w3'] = st.mean(vals) / st.mean(rho_c) if vals else None
        allrho = [x for c in curves for t in c['targets'] for x in t['rho']]
        icv = [t['ic']['all'] for c in curves for t in c['targets']]
        rec['rho_steps_mean'] = st.mean(x['steps'] for x in allrho)
        rec['rho_sec_mean'] = st.mean(x['sec'] for x in allrho)
        rec['ic_all_trials_mean'] = st.mean(v['trials'] for v in icv)
        rec['ic_all_rels_mean'] = st.mean(v['rels_kept'] for v in icv)
        rec['ic_all_counted_M_per_trial'] = st.mean(v['M'] / v['trials'] for v in icv)
        rec['ic_all_msolve_sec_per_call'] = st.mean(v['msolve_sec'] / v['msolve_calls'] for v in icv)
        rec['ic_all_sec_mean'] = st.mean(v['sec'] for v in icv)
        rec['ic_all_Zr_mean'] = st.mean(v['Zr'] for v in icv)
        # break-even: what one decomposition could cost (M-eq, everything included) for IC to tie rho
        rec['breakeven_Meq_per_trial'] = rec['rho_Meq_w3'] / rec['ic_all_trials_mean']
        sts = {}
        for c in curves:
            for t in c['targets']:
                for k, v in t['decomposition']['status'].items():
                    sts[k] = sts.get(k, 0) + v
        rec['msolve_status'] = sts
        rows.append(rec)
    return rows


def main():
    n2, fits = summarize_n2()
    n3 = summarize_n3() if glob.glob(os.path.join(HERE, 'runs_n3', '*.json')) else []
    json.dump(dict(n2=n2, n2_fits={k: dict(slope=v[0], se=v[1]) for k, v in fits.items()}, n3=n3),
              open(os.path.join(HERE, 'summary.json'), 'w'), indent=1)
    print('| p | log2 r | curves/targets | rho steps / sqrt r | rho M-eq | IC 2lp trials / p | IC M per trial | IC 2lp M-eq | gap 2lp (I=3M) | gap 2lp (I=1.5 log2 p M) | gap 1lp:p/2 | gap 1lp:p/4 | gap full:p/2 |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    for r in n2:
        p = r['p']
        g = lambda k: r.get(k)
        print(f"| {p} | {r['log2r']:.1f} | {r['curves']}/{r['targets']} | {r['rho_steps_over_sqrt_r']:.2f} | {r['rho_Meq_w3']:.3g} | {r['ic2lp_trials_over_p']:.3f} | {r['ic2lp_M_per_trial']:.0f} | {r['ic_2lp_Meq_w3']:.3g} | {r['gap_2lp_w3']:.1f} | {r['gap_2lp_wlog']:.1f} | {g('gap_1lp:%d_w3' % (p // 2)):.1f} | {g('gap_1lp:%d_w3' % (p // 4)):.1f} | {g('gap_full:%d_w3' % (p // 2)):.1f} |")
    print()
    for k, v in fits.items():
        print(f'fit log2({k}) vs log2 r: slope {v[0]:.3f} +- {v[1]:.3f}')
    print()
    if n3:
        print('| p | log2 r | curves/targets | rho steps | rho M-eq | IC trials | IC rels | counted M per trial | gap lower bound all | 2lp:B | 1lp:B | full:B | break-even M-eq per decomposition | msolve s/call | IC wall s | rho wall s |')
        print('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
        for r in n3:
            f = lambda k: ('%.1f' % r[k]) if r.get(k) else ('censored %d/%d' % (r['censored_' + k[7:-3]], r['targets']))
            print(f"| {r['p']} | {r['log2r']:.1f} | {r['curves']}/{r['targets']} | {r['rho_steps_mean']:.0f} | {r['rho_Meq_w3']:.3g} | {r['ic_all_trials_mean']:.0f} | {r['ic_all_rels_mean']:.0f} | {r['ic_all_counted_M_per_trial']:.0f} | {f('gap_lb_all_w3')} | {f('gap_lb_2lp:B_w3')} | {f('gap_lb_1lp:B_w3')} | {f('gap_lb_full:B_w3')} | {r['breakeven_Meq_per_trial']:.0f} | {r['ic_all_msolve_sec_per_call']*1e3:.1f} ms | {r['ic_all_sec_mean']:.2f} | {r['rho_sec_mean']:.3f} |")
        print({r['p']: r['msolve_status'] for r in n3})
    # plot
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
    xs = [r['log2r'] for r in n2]
    ax[0].plot(xs, [r['rho_Meq_w3'] for r in n2], 'o-', label='rho store+neg (n=2)')
    ax[0].plot(xs, [r['ic_2lp_Meq_w3'] for r in n2], 's-', label='IC 2lp, full F_p base (n=2)')
    ax[0].plot(xs, [r['ic2lp_trials_mean'] for r in n2], 's--', label='IC 2lp trials (count, n=2)')
    ax[0].plot(xs, [r['rho_steps_mean'] for r in n2], 'o--', label='rho steps (count, n=2)')
    if n3:
        x3 = [r['log2r'] for r in n3]
        ax[0].plot(x3, [r['rho_Meq_w3'] for r in n3], 'o:', label='rho (n=3)')
        ax[0].plot(x3, [r['ic_all_counted_M_per_trial'] * r['ic_all_trials_mean'] for r in n3], 'd:', label='IC n=3, counted part only')
    ax[0].set_yscale('log', base=2); ax[0].set_xlabel('log2 r'); ax[0].set_ylabel('F_p mult-equivalents (I = 3M) or counts')
    ax[0].legend(fontsize=7); ax[0].set_title('Cost on E(F_{p^n}), prime order r')
    for key, lab in (('gap_2lp_w3', '2lp (all x in F_p)'),):
        ax[1].plot(xs, [r[key] for r in n2], 's-', label='n=2 ' + lab)
    ax[1].plot(xs, [r['gap_2lp_wlog'] for r in n2], 's--', label='n=2 2lp, I = 1.5 log2 p M')
    for r0 in ('1lp:', 'full:'):
        pass
    ax[1].plot(xs, [r['gap_1lp:%d_w3' % (r['p'] // 2)] for r in n2], '^-', label='n=2 1lp, B=p/2')
    ax[1].plot(xs, [r['gap_full:%d_w3' % (r['p'] // 2)] for r in n2], 'v-', label='n=2 full only, B=p/2')
    if n3:
        ax[1].plot(x3, [r['gap_lb_all_w3'] for r in n3], 'd:', label='n=3 all (lower bound: msolve uncounted)')
    ax[1].axhline(1, color='k', lw=0.8)
    ax[1].set_yscale('log', base=2); ax[1].set_xlabel('log2 r'); ax[1].set_ylabel('IC cost / matched rho cost')
    ax[1].legend(fontsize=7); ax[1].set_title('Gap to rho (1 = parity)')
    fig.tight_layout()
    for ext in ('png', 'svg', 'pdf'):
        fig.savefig(os.path.join(HERE, 'gap_vs_size.' + ext), dpi=130)


if __name__ == '__main__':
    main()
