#!/usr/bin/env python3
"""Summarize runs/*.json: gaps to matched store-all +-rho per pricing variant, fold contributions,
slopes, curve-bootstrap intervals; writes summary.json and gap_vs_size.{png,svg,pdf}."""
import glob, json, math, os, random, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))

# ------------------------------------------------------------------ pricing variants
# each returns the M-equivalent cost of one IC result v for inversion price wI, lookup price wl


def V(name, arm, run, mode, f):
    return dict(name=name, arm=arm, run=run, mode=mode, f=f)


VARIANTS = [
    V('B0 ext_field oracle', 'base', 'naive', '2lp', lambda v, wI, wl: v['M'] + wI * v['I'] + wl * v['lk']),
    V('B1 +shift (s^p mod F)', 'base', 'naive', '2lp', lambda v, wI, wl: v['M'] - v['shift_save'] + wI * v['I'] + wl * v['lk']),
    V('B2 +shift+window', 'base', 'naive', '2lp', lambda v, wI, wl: v['M'] - v['shift_save'] - v['win_save'] + wI * v['I'] + wl * v['lk']),
    V('B3 +shift+Jacobi', 'base', 'naive', '2lp', lambda v, wI, wl: v['M'] - v['shift_save'] - v['ML'] + wI * (v['I'] + v['nL']) + wl * v['lk']),
    V('B4 +shift+sqrt table', 'base', 'table', '2lp', lambda v, wI, wl: v['M'] - v['shift_save'] + wI * v['I'] + wl * v['lk']),
    V('T0n tors sym oracle, no T fold', 'tors', 'naive', 'nofold', lambda v, wI, wl: v['M'] + wI * v['I'] + wl * v['lk']),
    V('T0 tors sym oracle + T fold', 'tors', 'naive', 'fold', lambda v, wI, wl: v['M'] + wI * v['I'] + wl * v['lk']),
    V('T1 +sqrt-with-check, sigma memo', 'tors', 'memo', 'fold', lambda v, wI, wl: v['M'] + wI * v['I'] + wl * v['lk']),
    V('T1w +window', 'tors', 'memo', 'fold', lambda v, wI, wl: v['M'] - v['win_save'] + wI * v['I'] + wl * v['lk']),
    V('T1j +Jacobi', 'tors', 'memo', 'fold', lambda v, wI, wl: v['M'] - v['ML'] + wI * (v['I'] + v['nL']) + wl * v['lk']),
    V('T2 +sqrt table', 'tors', 'table', 'fold', lambda v, wI, wl: v['M'] + wI * v['I'] + wl * v['lk']),
    V('T2n sqrt table, no T fold', 'tors', 'table', 'nofold', lambda v, wI, wl: v['M'] + wI * v['I'] + wl * v['lk']),
]


def rho_cost(x, wI, wl):
    return x['M'] + wI * x['I'] + wl * x['lk']


def fit(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs); sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    b = sxy / sxx; a = my - b * mx
    res = [y - a - b * x for x, y in zip(xs, ys)]
    se = math.sqrt(sum(r * r for r in res) / max(1, n - 2) / sxx) if n > 2 else float('nan')
    return b, se, a


def load():
    data = {}
    for f in sorted(glob.glob(os.path.join(HERE, 'runs', '*.json'))):
        r = json.load(open(f))
        data.setdefault((r['arm'], r['variant'], r['p']), []).append(r)
    return data


def per_curve(curves, var, wI, wl):
    """list of (ic_mean, rho_mean) per curve; None if any target censored"""
    out = []
    for c in curves:
        vals = [t['ic'][var['mode']] for t in c['targets']]
        if any(v.get('censored') for v in vals):
            return None
        rr = [x for t in c['targets'] for x in t['rho']]
        out.append((st.mean(var['f'](v, wI, wl) for v in vals), st.mean(rho_cost(x, wI, wl) for x in rr)))
    return out


def gap_ci(pc, B=2000, seed=1):
    rng = random.Random(seed)
    g = []
    for _ in range(B):
        s = [rng.choice(pc) for _ in pc]
        g.append(sum(a for a, _ in s) / sum(b for _, b in s))
    g.sort()
    return g[int(0.025 * B)], g[int(0.975 * B)]


def main():
    data = load()
    sizes = {'base': sorted({k[2] for k in data if k[0] == 'base'}),
             'tors': sorted({k[2] for k in data if k[0] == 'tors'})}
    out = dict(unit='F_p multiplications (M); inversions I priced 3M or 1.5 log2(p) M; '
                    'table/hash lookups priced wl in {0, 0.1, 1} M (both arms)',
               sizes={}, slopes={}, checks={})
    # pairing check: rho and fold trials identical across tors variants (same rng stream)
    pair_ok = 0; pair_n = 0
    for p in sizes['tors']:
        for i, c in enumerate(data[('tors', 'naive', p)]):
            for v in ('memo', 'table'):
                c2 = [x for x in data[('tors', v, p)] if x['seed'] == c['seed']][0]
                for t1, t2 in zip(c['targets'], c2['targets']):
                    pair_n += 1
                    pair_ok += ([x['steps'] for x in t1['rho']] == [x['steps'] for x in t2['rho']]
                                and t1['ic']['fold']['trials'] == t2['ic']['fold']['trials'])
    out['checks']['tors_variant_streams_identical'] = f'{pair_ok}/{pair_n}'
    pb_ok = pb_n = 0
    for p in sizes['base']:
        for c in data[('base', 'naive', p)]:
            c2 = [x for x in data[('base', 'table', p)] if x['seed'] == c['seed']][0]
            for t1, t2 in zip(c['targets'], c2['targets']):
                pb_n += 1; pb_ok += t1['ic']['2lp']['trials'] == t2['ic']['2lp']['trials']
    out['checks']['base_naive_vs_table_same_trials'] = f'{pb_ok}/{pb_n}'
    # ext_field reproduction check (target 0 of every base curve)
    ef_ok = ef_n = 0
    for p in sizes['base']:
        for c in data[('base', 'naive', p)]:
            fn = os.path.join(HERE, '..', 'ext_field', 'runs_n2', f'p{p}_s{c["seed"]}.json')
            if os.path.exists(fn):
                e = json.load(open(fn))
                ef_n += 1
                ef_ok += (e['r'] == c['r'] and e['targets'][0]['w'] == c['targets'][0]['w']
                          and e['targets'][0]['ic'][f'2lp:{p}']['trials'] == c['targets'][0]['ic']['2lp']['trials']
                          and e['targets'][0]['ic'][f'2lp:{p}']['M'] == c['targets'][0]['ic']['2lp']['M'])
    out['checks']['ext_field_target0_reproduced'] = f'{ef_ok}/{ef_n}'
    ntarg = sum(len(c['targets']) for k, cs in data.items() for c in cs)
    nrho = sum(len(t['rho']) for k, cs in data.items() for c in cs for t in c['targets'])
    out['checks']['logs_checked'] = f'{ntarg} IC targets (every mode) and {nrho} rho runs asserted equal to the planted log'

    rows = {}
    for arm in ('base', 'tors'):
        for i, p in enumerate(sizes[arm]):
            lg = math.log2(p)
            curves0 = data[(arm, 'naive', p)]
            key = f'{arm}:{i}'
            rec = dict(arm=arm, p=p, curves=len(curves0), log2r=st.mean(c['log2r'] for c in curves0))
            allrho = [x for c in curves0 for t in c['targets'] for x in t['rho']]
            rec['rho_steps_over_sqrt_r'] = st.mean(x['steps'] / math.sqrt(c['r']) for c in curves0 for t in c['targets'] for x in t['rho'])
            rec['rho_Meq_w3'] = st.mean(rho_cost(x, 3, 0) for x in allrho)
            mode = '2lp' if arm == 'base' else 'fold'
            icv = [(t['ic'][mode], c['r']) for c in curves0 for t in c['targets']]
            rec['ic_trials_over_sqrt_r'] = st.mean(v['trials'] / math.sqrt(r) for v, r in icv)
            rec['ic_trials_over_rho_steps'] = st.mean(v['trials'] for v, _ in icv) / st.mean(x['steps'] for x in allrho)
            rec['ic_rels_per_trial'] = st.mean(v.get('rels', v.get('rels_seen', 0)) / v['trials'] for v, _ in icv)
            if arm == 'tors':
                icn = [t['ic']['nofold'] for c in curves0 for t in c['targets']]
                rec['nofold_trials_over_fold'] = st.mean(v['trials'] for v in icn) / st.mean(v['trials'] for v, _ in icv)
                rec['curve_tries_mean'] = st.mean(c['curve_tries'] for c in curves0)
            rec['gaps'] = {}
            for var in VARIANTS:
                if var['arm'] != arm:
                    continue
                curves = data[(arm, var['run'], p)]
                g = {}
                for wn, wI in (('I=3M', 3.0), ('I=1.5log2p', 1.5 * lg)):
                    for wl in (0, 0.1, 1):
                        pc = per_curve(curves, var, wI, wl)
                        if pc is None:
                            g[f'{wn},wl={wl}'] = None; continue
                        gap = sum(a for a, _ in pc) / sum(b for _, b in pc)
                        ent = dict(gap=gap)
                        if wl == 0:
                            ent['ci95'] = gap_ci(pc)
                            ent['per_curve'] = [a / b for a, b in pc]
                        g[f'{wn},wl={wl}'] = ent
                vals = [t['ic'][var['mode']] for c in curves for t in c['targets']]
                g['M_per_trial'] = st.mean(var['f'](v, 0, 0) / v['trials'] for v in vals)
                g['I_per_trial'] = st.mean(v['I'] / v['trials'] for v in vals)
                g['lk_per_trial'] = st.mean(v['lk'] / v['trials'] for v in vals)
                rec['gaps'][var['name']] = g
            rows[key] = rec
    out['sizes'] = rows
    # slopes of log2 gap vs log2 r
    for var in VARIANTS:
        for wn in ('I=3M,wl=0', 'I=1.5log2p,wl=0', 'I=3M,wl=1'):
            xs = []; ys = []
            for k, rec in rows.items():
                if rec['arm'] != var['arm']:
                    continue
                e = rec['gaps'][var['name']][wn]
                if e:
                    xs.append(rec['log2r']); ys.append(math.log2(e['gap']))
            if len(xs) >= 3:
                b, se, a = fit(xs, ys)
                out['slopes'][f"{var['name']} [{wn}]"] = dict(slope=b, se=se, intercept=a,
                                                              parity_log2r=(-a / b if b < 0 else None))
    json.dump(out, open(os.path.join(HERE, 'summary.json'), 'w'), indent=1)

    # ---------------- printed tables
    def fmt(x, n=2):
        return '-' if x is None else (f'{x:.{n}f}' if abs(x) < 100 else f'{x:.0f}')
    print('## checks'); [print(f'- {k}: {v}') for k, v in out['checks'].items()]
    for arm in ('base', 'tors'):
        names = [v['name'] for v in VARIANTS if v['arm'] == arm]
        print(f'\n## {arm}: gap to matched rho (ratio of means over curves), I = 3M, lookups 0')
        print('| p | log2 r | rho steps/sqrt r | IC trials/sqrt r | trials/rho steps | rels/trial | ' + ' | '.join(names) + ' |')
        print('|' + '---|' * (6 + len(names)))
        for k, rec in rows.items():
            if rec['arm'] != arm:
                continue
            gs = [rec['gaps'][n]['I=3M,wl=0'] for n in names]
            print(f"| {rec['p']} | {rec['log2r']:.1f} | {rec['rho_steps_over_sqrt_r']:.2f} | {rec['ic_trials_over_sqrt_r']:.3f} | "
                  f"{rec['ic_trials_over_rho_steps']:.3f} | {rec['ic_rels_per_trial']:.3f} | " +
                  ' | '.join(fmt(g['gap']) if g else '-' for g in gs) + ' |')
        for wn in ('I=1.5log2p,wl=0', 'I=3M,wl=1', 'I=3M,wl=0.1', 'I=1.5log2p,wl=1'):
            print(f'\n### {arm}, {wn}')
            print('| p | log2 r | ' + ' | '.join(names) + ' |')
            print('|' + '---|' * (2 + len(names)))
            for k, rec in rows.items():
                if rec['arm'] != arm:
                    continue
                gs = [rec['gaps'][n][wn] for n in names]
                print(f"| {rec['p']} | {rec['log2r']:.1f} | " + ' | '.join(fmt(g['gap']) if g else '-' for g in gs) + ' |')
        print(f'\n### {arm}: M per trial (I per trial, lookups per trial)')
        print('| p | ' + ' | '.join(names) + ' |')
        print('|' + '---|' * (1 + len(names)))
        for k, rec in rows.items():
            if rec['arm'] != arm:
                continue
            print(f"| {rec['p']} | " + ' | '.join(f"{rec['gaps'][n]['M_per_trial']:.0f} ({rec['gaps'][n]['I_per_trial']:.1f}, {rec['gaps'][n]['lk_per_trial']:.1f})" for n in names) + ' |')
    print('\n## best variants with 95% curve-bootstrap CI, I = 3M, lookups 0')
    for name in ('B0 ext_field oracle', 'B4 +shift+sqrt table', 'T0 tors sym oracle + T fold', 'T2 +sqrt table'):
        print(f'- {name}: ' + '; '.join(
            f"{rec['log2r']:.1f}: {rec['gaps'][name]['I=3M,wl=0']['gap']:.2f} ({rec['gaps'][name]['I=3M,wl=0']['ci95'][0]:.2f}-{rec['gaps'][name]['I=3M,wl=0']['ci95'][1]:.2f})"
            for rec in rows.values() if name in rec['gaps']))
    print('\n## tors extra')
    for k, rec in rows.items():
        if rec['arm'] == 'tors':
            print(f"- p={rec['p']}: nofold trials / fold trials = {rec['nofold_trials_over_fold']:.3f}; curve tries {rec['curve_tries_mean']:.1f}")
    print('\n## slopes of log2(gap) against log2 r')
    for k, v in out['slopes'].items():
        print(f"- {k}: {v['slope']:+.3f} +- {v['se']:.3f}" + (f"; straight-line parity at log2 r = {v['parity_log2r']:.0f}" if v['parity_log2r'] else ''))

    # ---------------- plot
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.4), sharey=True)
    show = [('B0 ext_field oracle', '#9aa0a6', 'o', '--'), ('B4 +shift+sqrt table', '#5f6368', 'o', '-'),
            ('T0n tors sym oracle, no T fold', '#f29900', 's', '--'), ('T0 tors sym oracle + T fold', '#e8710a', 's', '-'),
            ('T1 +sqrt-with-check, sigma memo', '#1a73e8', '^', '-'), ('T2 +sqrt table', '#0b8043', 'D', '-')]
    for ax, wn, title in zip(axes, ('I=3M,wl=0', 'I=1.5log2p,wl=0'), ('I = 3M', 'I = 1.5 log2(p) M')):
        for name, col, mk, ls in show:
            xs = []; ys = []; lo = []; hi = []
            for rec in rows.values():
                if name in rec['gaps'] and rec['gaps'][name][wn]:
                    e = rec['gaps'][name][wn]
                    xs.append(rec['log2r']); ys.append(e['gap']); lo.append(e['gap'] - e['ci95'][0]); hi.append(e['ci95'][1] - e['gap'])
            ax.errorbar(xs, ys, yerr=[lo, hi], color=col, marker=mk, ls=ls, label=name, capsize=2, lw=1.5, ms=5)
        ax.axhline(1, color='k', lw=0.8)
        ax.set_yscale('log'); ax.set_xlabel('log2 r'); ax.set_title(f'gap to matched store-all +-rho, {title}')
        ax.grid(alpha=0.3, which='both')
    axes[0].set_ylabel('IC cost / rho cost (F_p mult. equiv.)')
    axes[1].legend(fontsize=7, loc='upper right')
    fig.tight_layout()
    for ext in ('png', 'svg', 'pdf'):
        fig.savefig(os.path.join(HERE, f'gap_vs_size.{ext}'), dpi=130)


if __name__ == '__main__':
    main()
