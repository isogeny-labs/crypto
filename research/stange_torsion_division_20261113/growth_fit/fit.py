"""Fit growth exponents of per-target decomposition cost vs factor-base size n.

Inputs (read only): ../sym_m4/m4_p*.json, ../sym_m4/split/*.json, ../sym_m4/speed/*.json,
../sp04/ms_p*.json, ../sp04/gb_p*.out, ../sp04/gb.out, ../ic_e2e/runs*/e2e_*.json,
and this directory's runs/pt_*.json (per-target replicate, m4_pertarget.py).
Outputs: fits.json, fit_table.md, growth_fit.{png,svg,pdf}.
Usage: python3 fit.py   (deterministic: bootstrap RNG seeded)
"""
import json, glob, math, os, re, random, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(HERE)
RNG = random.Random(20261115); NB = 4000
FLOOR_MSOLVE = 3e-3   # s; ~3x the largest spread of the trivial-system baseline seen (0.90 ms)
FLOOR_PY = 5e-5       # s; in-process python calls below 50 us are dominated by call overhead
TCRIT = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306, 9: 2.262, 10: 2.228,
         11: 2.201, 12: 2.179, 13: 2.160, 14: 2.145, 15: 2.131, 16: 2.120}
def tcrit(df): return TCRIT.get(df, 1.96 if df > 30 else 2.04)

def ols(xs, ys):
    k = len(xs); mx = sum(xs) / k; my = sum(ys) / k
    sxx = sum((x - mx) ** 2 for x in xs)
    if sxx == 0: return None
    b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx; a = my - b * mx
    if k > 2:
        s2 = sum((y - a - b * x) ** 2 for x, y in zip(xs, ys)) / (k - 2); se = math.sqrt(s2 / sxx)
    else: se = float('nan')
    return a, b, se
def pct(v, q):
    v = sorted(v); i = q * (len(v) - 1); lo = int(math.floor(i)); hi = min(lo + 1, len(v) - 1)
    return v[lo] + (v[hi] - v[lo]) * (i - lo)
def loglog(cells):  # cells: list of (n, y)
    return [math.log(n) for n, _ in cells], [math.log(y) for _, y in cells]

def boot_sizes(cells):
    """Cluster bootstrap over sizes: resample distinct n with replacement, keep all cells at each drawn n."""
    by = {}
    for n, y in cells: by.setdefault(n, []).append((n, y))
    ns = sorted(by); out = []
    if len(ns) < 4: return None  # with 3 sizes every admissible resample is the original set
    for _ in range(NB):
        draw = [RNG.choice(ns) for _ in ns]
        if len(set(draw)) < 3: continue
        cc = [c for d in draw for c in by[d]]
        f = ols(*loglog(cc))
        if f: out.append(f[1])
    return out

def summarize(cells, boot):
    xs, ys = loglog(cells); a, b, se = ols(xs, ys); ns = sorted({n for n, _ in cells})
    k = len(cells)
    r = {'n_values': ns, 'cells': k, 'slope': b, 'ols_se': se,
         'ols_t95': [b - tcrit(k - 2) * se, b + tcrit(k - 2) * se] if k > 2 else None}
    if boot: r.update({'boot95': [pct(boot, .025), pct(boot, .975)], 'boot_reps': len(boot)})
    # local slope between the two largest sizes (geometric mean of replicate cells)
    g = {}
    for n, y in cells: g.setdefault(n, []).append(math.log(y))
    if len(ns) >= 2:
        n1, n2 = ns[-2], ns[-1]
        r['last_two_slope'] = (st.mean(g[n2]) - st.mean(g[n1])) / (math.log(n2) - math.log(n1))
    return r

FITS = {}; EXCL = []
# ---------- A. saved sym_m4 outputs (medians over 8 targets; bootstrap over sizes) ----------
src = {'m4': sorted(glob.glob(f'{ROOT}/sym_m4/m4_p*.json')), 'split': sorted(glob.glob(f'{ROOT}/sym_m4/split/*.json')),
       'speed': sorted(glob.glob(f'{ROOT}/sym_m4/speed/speed_*.json')), 'hash': sorted(glob.glob(f'{ROOT}/sym_m4/speed/hash_*.json'))}
RELKEY = {'D': 'dense', 'Ds': 'dense', 'Dy': 'dense', 'M': 'dense', 'Mt': 'dense', 'Bn': 'binomial', 'Bs': 'binomial', 'By': 'binomial'}
saved = {}  # arm -> list of dict(n, med, min, rel, file)
for tag, files in src.items():
    for f in files:
        d = json.load(open(f))
        for arm, med in d['median_s'].items():
            rel = d['partitioned_relation_multisets']['Dp'] if arm == 'Dp' else d['valid_relation_multisets'][RELKEY[arm]]
            saved.setdefault(arm, []).append({'n': d['n'], 'p': d['p'], 'med': med, 'min': d['min_s'][arm], 'rel': rel,
                                              'targets': d['targets'], 'file': os.path.relpath(f, ROOT)})
for arm, rows in sorted(saved.items()):
    py = arm in ('M', 'Mt'); floor = FLOOR_PY if py else FLOOR_MSOLVE
    keep = []
    for r in rows:
        if r['min'] <= 0 or r['med'] < floor:
            EXCL.append(f"saved {arm} n={r['n']} ({r['file']}): median {r['med']*1e3:.3f} ms, min {r['min']*1e3:.3f} ms")
        else: keep.append(r)
    cells = [(r['n'], r['med']) for r in keep]
    if len({c[0] for c in cells}) >= 3:
        FITS[f'saved/{arm}/per_target'] = summarize(cells, boot_sizes(cells)) | {'unit': 'median s per target', 'files': sorted({r['file'] for r in keep})}
    cr = [(r['n'], r['med'] * r['targets'] / r['rel']) for r in keep if r['rel'] > 0]
    if len({c[0] for c in cr}) >= 3:
        FITS[f'saved/{arm}/per_relation'] = summarize(cr, boot_sizes(cr)) | {'unit': 'median s x targets / relation multisets'}

# ---------- B. per-target replicate (bootstrap over targets, stratified by n) ----------
pt = {}
for f in glob.glob(f'{HERE}/runs/pt_*.json'):
    d = json.load(open(f)); pt[d['n']] = d
def boot_targets(arm, relkey, floor):
    ns = sorted(n for n in pt if arm in pt[n]['arms'])
    cells_t, cells_r = [], []
    usable = []
    for n in ns:
        tt = [r['time_s'][arm] for r in pt[n]['per_target']]
        if min(tt) <= 0 or st.median(tt) < floor:
            EXCL.append(f"rerun {arm} n={n}: median {st.median(tt)*1e3:.3f} ms, min {min(tt)*1e3:.3f} ms"); continue
        usable.append(n); cells_t.append((n, st.median(tt)))
        rel = sum(r[relkey] for r in pt[n]['per_target'])
        if rel > 0: cells_r.append((n, sum(tt) / rel))
    out = {}
    if len(usable) < 3: return out
    bt, br, drop = [], [], 0
    for _ in range(NB):
        ct, cr = [], []
        for n in usable:
            rows = pt[n]['per_target']; s = [RNG.choice(rows) for _ in rows]
            tt = [r['time_s'][arm] for r in s]; ct.append((n, st.median(tt)))
            rel = sum(r[relkey] for r in s)
            if rel > 0: cr.append((n, sum(tt) / rel))
        bt.append(ols(*loglog(ct))[1])
        if len(cr) == len(usable): br.append(ols(*loglog(cr))[1])
        else: drop += 1
    out['per_target'] = summarize(cells_t, bt) | {'unit': 'median s per target', 'boot': 'targets within n'}
    if len(cells_r) == len(usable):
        out['per_relation'] = summarize(cells_r, br) | {'unit': 'sum s / relation multisets', 'boot': 'targets within n', 'boot_reps_dropped_zero_relations': drop}
    return out
for arm, relkey in [('D', 'rel_dense_multisets'), ('Bs', 'rel_binomial_multisets'), ('Dp', 'rel_Dp_multisets'),
                    ('M', 'rel_dense_multisets'), ('Mt', 'rel_dense_multisets')]:
    for k, v in boot_targets(arm, relkey, FLOOR_PY if arm in ('M', 'Mt') else FLOOR_MSOLVE).items():
        FITS[f'rerun/{arm}/{k}'] = v
# exact point-addition counts (deterministic; no interval needed)
for arm in ('M', 'Mt'):
    cells = [(n, st.median(r[f'adds_{arm}'] for r in pt[n]['per_target'])) for n in sorted(pt)]
    FITS[f'rerun/{arm}/adds_per_target'] = summarize(cells, None) | {'unit': 'point additions per target (exact count)'}
cells = [(n, pt[n]['Mt_table_adds']) for n in sorted(pt)]
FITS['rerun/Mt/table_adds_once'] = summarize(cells, None) | {'unit': 'point additions, one-time pair table'}

# ---------- C. sp04 (m = 2, S3) ----------
sp = []
for f in sorted(glob.glob(f'{ROOT}/sp04/ms_p*.json')):
    if os.path.getsize(f) == 0: EXCL.append(f"sp04 {os.path.basename(f)}: empty (msolve segfault, see sp04/README.md)"); continue
    sp.append(json.load(open(f)))
for arm in ('E', 'I', 'P', 'B'):
    keep = []
    for d in sp:
        if d['min_s'][arm] <= 0 or d['median_s'][arm] < FLOOR_MSOLVE:
            EXCL.append(f"sp04 msolve {arm} n={d['n']}: median {d['median_s'][arm]*1e3:.3f} ms, min {d['min_s'][arm]*1e3:.3f} ms")
        else: keep.append((d['n'], d['median_s'][arm]))
    if len(keep) >= 2:
        xs, ys = loglog(keep); a, b, se = ols(xs, ys)
        FITS[f'sp04_msolve/{arm}/per_target'] = {'n_values': sorted(k[0] for k in keep), 'cells': len(keep), 'slope': b,
            'ols_se': se, 'note': 'too few sizes for an interval; slope is descriptive', 'unit': 'median s per target (m=2, n ~ sqrt p)'}
gbcells = {}
for f in sorted(glob.glob(f'{ROOT}/sp04/gb*.out')):
    for line in open(f):
        m = re.match(r'\s*(\d+) \(.*?\) \| ([\d.]+)\[.*?\] \| ([\d.]+)\[.*?\] \| ([\d.]+)\[', line)
        if m:
            n = int(m.group(1))
            for arm, g in zip(('plain', 'image', 'pulled'), (2, 3, 4)):
                gbcells.setdefault(arm, []).append((n, float(m.group(g))))
for arm, cells in gbcells.items():
    FITS[f'sp04_sympy/{arm}/per_target'] = summarize(cells, boot_sizes(cells)) | {'unit': 'median s per target (sympy, m=2, 4 primes pooled)'}

# ---------- D. ic_e2e totals vs p (context for the cost model) ----------
e2e = {}
for sub, key in (('runs', 'ic'), ('runs_lp_slack1', None)):
    for f in glob.glob(f'{ROOT}/ic_e2e/{sub}/*.json'):
        d = json.load(open(f))
        if key and key in d:
            for c in d[key]: e2e.setdefault(('IC', d['p'], c['n']), []).append(c['total_ops'])
            e2e.setdefault(('rhoBrent', d['p'], 0), []).append(d['rho_ops_mean'])
e2e_best = {}
for (arm, p_, n), v in e2e.items():
    m = st.mean(v); e2e_best[(arm, p_)] = min(e2e_best.get((arm, p_), 1e99), m)
for arm in ('IC', 'rhoBrent'):
    cells = sorted((p_, v) for (a, p_), v in e2e_best.items() if a == arm)
    if len(cells) >= 2:
        xs, ys = loglog(cells); a, b, se = ols(xs, ys)
        FITS[f'ic_e2e/{arm}/total_ops_vs_p'] = {'p_values': [c[0] for c in cells], 'slope': b, 'ols_se': se,
            'unit': 'group additions, best n per p, mean of 3 curves', 'note': 'exponent of p, not of n'}

json.dump({'fits': FITS, 'excluded_cells': EXCL, 'floors_s': {'msolve': FLOOR_MSOLVE, 'python': FLOOR_PY},
           'bootstrap_reps': NB, 'rng_seed': 20261115}, open(f'{HERE}/fits.json', 'w'), indent=1)

# ---------- table ----------
def fmt(x): return '-' if x is None else f'{x:.2f}'
lines = ['| fit | unit | sizes n | slope alpha | 95% interval (method) | OLS t 95% | last-two-sizes slope | upper bound < 1? |', '|---|---|---|---|---|---|---|---|']
for k, v in FITS.items():
    if k.startswith('ic_e2e'): continue
    if 'boot95' in v: iv = f"[{v['boot95'][0]:.2f}, {v['boot95'][1]:.2f}] ({'targets' if k.startswith('rerun') else 'sizes'})"; ub = v['boot95'][1]
    elif v.get('ols_t95'): iv = 'exact count' if 'adds' in k else '-'; ub = v['ols_t95'][1] if 'adds' not in k else v['slope']
    else: iv = '-'; ub = None
    t95 = f"[{v['ols_t95'][0]:.2f}, {v['ols_t95'][1]:.2f}]" if v.get('ols_t95') else '-'
    ns = v['n_values']; nr = f"{ns[0]}-{ns[-1]} ({len(ns)})"
    lines.append(f"| {k} | {v['unit']} | {nr} | {v['slope']:.2f} | {iv} | {t95} | {fmt(v.get('last_two_slope'))} | {'-' if ub is None else ('yes' if ub < 1 else 'no')} |")
open(f'{HERE}/fit_table.md', 'w').write('\n'.join(lines) + '\n')
print('\n'.join(lines)); print('\nexcluded:'); print('\n'.join(EXCL))
for k, v in FITS.items():
    if k.startswith('ic_e2e'): print(k, v)

# ---------- plot ----------
import matplotlib; matplotlib.use('Agg'); import matplotlib.pyplot as plt
fig, ax = plt.subplots(1, 2, figsize=(12, 5.2))
COL = {'D': '#1f77b4', 'Bn': '#aec7e8', 'Bs': '#ff7f0e', 'Ds': '#2ca02c', 'Dy': '#9467bd', 'By': '#c5b0d5', 'Dp': '#d62728', 'M': '#8c564b', 'Mt': '#e377c2'}
for arm, rows in sorted(saved.items()):
    ok = [r for r in rows if r['min'] > 0 and r['med'] >= (FLOOR_PY if arm in ('M', 'Mt') else FLOOR_MSOLVE)]
    bad = [r for r in rows if r not in ok]
    ax[0].scatter([r['n'] for r in ok], [r['med'] for r in ok], color=COL[arm], s=22, label=f'{arm} (saved)')
    ax[0].scatter([r['n'] for r in bad], [max(r['med'], 1e-6) for r in bad], facecolors='none', edgecolors=COL[arm], s=22)
    f = FITS.get(f'saved/{arm}/per_target')
    if f:
        xs, ys = loglog([(r['n'], r['med']) for r in ok]); a, b, _ = ols(xs, ys)
        nn = [min(f['n_values']), max(f['n_values'])]; ax[0].plot(nn, [math.exp(a) * x ** b for x in nn], color=COL[arm], lw=1)
for arm in ('D', 'Bs', 'Dp', 'M', 'Mt'):
    ns = sorted(n for n in pt if arm in pt[n]['arms'])
    ax[0].scatter(ns, [max(st.median(r['time_s'][arm] for r in pt[n]['per_target']), 1e-6) for n in ns], marker='x', color=COL[arm], s=30)
ax[0].axhline(FLOOR_MSOLVE, color='grey', ls=':', lw=1); ax[0].text(8.2, FLOOR_MSOLVE * 1.15, 'msolve floor 3 ms', fontsize=7, color='grey')
for slope, lab, y0 in ((1, 'slope 1 (n^1)', 2e-4), (4, 'slope 4 (n^4)', 2e-3)):
    ax[0].plot([8, 60], [y0 * (x / 8) ** slope for x in (8, 60)], color='k', ls='--', lw=.8); ax[0].text(8.3, y0 * 0.6, lab, fontsize=7)
ax[0].axhline(FLOOR_PY, color='grey', ls='-.', lw=.8); ax[0].text(30, FLOOR_PY * 1.15, 'python floor 50 us (M, Mt)', fontsize=7, color='grey')
ax[0].set_xscale('log'); ax[0].set_yscale('log'); ax[0].set_ylim(3e-6, 2e3); ax[0].set_xlabel('factor-base size n (p ~ n^3/3)'); ax[0].set_ylabel('median seconds per target')
ax[0].set_title('Per-target decomposition time (dots: saved sym_m4; x: per-target rerun;\nopen: excluded at noise floor)', fontsize=9)
ax[0].legend(fontsize=7, ncol=3, loc='upper left', framealpha=.9)
ns = sorted(pt)
ax[1].plot(ns, [st.median(r['adds_M'] for r in pt[n]['per_target']) for n in ns], 'o-', color=COL['M'], label='M: adds per target')
ax[1].plot(ns, [st.median(r['adds_Mt'] for r in pt[n]['per_target']) for n in ns], 's-', color=COL['Mt'], label='Mt: adds per target')
ax[1].plot(ns, [pt[n]['Mt_table_adds'] for n in ns], '^--', color=COL['Mt'], label='Mt: one-time table adds')
ax[1].plot(ns, [n for n in ns], 'k:', lw=.8, label='n')
ax[1].set_xscale('log'); ax[1].set_yscale('log'); ax[1].set_xlabel('factor-base size n'); ax[1].set_ylabel('point additions (exact count)')
ax[1].set_title('Hash-join arms in counted group operations', fontsize=9); ax[1].legend(fontsize=8)
fig.tight_layout()
for ext in ('png', 'svg', 'pdf'): fig.savefig(f'{HERE}/growth_fit.{ext}', dpi=140)
