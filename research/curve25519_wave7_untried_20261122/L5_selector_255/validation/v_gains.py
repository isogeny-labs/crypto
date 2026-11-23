#!/usr/bin/env python3
"""Validator: recompute gains (U-statistic), bootstrap intervals with calibration, fits, decision rule,
P3a/P3b/P4/P8/P9, arm I, arm R1 from the raw per-frame data. Own code; reads data files only.
Usage: v_gains.py <out.json> [--drop-smoke]   (--drop-smoke recomputes on non-overlapping rows only)"""
import sys, os, json, math, glob, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from v_common import BASE, CURV, seed_rule, max_weights, order_stable, gain_U, gain_U_py, J_of_counts, S1_of_counts, read_gains_csv

DROP = '--drop-smoke' in sys.argv
OUT = sys.argv[1]
B = 500
KS = [10, 100, 200, 500, 1000, 2000]
LN2 = math.log(2)
res = dict(drop_smoke=DROP, checks={})

def boot(values, keys, K, seed, n_boot=B):
    values = np.asarray(values, float); keys = [np.asarray(k) for k in keys]
    N = len(values); w = max_weights(N, K); rng = np.random.default_rng(seed); out = np.empty(n_boot)
    for b in range(n_boot):
        idx = rng.integers(0, N, N)
        v = values[idx]; o = order_stable([k[idx] for k in keys])
        out[b] = np.dot(w, v[o]) / v.mean()
    return out

# ---------------- arm F ----------------
csvg = read_gains_csv()
G = {}; RAW = {}; meta = {}
row_issues = []
NSMOKE = 121 if DROP else 0
for f in sorted(glob.glob(os.path.join(BASE, 'data', 'frames', '*.npz'))):
    cur, t = re.match(r'(.+)_t(\d+)\.npz', os.path.basename(f)).groups(); t = int(t)
    z = np.load(f)
    rows = z['rows']; tors = z['tors']; sJ = z['surrogate_J']; sC = z['surrogate_counts']
    p, A, n = CURV[cur]
    # row consistency with the definitions
    L, c, m, J, S1, cov = rows[:, 0], rows[:, 1], rows[:, 2:7], rows[:, 7], rows[:, 8], rows[:, 9]
    if not np.all(c == m.sum(1)): row_issues.append((cur, t, 'c != sum m'))
    Jc = J_of_counts(*[m[:, i] for i in range(5)])
    bad = np.where((Jc != J) & (tors == 0))[0]
    if len(bad): row_issues.append((cur, t, 'J mismatch', len(bad)))
    S1c = S1_of_counts(*[m[:, i] for i in range(5)])
    if not np.all(S1c == S1): row_issues.append((cur, t, 'S1 mismatch'))
    sJc = J_of_counts(*[sC[:, i] for i in range(5)])
    if not np.all(sJc == sJ): row_issues.append((cur, t, 'surrogate J mismatch'))
    if not (L[0] == 0): row_issues.append((cur, t, 'row0 not bits_lo0'))
    meta[(cur, t)] = dict(torsion_frames=int(tors.sum()), N=len(rows) - 1)
    nsmoke = NSMOKE if cur == 'k20' else 0
    meta[(cur, t)]['nsmoke'] = nsmoke
    R = rows[1 + nsmoke:]; N = len(R)
    L, c, m, J, S1, cov = R[:, 0], R[:, 1], R[:, 2:7], R[:, 7].astype(float), R[:, 8], R[:, 9].astype(float)
    Phi = np.where(c > 0, 4.0 * J / np.maximum(c, 1) ** 2, 0.0)
    sJ1 = sJ[1 + nsmoke:].astype(float); sc = sC[1 + nsmoke:].sum(1)
    rho_hat = c.mean() / 2 ** t
    D = float(np.var(c, ddof=1) / (2 ** t * rho_hat * (1 - rho_hat)))
    sel = {('J', 'J'): ([J, c, -L], J), ('c', 'J'): ([c, -L], J), ('Phi', 'Phi'): ([Phi, -L], Phi),
           ('S1', 'J'): ([S1, c, -L], J), ('decoy', 'J'): ([sJ1, -L], J),
           ('surrogate', 'surrJ'): ([sJ1, sc, -np.arange(N)], sJ1)}
    if cov[0] >= 0:
        sel[('J', 'covered')] = ([J, c, -L], cov); sel[('S1', 'covered')] = ([S1, c, -L], cov)
    for (s, v), (keys, vals) in sel.items():
        for K in KS:
            if not DROP and K > N // 2: continue
            G[('F', cur, t, s, v, K)] = gain_U(vals, keys, K)
            if K in (2000, 200):
                RAW[('F', cur, t, s, v, K)] = boot(vals, keys, K, seed_rule('boot', cur, t, s, v, K))
    meta[(cur, t)].update(mean_c=float(c.mean()), mean_J=float(J.mean()), D=D, cv_J=float(np.std(J, ddof=1) / J.mean()),
                          cv_pred=2 * math.sqrt(D) * 2 ** (-t / 2), Jstd0=float(J.std()), mean_cov=float(cov.mean()))
res['checks']['row_consistency_issues'] = row_issues
res['torsion_frames'] = {f'{k[0]}_t{k[1]}': v['torsion_frames'] for k, v in meta.items()}

# cross-check a few point estimates with the pure-python sorter
if not DROP:
    z = np.load(os.path.join(BASE, 'data', 'frames', 'k20_t6.npz')); R = z['rows'][1:]
    gpy = gain_U_py(R[:, 7], [R[:, 7], R[:, 1], -R[:, 0]], 2000)
    res['checks']['pure_python_vs_numpy_k20_t6_J'] = [gpy, G[('F', 'k20', 6, 'J', 'J', 2000)]]

# ---------------- calibration factor from the 80 pools ----------------
pools = json.load(open(os.path.join(BASE, 'data', 'pc3_pools.json')))
cal = {}
for cell in pools['cells']:
    t = cell['t']; u = np.array(cell['ustat_J']); rl = np.array(cell['realised'])
    if DROP:
        u = u[2:]; rl = rl[4:]          # smoke pools 0..3 -> drop pairs (0,1),(2,3)
    sd_u = float(np.std(u, ddof=1))
    sd_b = float(np.std(RAW[('F', 'k20', t, 'J', 'J', 2000)], ddof=1))
    cal[t] = dict(n_pairs=len(u), n_pools=len(rl), sd_ustat=sd_u, sd_ustat_recorded=cell['ustat_J_sd'], sd_boot_raw=sd_b, ratio=sd_u / sd_b,
                  realised_lo=float(np.percentile(rl, 2.5)), realised_hi=float(np.percentile(rl, 97.5)),
                  realised_min=float(rl.min()), realised_max=float(rl.max()))
f_cal = max(1.0, float(np.mean([cal[t]['ratio'] for t in cal])))
if os.environ.get("F_OVERRIDE"): f_cal = float(os.environ["F_OVERRIDE"])
res['calibration'] = dict(cells=cal, f=f_cal, f_per_t_max=max(cal[t]['ratio'] for t in cal), f_per_t_min=min(cal[t]['ratio'] for t in cal))
res['PC3'] = {t: dict(lo=cal[t]['realised_lo'], hi=cal[t]['realised_hi'], published={5: 2.837651914698464, 6: 1.97750951860925}[t],
                      inside=cal[t]['realised_lo'] <= {5: 2.837651914698464, 6: 1.97750951860925}[t] <= cal[t]['realised_hi']) for t in cal}

def calib(bs, g, f):
    return g + f * (bs - bs.mean())
REP = {k: calib(v, G[k], f_cal) for k, v in RAW.items()}

# ---------------- arm I ----------------
kname = {20: 'k20', 24: 'k24r'}
Imeta = {}
for f in sorted(glob.glob(os.path.join(BASE, 'data', 'iso', 'iso_k*_t*.jsonl'))):
    k, t = map(int, re.match(r'iso_k(\d+)_t(\d+)\.jsonl', os.path.basename(f)).groups())
    rows = [json.loads(l) for l in open(f)]
    for x in rows:
        for key in ('J', 'columns', 'covered_points', 'seed', 'base_points', 'selector_group_ops', 'build_group_ops', 't', 'k'):
            x[key]  # KeyError if missing (identity scripts read real keys)
    assert all(x['t'] == t and x['k'] == k for x in rows)
    if DROP and k == 20 and t <= 7:
        rows = [x for x in rows if x['seed'] >= 40]
    J = np.array([x['J'] for x in rows], float); cols = np.array([x['columns'] for x in rows], float)
    cov = np.array([x['covered_points'] for x in rows], float); seed = np.array([x['seed'] for x in rows])
    bp = np.array([x['base_points'] for x in rows]); selops = np.array([x['selector_group_ops'] for x in rows])
    N = len(rows); cur = kname[k]
    rho_i = cols.mean() / 2 ** t
    Imeta[(cur, t)] = dict(N=N, mean_cols=float(cols.mean()), D=float(np.var(cols, ddof=1) / (2 ** t * rho_i * (1 - rho_i))),
                           build_min=int(min(x['build_group_ops'] for x in rows)), build_max=int(max(x['build_group_ops'] for x in rows)),
                           build_mean=float(np.mean([x['build_group_ops'] for x in rows])),
                           rP_per_point=float(np.mean((selops - (bp * (bp + 1) // 2 - cols)) / np.maximum(bp, 1))),
                           seeds_unique=len(set(seed.tolist())) == N)
    for v, vals in (('J', J), ('covered', cov)):
        for K in (10, 100, 500):
            G[('I', cur, t, 'J', v, K)] = gain_U(vals, [J, cols, -seed], K)
            if K == 500:
                bs = boot(vals, [J, cols, -seed], K, seed_rule('boot', 'iso', k, t, v, K))
                RAW[('I', cur, t, 'J', v, K)] = bs; REP[('I', cur, t, 'J', v, K)] = calib(bs, G[('I', cur, t, 'J', v, K)], f_cal)

# ---------------- arm R1 ----------------
Z = np.load(os.path.join(BASE, 'data', 'real25519.npz'))
Rm = {}
dropR = {6: 40, 8: 12} if DROP else {}
for key in Z.files:
    if key.startswith('r1_t'):
        t = int(key[4:]); rows = Z[key][dropR.get(t, 0):]
        c = rows[:, 0]; m = rows[:, 1:]
        assert np.all(c == m.sum(1))
        Jr = J_of_counts(*[m[:, i] for i in range(5)]).astype(float)
        N = len(rows); K = 200; idx = np.arange(N)
        g = gain_U(Jr, [Jr, c, -idx], K)
        bs = boot(Jr, [Jr, c, -idx], K, seed_rule('boot', 'real', t, K))
        G[('R', 'curve25519', t, 'J', 'J', K)] = g; RAW[('R', 'curve25519', t, 'J', 'J', K)] = bs
        REP[('R', 'curve25519', t, 'J', 'J', K)] = calib(bs, g, f_cal)
        rho = c.mean() / 2 ** t
        props = m.sum(0) / m.sum()
        Rm[f'r1_t{t}'] = dict(N=N, mean_c=float(c.mean()), D=float(np.var(c, ddof=1) / (2 ** t * rho * (1 - rho))),
                             props=props.tolist(), max_abs_dev_props=float(np.max(np.abs(props - np.array([1/8, 1/4, 1/4, 1/4, 1/8])))),
                             labels=int(c.sum()))
    if key.startswith('r1c_t'):
        t = int(key[5:]); c = Z[key][(6 if (DROP and t == 12) else 0):]
        rho = c.mean() / 2 ** t
        Rm[f'r1c_t{t}'] = dict(N=len(c), mean_c=float(c.mean()), var_c=float(np.var(c, ddof=1)),
                              D=float(np.var(c, ddof=1) / (2 ** t * rho * (1 - rho))))
res['R1'] = Rm

# ---------------- compare point estimates and intervals with gains.csv ----------------
if not DROP:
    maxrel = 0.0; maxint = 0.0; nmiss = 0; ncmp = 0; nint = 0; worst = None
    for key, g in G.items():
        if key not in csvg: nmiss += 1; continue
        r = csvg[key]; ncmp += 1
        rel = abs(g - float(r['gain'])) / abs(float(r['gain']))
        if rel > maxrel: maxrel = rel; worst = key
        if key in REP and r['lo95'] != 'nan':
            lo, hi = np.percentile(REP[key], [2.5, 97.5]); nint += 1
            maxint = max(maxint, abs(lo - float(r['lo95'])) / float(r['lo95']), abs(hi - float(r['hi95'])) / float(r['hi95']))
    extra = [k for k in csvg if k not in G]
    res['checks']['csv_compare'] = dict(compared=ncmp, missing_in_csv=nmiss, csv_rows_not_recomputed=len(extra),
                                        csv_rows_total=len(csvg), max_rel_gain_diff=maxrel, worst=str(worst),
                                        intervals_compared=nint, max_rel_interval_diff=maxint, extra_sample=[str(e) for e in extra[:5]])

# ---------------- fits ----------------
def wls(X, y, w):
    X = np.asarray(X, float); y = np.asarray(y, float); w = np.asarray(w, float)
    # normal equations solved by hand (2x2 or 1x1)
    if X.shape[1] == 1:
        return np.array([np.sum(w * X[:, 0] * y) / np.sum(w * X[:, 0] ** 2)])
    a11 = np.sum(w * X[:, 0] ** 2); a12 = np.sum(w * X[:, 0] * X[:, 1]); a22 = np.sum(w * X[:, 1] ** 2)
    b1 = np.sum(w * X[:, 0] * y); b2 = np.sum(w * X[:, 1] * y)
    det = a11 * a22 - a12 * a12
    return np.array([(a22 * b1 - a12 * b2) / det, (a11 * b2 - a12 * b1) / det])

def model_fit(mdl, ts, Gs, se):
    ts = np.asarray(ts, float); Gs = np.asarray(Gs, float)
    w = 1 / np.maximum(se, 1e-6) ** 2
    if mdl == 'M1':
        return wls(np.c_[np.ones_like(ts), ts], np.log(Gs), w)
    y = np.log(Gs - 1)
    if mdl == 'M2':
        return wls(np.c_[np.ones_like(ts), -ts * LN2], y, w)
    return wls(np.ones((len(ts), 1)), y + ts * LN2 / 2, w)

def pred_e(mdl, beta, t):
    t = np.asarray(t, float)
    if mdl == 'M1': return np.exp(beta[0] + beta[1] * t) - 1
    if mdl == 'M2': return np.exp(beta[0] - beta[1] * t * LN2)
    return np.exp(beta[0] - t * LN2 / 2)

def score(mdl, beta, tho, Gho):
    ep = pred_e(mdl, beta, tho); eo = np.asarray(Gho) - 1
    if np.any(ep <= 0) or np.any(eo <= 0): return float('inf')
    return float(np.sqrt(np.mean(np.log(ep / eo) ** 2)))

NPAR = {'M1': 2, 'M2': 2, 'M3': 1}
def decide(scores):
    worst = {m: max(v) for m, v in scores.items()}
    best = min(worst, key=lambda m: worst[m])
    for m in sorted(worst, key=lambda m: NPAR[m]):
        if NPAR[m] < NPAR[best] and worst[m] <= worst[best] + 0.1:
            best = m; break
    return best, worst

def fit_arm(arm, curves, K, train_max, s='J', v='J', weighted=True, tsel=None):
    out = {}
    for cur in curves:
        ts = sorted(t for (a_, c_, t, s_, v_, K_) in G if a_ == arm and c_ == cur and s_ == s and v_ == v and K_ == K and (tsel is None or tsel(cur, t)))
        if len(ts) < 3: continue
        Gv = np.array([G[(arm, cur, t, s, v, K)] for t in ts])
        Rr = np.array([REP[(arm, cur, t, s, v, K)] for t in ts])
        if weighted:
            se = {'M1': np.std(np.log(np.maximum(Rr, 1e-9)), axis=1, ddof=1), 'M2': np.std(np.log(np.maximum(Rr - 1, 1e-9)), axis=1, ddof=1)}
        else:
            se = {'M1': np.ones(len(ts)), 'M2': np.ones(len(ts))}
        se['M3'] = se['M2']
        tr = [i for i, t in enumerate(ts) if t <= train_max]; ho = [i for i, t in enumerate(ts) if t > train_max]
        o = dict(ts=ts, G=Gv.tolist(), scores={}, train={}, all={}, boot_alpha=None, boot_skipped={})
        for mdl in NPAR:
            bt = model_fit(mdl, [ts[i] for i in tr], Gv[tr], se[mdl][tr]); o['train'][mdl] = bt.tolist()
            o['scores'][mdl] = score(mdl, bt, [ts[i] for i in ho], Gv[ho]) if ho else None
            ba = model_fit(mdl, ts, Gv, se[mdl]); o['all'][mdl] = ba.tolist()
            if mdl == 'M2' or mdl == 'M3':
                bb = []; skipped = 0
                for b in range(Rr.shape[1]):
                    Gb = Rr[:, b]
                    if np.any(Gb <= 1): skipped += 1; continue
                    bb.append(model_fit(mdl, ts, Gb, se[mdl]))
                o['boot_skipped'][mdl] = skipped
                o.setdefault('boot', {})[mdl] = [x.tolist() for x in bb]
        o['alpha_M2_all'] = o['all']['M2'][1]
        al = [x[1] for x in o['boot']['M2']]
        o['alpha_M2_ci'] = [float(np.percentile(al, 2.5)), float(np.percentile(al, 97.5))]
        out[cur] = o
    return out

def summarize(fits):
    sc = {m: [f['scores'][m] for f in fits.values() if f['scores'][m] is not None] for m in NPAR}
    win, worst = decide(sc)
    return dict(winner=win, worst=worst, adequate=worst[win] <= math.log(2),
                scores={cu: f['scores'] for cu, f in fits.items()},
                alpha_M2={cu: f['alpha_M2_all'] for cu, f in fits.items()},
                alpha_M2_ci={cu: f['alpha_M2_ci'] for cu, f in fits.items()},
                boot_skipped={cu: f['boot_skipped'] for cu, f in fits.items()},
                M3_a={cu: f['all']['M3'][0] for cu, f in fits.items()})

curvesF = ['k20', 'k24f', 'k24r']
fitsF = fit_arm('F', curvesF, 2000, 10); res['decision_F'] = summarize(fitsF)
fitsF_ols = fit_arm('F', curvesF, 2000, 10, weighted=False); res['decision_F_unweighted'] = summarize(fitsF_ols)
fitsI = fit_arm('I', ['k20', 'k24r'], 500, 7); res['decision_I'] = summarize(fitsI)
fitsI_ols = fit_arm('I', ['k20', 'k24r'], 500, 7, weighted=False); res['decision_I_unweighted'] = summarize(fitsI_ols)
fitsPhi = fit_arm('F', curvesF, 2000, 99, 'Phi', 'Phi'); res['alpha_Phi'] = {cu: f['alpha_M2_all'] for cu, f in fitsPhi.items()}
fitsPhi_ols = fit_arm('F', curvesF, 2000, 99, 'Phi', 'Phi', weighted=False); res['alpha_Phi_unweighted'] = {cu: f['alpha_M2_all'] for cu, f in fitsPhi_ols.items()}
# descriptive: covered, unsaturated cells (mean covered / r <= 0.05)
unsat = {(cu, t) for (cu, t), mm in meta.items() if mm['mean_cov'] >= 0 and mm['mean_cov'] / (CURV[cu][2] // 8) <= 0.05}
fitsCov = fit_arm('F', curvesF, 2000, 99, 'J', 'covered', tsel=lambda cu, t: (cu, t) in unsat)
res['alpha_covered_unsat'] = {cu: f['alpha_M2_all'] for cu, f in fitsCov.items()}

# ---------------- P3b, P4, P8, P9 ----------------
share = {}; p4 = {}; p8 = {}; p9 = {}
for cur in curvesF:
    for t in range(4, 15):
        k = ('F', cur, t, 'J', 'J', 2000)
        if k not in G: continue
        gj = G[k]
        if t >= 8: share[f'{cur}_t{t}'] = (G[('F', cur, t, 'c', 'J', 2000)] - 1) / (gj - 1)
        if t >= 6: p4[f'{cur}_t{t}'] = (gj - 1) / (G[('F', cur, t, 'surrogate', 'surrJ', 2000)] - 1)
        N = meta[(cur, t)]['N'] - meta[(cur, t)]['nsmoke']
        w = max_weights(N, 2000)
        sd0 = math.sqrt(float(np.sum(w ** 2))) * meta[(cur, t)]['Jstd0'] / meta[(cur, t)]['mean_J']
        p8[f'{cur}_t{t}'] = (G[('F', cur, t, 'decoy', 'J', 2000)] - 1) / sd0
        if t >= 8: p9[f'{cur}_t{t}'] = (gj - 1) / (G[('F', cur, t, 'J', 'J', 100)] - 1)
gm = lambda d, cu: float(np.exp(np.mean([math.log(v) for k_, v in d.items() if k_.startswith(cu + '_')])))
res['P3b_share'] = share; res['P3b_min'] = min(share.values())
res['P4'] = p4; res['P4_geomean'] = {cu: gm(p4, cu) for cu in curvesF}
res['P8_z'] = p8; res['P8_max_abs_z'] = max(abs(v) for v in p8.values())
res['P9'] = p9; res['P9_geomean'] = {cu: gm(p9, cu) for cu in curvesF}
# cv check (README: mechanism CV matches measured within 2% at t >= 7)
res['cv_ratio_t_ge_7'] = {f'{cu}_t{t}': meta[(cu, t)]['cv_J'] / meta[(cu, t)]['cv_pred'] for (cu, t) in meta if t >= 7}

# ---------------- P6 ----------------
p6 = {}
for t in (6, 8, 10):
    kR = ('R', 'curve25519', t, 'J', 'J', 200)
    lo, hi = np.percentile(REP[kR], [2.5, 97.5]); lo_raw, hi_raw = np.percentile(RAW[kR], [2.5, 97.5])
    gm24 = G[('F', 'k24f', t, 'J', 'J', 200)]; e24 = gm24 - 1
    p6[t] = dict(gain=G[kR], lo=float(lo), hi=float(hi), k24f=gm24, k20=G[('F', 'k20', t, 'J', 'J', 200)],
                 band=[e24 / 1.5, e24 * 1.5], pass_calibrated=bool(hi - 1 >= e24 / 1.5 and lo - 1 <= 1.5 * e24),
                 raw_interval=[float(lo_raw), float(hi_raw)], pass_uncalibrated=bool(hi_raw - 1 >= e24 / 1.5 and lo_raw - 1 <= 1.5 * e24))
res['P6'] = p6

# ---------------- verdicts ----------------
dF = res['decision_F']; dI = res['decision_I']
V = dict(P1_form_not_M1=dF['winner'] in ('M2', 'M3'), P1_alpha=all(0.35 <= a <= 0.65 for a in dF['alpha_M2'].values()),
         P2_adequate=dF['adequate'], P3a=all(a >= 0.8 for a in res['alpha_Phi'].values()), P3b=res['P3b_min'] >= 0.7,
         P4=all(0.75 <= v <= 1.33 for v in res['P4_geomean'].values()), P7=all(0.35 <= a <= 0.65 for a in dI['alpha_M2'].values()),
         P8=res['P8_max_abs_z'] <= 3.5, P9=all(1.1 <= v <= 1.6 for v in res['P9_geomean'].values()),
         P6=all(v['pass_calibrated'] for v in p6.values()),
         R1=all(0.7 <= Rm[f'r1c_t{t}']['D'] <= 1.4 for t in (12, 14, 16)))
res['verdicts'] = V

# tables: point estimates and intervals for the README tables
tab = {}
for key in G:
    lo = hi = None
    if key in REP: lo, hi = [float(x) for x in np.percentile(REP[key], [2.5, 97.5])]
    tab['|'.join(map(str, key))] = [G[key], lo, hi]
res['gains'] = tab
res['meta'] = {f'{k[0]}_t{k[1]}': v for k, v in meta.items()}
res['Imeta'] = {f'{k[0]}_t{k[1]}': v for k, v in Imeta.items()}
# M3 extrapolation needs the fits (also stored raw for the derived script)
res['fitsF_all'] = {cu: f['all'] for cu, f in fitsF.items()}
res['fitsF_boot_M3_a'] = {cu: [x[0] for x in f['boot']['M3']] for cu, f in fitsF.items()}
res['fitsF_boot_M2'] = {cu: f['boot']['M2'] for cu, f in fitsF.items()}
json.dump(res, open(OUT, 'w'), indent=1, default=float)
print(json.dumps(dict(checks=res['checks'], calibration=res['calibration'], PC3=res['PC3'], verdicts=V), indent=1, default=float))
