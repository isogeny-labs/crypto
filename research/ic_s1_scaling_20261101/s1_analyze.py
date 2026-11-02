#!/usr/bin/env python3
"""S1 analysis (protocol S1_SCALING_20261101.md).  Per cell: best lab arm (lowest median whole run among 8/8-verified lab arms), and for each WDSat config A-D the best l
(lowest median over 8 targets of WDSat metric / best-lab whole run on the same target; whole run and solver-only separately).  R2W cells' config A comes from R2W's committed fit B2 json
(not re-measured).  Fits: OLS ln(ratio) on n, 95% t interval (k-2 dof).
usage: s1_analyze.py <out.json>"""
import json, glob, os, sys, math, statistics as st
import numpy as np
from scipy import stats
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from s1lib import load
B = '/Volumes/SSD990-2/s1-build/s1b-20261101'; PRIOR = '/Volumes/SSD990-2/s1-build'
R2W = '/Volumes/SSD990-2/crypto-worktrees/ic-r2w-wdsat-sweep-20261031/research/ic_r2w_wdsat_sweep_20261031/analysis/r2w_fitB2_l3-13_extlab.json'
INF = float('inf')
START = {'T1': json.load(open(f'{B}/start.json'))}
if os.path.exists(f'{B}/start2.json'): START['T2'] = json.load(open(f'{B}/start2.json'))
START['S1'] = json.load(open(f'{PRIOR}/start.json'))
# (a, n, target_seed) -> cell
KEY = {(0, 9, 202610111901): 'K0 n=9', (1, 9, 202610111901): 'K1 n=9', (1, 11, 202610111901): 'K1 n=11', (0, 13, 202610111901): 'K0 n=13', (0, 15, 202610111901): 'K0 n=15',
       (1, 15, 202610111901): 'K1 n=15', (1, 17, 202610111901): 'K1 n=17', (0, 19, 202610111901): 'K0 n=19', (1, 19, 202610111901): 'K1 n=19', (0, 13, 202611011301): 'K0 n=13 s2'}
NN = {'K0 n=9': 9, 'K1 n=9': 9, 'K1 n=11': 11, 'K0 n=13': 13, 'K0 n=15': 15, 'K1 n=15': 15, 'K1 n=17': 17, 'K0 n=19': 19, 'K1 n=19': 19, 'K0 n=13 s2': 13}
R2WCELLS = ['K0 n=9', 'K1 n=9', 'K1 n=11', 'K0 n=13', 'K0 n=15']
def finished(d):
    lg = d + '.log'
    return os.path.exists(lg) and any(l.startswith('exit ') for l in open(lg)) and any('complete ->' in l for l in open(lg))
def collect():
    data = {c: {} for c in NN}   # cell -> arm (cfg|'lab', l) -> {t: (ok, whole, lower)}
    rbyc = {}
    for root, pfx in ((f'{B}/runs', None), (f'{PRIOR}/runs', 'S1')):
        for d in sorted(glob.glob(f'{root}/*')):
            nm = os.path.basename(d)
            if d.endswith('_tm') or d.endswith('.log') or not os.path.isdir(d) or not os.path.exists(d + '/records.jsonl') or not finished(d): continue
            if pfx == 'S1' and (not nm.startswith('S1_') or '_AA' in nm or nm.startswith('S1_T')): continue
            sp = json.load(open(d + '/spec.json')); cv = sp['workloads']['curves'][0]
            cell = KEY.get((cv['a'], cv['n'], sp['workloads']['target_seed']))
            if cell is None: continue
            p = pfx or nm[:2]
            if p not in START: continue
            try: recs = load(d, d + '_tm', START[p])
            except AssertionError: continue
            for x in recs:
                cfg = 'lab' if x['arm'].startswith('lab') else x['arm'][2]
                rbyc[cell] = x['r']
                data[cell].setdefault((cfg, x['l']), {}).setdefault(x['t'], (x['ok'], x['whole'], x['lower'], nm))
    part = {c: {} for c in NN}   # discarded attempts (a WDSat call exited 1 on the last target): targets with a clean stderr only
    for d in sorted(glob.glob(f'{B}/runs_discarded/T?_*_err*')):
        nm = os.path.basename(d)
        if d.endswith('_tm') or not os.path.exists(d + '/records.jsonl'): continue
        tmd = d + '_tm'
        if not os.path.isdir(tmd): continue
        sp = json.load(open(d + '/spec.json')); cv = sp['workloads']['curves'][0]
        cell = KEY.get((cv['a'], cv['n'], sp['workloads']['target_seed']))
        if cell is None: continue
        bad = set(int(os.path.basename(f).split('.')[0]) for f in glob.glob(d + '/exec/*.stderr') if os.path.getsize(f) > 0)
        try: recs = load(d, tmd, START[nm[:2]])
        except AssertionError: continue
        for i, x in enumerate(recs):
            if i in bad: continue
            cfg = 'lab' if x['arm'].startswith('lab') else x['arm'][2]
            part[cell].setdefault((cfg, x['l']), {}).setdefault(x['t'], (x['ok'], x['whole'], x['lower'], nm))
    return data, rbyc, part
def cell_result(cell, arms, r2w, part=None):
    full = {a: d for a, d in arms.items() if len(d) == 8}
    projected = {a: d for a, d in (part or {}).items() if a not in full and a[0] != 'lab' and len(d) >= 7}
    labs = {l: d for (c, l), d in full.items() if c == 'lab' and all(o for o, *_ in d.values())}
    res = dict(n=NN[cell], r=None)
    lab_src = 'own'
    if labs:
        bl = min(labs, key=lambda l: st.median(w for _, w, *_ in labs[l].values())); labt = {t: v[1] for t, v in labs[bl].items()}
        res.update(best_lab=bl, lab_ir=st.median(labt.values()), lab_curve={l: st.median(w for _, w, *_ in d.values()) for l, d in sorted(labs.items())})
    else: labt = None
    res['cfg'] = {}
    for cfg in 'ABCD':
        cand = {}
        for (c, l), d in full.items():
            if c != cfg or labt is None: continue
            ts = sorted(t for t in d if d[t][0] and t in labt)
            if len(ts) < 8: cand[l] = dict(failed=True, n=len(ts)); continue
            cand[l] = dict(whole=st.median(d[t][1] / labt[t] for t in ts), lower=st.median(d[t][2] / labt[t] for t in ts),
                           wd_whole=st.median(d[t][1] for t in ts), wd_lower=st.median(d[t][2] for t in ts), failed=False)
        okm = {l: v for l, v in cand.items() if not v['failed']}
        e = dict(measured_best_l=None)
        for (c, l), d in projected.items():
            if c != cfg or labt is None: continue
            ts = sorted(t for t in d if d[t][0] and t in labt)
            if len(ts) >= 7: cand[l] = dict(whole=st.median(d[t][1] / labt[t] for t in ts), lower=st.median(d[t][2] / labt[t] for t in ts),
                                       wd_whole=st.median(d[t][1] for t in ts), wd_lower=st.median(d[t][2] for t in ts), failed=False, proj=True, nt=len(ts))
        ok = {l: v for l, v in cand.items() if not v['failed']}
        if okm:
            bwm = min(okm, key=lambda l: okm[l]['whole']); bom = min(okm, key=lambda l: okm[l]['lower'])
            e.update(meas_whole_l=bwm, meas_wd_whole=okm[bwm]['wd_whole'], meas_ratio_whole=okm[bwm]['whole'], meas_lower_l=bom, meas_wd_lower=okm[bom]['wd_lower'], meas_ratio_lower=okm[bom]['lower'])
        e['curve'] = {l: (v['wd_whole'] if not v['failed'] else None) for l, v in sorted(cand.items())}; e['proj_l'] = sorted(l for l, v in cand.items() if v.get('proj'))
        if ok:
            bw = min(ok, key=lambda l: ok[l]['whole']); bo = min(ok, key=lambda l: ok[l]['lower'])
            e.update(best_whole_l=bw, wd_whole=ok[bw]['wd_whole'], ratio_whole=ok[bw]['whole'], best_lower_l=bo, wd_lower=ok[bo]['wd_lower'], ratio_lower=ok[bo]['lower'])
        res['cfg'][cfg] = e
    if cell in r2w:   # R2W cell: A from the committed fit, lab from R2W
        j = r2w[cell]; res['r'] = j['r']
        res['best_lab'] = int(j['best_lab'].split('l')[-1]); res['lab_ir'] = j['lab_ir']
        res['cfg']['A'] = dict(curve={int(a.split('l')[-1]): v['wd_whole_med'] for a, v in j['cand'].items()}, best_whole_l=int(j['best_wd_whole'].split('l')[-1]), wd_whole=j['cand'][j['best_wd_whole']]['wd_whole_med'],
                               ratio_whole=j['ratio_whole'], best_lower_l=int(j['best_wd_lower'].split('l')[-1]), wd_lower=j['cand'][j['best_wd_lower']]['wd_lower_med'], ratio_lower=j['ratio_lower'], src='R2W', meas_ratio_whole=j['ratio_whole'], meas_ratio_lower=j['ratio_lower'], proj_l=[])
    return res

def projection(out, data):
    """K1 n=19 A l=12: WDSat Ir of targets 0-6 from the discarded attempts of DR_A_k1n19_x12 (a WDSat call on target 7 exited 1; the discarded attempts' child time files were overwritten by the retry)
    + the child overhead (whole - wdsat_ir) of K0 n=19 l12, measured; paired to K1 n=19 best lab (l9) per target.  Labelled PROJECTED."""
    res = {}
    k0 = data['K0 n=19'].get(('A', 12)); k1lab = data['K1 n=19']
    if not k0 or len(k0) != 8: return res
    sess = json.load(open(f'{B}/runs/T1_DR_A_k0n19_x12/records.jsonl'.replace('records.jsonl', 'session.json')))
    recs = [json.loads(l) for l in open(f'{B}/runs/T1_DR_A_k0n19_x12/records.jsonl')]
    ov = st.median(v[1] - (r['solver']['extra']['wdsat_ir'] - 0) for v, r in zip([k0[t] for t in sorted(k0)], sorted(recs, key=lambda r: r['workload']['target_index'])))
    labl = out['cells']['K1 n=19'].get('best_lab'); lab = {t: v[1] for t, v in data['K1 n=19'][('lab', labl)].items()}
    per = {}
    for d in sorted(glob.glob(f'{B}/runs_discarded/T1_DR_A_k1n19_x12_err*')):
        for r in (json.loads(l) for l in open(d + '/records.jsonl')):
            t = r['workload']['target_index']
            if t < 7 and r['outcome']['status'] == 'verified': per.setdefault(t, r['solver']['extra']['wdsat_ir'])
    ts = sorted(per)
    START1 = START['T1']
    ratios = [(per[t] + ov) / lab[t] for t in ts]
    lowers = [(per[t] - r_calls * START1['12']) / lab[t] for t, r_calls in ((t, 0) for t in ts)]
    res['K1 n=19 A l12'] = dict(n_targets=len(ts), child_overhead=ov, ratio_whole=st.median(ratios), wd_whole=st.median(per[t] + ov for t in ts), wd_ir_only=st.median(per[t] for t in ts))
    return res
def fit(pts, label):
    x = np.array([p[1] for p in pts], float); y = np.log([p[2] for p in pts]); k = len(x)
    sl, ic, rv, pv, se = stats.linregress(x, y); t = stats.t.ppf(0.975, k - 2)
    f = dict(label=label, k=k, slope=sl, half=t * se, lo=sl - t * se, hi=sl + t * se, p=pv, x4=math.exp(4 * sl), x4lo=math.exp(4 * (sl - t * se)), x4hi=math.exp(4 * (sl + t * se)), cells=[p[0] for p in pts])
    f['verdict'] = 'growing' if f['lo'] > 0 else 'shrinking' if f['hi'] < 0 else 'flat' if (f['lo'] >= -0.1 and f['hi'] <= 0.1) else 'undecided'
    return f
def main():
    r2w = json.load(open(R2W))['cells']
    data, rbyc, part = collect(); out = dict(cells={}, fits=[])
    for c in NN:
        res = cell_result(c, data[c], r2w, part[c])
        if res['r'] is None: res['r'] = rbyc.get(c)
        out['cells'][c] = res
        print(f"== {c} (n={NN[c]}, r={res['r']}) best lab l{res.get('best_lab')} {res.get('lab_ir', 0):.3g}  lab curve {({l: '%.3g' % v for l, v in res.get('lab_curve', {}).items()})}")
        for cfg in 'ABCD':
            e = res['cfg'][cfg]
            if 'best_whole_l' in e:
                print(f"   {cfg}{'(proj l%s)' % e['proj_l'] if e.get('proj_l') else ''}: whole l{e['best_whole_l']} {e['wd_whole']:.3g} ratio {e['ratio_whole']:.2f} | solver-only l{e['best_lower_l']} {e['wd_lower']:.3g} ratio {e['ratio_lower']:.2f} | curve {({l: ('%.3g' % v if v else 'x') for l, v in e['curve'].items()})}")
    out['proj'] = projection(out, data)
    print('PROJECTION', json.dumps(out['proj'], default=float))
    def pts(metric, cfgsel):
        P = []
        for c, res in out['cells'].items():
            vals = [res['cfg'][g].get(metric) for g in cfgsel(c) if res['cfg'][g].get(metric)]
            if vals: P.append((c, NN[c], min(vals)))
        return P
    sets = {'all cells, config A': lambda c: 'A', 'all cells, best config where swept': lambda c: 'ABCD'}
    for metric in ('ratio_whole', 'ratio_lower', 'meas_ratio_whole', 'meas_ratio_lower'):
        for lab, sel in (('A', lambda c: 'A'), ('best-available', lambda c: 'ABCD'), ('D', lambda c: 'D')):
            P = pts(metric, sel)
            for sub, flt in (('all', lambda p: True), ('without n=19', lambda p: p[1] < 19), ('R2W five', lambda p: p[0] in R2WCELLS), ('n>=13 only', lambda p: p[1] >= 13)):
                Q = [p for p in P if flt(p)]
                if len(Q) >= 3 and lab != 'D' or (lab == 'D' and len(Q) >= 3):
                    f = fit(Q, f'{metric} | cfg {lab} | {sub}'); out['fits'].append(f)
                    print(f"fit {f['label']:50s} k={f['k']}: slope {f['slope']:+.3f} +- {f['half']:.3f} (CI {f['lo']:+.3f}..{f['hi']:+.3f}, p={f['p']:.3f}) x{f['x4']:.2f}/4bits -> {f['verdict']}")
    pj = out['proj'].get('K1 n=19 A l12')
    if pj:
        for metric, val in (('meas_ratio_whole', pj['ratio_whole']), ('meas_ratio_lower', pj['ratio_whole'] * 0.70)):   # solver-only projection approximated: wd_ir/lab with the start-up share (about 3 percent) removed is not available per call; scaled by the measured solver/whole ratio at K0 n=19 l12 (0.66)
            P = pts(metric, lambda c: 'A')
            P = [(c, n, (min(v, val) if c == 'K1 n=19' else v)) for c, n, v in P]
            for sub, flt in (('all (K1 n=19 PROJECTED)', lambda p: True),):
                f = fit([q for q in P if flt(q)], f'{metric} | cfg A | {sub}'); out['fits'].append(f)
                print(f"fit {f['label']:50s} k={f['k']}: slope {f['slope']:+.3f} +- {f['half']:.3f} (CI {f['lo']:+.3f}..{f['hi']:+.3f}, p={f['p']:.3f}) x{f['x4']:.2f}/4bits -> {f['verdict']}")
    json.dump(out, open(sys.argv[1], 'w'), indent=1, default=float)
main()
