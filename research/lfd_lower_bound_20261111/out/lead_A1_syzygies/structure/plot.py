#!/usr/bin/env python3
"""Figure for RESULT.md: structure.svg / structure.png from cells.jsonl and a6scan.jsonl."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
cells = [json.loads(l) for l in open(os.path.join(H, 'cells.jsonl'))]
scan = [json.loads(l) for l in open(os.path.join(H, 'a6scan.jsonl'))]
fig, ax = plt.subplots(1, 3, figsize=(15, 4.6))
# (1) decomposition of K_{e_min}
lab = [f"n={c['n']} s{c['seed']}" for c in cells]
T = [c['rank_T'] for c in cells]
low = [c['dim_K_cap_G'] - c['rank_T'] for c in cells]
new = [c['dim_K'][-1] - c['dim_K_cap_G'] for c in cells]
x = range(len(cells))
ax[0].bar(x, T, color='#9aa5b1', label='trivial family T_e')
ax[0].bar(x, low, bottom=T, color='#4c78a8', label='generated from I_{e-1} (H_a part)')
ax[0].bar(x, new, bottom=[a + b for a, b in zip(T, low)], color='#e45756', label='new top-degree generators')
for i in x:
    ax[0].text(i, T[i] + low[i] + new[i], str(new[i]), ha='center', va='bottom', fontsize=8)
ax[0].set_xticks(list(x)); ax[0].set_xticklabels(lab, rotation=30, fontsize=8)
ax[0].set_yscale('log'); ax[0].set_ylim(10, 2000); ax[0].set_ylabel('dimension (log)'); ax[0].set_title('K at e = e_min')
ax[0].legend(fontsize=7)
# (2) R_d / M(k, d+1) by degree, marking e_min
for c in cells:
    fr = [r / m for r, m in zip(c['dim_R'], c['M_k_dplus1'])]
    ax[1].plot(range(len(fr)), fr, marker='o', label=f"n={c['n']} s{c['seed']} (e_min={c['e_min']})")
ax[1].set_xlabel('degree d'); ax[1].set_ylabel('dim R_d / M(k, d+1)')
ax[1].set_title('consequence space R_d = rho(I_d) (unsatisfiable instances)')
ax[1].axhline(1, color='k', lw=0.5); ax[1].legend(fontsize=7)
# (3) codim of R_{e_min} vs |Z| over the a6 scan
cs = ['#e45756' if r['satisfiable'] else '#4c78a8' for r in scan]
emin_of = {15: 3, 17: 3, 19: 4}
from math import comb
def M(k, d): return sum(comb(k, i) for i in range(d + 1))
import random
rj = random.Random(0)
zs = [r['Z_size'] + rj.uniform(-0.12, 0.12) for r in scan]
cod = [M(r['dim_I'][0] and (r['n'] + 1) // 2, emin_of[r['n']] + 1) - r['dim_R'][emin_of[r['n']]] for r in scan]
cod = [c + rj.uniform(-0.12, 0.12) for c in cod]
ax[2].scatter(zs, cod, c=cs, s=22)
ax[2].plot([0, 4], [0, 4], 'k--', lw=0.6)
ax[2].set_xlabel('|Z| = #{C : h_C in Phi*_C(V)} (jittered)')
ax[2].set_ylabel('codim of R_e in RM(e+1) at e = e_min(V,z)')
ax[2].set_title(f'a6 scan, {len(scan)} draws: red = satisfiable, blue = unsat')
fig.tight_layout()
fig.savefig(os.path.join(H, 'structure.svg')); fig.savefig(os.path.join(H, 'structure.png'), dpi=130)
print('ok')
