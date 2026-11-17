# Lays out the two recorded `ecbench table` outputs only (mean S and the harness's 95% interval); computes nothing new.
import re, matplotlib; matplotlib.use('svg')
import matplotlib.pyplot as plt
T = '../../tables/'
def rows(f):
    out = {}
    for l in open(T + f):
        c = [x.strip() for x in l.split('|')[1:-1]]
        if len(c) > 8 and c[0].startswith('ECBS'):
            lo, hi = map(float, re.findall(r'[\d.]+', c[7]))
            out[c[1]] = (float(c[6]), lo, hi, c[12])
    return out
o, v = rows('table-claim-n13.md'), rows('table-claim-n13-v2.md')
arms = ['rho-strong', 'rho-frob', 'ic-f4-m3', 'ic-rootfind-m3', 'ic-f4-m2', 'ic-rootfind-m2', 'rho-strong-aa']
fig, ax = plt.subplots(figsize=(10, 4.2))
for j, (d, lab, col) in enumerate([(o, 'claim-n13 ECBS1hfd5bf1885227 (2245b75, lazy F4 charge)', '#8fa8c8'),
                                   (v, 'claim-n13-v2 ECBS1h396db1437f4a (2b9f3f72f, eager v2)', '#c0504d')]):
    for i, a in enumerate(arms):
        m, lo, hi, lb = d[a]
        ax.bar(i + (j - 0.5) * 0.38, m, 0.38, color=col, label=lab if i == 0 else None,
               hatch='//' if lb == 'yes' else None, edgecolor='black', linewidth=0.4)
        ax.errorbar(i + (j - 0.5) * 0.38, m, yerr=[[m - lo], [hi - m]], color='black', capsize=2, lw=0.8)
ax.set_yscale('log'); ax.set_xticks(range(len(arms))); ax.set_xticklabels(arms, fontsize=8)
ax.set_ylabel('mean S (log scale), 24 runs per arm, 95% interval')
ax.set_title('claim-n13, Koblitz E_0 n=13 d=5: mean S per arm, original vs re-recorded (hatched = lower bound)', fontsize=9)
ax.legend(fontsize=7.5, frameon=False, loc='upper right'); fig.tight_layout()
fig.savefig('mean_s.svg'); fig.savefig('mean_s.png', dpi=150)
