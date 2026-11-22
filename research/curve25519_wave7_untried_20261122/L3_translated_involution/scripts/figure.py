#!/usr/bin/env python3
"""Figure for README/REPORT: l3_coverage.svg and l3_coverage.png (from analysis/summary.json only)."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
S = json.load(open(os.path.join(ROOT, 'analysis', 'summary.json')))
V = S['verdicts']
TS = [4, 5, 6, 7, 8]

def series(cells, k, arm, ia=2):
    rows = {c[1]: c for c in cells if c[0] == k and (arm is None or c[ia] == arm)}
    return rows

fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.2, 3.6))
k = 20
specs = [  # (label, cells, arm or None, column offsets: ratio, lo, hi, model), colour
    ('E odd T (no half) / E x-frame', V['P1_nohalf_below_null']['detail']['cells'], 'E.iota_odd', (3, 4, 5, 6), '#c0392b'),
    ("E' Huff x / E' x-frame", V['P1_nohalf_below_null']['detail']['cells'], 'Ep.huff_x', (3, 4, 5, 6), '#e67e22'),
    ("E' JI s (degree 4) / E' x-frame", V['P3_JI_degree4_halves_coverage']['detail']['cells'], 'Ep.ji_s', (3, 4, 5, 6), '#8e44ad'),
    ("E' x-frame / E x-frame", V['P2_Ep_xframes_above_E_xframes']['detail']['cells'], None, (2, 3, 4, 5), '#27ae60'),
]
for lab, cells, arm, (ir, il, ih, im), col in specs:
    rows = series(cells, k, arm)
    r = [rows[t][ir] for t in TS]; lo = [rows[t][il] for t in TS]; hi = [rows[t][ih] for t in TS]
    m = [rows[t][im] for t in TS]
    ax.errorbar(TS, r, yerr=[[a - b for a, b in zip(r, lo)], [b - a for a, b in zip(r, hi)]], fmt='o', color=col,
                ms=4, capsize=2, label=lab)
    ax.plot(TS, m, '--', color=col, lw=1)
c2 = {c[1]: c[3] for c in V['C2_controls_match_null']['detail'] if c[0] == k and c[2] == 'E.iota_even'}
ax.plot(TS, [c2[t] for t in TS], 's', color='#555555', ms=4, label='translated frame (control) / null')
ax.axhline(1, color='k', lw=0.6)
ax.set_xlabel('t (frame values 2^t)'); ax.set_ylabel('mean covered_D ratio (95% CI); dashed = model')
ax.set_title('k = 20: mean exact coverage vs matched null', fontsize=9)
ax.legend(fontsize=6.5, loc='lower right'); ax.set_xticks(TS)
# selection panel: selected / selected-null over k, t in {5,6,7}
cells = V['P6_no_fatter_selection_tail']['detail']
arms = ['E.iota_odd', 'Ep.iota_c01', 'Ep.iota_c10', 'Ep.iota_c11', 'Ep.huff_x', 'Ep.huff_y']
for i, arm in enumerate(arms):
    vals = [c[5] for c in cells if c[0] == arm and c[1] != 'geomean']
    bx.plot([i] * len(vals), vals, 'o', color='#c0392b' if arm.startswith('E.') else '#e67e22', ms=3.5, alpha=0.8)
p7 = [c[4] for c in V['P7_Ep_selected_vs_E_selected']['detail']['cells']]
bx.plot([len(arms)] * len(p7), p7, 'o', color='#27ae60', ms=3.5)
bx.axhline(1, color='k', lw=0.6)
bx.axhspan(0.80, 1.05, color='#c0392b', alpha=0.07)
bx.set_xticks(range(len(arms) + 1))
bx.set_xticklabels(['E odd', "c01", "c10", "c11", 'Huff x', 'Huff y', "E'x / Ex"], fontsize=7)
bx.set_ylabel('selected covered_D / selected null')
bx.set_title('argmax-J selected frame, k x t in {5,6,7}', fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(ROOT, 'l3_coverage.svg'))
fig.savefig(os.path.join(ROOT, 'l3_coverage.png'), dpi=80)
print(os.path.getsize(os.path.join(ROOT, 'l3_coverage.png')))
