#!/usr/bin/env python3
"""Analysis for L5 (PROTOCOL.md). Decides P0-P9 and R1-R4 and writes the derived 255-bit numbers.

Usage: analyse.py --frames DIR --iso DIR --real NPZ --pc JSON --out DIR [--B 500] [--smoke]
Reads only files written by the run scripts. Writes <out>/results.json, <out>/gains.csv,
<out>/fig_gain_vs_t.{png,svg}. Everything at 255 bits is a derived extrapolation and is labelled so.
"""
import argparse, glob, json, math, os, re, sys
here = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, here)
import numpy as np
from l5common import CURVES, derived_seed, expected_selected, order_by_key, selection_weights

R255 = 2 ** 252 + 27742317777372353535851937790883648493
LOG2R255 = math.log2(R255)
RHO_OPS_LOG2 = 0.5 * math.log2(math.pi * R255 / 4)          # rho: sqrt(pi r / 4) group operations

# ---------------- statistics helpers ----------------
def gammaincc(a, x):
    """Regularised upper incomplete gamma Q(a, x) (Numerical Recipes series / continued fraction)."""
    if x <= 0: return 1.0
    gln = math.lgamma(a)
    if x < a + 1:
        ap, s, d = a, 1.0 / a, 1.0 / a
        for _ in range(100000):
            ap += 1; d *= x / ap; s += d
            if abs(d) < abs(s) * 1e-15: break
        return max(0.0, 1.0 - s * math.exp(-x + a * math.log(x) - gln))
    b = x + 1 - a; c = 1e300; d = 1 / b; h = d
    for i in range(1, 100000):
        an = -i * (i - a); b += 2; d = an * d + b; d = 1e-300 if abs(d) < 1e-300 else d
        c = b + an / c; c = 1e-300 if abs(c) < 1e-300 else c; d = 1 / d; de = d * c; h *= de
        if abs(de - 1) < 1e-15: break
    return math.exp(-x + a * math.log(x) - gln) * h

def chi2_sf(x, df): return gammaincc(df / 2, x / 2)

def zK(K):
    """E[max of K iid N(0,1)] by numerical integration (used only for derived K-scaling)."""
    if K == 1: return 0.0
    xs = np.linspace(-8, 12 + math.sqrt(2 * math.log(K)), 40001)
    Phi = 0.5 * (1 + np.vectorize(math.erf)(xs / math.sqrt(2)))
    logF = K * np.log(np.clip(Phi, 1e-300, 1))
    cdf = np.exp(logF)
    pos = xs >= 0
    return float(np.trapezoid(1 - cdf[pos], xs[pos]) - np.trapezoid(cdf[~pos], xs[~pos]))

def zK_asym(log2K):
    """Asymptotic E[max of K N(0,1)] for huge K (derived scaling only)."""
    L = 2 * log2K * math.log(2); s = math.sqrt(L)
    return s - (math.log(math.log(2 ** min(log2K, 1000)) if log2K < 1000 else L / 2) + math.log(4 * math.pi)) / (2 * s) + 0.5772 / s

def boot_gain(values, keys, K, B, seed):
    values = np.asarray(values, float); keys = [np.asarray(k) for k in keys]
    N = len(values); w = selection_weights(N, K); rng = np.random.default_rng(seed); out = np.empty(B)
    for b in range(B):
        idx = rng.integers(0, N, N)
        v = values[idx]; o = order_by_key([k[idx] for k in keys])
        out[b] = np.dot(w, v[o]) / v.mean()
    return out

def gain_point(values, keys, K):
    return expected_selected(values, keys, K) / float(np.mean(values))

# ---------------- model fitting ----------------
MODELS = {'M1': 2, 'M2': 2, 'M3': 1}

def fit(model, ts, G, se_ln):
    """Weighted LS. M1: ln G = a + b t. M2: ln(G-1) = a - alpha t ln2. M3: ln(G-1) = a - t ln2 / 2.
    se_ln: bootstrap SE of the transformed response (ln G for M1, ln(G-1) for M2/M3)."""
    ts = np.asarray(ts, float); G = np.asarray(G, float)
    w = 1 / np.maximum(np.asarray(se_ln[model], float), 1e-6) ** 2
    if model == 'M1':
        y = np.log(G); X = np.vstack([np.ones_like(ts), ts]).T
    else:
        y = np.log(G - 1)
        if model == 'M2': X = np.vstack([np.ones_like(ts), -ts * math.log(2)]).T
        else: y = y + ts * math.log(2) / 2; X = np.ones((len(ts), 1))
    W = np.diag(w); beta = np.linalg.solve(X.T @ W @ X, X.T @ W @ y)
    return beta

def predict_excess(model, beta, t):
    t = np.asarray(t, float)
    if model == 'M1': return np.exp(beta[0] + beta[1] * t) - 1
    if model == 'M2': return np.exp(beta[0] - beta[1] * t * math.log(2))
    return np.exp(beta[0] - t * math.log(2) / 2)

def heldout_score(model, beta, t_ho, G_ho):
    e_pred = predict_excess(model, beta, t_ho); e_obs = np.asarray(G_ho) - 1
    if np.any(e_pred <= 0) or np.any(e_obs <= 0): return float('inf')
    return float(np.sqrt(np.mean(np.log(e_pred / e_obs) ** 2)))

def choose(scores_by_model):
    """Winner = min worst-case held-out score; a simpler model within 0.1 of it wins instead."""
    worst = {m: max(v) for m, v in scores_by_model.items()}
    best = min(worst, key=lambda m: worst[m])
    for m in sorted(worst, key=lambda m: MODELS[m]):
        if MODELS[m] < MODELS[best] and worst[m] <= worst[best] + 0.1: best = m; break
    return best, worst

# ---------------- loaders ----------------
def load_frames(d):
    cells = {}
    for f in glob.glob(os.path.join(d, '*_t*.npz')):
        m = re.match(r'(.+)_t(\d+)\.npz', os.path.basename(f)); cur, t = m.group(1), int(m.group(2))
        z = np.load(f); cells[(cur, t)] = dict(rows=z['rows'], surr_counts=z['surrogate_counts'], surr_J=z['surrogate_J'],
                                               rho=float(z['rho']))
    return cells

def load_iso(d):
    cells = {}
    for f in glob.glob(os.path.join(d, 'iso_k*_t*.jsonl')):
        m = re.match(r'iso_k(\d+)_t(\d+)\.jsonl', os.path.basename(f)); k, t = int(m.group(1)), int(m.group(2))
        rows = [json.loads(l) for l in open(f)]; rows = [x for x in rows if 'error' not in x]
        cells[(k, t)] = rows
    return cells

def J_counts(m):
    m = np.asarray(m, np.int64)
    c = m.sum(-1)
    return m[..., 1] ** 2 + m[..., 2] ** 2 + m[..., 3] ** 2 + m[..., 0] * (2 * m[..., 0] + 1) + m[..., 4] * (2 * m[..., 4] + 1) - c

# ---------------- main ----------------
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--frames', required=True); ap.add_argument('--iso', required=True)
    ap.add_argument('--real', required=True); ap.add_argument('--real-log', required=True)
    ap.add_argument('--pc', required=True); ap.add_argument('--out', required=True)
    ap.add_argument('--frames-log', nargs='+', required=True, help='selector_scan stdout JSONL files')
    ap.add_argument('--B', type=int, default=500); ap.add_argument('--K', type=int, default=2000)
    ap.add_argument('--Kiso', type=int, default=500); ap.add_argument('--Kreal', type=int, default=200)
    ap.add_argument('--train-max', type=int, default=10); ap.add_argument('--iso-train-max', type=int, default=7)
    ap.add_argument('--pools', required=True, help='pc3_pools.py output (PC3 and bootstrap calibration)')
    ap.add_argument('--smoke', action='store_true')
    a = ap.parse_args(); os.makedirs(a.out, exist_ok=True)
    res = dict(label='SMOKE, NOT DATA' if a.smoke else 'data', K=a.K, Kiso=a.Kiso, Kreal=a.Kreal, B=a.B)
    csv = ['arm,curve,t,selector,value,K,gain,lo95,hi95,mean_c,mean_value,dispersion_D']
    rowsout = []          # rendered into gains.csv at the end, with calibrated intervals
    calib = [1.0]         # bootstrap SE inflation factor f (PROTOCOL.md), set after PC3
    def centred(bs, g):   # bias-centred bootstrap replicates, spread scaled by the calibration factor
        return g + calib[0] * (bs - bs.mean())
    # ---------- arm F ----------
    cells = load_frames(a.frames); G = {}; reps = {}
    for (cur, t), C in sorted(cells.items()):
        rows = C['rows']; b0, R = rows[0], rows[1:]; N = len(R)
        L, c, m, J, S1, cov = R[:, 0], R[:, 1], R[:, 2:7], R[:, 7].astype(float), R[:, 8], R[:, 9].astype(float)
        k, p, A, n = CURVES[cur]; r = n // 8
        Phi = np.where(c > 0, J / np.maximum(c, 1) ** 2 * 4, 0.0)
        sJ = C['surr_J'].astype(float); sc = C['surr_counts'][1:].sum(1); sJ1 = sJ[1:]
        rho = C['rho']; D = float(c.var(ddof=1) / ((1 << t) * rho * (1 - rho)))
        K = min(a.K, N // 2)
        sels = {('J', 'J'): ([J, c, -L], J), ('c', 'J'): ([c, -L], J), ('Phi', 'Phi'): ([Phi, -L], Phi),
                ('S1', 'J'): ([S1, c, -L], J), ('decoy', 'J'): ([sJ1, -L], J),
                ('surrogate', 'surrJ'): ([sJ1, sc, -np.arange(N)], sJ1)}
        if cov[0] >= 0:
            sels[('J', 'covered')] = ([J, c, -L], cov); sels[('S1', 'covered')] = ([S1, c, -L], cov)
        for (sname, vname), (keys, vals) in sels.items():
            for KK in sorted({10, 100, a.Kreal, 500, 1000, K}):
                if KK > N // 2: continue
                g = gain_point(vals, keys, KK)
                if KK in (K, a.Kreal) and a.B:
                    reps[('F', cur, t, sname, vname, KK)] = boot_gain(vals, keys, KK, a.B, derived_seed('boot', cur, t, sname, vname, KK))
                G[('F', cur, t, sname, vname, KK)] = g
                rowsout.append((('F', cur, t, sname, vname, KK), c.mean(), np.mean(vals), D))
        res.setdefault('F_cells', []).append(dict(curve=cur, t=t, N=N, mean_c=float(c.mean()), mean_J=float(J.mean()),
            mean_cov=float(cov.mean()), cov_over_r=float(cov.mean() / r) if cov[0] >= 0 else None, J_over_r=float(J.mean() / r),
            dispersion_D=D, cv_J=float(J.std(ddof=1) / J.mean()), cv_J_pred=float(2 * math.sqrt(D) * 2 ** (-t / 2)),
            bits_lo0_cov=int(b0[9]), bits_lo0_J=int(b0[7])))
    # P0: PC3 predictive interval of the realised single-pool gain at k20 t = 5, 6 (covered, J selector)
    pc = json.load(open(a.pc)); res['PC1_exact_replay'] = pc['PC1_pass']; res['PC2_instrument_equivalence'] = pc['PC2_pass']
    pools = json.load(open(a.pools)); pc3 = []; ratios = []
    for cell in pools['cells']:
        t = cell['t']; pub = next(c for c in pc['pc1'] if c['t'] == t)['published']['gain_vs_mean']
        lo, hi = cell['realised_lo95'], cell['realised_hi95']
        pc3.append(dict(t=t, K=pools['K'], pools=pools['pools'], pred_lo=lo, pred_hi=hi, published=pub,
                        inside=bool(lo <= pub <= hi), median_realised=float(np.median(cell['realised']))))
        bkey = ('F', 'k20', t, 'J', 'J', pools['K'])
        if bkey in reps and len(cell['ustat_J']) >= 3:
            ratios.append(cell['ustat_J_sd'] / float(np.std(reps[bkey], ddof=1)))
    calib[0] = max(1.0, float(np.mean(ratios))) if ratios else 1.0
    res['bootstrap_calibration'] = dict(sd_ratio_empirical_over_bootstrap=ratios, factor=calib[0])
    for key_ in list(reps): reps[key_] = centred(reps[key_], G[key_])
    res['PC3'] = pc3; res['PC3_pass'] = bool(pc3) and all(x['inside'] for x in pc3)
    # ---------- fits (arm F: J selector on J, K) ----------
    def fit_arm(arm, curves, Kfit, train_max, sname='J', vname='J', tfilter=None):
        out = {}
        for cur in curves:
            ts = sorted(t for (ar, cu, t, s, v, KK) in G if ar == arm and cu == cur and s == sname and v == vname and KK == Kfit
                        and (tfilter is None or tfilter(cur, t)))
            if len(ts) < 3: continue
            Gv = np.array([G[(arm, cur, t, sname, vname, Kfit)] for t in ts])
            R_ = np.array([reps.get((arm, cur, t, sname, vname, Kfit), np.full(a.B or 2, Gv[i])) for i, t in enumerate(ts)])
            se = {'M1': np.std(np.log(np.maximum(R_, 1e-9)), axis=1, ddof=1),
                  'M2': np.std(np.log(np.maximum(R_ - 1, 1e-9)), axis=1, ddof=1)}
            se['M3'] = se['M2']
            tr = [i for i, t in enumerate(ts) if t <= train_max]; ho = [i for i, t in enumerate(ts) if t > train_max]
            o = dict(ts=ts, G=Gv.tolist(), heldout_t=[ts[i] for i in ho], scores={}, fits_train={}, fits_all={})
            for mdl in MODELS:
                if len(tr) < 2: continue
                bt = fit(mdl, [ts[i] for i in tr], Gv[tr], {k_: v[tr] for k_, v in se.items()})
                o['fits_train'][mdl] = bt.tolist()
                o['scores'][mdl] = heldout_score(mdl, bt, [ts[i] for i in ho], Gv[ho]) if ho else float('nan')
                ba = fit(mdl, ts, Gv, se); o['fits_all'][mdl] = ba.tolist()
                bsb = []
                for b in range(R_.shape[1]):
                    Gb = R_[:, b]
                    if mdl != 'M1' and np.any(Gb <= 1): continue
                    bsb.append(fit(mdl, ts, Gb, se).tolist())
                o.setdefault('boot_fits', {})[mdl] = bsb
            out[cur] = o
        return out
    curvesF = sorted({cu for (ar, cu, *_r) in G if ar == 'F'})
    KF = max(KK for (ar, cu, t, s, v, KK) in G if ar == 'F' and s == 'J' and v == 'J')
    fitsF = fit_arm('F', curvesF, KF, a.train_max)
    # ---------- arm I ----------
    iso = load_iso(a.iso); kname = {20: 'k20', 24: 'k24r'}
    for (k, t), rows in sorted(iso.items()):
        J = np.array([x['J'] for x in rows], float); cols = np.array([x['columns'] for x in rows])
        cov = np.array([x['covered_points'] for x in rows], float); seed = np.array([x['seed'] for x in rows])
        bp = np.array([x['base_points'] for x in rows]); sel_ops = np.array([x['selector_group_ops'] for x in rows])
        pair_adds = bp * (bp + 1) // 2 - cols                      # the binary's O(c^2) pair additions
        rP_ops = (sel_ops - pair_adds) / np.maximum(bp, 1)          # [r]P group ops per signed base point
        N = len(rows); K = min(a.Kiso, N // 2); cur = kname[k]; r = CURVES[cur][3] // 8
        rho_i = cols.mean() / (1 << t); Diso = float(cols.var(ddof=1) / ((1 << t) * rho_i * (1 - rho_i)))
        for vname, vals in (('J', J), ('covered', cov)):
            for KK in sorted({10, 100, K}):
                if KK > N // 2: continue
                g = gain_point(vals, [J, cols, -seed], KK)
                if KK == K and a.B:
                    reps[('I', cur, t, 'J', vname, KK)] = centred(boot_gain(vals, [J, cols, -seed], KK, a.B,
                                                                            derived_seed('boot', 'iso', k, t, vname, KK)), g)
                G[('I', cur, t, 'J', vname, KK)] = g
                rowsout.append((('I', cur, t, 'J', vname, KK), cols.mean(), vals.mean(), Diso))
        res.setdefault('I_cells', []).append(dict(curve=cur, t=t, N=N, mean_cols=float(cols.mean()), mean_J=float(J.mean()),
            cov_over_r=float(cov.mean() / r), dispersion_D=Diso, build_group_ops_mean=float(np.mean([x['build_group_ops'] for x in rows])),
            rP_group_ops_per_point=float(np.mean(rP_ops)), selector_group_ops_binary_mean=float(sel_ops.mean())))
    KI = max([KK for (ar, cu, t, s, v, KK) in G if ar == 'I'] or [0])
    fitsI = fit_arm('I', sorted({cu for (ar, cu, *_r) in G if ar == 'I'}), KI, a.iso_train_max) if KI else {}
    # ---------- decisions on the functional form ----------
    def decide(fits):
        sc = {m: [f['scores'][m] for f in fits.values() if m in f['scores'] and not math.isnan(f['scores'][m])] for m in MODELS}
        sc = {m: v for m, v in sc.items() if v}
        if not sc: return None
        win, worst = choose(sc)
        alpha = {cu: f['fits_all']['M2'][1] for cu, f in fits.items() if 'M2' in f['fits_all']}
        alpha_ci = {cu: [float(np.percentile([b[1] for b in f['boot_fits']['M2']], q)) for q in (2.5, 97.5)]
                    for cu, f in fits.items() if f.get('boot_fits', {}).get('M2')}
        return dict(winner=win, worst_heldout=worst, adequate=worst[win] <= math.log(2), alpha_M2=alpha, alpha_M2_ci=alpha_ci)
    res['fits_F'] = {cu: {k_: v for k_, v in f.items() if k_ != 'boot_fits'} for cu, f in fitsF.items()}
    res['fits_I'] = {cu: {k_: v for k_, v in f.items() if k_ != 'boot_fits'} for cu, f in fitsI.items()}
    res['decision_F'] = decide(fitsF); res['decision_I'] = decide(fitsI)
    # composition-only selector: alpha of Phi gain (P3)
    fitsPhi = fit_arm('F', curvesF, KF, 99, 'Phi', 'Phi')
    unsat = {(c_['curve'], c_['t']) for c_ in res['F_cells'] if c_['cov_over_r'] is not None and c_['cov_over_r'] <= 0.05}
    fitsCov = fit_arm('F', curvesF, KF, 99, 'J', 'covered', tfilter=lambda cu, t: (cu, t) in unsat)
    res['descriptive_alpha_covered_unsaturated'] = {cu: f['fits_all'].get('M2', [None, None])[1] for cu, f in fitsCov.items()}
    res['alpha_Phi'] = {cu: f['fits_all'].get('M2', [None, None])[1] for cu, f in fitsPhi.items()}
    # c-only share of the J-selector excess at t >= 8 (P3b)
    share = {}
    for cur in curvesF:
        for t in range(8, 20):
            if ('F', cur, t, 'J', 'J', KF) in G and ('F', cur, t, 'c', 'J', KF) in G:
                share[f'{cur}_t{t}'] = (G[('F', cur, t, 'c', 'J', KF)] - 1) / (G[('F', cur, t, 'J', 'J', KF)] - 1)
    res['c_only_share'] = share
    # real vs surrogate (P4), decoy (P8), K-scaling (P9)
    p4 = {}; p8 = {}; p9 = {}
    for cur in curvesF:
        for t in range(4, 20):
            gj = G.get(('F', cur, t, 'J', 'J', KF)); gs = G.get(('F', cur, t, 'surrogate', 'surrJ', KF))
            if gj and gs and t >= 6: p4[f'{cur}_t{t}'] = (gj - 1) / (gs - 1)
            gd = G.get(('F', cur, t, 'decoy', 'J', KF))
            if gd:
                Rr = cells[(cur, t)]['rows'][1:]; Jv = Rr[:, 7].astype(float)
                w = selection_weights(len(Jv), KF)
                sd0 = math.sqrt(float(np.sum(w ** 2))) * Jv.std() / Jv.mean()   # null SD of a label-blind choice
                p8[f'{cur}_t{t}'] = dict(gain=gd, z=(gd - 1) / sd0)
            g100 = G.get(('F', cur, t, 'J', 'J', 100))
            if gj and g100 and t >= 8: p9[f'{cur}_t{t}'] = (gj - 1) / (g100 - 1)
    res['P4_real_over_surrogate_excess'] = p4; res['P8_decoy_gain'] = p8; res['P9_excess_ratio_K2000_over_K100'] = p9
    res['P9_gaussian_ratio'] = zK(KF) / zK(100)
    # ---------- arm R (real Curve25519) ----------
    Z = np.load(a.real); real_log = [json.loads(l) for l in open(a.real_log)]
    rsum = {}
    for key in Z.files:
        if key.startswith('r1_t'):
            t = int(key[4:]); rows = Z[key]; c = rows[:, 0]; Jr = J_counts(rows[:, 1:]).astype(float)
            N = len(rows); KK = min(a.Kreal, N // 2); idx = np.arange(N)
            g = gain_point(Jr, [Jr, c, -idx], KK)
            bs = centred(boot_gain(Jr, [Jr, c, -idx], KK, a.B or 2, derived_seed('boot', 'real', t, KK)), g)
            reps[('R', 'curve25519', t, 'J', 'J', KK)] = bs; G[('R', 'curve25519', t, 'J', 'J', KK)] = g
            props = rows[:, 1:].sum(0) / rows[:, 1:].sum()
            gm = G.get(('F', 'k24f', t, 'J', 'J', KK)); gm20 = G.get(('F', 'k20', t, 'J', 'J', KK))
            lo, hi = np.percentile(bs, [2.5, 97.5])
            ok = None
            if gm:
                ok = bool((hi - 1) >= (gm - 1) / 1.5 and (lo - 1) <= (gm - 1) * 1.5)
            rho = c.mean() / (1 << t)
            rsum[f'r1_t{t}'] = dict(t=t, N=N, K=KK, gain_J=g, lo95=float(lo), hi95=float(hi), mini_k24f=gm, mini_k20=gm20,
                                    P6_pass=ok, class_props=props.tolist(),
                                    dispersion_D=float(c.var(ddof=1) / ((1 << t) * rho * (1 - rho))))
            rowsout.append((('R', 'curve25519', t, 'J', 'J', KK), c.mean(), Jr.mean(), rsum[f'r1_t{t}']['dispersion_D']))
        if key.startswith('r1c_t'):
            t = int(key[5:]); c = Z[key]; rho = c.mean() / (1 << t)
            rsum[f'r1c_t{t}'] = dict(t=t, N=len(c), dispersion_D=float(c.var(ddof=1) / ((1 << t) * rho * (1 - rho))))
    if 'r2_counts' in Z.files:
        cnt = Z['r2_counts']; tri = Z['r2_trials']; Kf = cnt.shape[0]
        tot = cnt.sum(1)                                     # K x 5
        E = np.outer(tot.sum(1), tot.sum(0)) / tot.sum(); x2 = float(((tot - E) ** 2 / E).sum()); df = (Kf - 1) * 4
        succ = tot.sum(1); trials = tri.sum(1); fail = trials - succ
        T2 = np.vstack([succ, fail]).T; E2 = np.outer(T2.sum(1), T2.sum(0)) / T2.sum()
        x2c = float(((T2 - E2) ** 2 / E2).sum())
        pooled = tot.sum(0); exp5 = np.array([1 / 8, 1 / 4, 1 / 4, 1 / 4, 1 / 8]) * pooled.sum()
        x2p = float(((pooled - exp5) ** 2 / exp5).sum())
        def jrel(h):
            mm = cnt[:, h, :].astype(float); nn = mm.sum(1); rho_h = nn / tri[:, h]
            phi = (mm[:, 1] * (mm[:, 1] - 1) + mm[:, 2] * (mm[:, 2] - 1) + mm[:, 3] * (mm[:, 3] - 1)
                   + 2 * mm[:, 0] * (mm[:, 0] - 1) + 2 * mm[:, 4] * (mm[:, 4] - 1)) / (nn * (nn - 1)) * 4
            return rho_h ** 2 * phi
        jA, jB = jrel(0), jrel(1)
        sel = int(np.argmax(jA)); cv_excess = float(jB[sel] / jB.mean() - 1)
        null = jB / jB.mean() - 1; q = np.percentile(null, [0.5, 99.5])
        within = float(np.mean((jA - jB) ** 2) / 2)                # per-half sampling variance of J_rel
        total = float(np.var(np.concatenate([jA, jB]), ddof=1))
        sb2 = (np.var((jA + jB) / 2, ddof=1) - within / 2)          # between-frame variance component
        rsum['r2'] = dict(K=Kf, n=int(cnt[0].sum()), chi2_classes=x2, df=df, p_classes=chi2_sf(x2, df),
                          chi2_oncurve=x2c, df_oncurve=Kf - 1, p_oncurve=chi2_sf(x2c, Kf - 1),
                          chi2_pooled_vs_uniform=x2p, p_pooled=chi2_sf(x2p, 4), pooled_props=(pooled / pooled.sum()).tolist(),
                          oncurve_fraction=float(succ.sum() / trials.sum()), cv_selected_excess=cv_excess,
                          cv_null_99=[float(q[0]), float(q[1])], cv_inside_null=bool(q[0] <= cv_excess <= q[1]),
                          rel_sd_Jrel_per_frame=float(math.sqrt(within) / np.mean(jA)), between_var_estimate=float(sb2 / np.mean(jA) ** 2))
    res['R'] = rsum; res['real_log'] = [x for x in real_log if x.get('event') != 'done']
    # ---------- derived 255-bit extrapolation (DERIVED, NOT MEASURED) ----------
    der = dict(label='DERIVED EXTRAPOLATION, not a measurement', log2_r=LOG2R255, log2_rho_ops=RHO_OPS_LOG2)
    if ('k24f' in fitsF) or curvesF:
        cur0 = 'k24f' if 'k24f' in curvesF else curvesF[0]
        tsJ = [c_['t'] for c_ in res['F_cells'] if c_['curve'] == cur0]
        mJ = [c_['mean_J'] for c_ in res['F_cells'] if c_['curve'] == cur0]
        b = float(np.mean(np.log2(mJ) - 2 * np.array(tsJ)))      # log2 mean J = 2t + b
        der['log2_meanJ_intercept'] = b
        der['tstar_m2'] = math.ceil((LOG2R255 - b) / 2)
        der['tstar_m3_formula'] = math.ceil((LOG2R255 + 3 + math.log2(6)) / 3)
        der['tstar_m4_formula'] = math.ceil((LOG2R255 + 3 + math.log2(24)) / 4)
        win = (res['decision_F'] or {}).get('winner', 'M3')
        ext = {}
        for cur, f in fitsF.items():
            for mdl in MODELS:
                if mdl not in f['fits_all']: continue
                e = {}
                for ts_ in (der['tstar_m4_formula'], der['tstar_m3_formula'], der['tstar_m2']):
                    pt = float(predict_excess(mdl, f['fits_all'][mdl], ts_))
                    bb = [float(predict_excess(mdl, bf, ts_)) for bf in f['boot_fits'].get(mdl, [])]
                    e[str(ts_)] = dict(excess=pt, lo95=float(np.percentile(bb, 2.5)) if bb else None,
                                       hi95=float(np.percentile(bb, 97.5)) if bb else None)
                ext.setdefault(cur, {})[mdl] = e
        der['excess_at_tstar_K'] = KF; der['excess_at_tstar'] = ext; der['winner'] = win
        ts_ = der['tstar_m2']
        e_w = max(ext[cu][win][str(ts_)]['excess'] for cu in ext if win in ext[cu])
        e_hi = max((ext[cu][win][str(ts_)]['hi95'] or 0) for cu in ext if win in ext[cu])
        der['excess_tstar_winner_max_over_curves'] = e_w; der['excess_tstar_winner_hi95'] = e_hi
        der['excess_tstar_K_2_64'] = e_w * zK_asym(64) / zK(KF)       # Gaussian-tail K scaling (derived)
        # selection cost at t* per candidate frame (derived), group operations; Legendre priced at w GAE
        c_star = 2.0 ** (ts_ - 1)
        lab_ops = [x for x in res['real_log'] if x.get('event') == 'r2']
        ops_label = lab_ops[0]['group_ops_per_label'] if lab_ops else 2 * R255.bit_length()
        der['ops_per_label_ladder'] = ops_label
        der['ops_per_label_double_and_add'] = (R255.bit_length() - 1) + (bin(R255).count('1') - 1)
        cost = {}
        for w in (0, 0.1, 1):
            full = c_star * ops_label + w * (2.0 ** ts_)
            onebit = w * (2.0 ** ts_ + c_star)
            cost[str(w)] = dict(full_label_log2=math.log2(full), full_label_over_rho=2 ** (math.log2(full) - RHO_OPS_LOG2),
                                onebit_log2=(math.log2(onebit) if onebit > 0 else None),
                                onebit_over_rho=(2 ** (math.log2(onebit) - RHO_OPS_LOG2) if onebit > 0 else 0.0))
        der['cost_per_frame_full_enumeration'] = cost
        # sampled selection: samples per frame needed to resolve the true excess (SE of J_rel ~ s/sqrt(n))
        s_rel = rsum.get('r2', {}).get('rel_sd_Jrel_per_frame')
        n_half = rsum.get('r2', {}).get('n', 0) / 2
        s1 = s_rel * math.sqrt(n_half) if s_rel else 2.0
        der['per_sample_rel_sd_s1'] = s1
        n_req = (s1 * zK(KF) / e_w) ** 2 if e_w > 0 else float('inf')
        der['samples_per_frame_to_resolve_log2'] = math.log2(n_req)
        der['sampled_cost_per_frame_full_label_over_rho'] = 2 ** (math.log2(n_req * (ops_label + 2)) - RHO_OPS_LOG2)
        der['breakeven_IC_cost_over_rho_min'] = (min(v['full_label_over_rho'] for v in cost.values()) * (1 + 1 / e_w)
                                                  if e_w > 0 else float('inf'))
        # isogeny-tower base at t*: needs an auxiliary E'/F_p with 2^t* | #E'(F_p) (prime_tower.rs). Traces a with
        # p + 1 - a = 0 mod 2^t* in the Hasse interval: about 4 sqrt(p) / 2^t* of them (derived count).
        der['iso_aux_trace_candidates_log2'] = math.log2(4) + 0.5 * math.log2(2 ** 255 - 19) - ts_
    res['derived_255'] = der
    # ---------- prediction verdicts ----------
    V = {}
    V['P0_PC1'] = res['PC1_exact_replay']; V['P0_PC2'] = res['PC2_instrument_equivalence']; V['P0_PC3'] = res['PC3_pass']
    dF = res['decision_F']
    if dF:
        V['P1_form_not_M1'] = dF['winner'] in ('M2', 'M3')
        V['P1_alpha_in_0.35_0.65'] = all(0.35 <= v <= 0.65 for v in dF['alpha_M2'].values())
        V['P2_adequate'] = dF['adequate']
    V['P3a_alpha_Phi_ge_0.8'] = bool(res['alpha_Phi']) and all(v is not None and v >= 0.8 for v in res['alpha_Phi'].values())
    V['P3b_c_share_ge_0.7'] = bool(share) and all(v >= 0.7 for v in share.values())
    gm4 = {cu: float(np.exp(np.mean([math.log(v) for k_, v in p4.items() if k_.startswith(cu + '_') and v > 0]))) for cu in curvesF
           if any(k_.startswith(cu + '_') for k_ in p4)}
    res['P4_geomean_by_curve'] = gm4
    V['P4_surrogate_geomean_within_0.75_1.33'] = bool(gm4) and all(0.75 <= v <= 1.33 for v in gm4.values())
    flog = [json.loads(l) for f in a.frames_log for l in open(f)]
    fc = [x for x in flog if x.get('event') == 'cell']
    res['P5_cost_cells'] = [dict(curve=x['curve'], t=x['t'], ops_per_point=x['cost_group_ops_per_point'],
                                 ops_over_log2r=x['cost_group_ops_per_point'] / x['log2r'],
                                 labels_ok=x['cost_label_matches'], points=x['cost_points'],
                                 legendre_per_frame=x['legendre_per_frame'],
                                 full_label_ops_per_frame=x['cost_group_ops_per_point'] * x['mean_c']) for x in fc]
    V['P5_cost'] = bool(fc) and all(x['cost_label_matches'] == x['cost_points'] and
                                    1.0 <= x['cost_group_ops_per_point'] / x['log2r'] <= 1.6 for x in fc)
    dI = res['decision_I']
    if dI: V['P7_iso_alpha_in_0.35_0.65'] = all(0.35 <= v <= 0.65 for v in dI['alpha_M2'].values())
    V['P8_decoy_abs_z_le_3.5'] = bool(p8) and all(abs(v['z']) <= 3.5 for v in p8.values())
    gm9 = {cu: float(np.exp(np.mean([math.log(v) for k_, v in p9.items() if k_.startswith(cu + '_') and v > 0]))) for cu in curvesF
           if any(k_.startswith(cu + '_') for k_ in p9)}
    res['P9_geomean_by_curve'] = gm9
    V['P9_K_ratio_geomean_in_1.1_1.6'] = bool(gm9) and all(1.1 <= v <= 1.6 for v in gm9.values())
    r1 = [v for k_, v in rsum.items() if k_.startswith('r1_t') and v['P6_pass'] is not None]
    V['P6_real_matches_mini'] = bool(r1) and all(v['P6_pass'] for v in r1)
    r1c = [v['dispersion_D'] for k_, v in rsum.items() if k_.startswith('r1c_t')]
    V['R1_dispersion_in_0.7_1.4'] = bool(r1c) and all(0.7 <= d <= 1.4 for d in r1c)
    if 'r2' in rsum:
        V['R2_homogeneity_p_ge_0.001'] = rsum['r2']['p_classes'] >= 1e-3 and rsum['r2']['p_oncurve'] >= 1e-3
        V['R2_pooled_props_p_ge_0.001'] = rsum['r2']['p_pooled'] >= 1e-3
        V['R3_cv_inside_null'] = rsum['r2']['cv_inside_null']
    par = sum(x.get('parity_mismatches', 0) for x in res['real_log'])
    V['R3b_parity_identity'] = par == 0
    if 'excess_tstar_winner_hi95' in der: V['R4_excess_tstar_le_1e-15'] = der['excess_tstar_winner_hi95'] <= 1e-15
    res['verdicts'] = V
    json.dump(res, open(os.path.join(a.out, 'results.json'), 'w'), indent=1, default=float)
    for key_, mc, mv, Dv in rowsout:
        g = G[key_]; lo = hi = float('nan')
        if key_ in reps: lo, hi = np.percentile(reps[key_], [2.5, 97.5])
        csv.append(','.join(str(x) for x in key_) + f',{g:.6g},{lo:.6g},{hi:.6g},{mc:.6g},{mv:.6g},{Dv:.4g}')
    open(os.path.join(a.out, 'gains.csv'), 'w').write('\n'.join(csv) + '\n')
    try:
        figure(a, G, fitsF, fitsI, rsum, der, KF, KI)
    except Exception as ex:   # figures are not evidence; report and continue
        print('figure failed:', ex, file=sys.stderr)
    print(json.dumps(V, indent=1))

def figure(a, G, fitsF, fitsI, rsum, der, KF, KI):
    import matplotlib; matplotlib.use('Agg'); import matplotlib.pyplot as plt
    fig, ax = plt.subplots(1, 2, figsize=(9, 3.6))
    col = {'k20': '#1f77b4', 'k24f': '#d62728', 'k24r': '#2ca02c'}
    for cur, f in fitsF.items():
        ts = np.array(f['ts']); e = np.array(f['G']) - 1
        ax[0].semilogy(ts, e, 'o', color=col.get(cur, 'k'), ms=4, label=f'frame {cur} (K={KF})')
        mdl = der.get('winner', 'M3')
        if mdl in f['fits_all']:
            tt = np.linspace(4, max(ts), 50); ax[0].semilogy(tt, predict_excess(mdl, f['fits_all'][mdl], tt), '-', color=col.get(cur, 'k'), lw=0.8)
    for cur, f in fitsI.items():
        ax[0].semilogy(f['ts'], np.array(f['G']) - 1, 's', mfc='none', color=col.get(cur, 'k'), ms=4, label=f'iso seed {cur} (K={KI})')
    for k_, v in rsum.items():
        if k_.startswith('r1_t'):
            ax[0].errorbar([v['t']], [v['gain_J'] - 1], yerr=[[v['gain_J'] - v['lo95']], [v['hi95'] - v['gain_J']]],
                           fmt='*', color='k', ms=7, label='Curve25519 (K=%d)' % v['K'] if v['t'] == 6 else None)
    ax[0].set_xlabel('t'); ax[0].set_ylabel('selected / mean J  - 1'); ax[0].legend(fontsize=6); ax[0].set_title('measured', fontsize=9)
    ts_ = der.get('tstar_m2', 128); tt = np.linspace(4, ts_ + 2, 200)
    for cur, f in fitsF.items():
        for mdl, ls in (('M2', '--'), ('M3', '-')):
            if mdl in f['fits_all']:
                ax[1].plot(tt, np.log10(np.maximum(predict_excess(mdl, f['fits_all'][mdl], tt), 1e-300)), ls, color=col.get(cur, 'k'), lw=0.8,
                           label=f'{cur} {mdl}')
    ax[1].axvline(ts_, color='gray', lw=0.6); ax[1].text(ts_ - 30, -3, f't* = {ts_} (derived)', fontsize=7)
    ax[1].set_xlabel('t'); ax[1].set_ylabel('log10 excess (DERIVED)'); ax[1].legend(fontsize=6); ax[1].set_title('extrapolation (derived)', fontsize=9)
    fig.tight_layout()
    fig.savefig(os.path.join(a.out, 'fig_gain_vs_t.svg')); fig.savefig(os.path.join(a.out, 'fig_gain_vs_t.png'), dpi=70)

if __name__ == '__main__':
    main()
