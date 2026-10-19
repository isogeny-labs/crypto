"""Document layout only: plots retained Rust counts; performs no mathematics."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst

root = Path(__file__).resolve().parent
rows = list(csv.DictReader((root / 'results/screen-v1/counts.csv').open()))
fig, axes = plt.subplots(2, 1, figsize=(10, 7), constrained_layout=True)
for ax, ell in zip(axes, ('3', '5')):
    panel = [r for r in rows if r['ell'] == ell]
    labels = [f"q{r['q_residue']}, t{r['t_residue']}\n{r['module'][0].upper()}" for r in panel]
    counts = [int(r['stable_lagrangians']) for r in panel]
    bars = ax.bar(labels, counts, color=['#b65920' if r['module']=='scalar' else '#176b89' for r in panel])
    ax.bar_label(bars, padding=2, fontsize=8)
    ax.set_ylim(0, 20 if ell=='3' else 44)
    ax.set_ylabel('Stable Lagrangians (exact count)')
    ax.set_title(f"ell = {ell}: {panel[0]['all_lagrangians']} isotropic subspaces per module", loc='left')
    ax.tick_params(axis='x', labelsize=7)
    ax.grid(axis='y', alpha=.2)
    ax.set_axisbelow(True)
fig.suptitle('Finite module screen — not curve coverage or DLP speed', fontsize=14)
fig.supxlabel('q and t are residues modulo ell. C = companion; S = scalar.\nSource: results/screen-v1/counts.csv; exhaustive sets, no sampling uncertainty.', fontsize=10)
fig.savefig(root/'figures/kernel-counts.svg')
fig.savefig(root/'figures/kernel-counts.png', dpi=150)
plt.close(fig)
(root/'figures/evidence-chain.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="205" viewBox="0 0 1000 205">
<rect width="1000" height="205" fill="white"/>
<text x="15" y="24" font-family="DejaVu Sans" font-size="18" fill="#17394a">A kernel-screen hit leaves three construction and solver gates open</text>
<g font-family="DejaVu Sans" font-size="14" text-anchor="middle">
<rect x="10" y="53" width="220" height="102" rx="8" fill="#def1e8" stroke="#24734b"/>
<text x="120" y="80"><tspan x="120">Abstract module</tspan><tspan x="120" dy="23">finite screen COMPLETE</tspan><tspan x="120" dy="23">32 specified cases</tspan></text>
<rect x="263" y="53" width="220" height="102" rx="8" fill="#eef1f4" stroke="#6c7a88"/>
<text x="373" y="80"><tspan x="373">Actual curve + torsion</tspan><tspan x="373" dy="23">rational smooth quotient</tspan><tspan x="373" dy="23">PROPOSED</tspan></text>
<rect x="516" y="53" width="220" height="102" rx="8" fill="#eef1f4" stroke="#6c7a88"/>
<text x="626" y="80"><tspan x="626">Executable transfer</tspan><tspan x="626" dy="23">exact subgroup + recovery</tspan><tspan x="626" dy="23">PROPOSED</tspan></text>
<rect x="769" y="53" width="220" height="102" rx="8" fill="#eef1f4" stroke="#6c7a88"/>
<text x="879" y="80"><tspan x="879">Complete cold solving</tspan><tspan x="879" dy="23">matched source rho</tspan><tspan x="879" dy="23">UNMEASURED</tspan></text>
</g>
<g stroke="#6c7a88" stroke-width="2" stroke-dasharray="5 4" fill="none"><path d="M230 104H258m-8 -6 8 6 -8 6"/><path d="M483 104H511m-8 -6 8 6 -8 6"/><path d="M736 104H764m-8 -6 8 6 -8 6"/></g>
<text x="15" y="187" font-family="DejaVu Sans" font-size="13" fill="#425968">Dashed arrows are unresolved obligations. No new curve transfer or speedup is certified by this round.</text>
</svg>''')
typst.compile(str(root/'reports/search-report.typ'), output=str(root/'reports/search-report.pdf'), root=str(root))
print('Rendered module counts, evidence diagram and report PDF')
