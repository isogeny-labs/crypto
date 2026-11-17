# Lays out kstar_recorded.json (recorded values) and the two registered k(ell) formulas; computes no result.
import json, matplotlib; matplotlib.use('svg')
import matplotlib.pyplot as plt
d = json.load(open('kstar_recorded.json')); m = 3
fig, ax = plt.subplots(figsize=(7.5, 4.2))
for r in d['rows']:
    if 'k_star' in r:
        ks = r['k_star']
        for k in sorted(set(ks)):
            ax.scatter(r['ell'], k, s=40 + 25 * ks.count(k), color='#1f5aa6', zorder=3)
            ax.annotate(f"{ks.count(k)}/{len(ks)}", (r['ell'], k), textcoords='offset points', xytext=(8, -3), fontsize=8)
    else:
        ax.plot([r['ell'], r['ell']], [3.5, 4], color='#1f5aa6', lw=3, zorder=3)
        ax.scatter(r['ell'], 4, s=60, color='#1f5aa6', zorder=3)
        ax.annotate('3 raw: 1/4 suffice\n4 raw: 4/4 suffice', (r['ell'], 4), textcoords='offset points', xytext=(8, -12), fontsize=8)
ells = [6, 7, 8, 9]
ax.plot(ells, [m * (l - 6) for l in ells], '--', color='#c0504d', label='K_excess  k = m(ℓ−6)  (registered)')
ax.plot(ells[1:], [m * (l - 7) + 1 for l in ells[1:]], ':', color='#6b9a5b', lw=2, label='K_marginal  k = m(ℓ−7)+1  (registered)')
ax.axvspan(8.6, 9.4, color='#eeeeee', zorder=0); ax.text(8.65, 8.3, 'ℓ = 9:\nunmeasured', fontsize=8)
ax.set_xlabel('ℓ (factor-base subspace dimension), m = 3'); ax.set_ylabel('k* (guesses until every branch refutes at degree 4)')
ax.set_xticks(ells); ax.set_ylim(-0.5, 9.5); ax.legend(fontsize=8, loc='upper left', frameon=False)
ax.set_title('Guesses needed to hold the MutantXL closure at degree 4 (recorded, m = 3)', fontsize=9)
fig.tight_layout(); fig.savefig('kstar.svg'); fig.savefig('kstar.png', dpi=150)
