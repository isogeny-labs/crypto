"""mllc training harness: arms, models, statistics, decision rule (see PROTOCOL.md).
Every number it writes is [pcc:unverified] until a pcc claim is filed."""
import os
for _v in ("OMP_NUM_THREADS","OPENBLAS_NUM_THREADS","MKL_NUM_THREADS"): os.environ.setdefault(_v, "2")
import json, math, sys, time, argparse, platform
import numpy as np
from scipy.stats import binomtest, norm
import sklearn
from sklearn.linear_model import LogisticRegression
from sklearn.ensemble import HistGradientBoostingClassifier
from sklearn.neural_network import MLPClassifier
import mllc_ecgen as G

PROTOCOL = dict(
    sizes_log2=[12, 16, 20, 24], curves_per_size=3, labels=['lsb', 'msb'],
    models=['logreg', 'hgb', 'mlp'], planted_delta=[0.02, 0.05, 0.10, 0.20],
    height_ladder=['1', '2', '3', '5/3', 'rand0', 'rand1'], null_retrains=20, null_retrains_mlp=10,
    n_total={12: None, 16: None, 20: 700_000, 24: 700_000}, test_frac={12: .2, 16: .2, 20: 2/7, 24: 2/7},
    mlp_train_cap=500_000, alpha_family=0.01, gate_delta=0.10, replicate_curves=2,
    master_seed=20261107)

def make_model(name, seed):
    if name == 'logreg': return LogisticRegression(max_iter=300, C=1.0)
    if name == 'hgb': return HistGradientBoostingClassifier(max_iter=200, max_depth=6, learning_rate=0.1, random_state=seed)
    if name == 'mlp': return MLPClassifier(hidden_layer_sizes=(128, 128), batch_size=512, max_iter=20, random_state=seed,
                                           early_stopping=False, learning_rate_init=1e-3)
    raise ValueError(name)

def fit_acc(name, Xtr, ytr, Xte, yte, seed, mlp_cap):
    if name == 'mlp' and len(ytr) > mlp_cap: Xtr, ytr = Xtr[:mlp_cap], ytr[:mlp_cap]
    m = make_model(name, seed); m.fit(Xtr, ytr)
    return int((m.predict(Xte) == yte).sum())

def wilson(c, n, z=1.96):
    ph = c/n; d = 1 + z*z/n; ce = (ph + z*z/(2*n))/d; h = z*math.sqrt(ph*(1-ph)/n + z*z/(4*n*n))/d
    return ce-h, ce+h

def mde(n, alpha, power=0.9):
    """minimum detectable accuracy advantage over 1/2 (normal approx, one-sided binomial test)."""
    return 0.5*(norm.ppf(1-alpha) + norm.ppf(power))/math.sqrt(n)

def cell_stats(c, n):
    return dict(correct=c, n=n, acc=c/n, ci95=wilson(c, n), p_one_sided=binomtest(c, n, 0.5, alternative='greater').pvalue)

# ---- arms -------------------------------------------------------------------
def inv_mod(a, N): return pow(a, -1, N)

def build_arms(curve, m, ks, rng, cfg):
    """Return {arm_name: (X (len(ks), f) float32, label_kind)} for all arms on this curve's N."""
    N = curve['N']; arms = {}
    x, y = G.points_for(curve, ks)
    assert G.on_curve(curve, x, y)
    Xc = np.hstack([G.bits(x, m), G.bits(y, m)])
    for lab in cfg['labels']: arms[f'curve|{lab}'] = (Xc, lab)
    yl = {lab: G.labels(ks, N, lab) for lab in cfg['labels']}
    for d in cfg['planted_delta']:       # explicit planted bit on the curve encoding
        flip = rng.random(len(ks)) < (0.5 - d)   # P(feature = label) = 0.5 + d
        plant = np.where(flip, 1 - yl['lsb'], yl['lsb']).astype(np.float32)[:, None]
        arms[f'PC-A|delta={d}|lsb'] = (np.hstack([Xc, plant]), 'lsb')
    for c in cfg['height_ladder']:       # Z/N, u = c*k mod N, features bits(u), height of c
        if c.startswith('rand'): cc = int(rng.integers(2, N-1)); cn = cc
        else:
            nu, _, de = c.partition('/'); cn = int(nu) * inv_mod(int(de or 1), N) % N
        u = (ks * cn) % N
        labs = ['lsb', 'msb'] if c in ('1', 'rand0') else ['lsb']
        for lab in labs: arms[f'PC-B|c={c}|{lab}'] = (G.bits(u, m), lab)
    return arms

def fpstar_arm(m, ks_seed, n, rng):
    p, g = G.safe_prime_group(m, ks_seed); N = p-1
    ks = rng.choice(N, size=n, replace=False) + 0
    h = G.fp_star_points(p, g, ks)
    return N, ks, (G.bits(h, m), 'lsb')   # label k mod 2 == Legendre(h): smooth character r=2

def run_cell(arm_name, X, lab, ks, N, ntr, models, cfg, seed, null_R, rng):
    y = G.labels(ks, N, lab) if lab in ('lsb', 'msb') else None
    out = {}
    for mn in models:
        Xtr, ytr, Xte, yte = X[:ntr], y[:ntr], X[ntr:], y[ntr:]
        c = fit_acc(mn, Xtr, ytr, Xte, yte, seed, cfg['mlp_train_cap'])
        rec = cell_stats(c, len(yte))
        if null_R and arm_name.startswith('curve'):    # random-relabelling null: permute labels on fixed encodings
            R = cfg['null_retrains_mlp'] if mn == 'mlp' else null_R
            accs = []
            for r in range(R):
                yp = rng.permutation(y)
                accs.append(fit_acc(mn, Xtr, yp[:ntr], Xte, yp[ntr:], seed + r + 1, cfg['mlp_train_cap'])/len(yte))
            rec['null_accs'] = accs; rec['null_max'] = max(accs); rec['null_mean'] = float(np.mean(accs))
        out[mn] = rec
    return out

def run_size(m, cfg, models, quick=False, null_R=None, log=print):
    res = dict(m=m, curves=[], mde={})
    for ci in range(cfg['curves_per_size']):
        seed = cfg['master_seed'] + 1000*m + ci
        t0 = time.time(); curve = G.make_curve(m, seed); G.validate(curve)
        N = curve['N']; rng = np.random.default_rng(seed)
        ntot = cfg['n_total'][m] or (N-1)
        ks = rng.permutation(N-1)[:ntot] + 1; ntr = int(round(ntot*(1-cfg['test_frac'][m])))
        arms = build_arms(curve, m, ks, rng, cfg)
        cell = dict(curve={k: curve[k] for k in ('p', 'a', 'b', 'N')}, gen_seconds=time.time()-t0, arms={})
        for name, (X, lab) in arms.items():
            cell['arms'][name] = run_cell(name, X, lab, ks, N, ntr, models, cfg, seed, null_R, rng)
            log(f"m={m} curve={ci} {name}: " + ", ".join(f"{mn}={v['acc']:.4f}" for mn, v in cell['arms'][name].items()))
        Nf, ksf, (Xf, labf) = fpstar_arm(m, seed, min(ntot, N-1), rng)
        cell['arms']['PC-D|Fp*-Legendre|lsb'] = run_cell('PC-D', Xf, labf, ksf, Nf, int(round(len(ksf)*(1-cfg['test_frac'][m]))), models, cfg, seed, 0, rng)
        res['curves'].append(cell)
        nte = ntot - ntr
    res['n_test_per_curve'] = nte
    a = cfg['alpha_family']/(len(cfg['sizes_log2'])*len(models)*len(cfg['labels']))
    res['mde'] = dict(per_curve=mde(nte, a), pooled_curves=mde(nte*cfg['curves_per_size'], a), alpha_cell=a)
    return res

# ---- decision rule (PROTOCOL.md section 7) ----------------------------------
def decide(res, cfg, model, label):
    """NULL / SIGNAL / UNDERPOWERED for the real-curve arm of one (size, model, label)."""
    a = res['mde']['alpha_cell']; cur = res['curves']
    def pooled(armname):
        cs = sum(c['arms'][armname][model]['correct'] for c in cur); n = sum(c['arms'][armname][model]['n'] for c in cur)
        return binomtest(cs, n, 0.5, alternative='greater').pvalue, cs/n
    p, acc = pooled(f'curve|{label}')
    indiv = sum(c['arms'][f'curve|{label}'][model]['p_one_sided'] < 0.05 and
                c['arms'][f'curve|{label}'][model]['acc'] > c['arms'][f'curve|{label}'][model].get('null_max', 1) for c in cur)
    gate = pooled(f"PC-A|delta={cfg['gate_delta']}|lsb")[0] < a
    if p < a and indiv >= cfg['replicate_curves']: return 'SIGNAL', dict(p=p, acc=acc, indiv=indiv, gate=gate)
    if not gate: return 'UNDERPOWERED', dict(p=p, acc=acc, indiv=indiv, gate=gate)
    return 'NULL', dict(p=p, acc=acc, indiv=indiv, gate=gate)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--sizes', type=int, nargs='+', default=PROTOCOL['sizes_log2'])
    ap.add_argument('--models', nargs='+', default=PROTOCOL['models'])
    ap.add_argument('--out', required=True)
    ap.add_argument('--dev', action='store_true', help='development check: 1 curve, null_R=5, no claim')
    a = ap.parse_args()
    cfg = json.loads(json.dumps(PROTOCOL)); cfg['n_total'] = {int(k): v for k, v in cfg['n_total'].items()}
    cfg['test_frac'] = {int(k): v for k, v in cfg['test_frac'].items()}
    null_R = cfg['null_retrains']
    if a.dev: cfg['curves_per_size'] = 3; null_R = 5; cfg['null_retrains_mlp'] = 5
    allres = dict(protocol=cfg, dev=a.dev, versions=dict(python=platform.python_version(), numpy=np.__version__,
                  sklearn=sklearn.__version__, platform=platform.platform()), sizes={})
    for m in a.sizes:
        r = run_size(m, cfg, a.models, null_R=null_R)
        r['decisions'] = {f'{mn}|{lab}': decide(r, cfg, mn, lab) for mn in a.models for lab in cfg['labels']}
        allres['sizes'][str(m)] = r
        json.dump(allres, open(a.out, 'w'), indent=1, default=float)
    print(json.dumps({m: {k: v[0] for k, v in s['decisions'].items()} for m, s in allres['sizes'].items()}, indent=1))

if __name__ == '__main__': main()
