#!/usr/bin/env python3
"""Validator: recompute every L1 ratio, bootstrap interval and prediction verdict from the raw
per-base counts.  Independent of analyse_l1.py (not imported).  usage: vdata.py L1_DIR OUT_JSON

Raw row layout (run_l1.py docstring; checked against the data below):
  0 N_actual, 1 exact covered points, 2 exact with T0 (rand only), 3 targets m, 4 hits,
  5 hits with T0 (rand only), 6 verified ok, 7 verified fail
"""
import hashlib, json, math, os, sys
import numpy as np

L1 = sys.argv[1]; OUT = sys.argv[2]
NB = 4000
ARMS = ['bits_rand', 'rand', 'rand2', 'hash', 'hash2', 'rand_proj', 'hash_proj', 'plant3', 'plant5']
COMPS = [('hash/rand', 'hash', 'rand'), ('hash/bits_rand', 'hash', 'bits_rand'),
         ('bits_rand/rand', 'bits_rand', 'rand'), ('hash_proj/rand_proj', 'hash_proj', 'rand_proj'),
         ('rand_proj/rand', 'rand_proj', 'rand'), ('rand/rand2 (A/A)', 'rand', 'rand2'),
         ('hash/hash2 (A/A)', 'hash', 'hash2'), ('plant3/rand', 'plant3', 'rand'),
         ('plant5/rand', 'plant5', 'rand'), ('T0/blind', 'T0', 'rand')]
CELLS = [(16, 18), (16, 36), (18, 36), (18, 73), (20, 72), (20, 145), (22, 145), (22, 290), (24, 290), (24, 579)]
rep = dict(integrity={}, cells=[], pooled={}, predictions={}, z=[], notes=[])


def sha(path):
    return hashlib.sha256(open(path, 'rb').read()).hexdigest()


# ---------- integrity: npz vs SHA256SUMS, JSON vs SHA256SUMS, JSON == npz exactly ----------
sums = dict(l.split()[::-1] for l in open(f'{L1}/out/SHA256SUMS').read().split('\n') if l.strip())
jsums = dict(l.split()[::-1] for l in open(f'{L1}/out_json/SHA256SUMS').read().split('\n') if l.strip())
data = {}
for k, N in CELLS:
    name = f'cell_k{k}_N{N}'
    npz = f'{L1}/out/{name}.npz'; js = f'{L1}/out_json/{name}.json'
    ok_npz = sha(npz) == sums[f'{name}.npz']
    ok_js = sha(js) == jsums[f'{name}.json']
    d = np.load(npz); J = json.load(open(js))
    assert J['source'] == f'{name}.npz'
    arrs = J['arrays']
    same = set(arrs) == set(d.files)
    for a in d.files:
        x = d[a]; v = arrs[a]
        if x.dtype.kind == 'U':
            same &= str(x) == v['data']
        else:
            y = np.array(v['data'], dtype=v['dtype']).reshape(v['shape'])
            same &= (x.dtype == y.dtype) and (x.shape == y.shape) and bool(np.array_equal(x, y))
    meta = json.loads(str(d['meta']))
    X = {a: d[a].astype(np.int64) for a in d.files if a != 'meta'}
    if os.environ.get('HOLDOUT') == '1' and k == 16:
        X = {a: (v[300:] if a != 'bits_lo0' else v) for a, v in X.items()}
    rep['integrity'][name] = dict(npz_sha_matches=ok_npz, json_sha_matches=ok_js, json_equals_npz=bool(same),
                                  arrays=sorted(d.files), meta=meta)
    data[(k, N)] = (meta, X)

# ---------- per-cell structural checks ----------
facts = {f['k']: f for f in json.load(open(os.environ.get('VFACTS', 'vfacts.json')))}
struct = []
for (k, N), (meta, X) in data.items():
    f = facts[k]; r = f['r']
    s = dict(k=k, N=N)
    s['meta_matches_facts'] = (meta['p'] == f['p'] and meta['A'] == f['A'] and meta['r'] == r
                               and meta['order'] == f['order_euler'] and meta['N'] == N and meta['B'] == 512)
    s['m_u_rule'] = meta['m_u'] == math.ceil(200 * 4 * r / N ** 2)
    s['m_p_rule'] = meta['m_p'] == math.ceil(200 * (r - 1) / (2 * N ** 2))
    s['B_rows'] = {a: int(X[a].shape[0]) for a in ARMS + ['bits_lo0']}
    s['all_512'] = all(X[a].shape == (512, 8) for a in ARMS) and X['bits_lo0'].shape == (1, 8)
    s['N_actual_ok'] = all(bool((X[a][:, 0] == N).all()) for a in ['bits_rand', 'rand', 'rand2', 'hash', 'hash2', 'rand_proj', 'hash_proj', 'bits_lo0'])
    fN = {'plant3': 1.0149 * N, 'plant5': 1.0247 * N}
    s['plant_N_in_floor_ceil'] = all(bool(np.isin(X[a][:, 0], [math.floor(fN[a]), math.floor(fN[a]) + 1]).all()) for a in fN)
    s['targets_ok'] = (all(bool((X[a][:, 3] == meta['m_u']).all()) for a in ['bits_rand', 'rand', 'rand2', 'hash', 'hash2', 'plant3', 'plant5', 'bits_lo0'])
                       and all(bool((X[a][:, 3] == meta['m_p']).all()) for a in ['rand_proj', 'hash_proj']))
    s['t0_cols_only_rand'] = all(bool((X[a][:, 2] == -1).all() and (X[a][:, 5] == -1).all()) for a in ARMS + ['bits_lo0'] if a != 'rand')
    s['t0_superset'] = bool((X['rand'][:, 2] >= X['rand'][:, 1]).all() and (X['rand'][:, 5] >= X['rand'][:, 4]).all())
    # every counted hit verified: ok == hits (+ T0-only hits for rand), fail == 0
    vf = 0; gap = 0
    for a in ARMS + ['bits_lo0']:
        x = X[a]
        expect = x[:, 4] + ((x[:, 5] - x[:, 4]) if a == 'rand' else 0)
        gap += int(np.abs(x[:, 6] - expect).sum()); vf += int(x[:, 7].sum())
    s['verification_failures'] = vf; s['verified_minus_counted_abs'] = gap
    # exact coverage is a count of points of E[r]\{O}, symmetric under negation -> even
    s['exact_even'] = all(bool((X[a][:, 1] % 2 == 0).all()) for a in ARMS + ['bits_lo0']) and bool((X['rand'][:, 2] % 2 == 0).all())
    # deterministic ceiling: covered points <= 2 N_actual^2 for every base of every arm
    s['ceiling_violations_all_arms'] = int(sum(int((X[a][:, 1] > 2 * X[a][:, 0] ** 2).sum()) for a in ARMS + ['bits_lo0']))
    s['instrument_check_failures'] = meta['instrument_check_failures']
    s['hits_total'] = {a: int(X[a][:, 4].sum()) for a in ARMS}
    struct.append(s)
rep['structure'] = struct


# ---------- estimator ----------
def rates(x, col):
    return x[:, col].sum() / x[:, 3].sum()


def get(X, arm):
    if arm == 'T0':
        return X['rand'], 5, 2
    return X[arm], 4, 1


rng = np.random.default_rng(int.from_bytes(hashlib.sha256(b'validator-L1-bootstrap-2026-11-22').digest()[:8], 'big'))
cell_lr = {c[0]: [] for c in COMPS}; cell_lrx = {c[0]: [] for c in COMPS}
cell_pt = {c[0]: [] for c in COMPS}; cell_ptx = {c[0]: [] for c in COMPS}
cells_out = []
for (k, N), (meta, X) in data.items():
    r = meta['r']
    co = dict(k=k, N=N, ratios={}, arms={})
    for a in ARMS + ['bits_lo0']:
        x = X[a]
        co['arms'][a] = dict(mc_rate=float(rates(x, 4)), exact_rate=float(x[:, 1].mean() / (r - 1)),
                             hits=int(x[:, 4].sum()), targets=int(x[:, 3].sum()),
                             cv_exact=float(x[:, 1].std() / x[:, 1].mean()))
    for name, na, da in COMPS:
        xn, hn, en = get(X, na); xd, hd, ed = get(X, da)
        pt = math.log(rates(xn, hn) / rates(xd, hd))
        ptx = math.log(xn[:, en].mean() / xd[:, ed].mean())
        B = len(xn)
        i1 = rng.integers(0, B, (NB, B))
        i2 = i1 if na == 'T0' else rng.integers(0, len(xd), (NB, len(xd)))
        rn = xn[i1, hn].sum(1) / xn[i1, 3].sum(1); rd = xd[i2, hd].sum(1) / xd[i2, 3].sum(1)
        lr = np.log(rn / rd)
        ex_n = xn[i1, en].mean(1); ex_d = xd[i2, ed].mean(1)
        lrx = np.log(ex_n / ex_d)
        lo, hi = np.percentile(lr, [2.5, 97.5]); lox, hix = np.percentile(lrx, [2.5, 97.5])
        # delta-method cross-check of the MC CI (ratio estimator, bases as units)
        def var_log_rate(x, hcol):
            h = x[:, hcol].astype(float); m = x[:, 3].astype(float); R = h.sum() / m.sum()
            res = h - R * m; return (res ** 2).sum() / (m.sum() ** 2) / R ** 2 * len(x) / (len(x) - 1)
        if na == 'T0':
            # paired: log(sum h5 / sum h4)
            h5 = xn[:, 5].astype(float); h4 = xn[:, 4].astype(float); Rr = h5.sum() / h4.sum()
            se = math.sqrt(((h5 - Rr * h4) ** 2).sum() / h4.sum() ** 2 / Rr ** 2 * B / (B - 1))
        else:
            se = math.sqrt(var_log_rate(xn, hn) + var_log_rate(xd, hd))
        co['ratios'][name] = dict(mc=math.exp(pt), mc_ci=[math.exp(lo), math.exp(hi)],
                                  half_width=(math.exp(hi) - math.exp(lo)) / 2,
                                  exact=math.exp(ptx), exact_ci=[math.exp(lox), math.exp(hix)],
                                  delta_ci=[math.exp(pt - 1.96 * se), math.exp(pt + 1.96 * se)])
        cell_lr[name].append(lr); cell_lrx[name].append(lrx); cell_pt[name].append(pt); cell_ptx[name].append(ptx)
    # z: MC hits vs exact expectation (binomial per base)
    for a in ARMS + ['T0']:
        x, hc, ec = get(X, a)
        y = x[:, ec] / (r - 1)
        z = (x[:, hc].sum() - (x[:, 3] * y).sum()) / math.sqrt((x[:, 3] * y * (1 - y)).sum())
        rep['z'].append([k, N, a, float(z)])
    # P2b percentiles (midrank, single draw within its arm's distribution)
    e = X['bits_rand'][:, 1]; v = X['bits_lo0'][0, 1]
    co['pct_bits_lo0'] = float((e < v).mean() * 100 + (e == v).mean() * 50)
    e = X['hash'][:, 1]; v = X['hash'][0, 1]
    co['pct_hash_lit'] = float((e < v).mean() * 100 + (e == v).mean() * 50)
    eo = X['hash'][1:, 1]
    co['pct_hash_lit_excluding_self'] = float((eo < v).mean() * 100 + (eo == v).mean() * 50)
    # projected vs ceiling; exact rand_proj/rand
    ceil_pts = 2 * N * N
    co['ceiling_rate'] = ceil_pts / (r - 1)
    co['proj_over_ceiling'] = {a: float(X[a][:, 1].mean() / ceil_pts) for a in ('rand_proj', 'hash_proj')}
    co['unproj_over_ceiling'] = {a: float(X[a][:, 1].mean() / ceil_pts) for a in ('rand', 'bits_rand', 'hash', 'hash2', 'rand2')}
    co['max_base_over_ceiling'] = {a: float((X[a][:, 1] / (2 * X[a][:, 0] ** 2)).max()) for a in ARMS + ['bits_lo0']}
    co['design'] = {a: float((X[a][:, 0].astype(float) ** 2).mean() / N ** 2) for a in ('plant3', 'plant5')}
    co['design_NN1'] = {a: float((X[a][:, 0] * (X[a][:, 0] + 1)).mean() / (N * (N + 1))) for a in ('plant3', 'plant5')}
    cells_out.append(co)
rep['cells'] = cells_out

for name, _, _ in COMPS:
    lr = np.mean(cell_lr[name], axis=0); lrx = np.mean(cell_lrx[name], axis=0)
    lo, hi = np.percentile(lr, [2.5, 97.5]); lox, hix = np.percentile(lrx, [2.5, 97.5])
    rep['pooled'][name] = dict(mc=math.exp(np.mean(cell_pt[name])), mc_ci=[math.exp(lo), math.exp(hi)],
                               exact=math.exp(np.mean(cell_ptx[name])), exact_ci=[math.exp(lox), math.exp(hix)])

# ---------- predictions (thresholds from PROTOCOL.md section 6) ----------
P = rep['pooled']; C = rep['cells']; pr = rep['predictions']


def pn(nm, lo=0.98, hi=1.02, clo=0.97, chi=1.03):
    q = P[nm]; return lo <= q['mc'] <= hi and q['mc_ci'][0] >= clo and q['mc_ci'][1] <= chi


def c1(nm):
    return sum(c['ratios'][nm]['mc_ci'][0] <= 1 <= c['ratios'][nm]['mc_ci'][1] for c in C)


def x1(nm):
    return 10 - c1(nm)


hw = [c['ratios']['hash/rand']['half_width'] for c in C]
pr['P1'] = dict(pooled=P['hash/rand'], cells_ci_contain_1=c1('hash/rand'),
                holds=pn('hash/rand') and c1('hash/rand') >= 8)
pr['P1p'] = dict(half_widths=hw, n_le_003=sum(h <= 0.03 for h in hw), holds=sum(h <= 0.03 for h in hw) >= 8)
pr['P2'] = dict(hash_bits=P['hash/bits_rand'], bits_rand=P['bits_rand/rand'],
                holds=pn('hash/bits_rand') and pn('bits_rand/rand'))
pb = [c['pct_bits_lo0'] for c in C]; ph = [c['pct_hash_lit'] for c in C]
pr['P2b'] = dict(bits_lo0_pct=pb, hash_lit_pct=ph, holds=sum(1 <= v <= 99 for v in pb) >= 8 and sum(1 <= v <= 99 for v in ph) >= 8)
pc = [v for c in C for v in c['proj_over_ceiling'].values()]
rpr = [c['ratios']['rand_proj/rand']['exact'] for c in C]; rpr_mc = [c['ratios']['rand_proj/rand']['mc'] for c in C]
cv = sum(s['ceiling_violations_all_arms'] for s in struct)
pr['P3'] = dict(ceiling_violations_all_arms=cv, proj_over_ceiling_range=[min(pc), max(pc)],
                rand_proj_over_rand_exact_range=[min(rpr), max(rpr)], rand_proj_over_rand_mc_range=[min(rpr_mc), max(rpr_mc)],
                hash_proj_pooled=P['hash_proj/rand_proj'],
                holds=cv == 0 and all(0.78 <= v <= 1.0 for v in pc) and all(6.0 <= v <= 8.4 for v in rpr)
                and all(6.0 <= v <= 8.4 for v in rpr_mc) and pn('hash_proj/rand_proj'))
t0x = [c['ratios']['T0/blind']['exact'] for c in C]
pr['P4'] = dict(exact=t0x, cells_excl_1=x1('T0/blind'), holds=all(1.8 <= v <= 2.2 for v in t0x) and x1('T0/blind') == 10)
d3 = float(np.mean([c['design']['plant3'] for c in C])); d5 = float(np.mean([c['design']['plant5'] for c in C]))
pr['P5'] = dict(design3=d3, design5=d5, pooled3=P['plant3/rand'], pooled5=P['plant5/rand'],
                cells_excl_1_plant3=x1('plant3/rand'), cells_excl_1_plant5=x1('plant5/rand'),
                abs_dev3=abs(P['plant3/rand']['mc'] - d3), abs_dev5=abs(P['plant5/rand']['mc'] - d5),
                holds=x1('plant5/rand') >= 9 and abs(P['plant5/rand']['mc'] - d5) <= 0.015 and P['plant3/rand']['mc_ci'][0] > 1
                and abs(P['plant3/rand']['mc'] - d3) <= 0.01 and x1('plant3/rand') >= 6)
pr['P6'] = dict(rand=P['rand/rand2 (A/A)'], hash=P['hash/hash2 (A/A)'], c1_rand=c1('rand/rand2 (A/A)'), c1_hash=c1('hash/hash2 (A/A)'),
                holds=pn('rand/rand2 (A/A)', .985, 1.015) and pn('hash/hash2 (A/A)', .985, 1.015)
                and c1('rand/rand2 (A/A)') >= 8 and c1('hash/hash2 (A/A)') >= 8)
worst = max(rep['z'], key=lambda t: abs(t[3]))
vf = sum(s['verification_failures'] for s in struct); gap = sum(s['verified_minus_counted_abs'] for s in struct)
chk = sum(s['instrument_check_failures'] for s in struct)
pr['P7'] = dict(verification_failures=vf, verified_minus_counted=gap, instrument_check_failures=chk, worst_z=worst,
                holds=vf == 0 and gap == 0 and chk == 0 and abs(worst[3]) <= 4)
cost = []
for (k, N), (meta, X) in data.items():
    p = meta['p']; G = N * N
    row = dict(k=k, N=N, G=G, p=p, per_col=p / N, valid_hashed=meta['hash_evals_build'])
    for w in (0.0, 0.1, 1.0):
        row[f'ratio_w{w}'] = (G + w * p) / G
        # band also pricing the shared native work (about 2N Legendre/sqrt per base) on both arms
        row[f'ratio_w{w}_with_shared_native'] = (G + w * p + w * 2 * N) / (G + w * 2 * N)
        # cost per relation (per target probed: same Q both arms) using measured rates
        row[f'cost_per_rel_ratio_w{w}'] = ((G + w * p) / rates(X['hash'], 4)) / (G / rates(X['rand'], 4))
    cost.append(row)
rep['cost'] = cost
pr['P8'] = dict(holds=all(r_['ratio_w0.0'] >= 1 and r_['ratio_w0.1'] > 1 and r_['ratio_w1.0'] > 1 for r_ in cost),
                note='derived inequality G + w p >= G; equal at w = 0, strict above')
json.dump(rep, open(OUT, 'w'), indent=1, default=float)
print('integrity', all(v['npz_sha_matches'] and v['json_sha_matches'] and v['json_equals_npz'] for v in rep['integrity'].values()))
for s in struct:
    print({kk: vv for kk, vv in s.items() if kk not in ('B_rows', 'hits_total')})
for k_, v in pr.items():
    print(k_, 'HOLDS' if v['holds'] else 'FAILS')
