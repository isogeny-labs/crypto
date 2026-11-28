#!/usr/bin/env python3
"""L2 figure: coverage per column (census, left) and M2 proxy cost (right). Reads
analysis/summary.json and data/macaulay_k*.json; writes l2_coverage.svg and .png."""
import glob, json, statistics
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

S = json.load(open('analysis/summary.json'))
arms = ['v', 'q', 'uv', 'jac6', 'ye', 'xAA', 'rand', 'xe', 'xeye', 'even']
cols = {16: '#3b6ea8', 18: '#c0792b', 20: '#4a9a5b'}
fig, (a1, a2) = plt.subplots(1, 2, figsize=(10, 3.8), gridspec_kw=dict(width_ratios=[1.6, 1]))
for i, arm in enumerate(arms):
    for c in S['cells']:
        if c['arm'] != arm:
            continue
        x = i + (c['k'] - 18) * 0.12 + (c['t'] - 5.5) * 0.03
        a1.errorbar(x, c['R_col'], yerr=[[c['R_col'] - c['R_col_ci'][0]], [c['R_col_ci'][1] - c['R_col']]],
                    fmt='o', ms=3, color=cols[c['k']], lw=0.8)
for y, lab in [(1, 'generic'), (0.5, '1/|H| = 1/2'), (0.25, '1/4'), (2, 'even cosets')]:
    a1.axhline(y, color='gray', lw=0.5, ls='--')
    a1.text(len(arms) - 0.4, y, lab, fontsize=7, va='bottom', ha='right', color='gray')
a1.set_xticks(range(len(arms)))
a1.set_xticklabels(['v', 'x/y', 'uv', 'Y²/X³', 'y_E', 'x A/A', 'random', 'x_E', 'x_E·y_E', '2E (PC2)'], fontsize=8)
a1.set_yscale('log'); a1.set_yticks([0.25, 0.5, 1, 2]); a1.set_yticklabels(['0.25', '0.5', '1', '2']); a1.minorticks_off(); a1.set_ylabel('coverage per column ÷ cols-matched x-frames')
a1.set_title('(a) census, k = 16 / 18 / 20, t = 4..7, K = 2000', fontsize=9)
for k, c in cols.items():
    a1.plot([], [], 'o', color=c, label=f'k = {k}')
a1.legend(fontsize=7, loc='upper left')
recs = [r for fn in sorted(glob.glob('data/macaulay_k*.json')) for r in json.load(open(fn))]
fs = ['x', 'ye', 'xe', 'xeye', 'v', 'q', 'uv', 'jac6']
for T, mk in ((4, 's'), (5, 'o')):
    ys = []
    for f in fs:
        v = [r['M2']['hist'][-1]['muladds'] for r in recs if r.get('M2', {}).get('mode') == 'ref' and r['f'] == f and r['T'] == T]
        ys.append(statistics.median(v))
    a2.plot(range(len(fs)), ys, mk + '-', ms=4, lw=0.8, label=f'T = {T} (n = {2*T})', color='#444' if T == 5 else '#999')
a2.set_xticks(range(len(fs))); a2.set_xticklabels(['x', 'y_E', 'x_E', 'x_E·y_E', 'v', 'x/y', 'uv', 'Y²/X³'], fontsize=8, rotation=30)
a2.set_yscale('log'); a2.set_ylabel('M2 counted mul-adds per refuted target (median)')
a2.set_title('(b) Macaulay-closure proxy, all k pooled', fontsize=9)
a2.legend(fontsize=7)
fig.tight_layout()
fig.savefig('l2_coverage.svg')
fig.savefig('l2_coverage.png', dpi=80)
