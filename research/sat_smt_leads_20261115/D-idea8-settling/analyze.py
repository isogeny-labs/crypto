"""Analysis fixed by PROTOCOL.md. usage: python3 analyze.py [dir]  -> results.json, analysis.json,
analysis.md, ratio_vs_position.svg/.png"""
import json, sys, glob, math, os
import numpy as np

D = sys.argv[1] if len(sys.argv) > 1 else '.'
NB = 10000
SEEDS = (0, 1, 2)


def load():
    metas, rows, ends = [], [], []
    for f in sorted(glob.glob(os.path.join(D, 'raw_l*_s*_*.jsonl'))):
        for L in open(f):
            r = json.loads(L)
            r['file'] = os.path.basename(f)
            (metas if r['kind'] == 'meta' else ends if r['kind'] == 'end' else rows).append(r)
    return metas, rows, ends


metas, rows, ends = load()
LS = sorted({m['l'] for m in metas})
tab = {}                                   # (l, s, arm) -> idx -> row
for r in rows:
    tab.setdefault((r['l'], r['seed_index'], r['arm']), {})[r['idx']] = r
NSTAR = {l: min(len(tab[(l, s, 'INC')]) for s in SEEDS) for l in LS}
L0 = {l: (2 * NSTAR[l]) // 3 for l in LS}
targets = {}
for m in metas:
    targets.setdefault((m['l'], m['seed_index']), m['targets'])
    assert targets[(m['l'], m['seed_index'])][:m['n_targets']] == m['targets'][:m['n_targets']] or \
        targets[(m['l'], m['seed_index'])][:len(m['targets'])] == m['targets'], 'target lists differ between processes'


def pairs(l, s, num, den, metric, lo, hi):
    """(idx, ln(num/den)) on non-decomposable targets lo <= idx < hi that both arms solved."""
    a, b = tab.get((l, s, num), {}), tab.get((l, s, den), {})
    out = []
    for i in range(lo, min(hi, NSTAR[l])):
        if i in a and i in b and not a[i]['decomposable']:
            out.append((i, math.log(max(a[i][metric], 1e-9) / max(b[i][metric], 1e-9))
                        if metric.startswith('secs') else
                        math.log(max(a[i][metric], 1) / max(b[i][metric], 1))))
    return out


def two_stage(per_seed, stat, seed):
    """per_seed: list of (k x 2) arrays (ln p or idx, ln ratio). Resample seeds, then targets."""
    rng = np.random.default_rng(seed)
    res = []
    k = len(per_seed)
    for _ in range(NB):
        parts = []
        for j in rng.integers(0, k, k):
            a = per_seed[j]
            parts.append(a[rng.integers(0, len(a), len(a))])
        res.append(stat(np.concatenate(parts)))
    return np.array(res)


def gm_stat(a):
    return a[:, 1].mean()


def fit_stat(a):
    x = np.log(a[:, 0] + 1.0); y = a[:, 1]
    xm, ym = x.mean(), y.mean()
    sl = ((x - xm) * (y - ym)).sum() / ((x - xm) ** 2).sum()
    return np.array([ym - sl * xm, -sl])          # a, beta  (ln rho = a - beta ln p)


def cross(a, beta, level):
    """position p at which exp(a) p^-beta = level; 1 if already below at p=1; inf if never."""
    c = a - math.log(level)
    if c <= 0:
        return 1.0
    if beta <= 0:
        return math.inf
    return math.exp(min(c / beta, 700))


def pct(v, q):
    return float(np.percentile(v, q))


def gm_block(l, num, den, metric, lo, hi, seed):
    ps = [np.array(pairs(l, s, num, den, metric, lo, hi), dtype=float).reshape(-1, 2) for s in SEEDS]
    if any(len(p) == 0 for p in ps):
        return None
    allp = np.concatenate(ps)
    b = two_stage(ps, gm_stat, seed)
    return dict(n=[len(p) for p in ps], gm=math.exp(allp[:, 1].mean()),
                lo=math.exp(pct(b, 2.5)), hi=math.exp(pct(b, 97.5)),
                per_seed=[math.exp(p[:, 1].mean()) for p in ps],
                sd_ln=float(allp[:, 1].std(ddof=1)), _boot=b)


def fit_block(l, num, den, metric, hi, seed):
    ps = [np.array(pairs(l, s, num, den, metric, 1, hi), dtype=float).reshape(-1, 2) for s in SEEDS]
    allp = np.concatenate(ps)
    a, beta = fit_stat(allp)
    b = two_stage(ps, fit_stat, seed)
    out = dict(n=[len(p) for p in ps], a=float(a), beta=float(beta),
               a_ci=[pct(b[:, 0], 2.5), pct(b[:, 0], 97.5)], beta_ci=[pct(b[:, 1], 2.5), pct(b[:, 1], 97.5)],
               ratio_at_p1=math.exp(a), per_seed=[dict(zip(('a', 'beta'), map(float, fit_stat(p)))) for p in ps],
               _boot=b)
    for lev, name in ((1.0, 'cross_1.0'), (0.5, 'cross_0.5')):
        c = np.array([cross(x, y, lev) for x, y in b])
        out[name] = dict(point=cross(a, beta, lev), lo=pct(c, 2.5), hi=pct(c, 97.5),
                         frac_never=float(np.isinf(c).mean()), frac_already=float((c == 1.0).mean()))
    return out


def clean(o):
    if isinstance(o, dict):
        return {k: clean(v) for k, v in o.items() if not str(k).startswith('_')}
    if isinstance(o, list):
        return [clean(v) for v in o]
    if isinstance(o, float) and math.isinf(o):
        return 'never'
    return o


A = dict(nstar=NSTAR, last_third_from=L0, cells={})
for l in LS:
    N = NSTAR[l]; c = {}
    c['table'] = []
    for s in SEEDS:
        m = next(m for m in metas if m['l'] == l and m['seed_index'] == s)
        T = targets[(l, s)][:N]
        nd = sum(1 for _, d in T if d)
        c['table'].append(dict(seed=s, V=m['V'], signed_base=m['signed_base'], nonrational_in_V=m['nonrational_in_V'],
                               twist_pair_elements=m['twist_pair_elements'], vars=m['vars'], clauses=m['clauses'],
                               inc_targets_run=len(tab[(l, s, 'INC')]), targets_analysed=N,
                               decomposable_targets=nd, decomposable_abscissae=m['decomposable_abscissae_exact'],
                               nondec_targets=N - nd,
                               nondec_abscissae=m['rational_abscissae_outside_V'] - m['decomposable_abscissae_exact'],
                               solves={a: sum(1 for i in tab.get((l, s, a), {}) if i < N) for a in ('INC', 'SPEC', 'FP', 'AA')},
                               errors=sum(1 for a in ('INC', 'SPEC', 'FP', 'AA') for r in tab.get((l, s, a), {}).values() if r['err'])))
    for num, den in (('INC', 'SPEC'), ('INC', 'FP'), ('FP', 'SPEC'), ('AA', 'SPEC')):
        for metric in ('propagations', 'conflicts', 'secs_solve'):
            c['%s/%s %s last third' % (num, den, metric)] = gm_block(l, num, den, metric, L0[l], N, 1508)
        c['%s/%s propagations first third' % (num, den)] = gm_block(l, num, den, 'propagations', 1, N // 3, 1508)
        if num == 'INC':
            c['%s/%s propagations fit' % (num, den)] = fit_block(l, num, den, 'propagations', N, 1509)
            c['%s/%s conflicts fit' % (num, den)] = fit_block(l, num, den, 'conflicts', N, 1509)
    med = {}
    for a in ('INC', 'SPEC', 'FP', 'AA'):
        for metric in ('propagations', 'conflicts'):
            v = [r[metric] for s in SEEDS for i, r in tab.get((l, s, a), {}).items()
                 if L0[l] <= i < N and not r['decomposable']]
            med['%s %s' % (a, metric)] = float(np.median(v)) if v else None
    c['median last third'] = med; c['scan_leaves'] = 2 ** l
    for a in ('INC', 'SPEC', 'FP'):
        c['secs_solve sum last third %s' % a] = [
            round(sum(r['secs_solve'] for i, r in tab.get((l, s, a), {}).items()
                      if L0[l] <= i < N and not r['decomposable'] and i in tab.get((l, s, 'SPEC'), {})
                      and i in tab.get((l, s, 'FP' if a == 'FP' else 'SPEC'), {})), 2) for s in SEEDS]
    A['cells'][l] = c

if 5 in LS:
    p = A['cells'][5]['INC/SPEC propagations last third']
    A['decision'] = 'ALIVE' if p['hi'] < 0.9 else 'KILL' if p['lo'] > 1.0 else 'INCONCLUSIVE'
if 4 in LS and 5 in LS:
    lo5, n5 = L0[5], NSTAR[5]
    tr = {}
    for num, den in (('INC', 'SPEC'), ('INC', 'FP')):
        g4 = gm_block(4, num, den, 'propagations', lo5, n5, 1510)
        g5 = gm_block(5, num, den, 'propagations', lo5, n5, 1511)
        d = g5['_boot'] - g4['_boot']                   # independent resampling per cell
        f4 = fit_block(4, num, den, 'propagations', n5, 1510)
        f5 = fit_block(5, num, den, 'propagations', n5, 1511)
        db = f5['_boot'][:, 1] - f4['_boot'][:, 1]
        tr['%s/%s' % (num, den)] = dict(window=[lo5, n5], l4=g4, l5=g5,
                                        dln=math.log(g5['gm'] / g4['gm']), dln_ci=[pct(d, 2.5), pct(d, 97.5)],
                                        beta4_matched=f4['beta'], beta4_matched_ci=f4['beta_ci'], beta5=f5['beta'],
                                        dbeta=f5['beta'] - f4['beta'], dbeta_ci=[pct(db, 2.5), pct(db, 97.5)])
    A['l_trend'] = tr

json.dump(clean(A), open(os.path.join(D, 'analysis.json'), 'w'), indent=1)
json.dump(dict(tag='[pcc:unverified]', protocol='PROTOCOL.md', meta=metas, end=ends, raw_rows=rows,
               analysis=clean(A)), open(os.path.join(D, 'results.json'), 'w'))

# ---------------- text ----------------
out = []
W = out.append


def g(b):
    return 'n/a' if b is None else '%.3f (%.3f..%.3f)' % (b['gm'], b['lo'], b['hi'])


def cr(c):
    f = lambda v: 'never' if math.isinf(v) else ('already below' if v == 1.0 else '%.3g' % v)
    return '%s (%s..%s)' % (f(c['point']), f(c['lo']), f(c['hi']))


W('decision (primary rule): %s' % A.get('decision'))
for l in LS:
    c = A['cells'][l]
    W('\n## l=%d, N*=%d, last third from idx %d, 2+1 scan reference %d leaf solves per target' % (l, NSTAR[l], L0[l], 2 ** l))
    W('| seed | signed base | non-rational x in V | INC targets run | analysed | decomposable targets / decomposable abscissae | non-dec targets / non-dec abscissae | solves INC,SPEC,FP,AA | errors |')
    W('|---|---|---|---|---|---|---|---|---|')
    for t in c['table']:
        W('| %d | %d | %d | %d | %d | %d / %d (%.1f%%) | %d / %d (%.1f%%) | %s | %d |' % (
            t['seed'], t['signed_base'], t['nonrational_in_V'], t['inc_targets_run'], t['targets_analysed'],
            t['decomposable_targets'], t['decomposable_abscissae'], 100 * t['decomposable_targets'] / t['decomposable_abscissae'],
            t['nondec_targets'], t['nondec_abscissae'], 100 * t['nondec_targets'] / t['nondec_abscissae'],
            ','.join(str(t['solves'][a]) for a in ('INC', 'SPEC', 'FP', 'AA')), t['errors']))
    W('\n| ratio | metric | last third GM (95%) | per seed | n per seed | first third GM (95%) |')
    W('|---|---|---|---|---|---|')
    for num, den in (('INC', 'SPEC'), ('INC', 'FP'), ('FP', 'SPEC'), ('AA', 'SPEC')):
        for metric in ('propagations', 'conflicts', 'secs_solve'):
            b = c['%s/%s %s last third' % (num, den, metric)]
            if b is None:
                continue
            ft = g(c['%s/%s propagations first third' % (num, den)]) if metric == 'propagations' else ''
            W('| %s/%s | %s | %s | %s | %s | %s |' % (num, den, metric, g(b), ', '.join('%.3f' % x for x in b['per_seed']),
                                                   ','.join(map(str, b['n'])), ft))
    W('\nA/A sd of ln(AA/SPEC) propagations, last third: %.3f' % c['AA/SPEC propagations last third']['sd_ln'])
    W('\n| fit ln ratio = a - beta ln p | ratio at p=1 | beta (95%) | per-seed beta | p where ratio = 1.0 (95%) | p where ratio = 0.5 (95%) | resamples never reaching 1.0 / 0.5 |')
    W('|---|---|---|---|---|---|---|')
    for k in ('INC/SPEC propagations fit', 'INC/SPEC conflicts fit', 'INC/FP propagations fit', 'INC/FP conflicts fit'):
        f = c[k]
        W('| %s | %.3f | %.4f (%.4f..%.4f) | %s | %s | %s | %.0f%% / %.0f%% |' % (
            k[:-4], f['ratio_at_p1'], f['beta'], f['beta_ci'][0], f['beta_ci'][1],
            ', '.join('%.4f' % x['beta'] for x in f['per_seed']), cr(f['cross_1.0']), cr(f['cross_0.5']),
            100 * f['cross_1.0']['frac_never'], 100 * f['cross_0.5']['frac_never']))
    W('\nmedians per target, last third, non-decomposable: ' + '; '.join('%s %s' % (k, ('%.0f' % v) if v is not None else 'n/a') for k, v in c['median last third'].items()))
    W('summed solver seconds on the last-third set, per seed: INC %s, SPEC %s' % (c['secs_solve sum last third INC'], c['secs_solve sum last third SPEC']))
if 'l_trend' in A:
    W('\n## l-trend on matched positions %s' % A['l_trend']['INC/SPEC']['window'])
    for k, t in A['l_trend'].items():
        W('%s propagations: l=4 %s; l=5 %s; ln GM(5) - ln GM(4) = %+.3f (%+.3f..%+.3f); beta l=4 (p <= N*(5)) %.4f (%.4f..%.4f), beta l=5 %.4f, difference %+.4f (%+.4f..%+.4f)' % (
            k, g(t['l4']), g(t['l5']), t['dln'], t['dln_ci'][0], t['dln_ci'][1], t['beta4_matched'],
            t['beta4_matched_ci'][0], t['beta4_matched_ci'][1], t['beta5'], t['dbeta'], t['dbeta_ci'][0], t['dbeta_ci'][1]))
open(os.path.join(D, 'analysis.md'), 'w').write('\n'.join(out) + '\n')
print('\n'.join(out))

# ---------------- figure ----------------
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
COL = ['#0072B2', '#D55E00', '#009E73']
fig, axs = plt.subplots(2, len(LS), figsize=(6.2 * len(LS), 8.2), squeeze=False)
for j, l in enumerate(sorted(LS, reverse=True)):
    for i, (num, den) in enumerate((('INC', 'SPEC'), ('INC', 'FP'))):
        ax = axs[i][j]; N = NSTAR[l]
        edges = np.unique(np.round(np.geomspace(1, N, 13)).astype(int))
        for s in SEEDS:
            p = np.array(pairs(l, s, num, den, 'propagations', 1, N), dtype=float).reshape(-1, 2)
            ax.plot(p[:, 0] + 1, np.exp(p[:, 1]), '.', color=COL[s], alpha=0.12, ms=3, zorder=1)
            bx, by = [], []
            for e0, e1 in zip(edges[:-1], edges[1:]):
                sel = (p[:, 0] + 1 >= e0) & (p[:, 0] + 1 < e1 if e1 < edges[-1] else p[:, 0] + 1 <= e1)
                if sel.sum() >= 3:
                    bx.append(math.exp(np.log(p[sel, 0] + 1).mean())); by.append(math.exp(p[sel, 1].mean()))
            m = next(m for m in metas if m['l'] == l and m['seed_index'] == s)
            ax.plot(bx, by, '-o', color=COL[s], lw=1.8, ms=4.5, zorder=3,
                    label='V seed %d (%d base points), binned geometric mean' % (s, m['signed_base']))
        f = A['cells'][l]['%s/%s propagations fit' % (num, den)]
        xx = np.geomspace(2, N, 50)
        ax.plot(xx, np.exp(f['a']) * xx ** (-f['beta']), 'k--', lw=1.3, zorder=4,
                label='pooled fit: %.2f p^(-%.3f)' % (math.exp(f['a']), f['beta']) if f['beta'] >= 0 else
                'pooled fit: %.2f p^(+%.3f)' % (math.exp(f['a']), -f['beta']))
        ax.axhline(1.0, color='0.35', lw=1)
        ax.axvspan(L0[l] + 1, N, color='0.9', zorder=0, label='last third (primary set)' if l == 5 else 'last third')
        ax.set_xscale('log'); ax.set_yscale('log')
        ys = np.concatenate([np.exp(np.array(pairs(l, s, num, den, 'propagations', 1, N), dtype=float).reshape(-1, 2)[:, 1]) for s in SEEDS])
        ax.set_ylim(max(0.2, np.percentile(ys, 0.5) * 0.9), min(8, np.percentile(ys, 99.5) * 1.1))
        ax.yaxis.set_major_formatter(matplotlib.ticker.FormatStrFormatter('%.2g'))
        ax.yaxis.set_minor_formatter(matplotlib.ticker.FormatStrFormatter('%.2g'))
        ax.tick_params(axis='y', which='minor', labelsize=7)
        ax.set_xlabel('position in the sequence (targets solved by INC so far + 1)')
        ax.set_ylabel('propagations %s / %s per target' % (num, 'FRESH-PARAM' if den == 'FP' else den))
        ax.set_title('l=%d, n=%d: %s / %s, non-decomposable targets' % (l, m['n'], num, 'FRESH-PARAM' if den == 'FP' else den), fontsize=10)
        ax.grid(True, which='major', color='0.88', lw=0.6)
        ax.legend(fontsize=7, loc='best', frameon=False)
fig.suptitle('Incremental (INC) vs fresh solver, propagation ratio along the target sequence; above 1 = INC does more work  [pcc:unverified]', fontsize=10)
fig.tight_layout(rect=(0, 0, 1, 0.97))
fig.savefig(os.path.join(D, 'ratio_vs_position.svg')); fig.savefig(os.path.join(D, 'ratio_vs_position.png'), dpi=140)
