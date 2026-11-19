#!/usr/bin/env python3
"""P7/P8: price the raw units of run_measure.sh (M-eq at w = 0, 0.1, 1) and compare with the frozen C-fit predictions.
M-eq = mul + r_inv*inv + r_lmul*lmul + w*(add + ladd) + w*gae*(hash + cmp), the pricing of research/ic_gw_nrm_lz_20261115/analyze.py.
usage: analyze_measure.py <lane dir>"""
import json, math, os, sys, glob, random
LANE = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RAW = f'{LANE}/raw'; W = (0.0, 0.1, 1.0)
pred = json.load(open(f'{LANE}/fit_criterion.json'))['P7_predictions']
def rows(f): return [json.loads(l) for l in open(f) if l.strip()]
def price(rec, ri, rl, g, w):
    pre = sorted({k.rsplit('.', 1)[0] for k in rec if k.endswith('.mul')})
    return sum(rec.get(p + '.mul', 0) + ri*rec.get(p + '.inv', 0) + rl*rec.get(p + '.lmul', 0) + w*(rec.get(p + '.add', 0) + rec.get(p + '.ladd', 0))
               + w*g*(rec.get(p + '.hash', 0) + rec.get(p + '.cmp', 0)) for p in pre)
out = {'units': [], 'fields': {}}
Q = (98299, 196579, 393187)
for q in Q:
    fld = {'plain': [], 'neg': []}; ic = {'B0': [[], [], []], 'D': [[], [], []], 'DL': [[], [], []], 'DL1': [[], [], []]}
    steps = {'plain': [], 'neg': []}; inst = []
    for s in (1, 2, 3):
        fi = f'{RAW}/ic_D_{q}_{s}.jsonl'; fb = f'{RAW}/ic_B0_{q}_{s}.jsonl'; fr = f'{RAW}/rho_{q}_{s}.jsonl'
        if not all(os.path.exists(x) for x in (fi, fb, fr)): continue
        R = rows(fi) + rows(fb); rr = rows(fr)
        rat = [r for r in R if r.get('rec') == 'ratios'][0]
        ri, rl, g = rat['r_inv'], rat['r_lmul'], rat['gae_mul']
        u = {'q': q, 'seed': s, 'ell': rat['ell'], 'r_inv': ri, 'r_lmul': rl, 'gae': g}
        for arm in ic:
            rs = [r for r in R if r.get('rec') == 'ic' and r['arm'] == arm]
            if not rs or not all(r['ok'] for r in rs[-1:]): u[f'{arm}_ok'] = False; continue
            u[f'{arm}_ok'] = True; u[f'{arm}_attempts'] = len(rs)
            for wi, w in enumerate(W):
                v = sum(price(r, ri, rl, g, w) for r in rs); u[f'{arm}_meq_w{w}'] = v; ic[arm][wi].append(v)
        for arm in ('plain', 'neg'):
            rs = [r for r in rr if r.get('rec') == 'rho' and r['arm'] == arm and r['ok']]
            u[f'rho_{arm}_runs'] = len(rs)
            for wi, w in enumerate(W):
                u[f'rho_{arm}_meq_w{w}'] = sum(price(r, ri, rl, g, w) for r in rs)/len(rs) if rs else None
            for r in rs: steps[arm].append(r['steps']/math.sqrt(r['ell']))
            if rs: fld[arm].append([u[f'rho_{arm}_meq_w{w}'] for w in W])
        inst.append(u); out['units'].append(u)
    if not inst: continue
    cal = {a: (sum(v)/len(v) if v else None) for a, v in steps.items()}
    ok = {'plain': cal['plain'] is not None and 1.0 <= cal['plain'] <= 1.5, 'neg': cal['neg'] is not None and 0.75 <= cal['neg'] <= 1.15}
    F = {'n_instances': len(inst), 'rho_steps_over_sqrt_ell': cal, 'rho_calibrated': ok, 'arms': {}}
    for arm in ic:
        if not ic[arm][0]: continue
        for rho in ('plain', 'neg'):
            if not fld[rho]: continue
            ratios = []
            for wi in range(3):
                m_ic = sum(ic[arm][wi])/len(ic[arm][wi]); m_rho = sum(x[wi] for x in fld[rho])/len(fld[rho])
                pr = pred.get(f'{arm}/{rho}/{q}', {}).get('ratio_w0_w01_w1', [None]*3)[wi]
                ratios.append({'w': W[wi], 'ic_meq': m_ic, 'rho_meq': m_rho, 'ratio': m_ic/m_rho, 'predicted': pr,
                               'measured_over_predicted': (m_ic/m_rho)/pr if pr else None,
                               'per_instance': [a/b[wi] for a, b in zip(ic[arm][wi], fld[rho])] if len(ic[arm][wi]) == len(fld[rho]) else None})
            F['arms'][f'{arm}/{rho}'] = ratios
    out['fields'][str(q)] = F
# P7 / P8 verdicts
p7_below = all(r['ratio'] < 1 for F in out['fields'].values() for k, v in F['arms'].items() if k.split('/')[0] in ('DL1', 'B0') for r in v)
p7_inst = all(x < 1 for F in out['fields'].values() for k, v in F['arms'].items() if k.split('/')[0] in ('DL1', 'B0') for r in v for x in (r['per_instance'] or []))
p7_band = all(r['measured_over_predicted'] is not None and 1/1.5 <= r['measured_over_predicted'] <= 1.5 for F in out['fields'].values() for k, v in F['arms'].items() if k.split('/')[0] in ('DL1', 'B0') for r in v)
ins = [json.loads(l) for l in open(f'{RAW}/instances.jsonl')]
tries = {q: [i['tries'] for i in ins if i['q'] == q] for q in Q}
out['P7'] = {'field_means_below_1_B0_DL1': p7_below, 'every_instance_below_1': p7_inst, 'within_factor_1_5_of_prediction': p7_band,
             'PASS': p7_below and p7_inst and p7_band}
out['P8'] = {'tries_per_instance': tries, 'mean_tries': {q: sum(v)/len(v) for q, v in tries.items()},
             'PASS': all(2 <= sum(v)/len(v) <= 60 for v in tries.values())}
json.dump(out, open(f'{LANE}/measure_results.json', 'w'), indent=1)
for q, F in out['fields'].items():
    print(q, 'n', F['n_instances'], 'rho steps/sqrt(ell)', {k: round(v, 3) if v else v for k, v in F['rho_steps_over_sqrt_ell'].items()}, F['rho_calibrated'])
    for k, v in F['arms'].items():
        if k.split('/')[0] in ('B0', 'DL1', 'D', 'DL'):
            print('  ', k, ' '.join(f"w={r['w']}: {r['ratio']:.4g} (pred {r['predicted']:.3g}, x{r['measured_over_predicted']:.2f})" for r in v))
print(out['P7'], out['P8'])
