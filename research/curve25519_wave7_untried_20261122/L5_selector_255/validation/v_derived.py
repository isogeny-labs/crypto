#!/usr/bin/env python3
"""Validator: arm R2/R3 statistics from raw counts (own chi-square tail), arm I cost cells, iso identity vs the
isogeny-seed study's data, and every derived 255-bit number from the validator's own fits.
Usage: v_derived.py <gains_full.json> <out.json>"""
import sys, os, json, math, glob, re
import numpy as np
BASE = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L5_selector_255'
G = json.load(open(sys.argv[1])); OUT = sys.argv[2]
res = {}

# ---------- chi-square upper tail, closed forms ----------
def chi2_sf(x, df):
    h = x / 2.0
    if df % 2 == 0:
        # Q = e^{-h} sum_{k < df/2} h^k / k!   (log-domain terms)
        terms = [k * math.log(h) - h - math.lgamma(k + 1) for k in range(df // 2)]
        mx = max(terms); return math.exp(mx) * sum(math.exp(t - mx) for t in terms)
    # odd df: Q = erfc(sqrt(h)) + e^{-h} sum_{k=1}^{(df-1)/2} h^{k-1/2} / Gamma(k + 1/2)
    s = math.erfc(math.sqrt(h))
    for k in range(1, (df - 1) // 2 + 1):
        s += math.exp((k - 0.5) * math.log(h) - h - math.lgamma(k + 0.5))
    return s

def chi2_table(T):
    T = np.asarray(T, float); E = np.outer(T.sum(1), T.sum(0)) / T.sum()
    return float(((T - E) ** 2 / E).sum())

Z = np.load(os.path.join(BASE, 'data', 'real25519.npz'))
cnt = Z['r2_counts'].astype(np.int64); tri = Z['r2_trials'].astype(np.int64)
def r2_stats(cnt, tri):
    K = cnt.shape[0]
    tot = cnt.sum(1)
    x_cls = chi2_table(tot); df_cls = (K - 1) * 4
    succ = tot.sum(1); fail = tri.sum(1) - succ
    x_on = chi2_table(np.c_[succ, fail]); df_on = K - 1
    pooled = tot.sum(0); e5 = np.array([1, 2, 2, 2, 1]) / 8 * pooled.sum()
    x_pool = float(((pooled - e5) ** 2 / e5).sum())
    # J_rel per half: rho_h^2 * 4 * (unbiased pair-collision rate of the folded classes, weighted 1,1,1 / 2,2)
    def jrel(h):
        m = cnt[:, h, :].astype(float); nn = m.sum(1); rho = nn / tri[:, h]
        wts = np.array([2, 1, 1, 1, 2], float)
        pair = (wts * m * (m - 1)).sum(1) / (nn * (nn - 1))
        return rho ** 2 * 4 * pair
    jA, jB = jrel(0), jrel(1)
    sel = int(np.argmax(jA)); exB = jB / jB.mean() - 1
    q = np.percentile(exB, [0.5, 99.5])
    within = float(np.mean((jA - jB) ** 2) / 2)
    between = float(np.var((jA + jB) / 2, ddof=1) - within / 2)
    # plug-in variant of J_rel (sensitivity of the R3 choice)
    def jplug(h):
        m = cnt[:, h, :].astype(float); nn = m.sum(1); rho = nn / tri[:, h]
        return rho ** 2 * 4 * ((np.array([2, 1, 1, 1, 2]) * m ** 2).sum(1) / nn ** 2)
    pA, pB = jplug(0), jplug(1); selp = int(np.argmax(pA)); exBp = pB / pB.mean() - 1; qp = np.percentile(exBp, [0.5, 99.5])
    # swap halves (select on B, score on A)
    selB = int(np.argmax(jB)); exA = jA / jA.mean() - 1; qA = np.percentile(exA, [0.5, 99.5])
    return dict(frames=K, chi2_classes=x_cls, df_classes=df_cls, p_classes=chi2_sf(x_cls, df_cls),
                chi2_oncurve=x_on, df_oncurve=df_on, p_oncurve=chi2_sf(x_on, df_on),
                chi2_pooled=x_pool, p_pooled=chi2_sf(x_pool, 4), pooled_props=(pooled / pooled.sum()).tolist(),
                oncurve_fraction=float(succ.sum() / tri.sum()), trials=int(tri.sum()), labels=int(tot.sum()),
                R3_selected=sel, R3_excess=float(exB[sel]), R3_null=[float(q[0]), float(q[1])], R3_inside=bool(q[0] <= exB[sel] <= q[1]),
                R3_plugin=dict(selected=selp, excess=float(exBp[selp]), null=[float(qp[0]), float(qp[1])]),
                R3_swapped=dict(selected=selB, excess=float(exA[selB]), null=[float(qA[0]), float(qA[1])], inside=bool(qA[0] <= exA[selB] <= qA[1])),
                rel_sd_per_frame=math.sqrt(within) / float(np.mean(jA)), between_var_rel=between / float(np.mean(jA)) ** 2,
                between_var_rel_se_rough=float(math.sqrt(2 / (K - 1)) * np.var((jA + jB) / 2, ddof=1) / float(np.mean(jA)) ** 2))
res['R2_all'] = r2_stats(cnt, tri)
res['R2_drop_frame0'] = r2_stats(cnt[1:], tri[1:])
res['chi2_sf_selftest'] = dict(df4_x9488=chi2_sf(9.488, 4), df63_x82529=chi2_sf(82.529, 63), df252_x290=chi2_sf(290.0, 252))

# ---------- arm I identity vs the isogeny-seed study's rows ----------
ref_dir = '/home/user/crypto/research/curve25519_isogeny_seed_20261103/data'
iso_id = {}
for k in (20, 24):
    ref = {}
    for l in open(os.path.join(ref_dir, f'k{k}.jsonl')):
        d = json.loads(l); ref[(d['t'], d['seed'])] = d
    for t in range(4, 10):
        f = os.path.join(BASE, 'data', 'iso', f'iso_k{k}_t{t}.jsonl')
        rows = [json.loads(l) for l in open(f)]
        cmp = [x for x in rows if (x['t'], x['seed']) in ref]
        keys_mine = set().union(*[set(x) for x in rows]); keys_ref = set().union(*[set(ref[(x['t'], x['seed'])]) for x in cmp]) if cmp else set()
        iso_id[f'k{k}_t{t}'] = dict(rows=len(rows), in_ref=len(cmp), equal=sum(ref[(x['t'], x['seed'])] == x for x in cmp),
                                    same_keys=keys_mine == keys_ref if cmp else None,
                                    J_cov_cols_equal=sum(all(ref[(x['t'], x['seed'])][kk] == x[kk] for kk in ('J', 'covered_points', 'columns', 'base_points')) for x in cmp))
res['iso_identity'] = iso_id

# ---------- derived 255-bit numbers ----------
r = 2 ** 252 + 27742317777372353535851937790883648493
p = 2 ** 255 - 19
log2r = math.log2(r)
rho_log2 = 0.5 * math.log2(math.pi * r / 4)
meta = G['meta']
ts = list(range(4, 15)); mJ = [meta[f'k24f_t{t}']['mean_J'] for t in ts]
b = float(np.mean([math.log2(v) - 2 * t for v, t in zip(mJ, ts)]))
tstar = math.ceil((log2r - b) / 2)
der = dict(log2_r=log2r, log2_rho=rho_log2, b_intercept=b, tstar_unrounded=(log2r - b) / 2, tstar=tstar,
           t_m3=math.ceil((log2r + 3 + math.log2(6)) / 3), t_m4=math.ceil((log2r + 3 + math.log2(24)) / 4),
           t_m3_unrounded=(log2r + 3 + math.log2(6)) / 3, t_m4_unrounded=(log2r + 3 + math.log2(24)) / 4)
LN2 = math.log(2)
ex = {}
for cu, f in G['fitsF_all'].items():
    a3 = f['M3'][0]; a2, al = f['M2']
    boots = G['fitsF_boot_M3_a'][cu]
    bM2 = G['fitsF_boot_M2'][cu]
    e3 = lambda t, a=a3: math.exp(a - t * LN2 / 2)
    ex[cu] = dict(M3_C=math.exp(a3), M3_128=e3(tstar), M3_86=e3(86), M3_65=e3(65),
                  M3_128_hi95=float(np.percentile([math.exp(x - tstar * LN2 / 2) for x in boots], 97.5)),
                  M3_128_lo95=float(np.percentile([math.exp(x - tstar * LN2 / 2) for x in boots], 2.5)),
                  M2_128=math.exp(a2 - al * tstar * LN2),
                  M2_128_hi95=float(np.percentile([math.exp(x[0] - x[1] * tstar * LN2) for x in bM2], 97.5)))
der['excess'] = ex
e_w = max(v['M3_128'] for v in ex.values()); e_hi = max(v['M3_128_hi95'] for v in ex.values())
der['e_max_M3'] = e_w; der['e_hi95_max_M3'] = e_hi
der['mechanism_point_prediction'] = 3.44 * 2 * 2 ** (-tstar / 2)

def E_max_normal(log2K):
    """E[max of K iid N(0,1)] = int x K phi(x) Phi(x)^(K-1) dx, numerically, log-domain (own implementation)."""
    xs = np.linspace(-10, 14, 240001)
    lphi = -0.5 * xs ** 2 - 0.5 * math.log(2 * math.pi)
    # log Phi(x) = log(1 - Q(x)) with Q via erfc
    Q = np.array([0.5 * math.erfc(x / math.sqrt(2)) for x in xs])
    lPhi = np.log1p(-np.minimum(Q, 1 - 1e-300))
    lPhi = np.where(xs < -5, np.log(np.maximum(1 - Q, 1e-300)), lPhi)
    lK = log2K * LN2
    dens = np.exp(lK + lphi + (2 ** log2K - 1) * lPhi)
    return float(np.trapezoid(xs * dens, xs)), float(np.trapezoid(dens, xs))
z2000, n2000 = E_max_normal(math.log2(2000)); z100, _ = E_max_normal(math.log2(100)); z264, n264 = E_max_normal(64)
der['z_2000'] = z2000; der['z_100'] = z100; der['z_2_64'] = z264; der['density_norm_checks'] = [n2000, n264]
der['gaussian_P9_ratio'] = z2000 / z100
der['excess_K_2_64'] = e_w * z264 / z2000
ops_label = 2 * r.bit_length()
der['ops_per_label_ladder'] = ops_label
der['double_and_add'] = dict(doublings=r.bit_length() - 1, additions=bin(r).count('1') - 1, total=r.bit_length() - 1 + bin(r).count('1') - 1)
c_star = 2.0 ** (tstar - 1)
cost = {}
for w in (0, 0.1, 1):
    full = c_star * ops_label + w * 2.0 ** tstar
    onebit = w * (2.0 ** tstar + c_star)
    cost[str(w)] = dict(full_log2=math.log2(full), full_over_rho=2 ** (math.log2(full) - rho_log2),
                        onebit_log2=math.log2(onebit) if onebit else None, onebit_over_rho=(2 ** (math.log2(onebit) - rho_log2)) if onebit else 0.0,
                        full_dadd_over_rho=2 ** (math.log2(c_star * der['double_and_add']['total'] + w * 2.0 ** tstar) - rho_log2))
der['cost_per_frame'] = cost
der['breakeven'] = {
    'full_label_w0': cost['0']['full_over_rho'] * (1 + 1 / e_w),
    'legendre_only_w0.1': cost['0.1']['onebit_over_rho'] * (1 + 1 / e_w),
    'legendre_only_w1': cost['1']['onebit_over_rho'] * (1 + 1 / e_w)}
R2 = res['R2_all']; s1 = R2['rel_sd_per_frame'] * math.sqrt(R2['labels'] / R2['frames'] / 2)
der['s1'] = s1
n_req = (s1 * z2000 / e_w) ** 2
der['samples_log2'] = math.log2(n_req)
der['sampled_cost_over_rho'] = n_req * (ops_label + 2) / 2 ** rho_log2
# isogeny-tower: traces a, |a| <= 2 sqrt(p), with p + 1 - a = 0 mod 2^tstar (exact count) and the rough 4 sqrt(p)/2^t*
s = math.isqrt(4 * p)                       # floor(2 sqrt p)
M = 2 ** tstar; a0 = (p + 1) % M
a_min = -s + ((a0 + s) % M)
count = (s - a_min) // M + 1 if a_min <= s else 0
cands = list(range(a_min, s + 1, M))
assert len(cands) == count and all(abs(a) <= s and (p + 1 - a) % M == 0 for a in cands)
der['iso_trace_count_exact'] = len(cands)
der['iso_trace_count_rough_log2'] = math.log2(4) + 0.5 * math.log2(p) - tstar
res['derived'] = der
json.dump(res, open(OUT, 'w'), indent=1, default=float)
print(json.dumps(res, indent=1, default=float)[:6000])
