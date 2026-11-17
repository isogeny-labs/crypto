import json, subprocess
import matplotlib; matplotlib.use('Agg')
import matplotlib.pyplot as plt
o=json.load(open('RESULT.json'))
order=[('m4-11-5','m=4 n=11 l=5\nN=42, 5272 col'),('m4-12-3','m=4 n=12 l=3\nN=36, 6585 col'),('m4-12-5','m=4 n=12 l=5\nN=44, ~6000 col'),('m3-16-6','m=3 n=16 l=6\nN=34, 4861 col'),('m3-11-5-deg4','m=3 n=11 l=5 degree 4\nN=26, 17902 col')]
fig,ax=plt.subplots(figsize=(8,4.2))
for i,(c,l) in enumerate(order):
    v=o['cells'][c]['n83']; ks=v['K']
    ax.scatter([i]*len(ks),ks,color='#1f77b4',zorder=3,s=22)
    ax.vlines(i,v['min'],v['max'],color='#1f77b4',lw=2,zorder=2)
    ax.hlines(v['median'],i-.2,i+.2,color='#d62728',lw=2,zorder=4)
ax.set_yscale('log'); ax.set_xticks(range(5)); ax.set_xticklabels([l for _,l in order],fontsize=7)
ax.set_ylabel('K = Ir per column / Ir per rho step (n=83)'); ax.grid(axis='y',alpha=.3)
ax.set_title('K per cell: dots = draws, bar = min..max, red = median')
fig.tight_layout(); fig.savefig('k_per_cell.svg'); fig.savefig('k_per_cell.png',dpi=150)
f=lambda x:f'{x:.2f}'
rows=''.join(f"| {c} | {o['cells'][c]['columns'][0]}{'-'+str(max(o['cells'][c]['columns'])) if min(o['cells'][c]['columns'])!=max(o['cells'][c]['columns']) else ''} | {min(o['cells'][c]['Icol']):.0f} to {max(o['cells'][c]['Icol']):.0f} | {f(o['cells'][c]['n83']['median'])} [{f(o['cells'][c]['n83']['min'])}, {f(o['cells'][c]['n83']['max'])}] | {f(o['cells'][c]['n61']['median'])} [{f(o['cells'][c]['n61']['min'])}, {f(o['cells'][c]['n61']['max'])}] |\n" for c,_ in order)
cv=''
for e in o['converted']:
    for k in ('low(min,n83)','primary(median,n83)','high(max,n83)','n61 median','deg4 growth cell n83'):
        v=e[k]; cv+=f"| {e['name']} | {k} | {f(v['K'])} | {f(v['log2K'])} | 2^{f(v['row'])} | +{f(v['gap'])} |\n"
I=o['Istep']
md=f"""# K conversion: column touches to rho steps

**Result.** Measured with Callgrind on this host, one binary. One rho step costs
{I['n83']['median']:.0f} Ir at n = 83 and {I['n61']['median']:.0f} Ir at n = 61. One column of the dense degree-3
closure costs 26,000 to 105,000 Ir on the m = 4 cells, so K (rho steps per
column touch) is **{o['cells']['m4-11-5']['n83']['median']:.1f} to {o['cells']['m4-12-3']['n83']['median']:.1f}** with the n = 83 step (pooled m = 4 median
{o['pooled_m4_deg3']['n83']['median']:.2f}, range {o['pooled_m4_deg3']['n83']['min']:.2f} to {o['pooled_m4_deg3']['n83']['max']:.2f}), and K grows with matrix size: {o['cells']['m3-11-5-deg4']['n83']['median']:.0f} at 17,902 columns
(degree 4, N = 26). K is above 1, so the conversion **raises** the priced rows,
the opposite of the sign the note guessed (its K is the reciprocal).

![K per cell](k_per_cell.svg)

## Definitions (frozen in PROTOCOL.md before any run)

K = (Ir per column touch) / (Ir per rho step). A column touch is one column of the
degree-D Macaulay matrix (the engine's `columns` = C(N, <=D), the note's w = 1
oracle unit). Ir per column = Callgrind Ir inside the dense full-MutantXL closure
(including the matrix build) divided by `columns`. A rho step = Ir inside
`StrongRhoG::solve` divided by the solver's `walk_steps`. Units are
`callgrind.Ir`, simulated user-space instructions; no wall-clock claim.

## K per cell (4 draws each; median [min, max])

| cell | columns | Ir per column | K, n = 83 step | K, n = 61 step |
|:--|--:|--:|:--|:--|
{rows}
Rho step: n = 83 capped walk of 10^6 steps, seeds 1 to 3: {', '.join(f'{x:.1f}' for x in I['n83']['vals'])} Ir/step.
n = 61 complete solves, seeds 1 to 3 (all verified): {', '.join(f'{x:.1f}' for x in I['n61']['vals'])} Ir/step. The ecbench record
(`research/ecbench_rho_optimized_20261014`) gives 1003.5 Ir/step on another curve, binary and compiler;
it is not mixed in.

## Controls

- A/A: the same closure draw twice differs by {o['AA']['closure_rel']*100:.4f}%; the same rho seed twice is identical ({o['AA']['rho'][0]} Ir both).
- Positive control: Ir inside the collected interval tracks the engine's own `closure_word_ops` across the 16 degree-3 draws (Spearman {o['spearman']['rho']:.2f}).
- Null control: the collected interval is {min(o['share'].values())*100:.0f} to {max(o['share'].values())*100:.0f}% of total process Ir (the rest is parsing, system build and the plain XL basis).
- Cell m4-12-3 (l = 3, a denser system: 6585 columns, 18 to 29 M word-ops) has K about 3.5 times the other three, which sit within 12% of each other. Within-cell draw spread is 1% to 45%.

## Derived conversion of the two ECC2K-130 rows

This is **derived arithmetic**: row + log2 K, on rows the descent-degree note priced and on K measured at N = 26 to 44.
It is a change of unit (accounting), not an improvement. The degree question (does the closure stay at degree 4, or
5, up to N near 394) is unchanged and unmeasured; rho stays 2^60.81 and its n = 131 step cost is unmeasured.

| row | K used | K | log2 K | converted row | converted gap to rho |
|:--|:--|--:|--:|--:|--:|
{cv}
Reading: the old gaps were +7.62 and +10.32 bits. At the pooled median K the converted gaps are
+{o['converted'][0]['primary(median,n83)']['gap']:.2f} and +{o['converted'][1]['primary(median,n83)']['gap']:.2f}; across the m = 4 degree-3 range
+{o['converted'][0]['low(min,n83)']['gap']:.2f} to +{o['converted'][0]['high(max,n83)']['gap']:.2f} and +{o['converted'][1]['low(min,n83)']['gap']:.2f} to +{o['converted'][1]['high(max,n83)']['gap']:.2f}. Because K grows with
matrix size and these rows have about 2^49 columns, the rows with K from small cells are **lower bounds** on the
oracle side, and the n = 131 step (wider field) would make K smaller by an unmeasured factor. No conversion measured here brings either row below rho.

## Limits

- n = 131 rho is not supported by the binary (group order exceeds u128): the denominator is n = 83 (primary) or n = 61.
- Degree 3 cells for the m = 4 numbers; degree 4 only at N = 26 (two draws). Degree 4 at N >= 34 costs too much under Callgrind.
- Symbol deviation from PROTOCOL.md 2.1: `from_system_closed_full` is inlined, so the toggle symbol is the function it calls, `ReducedBasis::from_system_closed_impl`. Same interval.
- Host: shared 4-vCPU Xeon 2.10 GHz VM, no PMU, no isolation tier; Ir is contention-independent. Binary and versions in PROVENANCE.txt. Conductor not installed in this container.
- The "(11,4), (12,4)" cells were read as n = 11 and n = 12 at m = 4.

Files: PROTOCOL.md, raw.jsonl, RESULT.json, run_k.py, analyze.py, make_report.py, PROVENANCE.txt.
Next improvement to try: measure K at degree 4 for N >= 34 on a larger host, and an n = 131 wide rho step.
"""
open('README.md','w').write(md)
