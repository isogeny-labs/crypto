"""Render only recorded kernel-proved finite counts."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
here=Path(__file__).resolve().parent
rows=json.loads((here/'proof-values.json').read_text())['groups']
fig,axes=plt.subplots(1,2,figsize=(7,2.7),sharey=True)
for ax,domain in zip(axes,['Including identity','Nonzero only']):
    rs=[r for r in rows if r['domain']==domain]
    smooth=[r['smooth'] for r in rs]
    rest=[r['total']-r['smooth'] for r in rs]
    ax.bar(range(2),smooth,color='#285d8e',label='Split u')
    ax.bar(range(2),rest,bottom=smooth,color='#c8cdd1',label='Nonsplit u')
    for i,r in enumerate(rs):
        ax.text(i,r['total']+.25,str(r['smooth'])+'/'+str(r['total']),ha='center',fontsize=10)
    ax.set_xticks(range(2),['Ambient','Order-5 subgroup'])
    ax.set_title(domain,fontsize=10)
    ax.set_ylim(0,12)
    ax.spines[['top','right']].set_visible(False)
axes[0].set_ylabel('Actual polynomial pairs')
axes[0].legend(frameon=False,fontsize=8)
fig.tight_layout()
for ext in ['svg','pdf','png']:
    fig.savefig(here/('counts.'+ext),dpi=150)
p=here/'counts.svg'
p.write_text('\n'.join(s.rstrip() for s in p.read_text().splitlines())+'\n')
