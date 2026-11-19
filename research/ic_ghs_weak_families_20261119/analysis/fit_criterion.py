#!/usr/bin/env python3
"""C-fit of PROTOCOL.md: fit ln cost = a + e ln q on a window, classify held-out sizes (P1, P2, P3, P4)
and write the P7 predictions.  Reads only committed data.  usage: fit_criterion.py <lane dir>"""
import json, math, re, sys, os
LANE = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = os.path.join(LANE, '..')
rows = [json.loads(l) for l in open(f'{R}/quartic_n4v2_20261020/results/summary.json') if l.strip()]
excl = {(r['arm'], r['q']) for r in rows if r['kind'] == 'rho_excluded'}
ic = {}; rho = {}
for r in rows:
    if r['kind'] == 'ic_size' and r['arm'] in ('B0', 'D', 'DL', 'DL1'): ic[(r['arm'], r['q'])] = r['meq']
    if r['kind'] == 'rho_size' and (r['arm'], r['q']) not in excl: rho[(r['arm'], r['q'])] = r['meq']

def ols(x, y):
    n = len(x); mx = sum(x)/n; my = sum(y)/n
    sxx = sum((a-mx)**2 for a in x); e = sum((a-mx)*(b-my) for a, b in zip(x, y))/sxx
    return my - e*mx, e
def fit(table, arm, qmax=None, qmin=None, w=0):
    pts = sorted((q, v[w]) for (a, q), v in table.items() if a == arm and (qmax is None or q <= qmax) and (qmin is None or q >= qmin))
    return ols([math.log(q) for q, _ in pts], [math.log(v) for _, v in pts]) + (len(pts),)

out = {'P1': {}, 'P2': {}, 'P3': {}, 'P4': {}, 'P7_predictions': {}}
# P1
agree = tot = 0
for arm in ('B0', 'D', 'DL', 'DL1'):
    for rr in ('plain', 'neg'):
        ai, ei, ni = fit(ic, arm, 4051); ar, er, nr = fit(rho, rr, 4051)
        cells = []
        for (a, q), v in sorted(ic.items()):
            if a != arm or q < 8191 or (rr, q) not in rho: continue
            pred = (ai-ar) + (ei-er)*math.log(q); meas = math.log(v[0]/rho[(rr, q)][0])
            cells.append({'q': q, 'pred_ratio': math.exp(pred), 'meas_ratio': math.exp(meas), 'ok': (pred < 0) == (meas < 0)})
        k = sum(c['ok'] for c in cells); agree += k; tot += len(cells)
        out['P1'][f'{arm}/{rr}'] = {'fit_e_ic': ei, 'fit_e_rho': er, 'n_fit': (ni, nr), 'agree': k, 'cells': len(cells), 'detail': cells}
out['P1']['pooled'] = {'agree': agree, 'cells': tot, 'frac': agree/tot, 'PASS': agree/tot >= 0.9}
# P2
def qstar(ai, ei, ar, er): return math.exp((ar-ai)/(ei-er)) if ei != er else float('inf')
a1, e1, _ = fit(ic, 'B0', 4051); a2, e2, _ = fit(rho, 'plain', 4051)
a3, e3, _ = fit(ic, 'B0'); a4, e4, _ = fit(rho, 'plain')
qs_w, qs_f = qstar(a1, e1, a2, e2), qstar(a3, e3, a4, e4)
fac = max(qs_w, qs_f)/min(qs_w, qs_f) if qs_w > 0 and qs_f > 0 and math.isfinite(qs_w) else float('inf')
out['P2'] = {'qstar_window_le4051': qs_w, 'qstar_full': qs_f, 'factor': fac, 'PASS': fac <= 4, 'e_ic_window': e1, 'e_rho_window': e2}
# P3
s = [json.loads(l) for l in open(f'{R}/quartic_n4_20261019/results/summary_all.json') if l.strip()]
a0 = {('A0', r['q']): r['ic_meq'] for r in s if r.get('arm') == 'A0' and 'q' in r and r['q'] and 'ic_meq' in r}
a0r = {('A0r', r['q']): r['ic_meq'] for r in s if r.get('arm') == 'A0r' and 'q' in r and r['q'] and 'ic_meq' in r}
e_a0 = fit(a0, 'A0')[1]; e_a0r = fit(a0r, 'A0r')[1]
rho_n4 = {('r', r['q']): r['rho_meq'] for r in s if r.get('arm') == 'A0' and r.get('q')}
e_rho_n4 = fit(rho_n4, 'r')[1]
exps = {arm: {'full': fit(ic, arm)[1], 'q>=4051': fit(ic, arm, None, 4051)[1]} for arm in ('B0', 'D', 'DL', 'DL1')}
out['P3'] = {'e_A0': e_a0, 'e_A0r': e_a0r, 'e_rho_on_A0_sizes': e_rho_n4, 'exponents_non_A0': exps,
             'rho_exps_v2': {rr: fit(rho, rr)[1] for rr in ('plain', 'neg')},
             'PASS_A0_above_3_2': e_a0 > 1.5 and e_a0r > 1.5,
             'PASS_non_A0_below_3_2': all(v['full'] < 1.5 for v in exps.values()),
             'PASS_non_A0_in_1_to_1_4': all(1.0 <= v['full'] <= 1.4 for v in exps.values())}
# P4
t = open(f'{R}/ghs3_vs_rho_20261023/TABLES.md').read()
pts = []
for l in t.splitlines():
    m = re.match(r'\|\s*(\d+)\s*\|\s*(\d+)\s*\|\s*([\d.]+)\s*\|\s*([\d.e+]+)\s*\|\s*([\d.]+)\s*\|\s*([\d.e+]+)\s*\|\s*([\d.e+]+)\s*\|\s*([\d.]+)\s*\|\s*([\d.]+)\s*\|', l)
    if m and 14 <= int(m.group(1)) <= 20: pts.append((int(m.group(1)), float(m.group(4)), float(m.group(6)), float(m.group(8))))
x = [p[0]*math.log(2) for p in pts]; y = [math.log(p[1]/p[2]) for p in pts]
_, slope = ols(x, y)
_, e_ic = ols(x, [math.log(p[1]) for p in pts]); _, e_rid = ols(x, [math.log(p[2]) for p in pts])
out['P4'] = {'n_pts': len(pts), 'slope_ln_IC_over_Rideal_vs_ln_q': slope, 'e_IC': e_ic, 'e_Rideal': e_rid,
             'min_IC_over_Rideal': min(p[3] for p in pts),
             'PASS': (-0.25 < slope < 0) and min(p[3] for p in pts) >= 0.5}
# P7 predictions (full-range fit, all w)
for arm in ('B0', 'DL1', 'D', 'DL'):
    for rr in ('plain', 'neg'):
        for qn in (98299, 196579, 393187):
            vals = []
            for w in (0, 1, 2):
                ai, ei, _ = fit(ic, arm, None, None, w); ar, er, _ = fit(rho, rr, None, None, w)
                vals.append(math.exp((ai-ar) + (ei-er)*math.log(qn)))
            out['P7_predictions'][f'{arm}/{rr}/{qn}'] = {'ratio_w0_w01_w1': vals}
json.dump(out, open(f'{LANE}/fit_criterion.json', 'w'), indent=1)
print(json.dumps({k: (v if k != 'P1' else {kk: (vv if kk == 'pooled' else {'agree': vv['agree'], 'cells': vv['cells'], 'e_ic': round(vv['fit_e_ic'], 3), 'e_rho': round(vv['fit_e_rho'], 3)}) for kk, vv in v.items()}) for k, v in out.items() if k != 'P7_predictions'}, indent=1))
