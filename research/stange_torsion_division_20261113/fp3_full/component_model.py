#!/usr/bin/env python3
"""Component extrapolation: fit each phase of IC (mode all) and rho as separate power laws in r
(OLS of log2 cost on log2 r over curves) and find where the summed IC cost meets rho, for several
linear-algebra pricings. Reads summary.json; prints a table (component_model.txt)."""
import json, math, os
HERE = os.path.dirname(os.path.abspath(__file__))
S = json.load(open(os.path.join(HERE, 'summary.json')))


def ols(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs); b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    return my - b * mx, b


def comp_fits(curves, la_key):
    xs = [c['log2r'] for c in curves]
    out = {}
    for k in ['walk', 'build', 'f4', 'multmatrix', 'wiedemann', 'rootfind', 'eigen', 'post']:
        out[k] = ols(xs, [math.log2(max(1, c['modes']['all']['phases'][k])) for c in curves])
    out['la'] = ols(xs, [math.log2(max(1, c['modes']['all']['phases'][la_key])) for c in curves])
    rho = ols(xs, [math.log2(c['rho_meq']) for c in curves])
    return out, rho


def ratio(fits, rho, x):
    tot = sum(2 ** (a + b * x) for a, b in fits.values())
    return tot / 2 ** (rho[0] + rho[1] * x)


print('| LA pricing | LA slope | min gap over log2 r <= 400 | at log2 r | parity at log2 r | dominant phase at the minimum |')
print('|---|---|---|---|---|---|')
res = []
for tag, key, label in (('Zr=1M', 'la', 'incremental elimination, Z_r = 1 M'), ('Zr=9M', 'la', 'incremental elimination, Z_r = 9 M'),
                        ('Zr=1M', 'la_wied_est', 'Wiedemann estimate 9NK, Z_r = 1 M'), ('Zr=9M', 'la_wied_est', 'Wiedemann estimate 9NK, Z_r = 9 M')):
    fits, rho = comp_fits(S['curves_' + tag], key)
    xs = [x / 4 for x in range(60, 1601)]
    rs = [ratio(fits, rho, x) for x in xs]
    i = min(range(len(xs)), key=lambda j: rs[j])
    par = next((x for x, r in zip(xs, rs) if r <= 1), None)
    dom = max(fits, key=lambda k: fits[k][0] + fits[k][1] * xs[i])
    res.append(dict(pricing=label, la_slope=fits['la'][1], min_gap=rs[i], at=xs[i], parity=par, dominant=dom,
                    slopes={k: v[1] for k, v in fits.items()}, rho_slope=rho[1]))
    print(f"| {label} | {fits['la'][1]:.3f} | {rs[i]:.3g} | {xs[i]:.0f} | {par if par is None else round(par)} | {dom} |")
json.dump(res, open(os.path.join(HERE, 'component_model.json'), 'w'), indent=1)


# ------------------------------------------------------------------ structured model
# The per-phase power laws above overstate root finding: z^p mod f needs no reduction while p < 64
# (deg f = 64), so its per-trial cost is linear in log2 p, not a power of r. Structured model, fitted
# on curves with p >= 251 and bootstrapped over curves:
#   trials = tau * p ; non-LA M-eq per trial = A + beta * log2 p ;
#   LA = Wiedemann 9 N K Z_r with N = nu * p, K = kappa * p (counted incremental LA is lower here);
#   rho M-eq = c * sqrt(r), r = p^3.
import random
print('\n## Structured model (mode all, p >= 251 curves, bootstrap over curves, 2000 resamples)\n')
print('| Z_r price | tau | A | beta | nu | kappa | c_rho | min gap (median, 95%) | at log2 r (median, 95%) | LA share at min | parity |')
print('|---|---|---|---|---|---|---|---|---|---|---|')
raw = {}
import glob
for f in sorted(glob.glob(os.path.join(HERE, 'runs', '*.json'))):
    d = json.load(open(f))
    if d['p'] < 251:
        continue
    for t in d['targets']:
        v = t['ic']['all']
        ph = v['phases']
        nonla = sum(x[0] + 3.0 * x[1] for x in ph.values())
        rho = sum(x['M'] + 3.0 * x['I'] for x in t['rho']) / len(t['rho'])
        raw.setdefault((d['p'], d['seed']), []).append(dict(p=d['p'], r=d['r'], trials=v['trials'], nonla=nonla,
                                                             N=v['ncols'], K=v['rels_kept'], rho=rho))


def model(curves, zrw):
    rows = [x for c in curves for x in c]
    tau = sum(x['trials'] for x in rows) / sum(x['p'] for x in rows)
    nu = sum(x['N'] for x in rows) / sum(x['p'] for x in rows)
    kap = sum(x['K'] for x in rows) / sum(x['p'] for x in rows)
    A, beta = ols([math.log2(x['p']) for x in rows], [x['nonla'] / x['trials'] for x in rows])
    c = sum(x['rho'] for x in rows) / sum(math.sqrt(x['r']) for x in rows)
    best = None; par = None
    for i in range(16 * 3, 16 * 200):
        lp = i / 16.0
        P_ = 2.0 ** lp
        dec = tau * P_ * (A + beta * lp)
        la = zrw * 9 * nu * kap * P_ * P_
        g = (dec + la) / (c * P_ ** 1.5)
        if best is None or g < best[0]:
            best = (g, 3 * lp, la / (dec + la))
        if par is None and g <= 1:
            par = 3 * lp
    return dict(tau=tau, A=A, beta=beta, nu=nu, kappa=kap, c=c, min_gap=best[0], at=best[1], la_share=best[2], parity=par)


keys = sorted(raw)
rng = random.Random(7)
SM = {}
for zrw in (1.0, 9.0):
    m0 = model([raw[k] for k in keys], zrw)
    bs = [model([raw[rng.choice(keys)] for _ in keys], zrw) for _ in range(2000)]
    q = lambda L, f: sorted(L)[int(f * (len(L) - 1))]
    g = [b['min_gap'] for b in bs]; a = [b['at'] for b in bs]
    npar = sum(1 for b in bs if b['parity'] is not None)
    SM[str(zrw)] = dict(point=m0, min_gap_95=[q(g, .025), q(g, .975)], at_95=[q(a, .025), q(a, .975)], boot_with_parity=npar)
    print(f"| {zrw:g} M | {m0['tau']:.2f} | {m0['A']:.3g} | {m0['beta']:.3g} | {m0['nu']:.3f} | {m0['kappa']:.3f} | {m0['c']:.1f} | "
          f"{m0['min_gap']:.0f} ({q(g, .025):.0f}..{q(g, .975):.0f}) | {m0['at']:.0f} ({q(a, .025):.0f}..{q(a, .975):.0f}) | "
          f"{100 * m0['la_share']:.0f}% | {'none' if m0['parity'] is None else round(m0['parity'])} ({npar}/2000 resamples reach parity) |")
json.dump(dict(power_laws=res, structured=SM), open(os.path.join(HERE, 'component_model.json'), 'w'), indent=1)
