# Rendering recorded native analysis only; no cryptographic computation or statistics.
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parents[2]
main=json.loads((root/'scale-20261115T2053Z/summary.json').read_text())
repair=json.loads((root/'scale-20261115T2053Z/repair-summary.json').read_text())
out=Path(__file__).parent
fields=['word17','word31','word62','p224']
colors={'vieta':'#c34d28','batched':'#216b9a','scalar':'#818181','stream-vieta':'#c34d28','mitm':'#216b9a','nested-batched':'#5d8d40'}
fig,axs=plt.subplots(2,2,figsize=(10,7),constrained_layout=True)
for ax,field in zip(axs.flat,fields):
 for arm in ['vieta','batched','scalar']:
  rows=sorted([r for r in main['groups'] if r['panel']=='two' and r['kind']=='query' and r['curve']==field and r['arm']==arm and r['population']=='conditioned-two-sum' and r['muls_median'] is not None],key=lambda r:r['m'])
  ax.plot([r['m'] for r in rows],[int(r['muls_median']) for r in rows],'.-',label=arm,color=colors[arm])
 ax.set(xscale='log',yscale='log',title=field,xlabel='Coset size m',ylabel='Field multiplications / query')
 ax.grid(alpha=.2);ax.legend(fontsize=8)
 if field=='p224':ax.text(8192,.78,'12 Vieta deadlines\n(2 warmup, 10 retained)',ha='right',transform=ax.get_xaxis_transform(),fontsize=8,color=colors['vieta'])
fig.suptitle('Native two-term queries: conditioned sums, median counts\nFive retained targets per population; primitive inverse/sqrt work included')
fig.savefig(out/'two-counts.svg');fig.savefig(out/'two-counts.png',dpi=150);plt.close(fig)
fig,ax=plt.subplots(figsize=(9,4),constrained_layout=True)
for field in fields:
 rows=sorted([r for r in main['comparisons'] if r['panel']=='two' and r['curve']==field and r['arm']=='vieta' and r['population']=='conditioned-two-sum'],key=lambda r:r['m'])
 ax.plot([r['m'] for r in rows],[r['cold_muls_ratio_median'] for r in rows],'.-',label=field)
ax.axhline(1,color='black',linestyle='--',linewidth=.8)
ax.set(xscale='log',yscale='log',xlabel='Coset size m',ylabel='Cold multiplication ratio: Vieta / batched',title='Cold descriptor + extraction versus descriptor + base setup + lookup\nGeneric counted reference; not the optimized repository backend')
ax.grid(alpha=.2);ax.legend();fig.savefig(out/'cold-ratios.svg');fig.savefig(out/'cold-ratios.png',dpi=150);plt.close(fig)
fig,axs=plt.subplots(2,2,figsize=(10,7),constrained_layout=True)
for ax,field in zip(axs.flat,fields):
 for arm in ['stream-vieta','mitm','nested-batched']:
  rows=[r for r in main['groups'] if r['panel']=='three' and r['kind']=='three-query' and r['curve']==field and r['arm']==arm and r['population']=='conditioned-three-sum' and not(field=='p224' and r['m']==128)]
  rows += [r for r in repair['groups'] if r['kind']=='three-query' and r['curve']==field and r['arm']==arm and r['population']=='conditioned-three-sum']
  rows=sorted(rows,key=lambda r:r['m'])
  ax.plot([r['m'] for r in rows],[int(r['muls_median']) for r in rows],'.-',label=arm,color=colors[arm])
 ax.set(xscale='log',yscale='log',title=field,xlabel='Coset size m',ylabel='Field multiplications / query');ax.grid(alpha=.2);ax.legend(fontsize=8)
fig.suptitle('Three-term query counts; table construction charged separately\nP-224/m128 uses the repaired fixture-clock epoch, not pooled with its incomplete predecessor')
fig.savefig(out/'three-counts.svg');fig.savefig(out/'three-counts.png',dpi=150);plt.close(fig)
