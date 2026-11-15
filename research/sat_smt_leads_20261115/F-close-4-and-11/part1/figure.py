"""Part 1 figure: xor/lit and sym/lit best-k conflict ratio vs l (UNSAT), with 95% intervals and
the within-lit A/A floor band. SVG + PNG."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, 'results.json')))
ls = R['sizes']
fig, ax = plt.subplots(figsize=(7, 4.6))
x = ls


def series(arm):
    g, lo, hi = [], [], []
    for l in ls:
        d = R['cells'][str(l)]['UNSAT'][f'{arm}_vs_lit']
        g.append(d['geomean']); lo.append(d['ci'][0]); hi.append(d['ci'][1])
    return g, lo, hi


for arm, col, mk in (('xor', '#1f77b4', 'o'), ('sym', '#d62728', 's')):
    g, lo, hi = series(arm)
    yerr = [[gi - l for gi, l in zip(g, lo)], [h - gi for gi, h in zip(g, hi)]]
    ax.errorbar(x, g, yerr=yerr, color=col, marker=mk, capsize=4, lw=1.6, label=f'{arm}/lit best-k (95% CI)')

# A/A floor band from lit
aa_lo = [R['cells'][str(l)]['UNSAT']['aa_lit']['ci'][0] for l in ls]
aa_hi = [R['cells'][str(l)]['UNSAT']['aa_lit']['ci'][1] for l in ls]
ax.fill_between(x, aa_lo, aa_hi, color='#999999', alpha=0.3, label='lit A/A floor (cube order, 95%)')
ax.axhline(1.0, color='k', lw=0.8, ls='--')
ax.axhline(0.8, color='green', lw=0.8, ls=':', label='alive threshold 0.8')
ax.set_xlabel('l (subspace dimension)'); ax.set_ylabel('conflict ratio vs lit (geometric mean)')
ax.set_xticks(ls)
ax.set_title('Part 1 — XOR cubes vs literal cubes (UNSAT, best k)\n[pcc:unverified] pilot')
ax.legend(fontsize=8, loc='best')
ax.set_yscale('log')
fig.tight_layout()
fig.savefig(os.path.join(HERE, 'figure.svg'))
fig.savefig(os.path.join(HERE, 'figure.png'), dpi=130)
print('wrote figure.svg / figure.png')
