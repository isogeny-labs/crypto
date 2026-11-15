#!/usr/bin/env python3
"""Validator recompute (own code; the producer's j2_score.py / j2_band.py are not used or imported).

Reads raw solver records, prices them, recomputes every number J2 claims, and writes evidence/recompute.json.
usage: recompute.py <J2 dir> <N4v2 raw units dir> <N4v2 supp rho dir> <replay dir or ''> <out.json>
Pricing rule (N4v2 PROTOCOL / summ2.rs, read as a specification): M-eq(unit) = sum over the 7 phases and over EVERY attempt
record of the arm of (mul + r_inv * inv + r_lmul * lmul), r_inv / r_lmul = median of the size's `calib` records;
unpriced counters (hash + cmp) priced at w * GAE, GAE = gae_mul + r_inv * gae_inv, w in {0, 0.1, 1}."""
import json, glob, os, sys, statistics as st, math, itertools, collections
J2, N4RAW, N4SUPP, REPLAY, OUT = sys.argv[1:6]
PH = ['fb', 'relations', 'matrix', 'linear_algebra', 'log_recovery', 'descent', 'verify']
W = [0.0, 0.1, 1.0]
def recs(path):
    return [json.loads(l) for l in open(path) if l.startswith('{')]
def med(v): return st.median(v)
# ------------------------------------------------------------------ load J2 records
def load_units(dirs, pattern='ic_*_c*.jsonl'):
    U = {}
    for d in dirs:
        for f in sorted(glob.glob(os.path.join(d, pattern))):
            b = os.path.basename(f)[3:-6]; q, sd, c = b.split('_'); key = (int(q), int(sd), c[1:])
            U[key] = recs(f)
    return U
def load_calib(dirs):
    C = collections.defaultdict(list)
    for d in dirs:
        for f in sorted(glob.glob(os.path.join(d, 'calib_*_*.jsonl'))):
            for r in recs(f):
                if r.get('rec') == 'calib': C[r['q']].append(r)
    return C
def calib_median(C):
    return {q: dict(r_inv=med([r['r_inv'] for r in v]), r_lmul=med([r['r_lmul'] for r in v]), n=len(v)) for q, v in C.items()}
U = load_units([os.path.join(J2, 'units'), os.path.join(J2, 'heldout')])
CAL = calib_median(load_calib([os.path.join(J2, 'units'), os.path.join(J2, 'heldout')]))
def gae_of(recl, ri):
    rat = [r for r in recl if r.get('rec') == 'ratios']
    return med([r['gae_mul'] for r in rat]) + ri * med([r['gae_inv'] for r in rat])
def cost(recl, arm, ri, rl, w, strict=False):
    g = gae_of(recl, ri); tot = 0.0
    for r in recl:
        if r.get('rec') != 'ic' or r['arm'] != arm: continue
        for p in PH:
            tot += r.get(f'{p}.mul', 0) + ri * r.get(f'{p}.inv', 0) + rl * r.get(f'{p}.lmul', 0)
            u = r.get(f'{p}.hash', 0) + r.get(f'{p}.cmp', 0)
            if strict: u += r.get(f'{p}.add', 0) + r.get(f'{p}.ladd', 0)
            tot += w * g * u
    return tot
def first_ok(recl, arm='DL1'):
    a = [r for r in recl if r.get('rec') == 'ic' and r['arm'] == arm]
    return bool(a[0]['ok'] and a[0]['attempt'] == 1)
def final_ok(recl, arm='DL1'):
    a = [r for r in recl if r.get('rec') == 'ic' and r['arm'] == arm]
    return bool(a[-1]['ok'] and a[-1]['k_found'] == a[-1]['k'])
out = dict(calibration_j2=CAL)
QS = [65479, 262111, 1048507, 2097091, 4194271]
GRID_SEEDS = (1, 2, 3); HELD_SEEDS = (4, 5, 6, 7, 8)
def seeds_for(q, kind): return HELD_SEEDS if (kind == 'held' and q in (2097091, 4194271)) else GRID_SEEDS
def has(q, sd, c): return (q, sd, c) in U
# ------------------------------------------------------------------ success and paired ratios
CS = ['1.40', '1.50', '1.60', '1.75', '2.00']
succ = {}; att = {}
for q in QS:
    for kind in ('grid', 'held'):
        if kind == 'held' and q not in (2097091, 4194271): continue
        if kind == 'grid' and q == 2097091: continue
        for c in CS:
            ss = [(sd, has(q, sd, c)) for sd in seeds_for(q, kind)]
            if not all(h for _, h in ss): continue
            succ[(q, kind, c)] = [first_ok(U[(q, sd, c)]) for sd in seeds_for(q, kind)]
            att[(q, kind, c)] = [len([r for r in U[(q, sd, c)] if r.get('rec') == 'ic' and r['arm'] == 'DL1']) for sd in seeds_for(q, kind)]
out['first_attempt_success'] = {f'{q}|{kind}|{c}': dict(succ=sum(v), n=len(v), per_seed=v, attempts=att[(q, kind, c)]) for (q, kind, c), v in succ.items()}
def pair_ratios(q, kind, c, w, strict=False, ri=None, rl=None):
    ri = CAL[q]['r_inv'] if ri is None else ri; rl = CAL[q]['r_lmul'] if rl is None else rl
    r = []
    for sd in seeds_for(q, kind):
        r.append(cost(U[(q, sd, c)], 'DL1', ri, rl, w, strict) / cost(U[(q, sd, '2.00')], 'DL1', ri, rl, w, strict))
    return r
ratios = {}
for (q, kind, c) in succ:
    if c in ('1.40', '1.50', '1.60', '1.75', '2.00'):
        ratios[(q, kind, c)] = {w: pair_ratios(q, kind, c, w) for w in W}
out['paired_ratio_to_c2.00'] = {f'{q}|{kind}|{c}|w={w}': dict(per_seed=v, lo=min(v), hi=max(v), mean=sum(v) / len(v)) for (q, kind, c), d in ratios.items() for w, v in d.items()}
# ------------------------------------------------------------------ band ordering per (q, seed, w), with sensitivities
def band(strict, ri=None, rl=None):
    res = {}
    for q in QS:
        for kind in ('grid', 'held'):
            if kind == 'held' and q not in (2097091, 4194271): continue
            if kind == 'grid' and q == 2097091: continue
            for w in W:
                ok = []
                for sd in seeds_for(q, kind):
                    ri_ = CAL[q]['r_inv'] if ri is None else ri; rl_ = CAL[q]['r_lmul'] if rl is None else rl
                    c = {cc: cost(U[(q, sd, cc)], 'DL1', ri_, rl_, w, strict) for cc in ('1.60', '1.75', '2.00')}
                    ok.append(c['1.60'] < c['1.75'] < c['2.00'])
                res[f'{q}|{kind}|w={w}'] = dict(order_holds_seeds=sum(ok), n=len(ok), all=all(ok))
    return res
out['band_frozen_rule'] = band(False)
out['band_strict_add_ladd'] = band(True)
sens = {}
for name, (ri, rl) in {'xeon_N4_const(15,1.1)': (15.0, 1.1), 'high(30,2.5)': (30.0, 2.5), 'low(8,1.0)': (8.0, 1.0)}.items():
    sens[name] = dict(band=band(False, ri, rl))
    sens[name]['ratio_c1.60_range_by_q'] = {}
    for (q, kind) in sorted({(q, k) for (q, k, c) in succ}):
        if (q, kind, '1.60') in succ and (q, kind, '2.00') in succ:
            v = pair_ratios(q, kind, '1.60', 0.0, False, ri, rl); sens[name]['ratio_c1.60_range_by_q'][f'{q}|{kind}'] = [min(v), max(v)]
out['calibration_sensitivity'] = sens
# strict ratios at w = 0,0.1,1
out['paired_ratio_strict'] = {f'{q}|{kind}|{c}|w={w}': [min(v), max(v)] for (q, kind, c) in succ if c in ('1.60', '1.75') for w in W for v in [pair_ratios(q, kind, c, w, True)]}
# ------------------------------------------------------------------ per-unit arithmetic cross-check on every unit: recovered log, attempts, bad counters
chk = dict(units=0, final_ok_all=True, relations_bad=0, holdout_bad=0, not_final_ok=[])
for key, rl_ in U.items():
    for arm in ('D', 'DL1', 'DL'):
        chk['units'] += 1
        if not final_ok(rl_, arm): chk['final_ok_all'] = False; chk['not_final_ok'].append(list(key) + [arm])
    for r in rl_:
        if r.get('rec') == 'ic' and r['ok']:
            chk['relations_bad'] += r.get('s.relations_bad', 0); chk['holdout_bad'] += r.get('s.holdout_bad', 0)
out['unit_checks'] = chk
# ------------------------------------------------------------------ seeds agreement ("within 1% at k >= 20")
agree = {}
for (q, kind) in sorted({(q, k) for (q, k, c) in succ}):
    if q < 1048507: continue
    for c in ('1.60', '2.00'):
        if (q, kind, c) not in succ: continue
        v = [cost(U[(q, sd, c)], 'DL1', CAL[q]['r_inv'], CAL[q]['r_lmul'], 0.0) for sd in seeds_for(q, kind)]
        agree[f'{q}|{kind}|c={c}|cost'] = dict(max_over_min_minus_1=max(v) / min(v) - 1, cv=st.pstdev(v) / st.mean(v))
    v = ratios[(q, kind, '1.60')][0.0]; agree[f'{q}|{kind}|ratio1.60'] = dict(max_over_min_minus_1=max(v) / min(v) - 1, cv=st.pstdev(v) / st.mean(v))
out['seed_agreement'] = agree
# ------------------------------------------------------------------ exact one-sided binomial bound
def cp_lower(n, alpha=0.05): return alpha ** (1.0 / n)   # x = n successes of n
n160 = sum(len(v) for (q, kind, c), v in succ.items() if c == '1.60'); s160 = sum(sum(v) for (q, kind, c), v in succ.items() if c == '1.60')
n175 = sum(len(v) for (q, kind, c), v in succ.items() if c == '1.75'); s175 = sum(sum(v) for (q, kind, c), v in succ.items() if c == '1.75')
nh160 = sum(len(v) for (q, kind, c), v in succ.items() if c == '1.60' and kind == 'held'); sh160 = sum(sum(v) for (q, kind, c), v in succ.items() if c == '1.60' and kind == 'held')
out['binomial'] = dict(c160_all=dict(n=n160, successes=s160, cp95_lower=cp_lower(n160) if s160 == n160 else None),
                       c160_heldout=dict(n=nh160, successes=sh160, cp95_lower=cp_lower(nh160) if sh160 == nh160 else None),
                       c175_all=dict(n=n175, successes=s175, cp95_lower=cp_lower(n175) if s175 == n175 else None))
# ------------------------------------------------------------------ c_50 model recomputed (lane J desk formula)
def model_c50(q):
    c = 1.6
    for _ in range(200):
        e = (1.58 * c) ** (1 / 3) * q ** (-1 / 6); c = math.sqrt(2 * (1 + e))
    return c
out['c50_model'] = {str(q): model_c50(q) for q in QS}
# ------------------------------------------------------------------ N4v2 cross-check of the pricing code (committed summary_supp.json)
n4 = {}
for f in glob.glob(os.path.join(N4RAW, 'ic_*_SD.jsonl')):
    b = os.path.basename(f)[3:-9]; q, sd = b.split('_'); n4[(int(q), int(sd))] = recs(f)
n4cal = collections.defaultdict(list)
for f in glob.glob(os.path.join(N4RAW, 'calib_*_*.jsonl')):
    for r in recs(f):
        if r.get('rec') == 'calib': n4cal[r['q']].append(r)
n4cal = calib_median(n4cal)
summ = {}
for l in open(os.path.join(os.path.dirname(os.path.abspath(J2)), '..', 'quartic_n4v2_20261020', 'results', 'summary_supp.json')):
    if l.strip():
        r = json.loads(l); summ[(r['kind'], r.get('arm'), r.get('q'))] = r
xc = {}
for q in (65479, 262111, 1048507, 2097091, 4194271):
    for arm in ('D', 'DL1', 'DL'):
        cs = [n4[(q, sd)] for sd in range(1, 6) if (q, sd) in n4]
        ri, rl = n4cal[q]['r_inv'], n4cal[q]['r_lmul']
        mine = [sum(cost(u, arm, ri, rl, w) for u in cs) / len(cs) for w in W]   # all units here are ok units
        ref = summ[('ic_size', arm, q)]
        xc[f'{arm}|{q}'] = dict(mine=mine, committed=ref['meq'], units=len(cs), committed_units=ref['units'],
                                rel_diff=[abs(a - b) / b for a, b in zip(mine, ref['meq'])], r_inv=ri, r_lmul=rl, committed_r_inv=ref['r_inv'], committed_r_lmul=ref['r_lmul'])
out['n4v2_pricing_crosscheck'] = xc
# ------------------------------------------------------------------ rho reference (N4v2 raw records) and IC vs rho band
def load_rho():
    R = collections.defaultdict(list)
    for d in (N4RAW, N4SUPP):
        for f in sorted(glob.glob(os.path.join(d, 'rho_*_*_*_*.jsonl'))):
            for r in recs(f):
                if r.get('rec') == 'rho': R[(r['arm'], r['q'])].append(dict(r, _file=os.path.basename(f), _supp=(d == N4SUPP)))
    return R
RHO = load_rho()
def rho_cost(r, ri, rl, g, w, strict=False):
    unp = r['c.hash'] + r['c.cmp'] + ((r['c.add'] + r['c.ladd']) if strict else 0)
    return r['c.mul'] + ri * r['c.inv'] + rl * r['c.lmul'] + w * g * unp
rho_out = {}
for q in (2097091, 4194271):
    for arm in ('plain', 'neg'):
        rs = RHO[(arm, q)]
        if q == 4194271: pass
        sub = {'all': rs, 'frozen_only': [r for r in rs if not r['_supp']]}
        for tag, rr in sub.items():
            if not rr: continue
            for label, (ri, rl) in {'n4v2_calib': (n4cal[q]['r_inv'], n4cal[q]['r_lmul']), 'j2_calib': (CAL[q]['r_inv'], CAL[q]['r_lmul'])}.items():
                g = 56.0 + ri * 1.0
                rho_out[f'{arm}|{q}|{tag}|{label}'] = dict(runs=len(rr), runs_ok=sum(1 for r in rr if r['ok']), mean_meq=[sum(rho_cost(r, ri, rl, g, w) for r in rr) / len(rr) for w in W],
                    mean_meq_strict=[sum(rho_cost(r, ri, rl, g, w, True) for r in rr) / len(rr) for w in W],
                    steps_over_sqrt_ell=sum(r['steps'] / math.sqrt(r['ell']) for r in rr) / len(rr), gae=g, r_inv=ri, r_lmul=rl,
                    unpriced_per_run_mean=dict(hash=sum(r['c.hash'] for r in rr) / len(rr), cmp=sum(r['c.cmp'] for r in rr) / len(rr), add=sum(r['c.add'] for r in rr) / len(rr), ladd=sum(r['c.ladd'] for r in rr) / len(rr)),
                    files=sorted(r['_file'] for r in rr))
out['rho'] = rho_out
# committed rho rows for reproduction check
out['rho_committed_supp'] = {f'{arm}|{q}': summ[('rho_size', arm, q)] for q in (2097091, 4194271) for arm in ('plain', 'neg')}
# IC vs rho, both priced with the same calibration (two readings)
def ic_mean(q, c, kind, label, strict=False):
    ri, rl = (CAL[q]['r_inv'], CAL[q]['r_lmul']) if label == 'j2_calib' else (n4cal[q]['r_inv'], n4cal[q]['r_lmul'])
    return [sum(cost(U[(q, sd, c)], 'DL1', ri, rl, w, strict) for sd in seeds_for(q, kind)) / len(seeds_for(q, kind)) for w in W]
tbl = {}
for q, kind in ((2097091, 'held'), (4194271, 'held')):
    for label in ('j2_calib', 'n4v2_calib'):
        for strict in (False, True):
            for c in ('1.60', '1.75', '2.00'):
                ic = ic_mean(q, c, kind, label, strict)
                for arm in ('plain', 'neg'):
                    rr = rho_out[f'{arm}|{q}|all|{label}']['mean_meq_strict' if strict else 'mean_meq']
                    tbl[f'{q}|{label}|{"strict" if strict else "frozen_rule"}|c={c}|{arm}'] = dict(ic=ic, rho=rr, ratio=[a / b for a, b in zip(ic, rr)])
out['ic_vs_rho'] = tbl
# ------------------------------------------------------------------ price the counters the frozen unit leaves out: add and ladd (sensitivity, labelled)
def cost_extra(recl, arm, ri, rl, wx):
    """M-eq (w = 0) plus wx M-eq per counted F_q addition and per counted mod-ell addition (add, ladd)."""
    base = cost(recl, arm, ri, rl, 0.0); ex = 0.0
    for r in recl:
        if r.get('rec') != 'ic' or r['arm'] != arm: continue
        for p in PH: ex += r.get(f'{p}.add', 0) + r.get(f'{p}.ladd', 0)
    return base + wx * ex
sweep = {}
for (q, kind) in ((262111, 'grid'), (1048507, 'grid'), (4194271, 'grid'), (2097091, 'held'), (4194271, 'held')):
    ri, rl = CAL[q]['r_inv'], CAL[q]['r_lmul']
    row = {}
    for wx in (0.0, 0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0):
        for c in ('1.60', '1.75'):
            v = [cost_extra(U[(q, sd, c)], 'DL1', ri, rl, wx) / cost_extra(U[(q, sd, '2.00')], 'DL1', ri, rl, wx) for sd in seeds_for(q, kind)]
            row[f'wx={wx}|c={c}'] = [min(v), max(v), sum(v) / len(v)]
    def mean_ratio(wx, c='1.60'):
        v = [cost_extra(U[(q, sd, c)], 'DL1', ri, rl, wx) / cost_extra(U[(q, sd, '2.00')], 'DL1', ri, rl, wx) for sd in seeds_for(q, kind)]; return sum(v) / len(v)
    lo, hi = 0.0, 50.0
    for _ in range(80):
        mid = (lo + hi) / 2
        if mean_ratio(mid) < 1: lo = mid
        else: hi = mid
    row['breakeven_wx_c1.60_M-eq_per_add_or_ladd'] = lo
    sweep[f'{q}|{kind}'] = row
out['add_ladd_weight_sweep'] = sweep
# ------------------------------------------------------------------ measured CPU and wall of the same DL1 solves (L0 host: practicality evidence, not a speed claim)
def total_time(recl, key):
    return sum(r[f'{p}.{key}'] for r in recl if r.get('rec') == 'ic' and r['arm'] == 'DL1' for p in PH if f'{p}.{key}' in r)
def la_time(recl, key='cpu'):
    return sum(r[f'linear_algebra.{key}'] for r in recl if r.get('rec') == 'ic' and r['arm'] == 'DL1' and f'linear_algebra.{key}' in r)
def rel_time(recl, key='cpu'):
    return sum(r[f'relations.{key}'] for r in recl if r.get('rec') == 'ic' and r['arm'] == 'DL1' and f'relations.{key}' in r)
tm = {}
def time_table(UU, tag):
    for (q, kind) in ((262111, 'grid'), (1048507, 'grid'), (4194271, 'grid'), (2097091, 'held'), (4194271, 'held')):
        for c in ('1.60', '1.75'):
            if not all((q, sd, c) in UU and (q, sd, '2.00') in UU for sd in seeds_for(q, kind)): continue
            tm[f'{tag}|{q}|{kind}|c={c}|cpu_total'] = [total_time(UU[(q, sd, c)], 'cpu') / total_time(UU[(q, sd, '2.00')], 'cpu') for sd in seeds_for(q, kind)]
            tm[f'{tag}|{q}|{kind}|c={c}|wall_total'] = [total_time(UU[(q, sd, c)], 'wall') / total_time(UU[(q, sd, '2.00')], 'wall') for sd in seeds_for(q, kind)]
            if c == '1.60':
                tm[f'{tag}|{q}|{kind}|c=1.60|cpu_LA_phase'] = [la_time(UU[(q, sd, c)]) / la_time(UU[(q, sd, '2.00')]) for sd in seeds_for(q, kind)]
                tm[f'{tag}|{q}|{kind}|c=1.60|cpu_relations_phase'] = [rel_time(UU[(q, sd, c)]) / rel_time(UU[(q, sd, '2.00')]) for sd in seeds_for(q, kind)]
time_table(U, 'producer_records')
if REPLAY:
    UR = load_units([REPLAY]); time_table(UR, 'validator_replay')
out['cpu_wall_ratios_to_c2.00'] = tm
# in-situ cost per counted operation: least squares on the LA and relations phases of the producer's DL1 records
try:
    import numpy as np
    rowsl = [r for rl_ in U.values() for r in rl_ if r.get('rec') == 'ic' and r['arm'] == 'DL1' and 'linear_algebra.lmul' in r]
    X = np.array([[r['linear_algebra.lmul'], r['linear_algebra.ladd']] for r in rowsl], float); y = np.array([r['linear_algebra.cpu'] for r in rowsl]); big = y > 0.5
    coef = np.linalg.lstsq(X[big], y[big], rcond=None)[0]; pred = X[big] @ coef
    r2 = 1 - ((y[big] - pred) ** 2).sum() / ((y[big] - y[big].mean()) ** 2).sum()
    mulns = med([r['mul_ns_median'] for v in load_calib([os.path.join(J2, 'units'), os.path.join(J2, 'heldout')]).values() for r in v])
    out['in_situ_LA_regression'] = dict(n=int(big.sum()), ns_per_lmul=float(coef[0] * 1e9), ns_per_ladd=float(coef[1] * 1e9), r2=float(r2), median_calib_mul_ns=mulns,
                                        ladd_in_M_eq=float(coef[1] * 1e9 / mulns), lmul_in_M_eq=float(coef[0] * 1e9 / mulns))
except Exception as e:
    out['in_situ_LA_regression'] = dict(error=str(e))
json.dump(out, open(OUT, 'w'), indent=1, sort_keys=True, default=lambda o: list(o) if isinstance(o, tuple) else str(o))
print('wrote', OUT)
