# Lays out kstar_recorded.json and results_recorded.json (recorded values) and the registered
# K formulas; computes no result.
import json, matplotlib; matplotlib.use('svg')
import matplotlib.pyplot as plt
k = json.load(open('kstar_recorded.json')); r = json.load(open('results_recorded.json')); m = 3
fig, (a, b) = plt.subplots(1, 2, figsize=(11.5, 4.3), gridspec_kw={'width_ratios': [1.4, 1]})
for row in k['rows']:
    if 'k_star' in row:
        ks = row['k_star']
        for v in sorted(set(ks)):
            a.scatter(row['ell'], v, s=40 + 25 * ks.count(v), color='#1f5aa6', zorder=3)
            a.annotate(f"{ks.count(v)}/{len(ks)}", (row['ell'], v), textcoords='offset points', xytext=(8, -3), fontsize=8)
a.scatter(8, 4, s=60, color='#1f5aa6', zorder=3); a.annotate('4 raw: 4/4 suffice', (8, 4), textcoords='offset points', xytext=(8, -3), fontsize=8)
a.scatter(9, 5, s=110, color='#2e8b57', marker='v', zorder=4)
a.annotate('5 guesses (S5): 3/4 draws suffice\n(draw 1: 28/32 branches)', (9, 5), textcoords='offset points', xytext=(-150, 18), fontsize=8)
ells = [6, 7, 8, 9]
a.plot(ells, [m * (l - 6) for l in ells], '--', color='#c0504d', label='K_excess  k = m(ℓ−6)')
a.plot(ells[1:], [m * (l - 7) + 1 for l in ells[1:]], ':', color='#6b9a5b', lw=2, label='K_marginal  k = m(ℓ−7)+1')
a.set_xticks(ells); a.set_ylim(-0.5, 9.5); a.set_xlabel('ℓ (subspace dimension), m = 3')
a.set_ylabel('guesses that hold degree 4 (k*; ▼ = upper bound)'); a.legend(fontsize=8, frameon=False, loc='upper left')
a.set_title('Guess count: measured k* at ℓ = 9 is ≤ 5 (3 of 4 draws)', fontsize=9)
ls = ['7', '8', '9']; x = range(3)
d3 = [r['summand'][l].get('3', 0) for l in ls]; d4 = [r['summand'][l].get('4', 0) for l in ls]
b.bar(x, d4, color='#1f5aa6', label='refutes at degree 4'); b.bar(x, d3, bottom=d4, color='#9ecae1', label='refutes at degree 3')
b.set_xticks(list(x)); b.set_xticklabels([f'ℓ = {l}' for l in ls]); b.set_ylabel('branches (of 16: 4 draws × 4 x₁ values)')
b.set_title('Whole summand x₁ fixed: residual never above degree 4', fontsize=9); b.legend(fontsize=8, frameon=False, loc='lower right')
fig.tight_layout(); fig.savefig('results.svg'); fig.savefig('results.png', dpi=150)
