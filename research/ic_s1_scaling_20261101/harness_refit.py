#!/usr/bin/env python3
"""POST HOC (requested after the S1 report): harness-free WDSat cost refit.  Config A best-whole point of each cell (as in analysis/final.json).
solver-only (S1 metric)  = wdsat_ir - calls*START[l]            (s1lib.py load(); r2w_analyze.py: lower=wi-wc*START[l])  -> removes the WHOLE ecbench child (emitter + pipeline) and start-up
whole                    = child_ir + wdsat_ir - 3.51e7
harness-free (this file) = whole - (emitter + calls*START[l]) ; emitter = measured where S2 gives it (K1 n=11 l8 6.0e7, K0 n=13 l8 7.1e7, K0 n=15 l10 1.3e8 per system), else
                           e(cell,l) = 8.9e4*(9l-3)*n per system (S2's three points scaled by system size 9l-3 and field degree n; reproduces them within 12%), capped at the measured child per call.
Ratios of medians (wd cost median over 8 targets) / (best-lab whole median)."""
import json, glob, os, sys, math, statistics as st
import numpy as np
from scipy import stats
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from s1lib import load
B = '/Volumes/SSD990-2/s1-build/s1b-20261101'; P = '/Volumes/SSD990-2/s1-build'
R2WR = '/Volumes/SSD990-2/crypto-worktrees/ic-r2w-wdsat-sweep-20261031/research/ic_r2w_wdsat_sweep_20261031/runs'
R2WSTART = {str(k): v for k, v in {3: 13.90e6, 4: 14.30e6, 5: 13.97e6, 6: 14.31e6, 7: 14.74e6, 8: 14.73e6, 9: 15.56e6, 10: 16.43e6, 11: 17.61e6, 12: 18.88e6, 13: 20.96e6}.items()}
S = {'T1': json.load(open(f'{B}/start.json')), 'T2': json.load(open(f'{B}/start2.json')), 'S1': json.load(open(f'{P}/start.json'))}
A = json.load(open(f'{os.path.dirname(os.path.abspath(__file__))}/analysis/final.json'))['cells']
SHORT = {'K0 n=9': 'k0n9', 'K1 n=9': 'k1n9', 'K1 n=11': 'k1n11', 'K0 n=13': 'k0n13', 'K0 n=15': 'k0n15'}
S2E = {('K1 n=11', 8): 6.0e7, ('K0 n=13', 8): 7.1e7, ('K0 n=15', 10): 1.3e8}
KEY = {(0, 9, 202610111901): 'K0 n=9', (1, 9, 202610111901): 'K1 n=9', (1, 11, 202610111901): 'K1 n=11', (0, 13, 202610111901): 'K0 n=13', (0, 15, 202610111901): 'K0 n=15', (1, 15, 202610111901): 'K1 n=15',
       (1, 17, 202610111901): 'K1 n=17', (0, 19, 202610111901): 'K0 n=19', (1, 19, 202610111901): 'K1 n=19', (0, 13, 202611011301): 'K0 n=13 s2'}
NN = {c: int(c.split('n=')[1].split()[0]) for c in A}
recs = {}   # (cell, l) -> {t: rec}  for config A
def add(d, st_):
    sp = json.load(open(d + '/spec.json')); cv = sp['workloads']['curves'][0]
    cell = KEY.get((cv['a'], cv['n'], sp['workloads']['target_seed']))
    if cell is None: return
    try: L = load(d, d + '_tm', st_)
    except AssertionError: return
    for x in L:
        if x['arm'].startswith('wdA'): recs.setdefault((cell, x['l']), {}).setdefault(x['t'], x)
def done(d): lg = d + '.log'; return os.path.exists(lg) and any('complete ->' in l for l in open(lg))
for d in sorted(glob.glob(f'{B}/runs/T?_*')):
    if d.endswith('_tm') or d.endswith('.log') or not os.path.isdir(d) or not done(d): continue
    add(d, S[os.path.basename(d)[:2]])
for d in sorted(glob.glob(f'{P}/runs/S1_*')):
    if d.endswith('_tm') or d.endswith('.log') or not os.path.isdir(d) or '_AA' in d or not done(d): continue
    add(d, S['S1'])
for c, sh in SHORT.items():
    for d in sorted(glob.glob(f'{R2WR}/R2W_*_{sh}')):
        if d.endswith('_tm') or d.endswith('.log') or 'LX' in d or '_V3_' in d or not os.path.isdir(d + '_tm') or not os.path.exists(d + '/records.jsonl'): continue
        try:
            L = load(d, d + '_tm', R2WSTART)
        except AssertionError: continue
        for x in L:
            if x['arm'].startswith('wdA'): recs.setdefault((c, x['l']), {}).setdefault(x['t'], x)
ST = lambda c, l: (R2WSTART if c in SHORT else (S['T1'] if c in ('K0 n=19', 'K1 n=19', 'K0 n=13 s2') else S['T2']))[str(l)]
rows = {}
for c in A:
    a = A[c]['cfg']['A']; l = a['best_whole_l']; d = recs.get((c, l))
    if not d or len(d) < 8: print('missing records', c, l, len(d or [])); continue
    ts = sorted(d); calls = st.median(d[t]['wd_calls'] for t in ts)
    whole = st.median(d[t]['whole'] for t in ts); wd = st.median(d[t]['wd_ir'] for t in ts)
    stup = st.median(d[t]['wd_calls'] * float(ST(c, l)) if False else d[t]['wd_calls'] * float(ST(c, l)) for t in ts)
    child = st.median(d[t]['whole'] - d[t]['wd_ir'] for t in ts)             # child Ir - 3.51e7 (emitter + pipeline)
    n = NN[c]
    if (c, l) in S2E: e1 = S2E[(c, l)]; src = 'S2'
    else: e1 = 8.9e4 * (9 * l - 3) * n; src = 'est'
    emitter = min(e1 * calls, child)
    sonly = wd - stup
    free = whole - emitter - stup
    lab = A[c]['lab_ir']
    rows[c] = dict(n=n, l=l, calls=calls, whole=whole, child=child, startup=stup, emitter=emitter, src=src, solver_only=sonly, harness_free=free, lab=lab,
                   share_child_start=(whole - sonly) / whole, share_emit_start=(emitter + stup) / whole, r_whole=whole / lab, r_free=free / lab, r_sonly=sonly / lab)
print('cell | n l calls | whole | child(-3.5e7) | startup | emitter(src) | solver-only | harness-free | share(child+start) | share(emitter+start) | ratio whole / harness-free / solver-only')
for c, r in rows.items():
    print(f"{c:11s} n{r['n']} l{r['l']} calls {r['calls']:.0f} | {r['whole']:.3g} | {r['child']:.3g} | {r['startup']:.3g} | {r['emitter']:.3g} ({r['src']}) | {r['solver_only']:.3g} | {r['harness_free']:.3g} | {r['share_child_start']:.2f} | {r['share_emit_start']:.2f} | {r['r_whole']:.2f} / {r['r_free']:.2f} / {r['r_sonly']:.2f}")
def fit(P, label):
    x = np.array([p[0] for p in P], float); y = np.log([p[1] for p in P]); k = len(x)
    sl, ic, rv, pv, se = stats.linregress(x, y); t = stats.t.ppf(0.975, k - 2); lo, hi = sl - t * se, sl + t * se
    v = 'growing' if lo > 0 else 'shrinking' if hi < 0 else 'flat' if (lo >= -0.1 and hi <= 0.1) else 'undecided'
    print(f"fit {label:40s} k={k}: slope {sl:+.3f} +- {t*se:.3f} (CI {lo:+.3f}..{hi:+.3f}) x{math.exp(4*sl):.2f}/4bits -> {v}")
    return dict(label=label, k=k, slope=sl, lo=lo, hi=hi, verdict=v)
out = []
for key, nm in (('r_free', 'harness-free (whole - emitter - start-up)'), ('r_sonly', 'solver-only (S1 metric, ratio of medians)'), ('r_whole', 'whole run (ratio of medians)')):
    for sub, f in (('all 10', lambda c: True), ('without n=19', lambda c: rows[c]['n'] < 19)):
        out.append(fit([(r['n'], r[key]) for c, r in rows.items() if f(c)], f'{nm} | {sub}'))
# share vs n
x = np.array([r['n'] for r in rows.values()], float)
for key in ('share_child_start', 'share_emit_start'):
    y = np.array([r[key] for r in rows.values()]); sl, ic, rv, pv, se = stats.linregress(x, y); t = stats.t.ppf(0.975, len(x) - 2)
    print(f'share {key}: slope per bit {sl:+.4f} +- {t*se:.4f} (p={pv:.3f})')
json.dump(dict(rows=rows, fits=out), open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'analysis', 'harness_refit.json'), 'w'), indent=1, default=float)
