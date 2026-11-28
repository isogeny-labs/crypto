#!/usr/bin/env python3
"""Validator's independent recomputation of every L3 verdict from the raw census arrays.
Reads the npz files and the JSON mirrors directly (no study code). Own bootstrap streams."""
import glob, hashlib, json, math, os, sys
import numpy as np

ROOT = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L3_translated_involution'
FS = '/home/user/crypto/research/curve25519_frame_search_20261103/analysis/summary.json'
KS, TS = (16, 18, 20), (4, 5, 6, 7, 8)
TOL = {4: 0.07, 5: 0.05, 6: 0.04, 7: 0.04, 8: 0.04}        # PROTOCOL.md "Tolerances"
NOHALF = ['E.iota_odd', 'Ep.iota_c01', 'Ep.iota_c10', 'Ep.iota_c11', 'Ep.huff_x', 'Ep.huff_y']
CONTROLS = ['E.iota_even', 'E.iota_T0', 'Ep.iota_2E']
JI = ['Ep.ji_s', 'Ep.ji_c', 'Ep.ji_d']
ARMS = ['E.x', 'E.x_noself', 'E.iota_even', 'E.iota_T0', 'E.iota_odd', 'Ep.x', 'Ep.iota_2E', 'Ep.iota_c01',
        'Ep.iota_c10', 'Ep.iota_c11', 'Ep.huff_x', 'Ep.huff_y', 'Ep.ji_s', 'Ep.ji_c', 'Ep.ji_d']
BOOT = int(sys.argv[2]) if len(sys.argv) > 2 else 2000

# ---------------------------------------------------------------- load and cross-check npz vs JSON
D, mism, narr = {}, 0, 0
for k in KS:
    for arm in ARMS:
        z = np.load(f'{ROOT}/data/k{k}/{arm}.npz')
        js = json.load(open(f'{ROOT}/data_json/k{k}/{arm}.json'))
        src = hashlib.sha256(open(f'{ROOT}/data/k{k}/{arm}.npz', 'rb').read()).hexdigest()
        assert js['source_sha256'] == src, (k, arm)
        assert set(js['arrays']) == set(z.files), (k, arm)
        d = {}
        for name in z.files:
            a = z[name]
            j = js['arrays'][name]
            narr += 1
            if list(a.shape) != j['shape'] or str(a.dtype) != j['dtype'] or a.tolist() != j['data']:
                mism += 1
            assert a.shape == (2000,), (k, arm, name, a.shape)
            d[name] = a.astype(np.int64)
        D[(k, arm)] = d
META = {k: json.load(open(f'{ROOT}/data/k{k}/meta.json')) for k in KS}
MODEL = json.load(open(f'{ROOT}/model_predictions.json'))
FSC = {(c['k'], c['t']): c for c in json.load(open(FS))['cells']}
OUT = dict(npz_vs_json=dict(arrays=narr, mismatches=mism))


def col(k, arm, t, c='covered_D'):
    return D[(k, arm)][f't{t}_{c}']          # [] indexing: a missing key raises (lesson L-99ee952350b2)


def rng(name):
    return np.random.Generator(np.random.PCG64(int(hashlib.blake2b(('validator-L3|' + name).encode(),
                                                                    digest_size=8).hexdigest(), 16)))


def rci(a, b, name):
    a, b = np.asarray(a, float), np.asarray(b, float)
    g = rng(name)
    ma = a[g.integers(0, a.size, (BOOT, a.size))].mean(1)
    mb = b[g.integers(0, b.size, (BOOT, b.size))].mean(1)
    rb = ma / mb
    return a.mean() / b.mean(), float(np.quantile(rb, 0.025)), float(np.quantile(rb, 0.975))


def ks(a, b):
    a, b = np.sort(a), np.sort(b)
    grid = np.unique(np.concatenate([a, b]))
    return float(np.max(np.abs(np.searchsorted(a, grid, 'right') / a.size - np.searchsorted(b, grid, 'right') / b.size)))


def sel(k, arm, t):
    J, npt, cov = col(k, arm, t, 'J_D'), col(k, arm, t, 'n_points'), col(k, arm, t)
    best = None
    for i in range(J.size):                 # explicit argmax: J desc, points desc, index asc
        key = (-int(J[i]), -int(npt[i]), i)
        if best is None or key < best[0]:
            best = (key, i)
    i = best[1]
    return dict(i=i, cov=int(cov[i]), gain=float(cov[i] / cov.mean()), oracle=int(cov.max()))


def mD(k, t, arm):
    return MODEL[f'{k}|{t}|{arm}']['D']


def null(arm):
    return 'E.x' if arm.startswith('E.') else 'Ep.x'


# ---------------------------------------------------------------- independent model closed forms
def own_model(k, t, arm):
    p, A, n = {16: (65437, 1058, 65672), 18: (262133, 78, 263096), 20: (1048573, 926, 1049624)}[k]
    r = n // 8
    q, nv = n / (2 * p), 2 ** t
    EN, ENN = nv * q, nv * (nv - 1) * q * q
    sat = lambda m: (r - 1) * (1 - math.exp(-m / (r - 1)))
    if arm in ('E.x', 'E.iota_even', 'E.iota_T0'):
        return sat(ENN / 4 + EN / 2)                 # 2 of 8 cosets self-paired, 2 targets each
    if arm in ('Ep.x', 'Ep.iota_2E'):
        return sat(ENN / 4 + EN)                     # 4 of 8
    if arm in NOHALF or arm == 'E.x_noself':
        return sat(ENN / 4)
    if arm.startswith('Ep.ji'):
        q4 = n / (4 * p)
        EN4, ENN4 = nv * q4, nv * (nv - 1) * q4 * q4
        legs = lambda v: pow(v % p, (p - 1) // 2, p) == 1
        w = {'Ep.ji_s': 1, 'Ep.ji_c': 1 + legs(A - 2), 'Ep.ji_d': 1 + legs(A + 2)}[arm]
        return sat(ENN4 / 2 + EN4 * w)


mm = []
for k in KS:
    for t in TS:
        for arm in ARMS:
            mm.append(abs(own_model(k, t, arm) / mD(k, t, arm) - 1))
OUT['model_json_vs_own_closed_forms_max_rel_diff'] = max(mm)

V = {}


def verdict(name, held, nok, ntot, **kw):
    V[name] = dict(held=bool(held), n_ok=int(nok), n_total=int(ntot), **kw)


# CAL1
det, nok = [], 0
for k in KS:
    for t in TS:
        fx = META[k]['fixed'][str(t)]
        for mine, theirs in (('E.bits_lo0', 'bits_lo0'), ('E.edwards_bits', 'edwards')):
            ok = fx[mine]['covered_full'] == FSC[(k, t)][theirs]
            nok += ok
            det.append((k, t, mine, fx[mine]['covered_full'], FSC[(k, t)][theirs]))
verdict('CAL1', nok == 30, nok, 30)
# CAL2 (SE of the difference; frame_search sd not recorded for Moebius frames -> two variants)
det, nok, nok_own = [], 0, 0
for k in KS:
    for t in TS:
        x = col(k, 'E.x', t)
        m = FSC[(k, t)]['mobius_mean']
        se_a = math.sqrt(x.var() / x.size + FSC[(k, t)]['trans_sd'] ** 2 / 2000)
        se_b = math.sqrt(2 * x.var(ddof=1) / 2000)       # own variant: same sd assumed on both sides
        nok += abs(x.mean() - m) <= 3 * se_a
        nok_own += abs(x.mean() - m) <= 3 * se_b
        det.append((k, t, round(x.mean(), 3), m, round((x.mean() - m) / se_b, 2)))
verdict('CAL2', nok >= 14, nok, 15, own_se_variant=nok_own, z=[d[-1] for d in det])
# CAL3
det, nok = [], 0
for k in KS:
    for t in TS:
        for arm in ('E.x', 'Ep.x'):
            rr = col(k, arm, t).mean() / mD(k, t, arm)
            nok += abs(rr - 1) <= TOL[t]
            det.append(rr)
verdict('CAL3', nok == 30, nok, 30, range=(round(min(det), 4), round(max(det), 4)))
# C1
n1 = n0 = n2 = ntot = 0
c2cells = []
for k in KS:
    for t in TS:
        for arm in CONTROLS:
            c = col(k, arm, t, 'control_ok')
            n1 += int((c == 1).sum()); n0 += int((c == 0).sum()); n2 += int((c == 2).sum()); ntot += c.size
            other = int(((c != 0) & (c != 1) & (c != 2)).sum())
            assert other == 0
            if (c == 2).sum():
                c2cells.append((k, t, arm, int((c == 2).sum())))
verdict('C1', n0 == 0 and n2 <= 0.01 * ntot, n1, ntot, code0=n0, code2=n2, code2_cells=c2cells)
# C2
det, nok = [], 0
for k in KS:
    for t in TS:
        for arm in CONTROLS:
            rr = col(k, arm, t).mean() / col(k, null(arm), t).mean()
            nok += abs(rr - 1) <= TOL[t]
            det.append(rr)
verdict('C2', nok >= 44, nok, 45, range=(round(min(det), 4), round(max(det), 4)))
# AA
det, nok = [], 0
for k in KS:
    for t in TS:
        for arm in ('E.x', 'Ep.x'):
            x = col(k, arm, t)
            rr, lo, hi = rci(x[0::2], x[1::2], f'AA|{k}|{t}|{arm}')
            ok = lo <= 1 <= hi
            nok += ok
            if not ok:
                det.append((k, t, arm, round(rr, 4), round(lo, 4), round(hi, 4)))
verdict('AA', nok >= 27, nok, 30, excluded=det)
# PC1
okd = okm = 0
his, det = [], []
for k in KS:
    for t in TS:
        rr, lo, hi = rci(col(k, 'E.x_noself', t, 'covered_D_noself'), col(k, 'E.x', t), f'PC1|{k}|{t}')
        pm = mD(k, t, 'E.x_noself') / mD(k, t, 'E.x')
        if t in (4, 5, 6):
            okd += hi < 1
            his.append(hi)
        okm += abs(rr - pm) <= TOL[t]
verdict('PC1', okd == 9 and okm >= 14, okd + okm, 24, direction=f'{okd}/9', model=f'{okm}/15', max_hi=round(max(his), 4))
# PC2
g = {}
nok = 0
for k in KS:
    for t in (5, 6, 7):
        gg = sel(k, 'E.x', t)['gain']
        g[(k, t)] = round(gg, 3)
        nok += gg >= (2.0 if t == 5 else 1.3)
verdict('PC2', nok == 9, nok, 9, gains={f'{k}|{t}': v for (k, t), v in g.items()})
# P1
okd = okm = 0
worst_hi, rat = [], {}
for k in KS:
    for t in TS:
        for arm in NOHALF:
            rr, lo, hi = rci(col(k, arm, t), col(k, null(arm), t), f'P1|{k}|{t}|{arm}')
            pm = mD(k, t, arm) / mD(k, t, null(arm))
            if t in (4, 5):
                okd += hi < 1
                worst_hi.append(hi)
            okm += abs(rr - pm) <= TOL[t]
            rat[f'{k}|{t}|{arm}'] = (round(rr, 4), round(lo, 4), round(hi, 4), round(pm, 4))
verdict('P1', okd == 36 and okm >= 85, okd + okm, 126, direction=f'{okd}/36', model=f'{okm}/90', max_hi_t45=round(max(worst_hi), 4))
OUT['P1_cells'] = rat
# P2
okd = okm = 0
p2 = {}
for k in KS:
    for t in TS:
        rr, lo, hi = rci(col(k, 'Ep.x', t), col(k, 'E.x', t), f'P2|{k}|{t}')
        pm = mD(k, t, 'Ep.x') / mD(k, t, 'E.x')
        if t in (4, 5):
            okd += lo > 1
        okm += abs(rr - pm) <= TOL[t]
        p2[f'{k}|{t}'] = (round(rr, 4), round(lo, 4), round(hi, 4), round(pm, 4))
verdict('P2', okd == 6 and okm >= 14, okd + okm, 21, direction=f'{okd}/6', model=f'{okm}/15')
OUT['P2_cells'] = p2
# P3
okd = okm = 0
p3 = {}
mx = []
for k in KS:
    for t in TS:
        for arm in JI:
            rr, lo, hi = rci(col(k, arm, t), col(k, 'Ep.x', t), f'P3|{k}|{t}|{arm}')
            pm = mD(k, t, arm) / mD(k, t, 'Ep.x')
            if t in (5, 6, 7):
                okd += hi < 0.75
                mx.append(hi)
            okm += abs(rr - pm) <= max(TOL[t], 0.06)
            p3[f'{k}|{t}|{arm}'] = (round(rr, 4), round(lo, 4), round(hi, 4), round(pm, 4))
verdict('P3', okd == 27 and okm >= 40, okd + okm, 72, direction=f'{okd}/27', model=f'{okm}/45', max_hi=round(max(mx), 4))
OUT['P3_cells'] = p3
# P4a (cells with measured null mean >= 50; also the model-based selection, identical set)
nok = ntot = 0
rs, kss = [], []
for k in KS:
    for t in TS:
        if col(k, 'Ep.x', t).mean() < 50:
            continue
        assert mD(k, t, 'Ep.x') >= 50
        for x, y in (('Ep.huff_x', 'Ep.huff_y'), ('Ep.huff_x', 'Ep.iota_c01'), ('Ep.huff_y', 'Ep.iota_c01')):
            a, b = col(k, x, t), col(k, y, t)
            rr, dd = a.mean() / b.mean(), ks(a, b)
            ntot += 1
            nok += (0.95 <= rr <= 1.05) and dd <= 1.95 * math.sqrt(2 / 2000)
            rs.append(rr); kss.append(dd)
verdict('P4a', nok == ntot == 36, nok, ntot, ratio_range=(round(min(rs), 4), round(max(rs), 4)), max_ks=round(max(kss), 4),
        crit=round(1.95 * math.sqrt(2 / 2000), 4))
# P4b
nok = ntot = 0
rs = []
for k in KS:
    for t in TS:
        if col(k, 'Ep.x', t).mean() < 50:
            continue
        for x, y in (('Ep.iota_c01', 'Ep.iota_c10'), ('Ep.iota_c01', 'Ep.iota_c11'), ('Ep.iota_c10', 'Ep.iota_c11')):
            rr = col(k, x, t).mean() / col(k, y, t).mean()
            ntot += 1
            nok += 0.95 <= rr <= 1.05
            rs.append(rr)
verdict('P4b', nok >= 34 and ntot == 36, nok, ntot, ratio_range=(round(min(rs), 4), round(max(rs), 4)))
# P5
SH = {'E.iota_odd': ('E.x', 'covered_shift0', 'E.x|shift_odd'), 'Ep.iota_c01': ('Ep.x', 'covered_shift0', 'Ep.x|shift_01'),
      'Ep.iota_c10': ('Ep.x', 'covered_shift1', 'Ep.x|shift_10'), 'Ep.iota_c11': ('Ep.x', 'covered_shift2', 'Ep.x|shift_11')}
assert META[16]['shift_columns'] == {'E.x': ['odd'], 'Ep.x': ['(0, 1)', '(1, 0)', '(1, 1)']}
nok = 0
fd, r5 = [], []
for k in KS:
    for t in TS:
        for arm, (ref, c, mref) in SH.items():
            rr, lo, hi = rci(col(k, arm, t, 'covered_full'), col(k, ref, t, c), f'P5|{k}|{t}|{arm}')
            pm = MODEL[f'{k}|{t}|{arm}']['full'] / MODEL[f'{k}|{t}|{mref}']['full']
            nok += abs(rr - pm) <= TOL[t]
            fd.append((k, t, arm, col(k, arm, t, 'covered_full').mean() / col(k, arm, t).mean()))
            r5.append(rr)
verdict('P5', nok >= 50, nok, 60, full_over_D_range=(round(min(f[3] for f in fd), 3), round(max(f[3] for f in fd), 3)),
        full_over_D_min_cell=min(fd, key=lambda f: f[3])[:3], ratio_range=(round(min(r5), 4), round(max(r5), 4)))
# P6
ok6, det6 = [], {}
for arm in NOHALF:
    rs = []
    for k in KS:
        for t in (5, 6, 7):
            rs.append(sel(k, arm, t)['cov'] / sel(k, null(arm), t)['cov'])
    gm = math.exp(sum(math.log(v) for v in rs) / len(rs))
    ok6.append(0.80 <= gm <= 1.05 and max(rs) <= 1.15)
    det6[arm] = dict(geomean=round(gm, 4), max=round(max(rs), 4), min=round(min(rs), 4))
verdict('P6', all(ok6), sum(ok6), 6, arms=det6)
# P7
rs = []
cells7 = {}
for k in KS:
    for t in (5, 6, 7):
        a, b = sel(k, 'Ep.x', t)['cov'], sel(k, 'E.x', t)['cov']
        rs.append(a / b)
        cells7[f'{k}|{t}'] = (a, b, round(a / b, 4))
gm = math.exp(sum(math.log(v) for v in rs) / len(rs))
verdict('P7', 1.0 <= gm <= 1.30, int(1.0 <= gm <= 1.30), 1, geomean=round(gm, 4), range=(round(min(rs), 4), round(max(rs), 4)),
        cells=cells7)

# README-cited numbers
cells = {}
for k in KS:
    for t in TS:
        for arm in ARMS:
            cells[f'{k}|{t}|{arm}'] = dict(mean_D=round(col(k, arm, t).mean(), 3), sd_D=round(col(k, arm, t).std(ddof=1), 3),
                                           mean_full=round(col(k, arm, t, 'covered_full').mean(), 3),
                                           mean_points=round(col(k, arm, t, 'n_points').mean(), 3))
OUT['cells'] = cells
# t=4 no-half ratio ranges (README item 2) and t-ranges for P2 (item 5), JI range (item 3), P5 (item 4)
def rng_of(keys):
    v = [OUT['P1_cells'][k_][0] for k_ in keys]
    return (min(v), max(v))
OUT['readme_ranges'] = dict(
    E_odd_t4=rng_of([f'{k}|4|E.iota_odd' for k in KS]),
    Ep_nohalf_t4=rng_of([f'{k}|4|{a}' for k in KS for a in NOHALF[1:]]),
    JI_all=(min(v[0] for v in p3.values()), max(v[0] for v in p3.values())),
    P2_t4=(min(p2[f'{k}|4'][0] for k in KS), max(p2[f'{k}|4'][0] for k in KS)),
    P2_t5=(min(p2[f'{k}|5'][0] for k in KS), max(p2[f'{k}|5'][0] for k in KS)),
    P2_t8=(min(p2[f'{k}|8'][0] for k in KS), max(p2[f'{k}|8'][0] for k in KS)),
    P2_lo_all=min(v[1] for v in p2.values()),
    P5_ratio=V['P5']['ratio_range'])
OUT['verdicts'] = V
OUT['boot'] = BOOT
json.dump(OUT, open(sys.argv[1], 'w'), indent=1, default=str)
for kk, v in V.items():
    print(kk, 'held' if v['held'] else 'FAILED', f"{v['n_ok']}/{v['n_total']}",
          {a: b for a, b in v.items() if a not in ('held', 'n_ok', 'n_total')})
print('npz vs json', OUT['npz_vs_json'], 'model max rel diff', OUT['model_json_vs_own_closed_forms_max_rel_diff'])
print(OUT['readme_ranges'])
