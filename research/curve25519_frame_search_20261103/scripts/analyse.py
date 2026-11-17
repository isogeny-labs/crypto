#!/usr/bin/env python3
"""Decide P1-P4 of PROTOCOL.md from confirm/k*.json; write analysis/summary.json."""
import json, glob, numpy as np
HELD = {16, 18, 22, 24}
out = {'cells': [], 'P1_inside': 0, 'P1_total': 0}
P2_ok = P3_ok = P4_ok = True; P3_fail = []; P2_fail = []; P4_fail = []
for f in sorted(glob.glob('confirm/k*.json'), key=lambda s: int(s.split('k')[-1][:-5])):
    d = json.load(open(f)); k = int(f.split('k')[-1][:-5]); r = d['r']
    for t in sorted({x['t'] for x in d['rows']}):
        R = [x for x in d['rows'] if x['t'] == t]
        fx = {x['family']: x for x in R if x['family'] in ('bits_lo0', 'edwards_bits')}
        tr = [x for x in R if x['family'] == 'translate']; mo = [x for x in R if x['family'] == 'mobius']
        ct = np.array([x['covered'] for x in tr]); cm = np.array([x['covered'] for x in mo])
        lo1, hi99 = np.percentile(ct, 1), np.percentile(ct, 99)
        for name, x in fx.items():
            out['P1_total'] += 1; out['P1_inside'] += int(lo1 <= x['covered'] <= hi99)
        ratio_tm = ct.mean() / cm.mean()
        if ct.mean() >= 50 and not (0.93 <= ratio_tm <= 1.07): P2_ok = False; P2_fail.append((k, t, ratio_tm))
        sel = sorted(tr, key=lambda x: (-x['J'], -x['cols'], x['frame'][1]))[0]
        best = max(tr, key=lambda x: x['covered'])
        b0 = fx['bits_lo0']['covered']
        gain = sel['covered'] / b0 if b0 else float('inf')
        if k in HELD and t in (5, 6, 7) and gain < 1.5: P3_ok = False; P3_fail.append((k, t, gain))
        p4 = sel['covered'] / best['covered']
        if p4 < 0.9: P4_ok = False; P4_fail.append((k, t, p4))
        pts_per_frame = np.mean([2 * x['cols'] for x in tr])
        search_group_ops = 2000 * pts_per_frame / 2 * 1.5 * np.log2(r)  # [r]P per column (one sign suffices)
        out['cells'].append(dict(k=k, r=r, t=t, held_out=k in HELD,
            bits_lo0=b0, edwards=fx['edwards_bits']['covered'], trans_mean=float(ct.mean()), trans_sd=float(ct.std()),
            mobius_mean=float(cm.mean()), trans_over_mobius=float(ratio_tm),
            pct_bits_lo0=float((ct < b0).mean()), pct_edwards=float((ct < fx['edwards_bits']['covered']).mean()),
            selected_L=sel['frame'][1], selected_cols=sel['cols'], selected_covered=sel['covered'],
            oracle_best_covered=best['covered'], gain_vs_bits_lo0=gain, gain_vs_mean=sel['covered'] / ct.mean(),
            selected_over_oracle=p4, cols_bits_lo0=fx['bits_lo0']['cols'],
            search_legendre=2000 * 2 ** t, search_group_ops=float(search_group_ops)))
out['P1'] = out['P1_inside'] >= 45 and out['P1_total'] == 50
out.update(P2=P2_ok, P2_fail=P2_fail, P3=P3_ok, P3_fail=P3_fail, P4=P4_ok, P4_fail=P4_fail)
json.dump(out, open('analysis/summary.json', 'w'), indent=1)
print(f"P1 {out['P1']} ({out['P1_inside']}/{out['P1_total']} inside 1-99%)  P2 {P2_ok} {P2_fail}  P3 {P3_ok} {P3_fail}  P4 {P4_ok} {P4_fail}")
print(" k  t  bits0  edw  tr_mean  sel(L)              sel  oracle  gain/b0 gain/mean  pct_b0 pct_edw  tr/mob")
for c in out['cells']:
    print(f"{c['k']:2d} {c['t']:2d} {c['bits_lo0']:6d} {c['edwards']:5d} {c['trans_mean']:8.1f}  {c['selected_L']:<18d} {c['selected_covered']:5d} {c['oracle_best_covered']:6d}  {c['gain_vs_bits_lo0']:6.2f}  {c['gain_vs_mean']:6.2f}   {c['pct_bits_lo0']:.2f}   {c['pct_edwards']:.2f}  {c['trans_over_mobius']:.3f}")
