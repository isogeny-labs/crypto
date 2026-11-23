"""Part 2 figure, two panels:
(left)  part (a): min dim(V.V) found vs the 2l-1 lower bound, per covered cell.
(right) part (c)/(b): S/A ratio with A/B (A/A) floor for L21 candidates (and SAT conflicts if run),
        non-decomposable, with 95% intervals. SVG + PNG."""
import json, os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, 'results.json')))
fig, (axL, axR) = plt.subplots(1, 2, figsize=(11, 4.6))

# left: min w vs 2l-1
pa = R['part_a']
rows = []
for c in pa.get('exhaustive', []):
    rows.append((f"l{c['l']}n{c['n']}\n(exh)", c['lower_bound_2lm1'], c['min_w']))
for c in pa.get('sampled', []):
    rows.append((f"l{c['l']}n{c['n']}\n(smp)", c['lower_bound_2lm1'], c['random_min']))
labels = [r[0] for r in rows]
x = range(len(rows))
axL.plot(x, [r[1] for r in rows], 'k--', marker='_', ms=18, lw=0, label='lower bound 2l-1 (HLX/BSZ)')
axL.plot(x, [r[2] for r in rows], 'o', color='#1f77b4', label='min dim(V.V) found')
axL.set_xticks(list(x)); axL.set_xticklabels(labels, fontsize=7)
axL.set_ylabel('dim(V.V)')
axL.set_title('(a) minimum product-space dimension\nexhaustive up to scaling / 5000-sample')
axL.legend(fontsize=8)

# right: S/A with A/B floor, L21 candidates (and SAT conflicts) nondec
def series(unitdict, label, col, mk):
    pts = []
    for key, d in sorted(unitdict.items()):
        if key.endswith('nondec'):
            l = d['l']
            pts.append((l, d['S_over_A'], d['S_over_A_ci'], d['A_over_B'], d['A_over_B_ci']))
    if not pts:
        return
    ls = [p[0] for p in pts]
    sa = [p[1] for p in pts]
    se = [[p[1] - p[2][0] for p in pts], [p[2][1] - p[1] for p in pts]]
    axR.errorbar(ls, sa, yerr=se, color=col, marker=mk, capsize=4, lw=1.6, label=f'{label} S/A (95% CI)')
    ab_lo = [p[4][0] for p in pts]; ab_hi = [p[4][1] for p in pts]
    axR.fill_between(ls, ab_lo, ab_hi, color=col, alpha=0.15)

if R.get('part_c', {}).get('cands'):
    series(R['part_c']['cands'], 'L21 candidates', '#1f77b4', 'o')
if R.get('part_b', {}).get('conflicts'):
    series(R['part_b']['conflicts'], 'SAT conflicts', '#d62728', 's')
axR.axhline(1.0, color='k', lw=0.8, ls='--')
axR.axhline(0.5, color='green', lw=0.8, ls=':', label='alive threshold (2x)')
axR.set_yscale('log')
axR.set_xlabel('l'); axR.set_ylabel('structured/random ratio (geom. mean)')
axR.set_title('(b,c) S vs random, non-decomposable\nshaded = A/B (A/A) floor')
axR.legend(fontsize=8)
fig.suptitle('Part 2 — choosing V (idea 11) [pcc:unverified] pilot', fontsize=11)
fig.tight_layout()
fig.savefig(os.path.join(HERE, 'figure.svg'))
fig.savefig(os.path.join(HERE, 'figure.png'), dpi=130)
print('wrote figure.svg / figure.png')
