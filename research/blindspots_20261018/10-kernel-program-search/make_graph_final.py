#!/usr/bin/env python3
"""Plots recorded values of item 10 (held-out ratios and intervals from raw-r2/summary.json, running-best curves
from raw-r2/state/*.jsonl). Plots recorded values only; no new computation beyond reading them."""
import json, math, sys, textwrap
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
S = json.load(open('raw-r2/summary.json'))
fig, ax = plt.subplots(1, 2, figsize=(11.5, 4.8))
comps = [('B', 'A', 'search finalist / incumbent'), ('B', 'A1', 'search finalist / incumbent+lift patch'),
         ('B', 'C', 'search finalist / random-search finalist')]
cols = {'0': '#7f7f7f', '01': '#1f77b4', '1': '#d62728'}
ylab, ypos = [], 0
for cell in S['cells']:
    for (x, y, lab) in comps:
        c = [k for k in cell['comparisons'] if k.get('x') == x and k.get('y') == y][0]
        for j, (wk, wn) in enumerate((('w0', '0'), ('w01', '01'), ('w1', '1'))):
            m, lo, hi = c[wk]
            ax[0].errorbar(m, ypos + (j - 1) * 0.22, xerr=[[m - lo], [hi - m]], fmt='o', ms=3, color=cols[wn], capsize=2, lw=1)
        ylab.append(f"n={cell['n']}  {x}/{y}"); ypos += 1
    ypos += 0.6
ax[0].axvline(1, color='k', lw=0.8, ls='--')
ax[0].set_xscale('log'); ax[0].set_yticks([i for i in range(0)])
pos = []; p = 0
for cell in S['cells']:
    for _ in comps: pos.append(p); p += 1
    p += 0.6
ax[0].set_yticks(pos); ax[0].set_yticklabels(ylab, fontsize=7); ax[0].invert_yaxis()
ax[0].set_xlabel('held-out Q ratio (log scale; <1 means the numerator is cheaper per relation)')
for wn, lab in (('0', 'w=0'), ('01', 'w=0.1'), ('1', 'w=1')):
    ax[0].plot([], [], 'o', color=cols[wn], label=f'unpriced counters at {lab}')
ax[0].legend(fontsize=7, loc='center left', bbox_to_anchor=(0.02, 0.38))
ax[0].set_title('Held-out Q ratios, 95% bootstrap intervals (2000 resamples)\n128 natural + 128 planted targets per cell', fontsize=9)
A = {(n, a): [json.loads(l) for l in open(f'raw-r2/state/n{n}-{a}.jsonl')] for n in (31, 37, 41) for a in 'bcp' if not (a == 'p' and n != 31)}
colr = {'b': '#1f77b4', 'c': '#d62728', 'p': '#2ca02c'}
for (n, a), L in A.items():
    rb = []; m = math.inf
    for r in L:
        v = r['q01']; v = math.inf if v is None else v
        m = min(m, v); rb.append(m if m < math.inf else float('nan'))
    ax[1].step(range(len(rb)), rb, {31: '-', 37: '--', 41: ':'}[n], color=colr[a], where='post', label=f'n={n} arm {a}')
ax[1].set_yscale('log'); ax[1].set_xlabel('evaluation index'); ax[1].set_ylabel('running-best training Q (w=0.1)')
ax[1].set_title('Search archives (training set; selected on it)', fontsize=9); ax[1].legend(fontsize=6, ncol=2)
fig.suptitle('Item 10 kernel program search, held-out assessment: per-cell verdicts ' +
             ', '.join(f"n={c['n']} {c['verdict']}" for c in S['cells']) + '; overall INCONCLUSIVE (cells disagree)', fontsize=9)
fig.tight_layout()
fig.savefig('graph.svg'); fig.savefig('graph.png', dpi=150)
if len(sys.argv) > 1:
    with PdfPages('REPORT.pdf') as pdf:
        pdf.savefig(fig)
        lines = []
        for ln in open('README.md').read().splitlines():
            lines += textwrap.wrap(ln, 110) or ['']
        for i in range(0, len(lines), 64):
            f = plt.figure(figsize=(8.27, 11.69))
            f.text(0.05, 0.97, '\n'.join(lines[i:i + 64]), va='top', family='monospace', fontsize=6.4)
            pdf.savefig(f); plt.close(f)
