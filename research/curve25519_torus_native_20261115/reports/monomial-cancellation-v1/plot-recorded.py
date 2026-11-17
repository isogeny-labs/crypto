"""Layout only: copy native values, no solver or result aggregation."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
plt.rcParams.update({'svg.hashsalt':'monomial-cancellation-v1','font.size':9})
p=Path(__file__).parent;d=json.loads((p/'native-counts.json').read_text());a=d['curve_aggregate']
fig,axes=plt.subplots(1,2,figsize=(10.6,2.45),gridspec_kw={'width_ratios':[1,1.6]})
axes[0].bar(['Old baseline','Bounded monomial'],[a['before_roots'],a['after_roots']],color=['#6c8094','#397e9a'])
axes[0].set(ylabel='Root-returning curve inputs (of 28)',ylim=(0,28),title='Preservation; no curve pruning gain')
axes[1].plot([r['before_nodes']for r in a['table']],color='#6c8094',linewidth=2,label='Old baseline')
axes[1].plot([r['after_nodes']for r in a['table']],linestyle='none',marker='o',markersize=4,markerfacecolor='none',color='#397e9a',label='Bounded monomial')
axes[1].set(xlabel='Declared public input order (all 28)',ylabel='Search nodes',title='All native outcomes retained')
axes[1].legend(frameon=False,fontsize=8)
for ax in axes:ax.spines[['top','right']].set_visible(False)
fig.tight_layout();fig.savefig(p/'outcomes.svg',metadata={'Date':None})
svg=p/'outcomes.svg';svg.write_text('\n'.join(line.rstrip()for line in svg.read_text().splitlines())+'\n')
fig.savefig(p/'outcomes.pdf',metadata={'CreationDate':None,'ModDate':None})
