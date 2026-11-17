"""Layout only: every plotted number is copied from frozen native outputs."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
plt.rcParams.update({'svg.hashsalt':'leased-affine-v1','font.size':9})
p=Path(__file__).parent
d=json.loads((p/'native-counts.json').read_text())
a=d['current_native_aggregate'];old=d['earlier_native_counts']
fig,axes=plt.subplots(1,2,figsize=(10.6,2.5),gridspec_kw={'width_ratios':[1,1.6]})
axes[0].bar(['Old baseline','Broader\nrecompute v1','Bounded\nleased v1'],
    [old['baseline_curve_outcomes']['Root'],old['enabled_curve_outcomes']['Root'],a['after_roots']],
    color=['#667c91','#cf8b69','#397e9a'])
axes[0].set(ylabel='Root-returning inputs (of 28)',ylim=(0,28))
axes[0].set_title('Completed roots; different enabled mechanisms')
axes[1].plot([r['before_nodes'] for r in a['table']],label='Old baseline',color='#667c91',linewidth=2)
axes[1].plot([r['after_nodes'] for r in a['table']],label='Bounded leased',linestyle='none',marker='o',markersize=4,markerfacecolor='none',color='#397e9a')
axes[1].set(xlabel='Declared public input order (all 28)',ylabel='Search nodes')
axes[1].set_title('Capped prefixes are not used as gains')
axes[1].legend(frameon=False,fontsize=8)
for ax in axes:ax.spines[['top','right']].set_visible(False)
fig.tight_layout()
fig.savefig(p/'outcomes.svg',metadata={'Date':None})
svg=p/'outcomes.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
fig.savefig(p/'outcomes.pdf',metadata={'CreationDate':None,'ModDate':None})
