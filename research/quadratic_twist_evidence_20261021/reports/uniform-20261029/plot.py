"""Presentation only: plot recorded native fractions; no rank/state computation."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
data=json.loads((root/'plot-data.json').read_text())
def coordinate(x): return float(x['num'])/float(x['den'])
styles={'main':('o','#195c9e'),'twist':('s','#bf4b17'),'matched':('^','#39936c'),'pullback':('x','#8e5ab5')}
fig,axes=plt.subplots(2,2,figsize=(9,6.8),constrained_layout=True)
for ax,(p,b) in zip(axes.flat,[(43,2),(43,3),(13,2),(13,3)]):
    for role,(marker,color) in styles.items():
        records=[v for v in data['arms'] if v['p']==p and v['b']==b and v['name'].split('-')[-1].startswith(role)]
        ax.scatter([coordinate(v['uniform']['mean_rank_gain']) for v in records],
                   [coordinate(v['uniform']['mean_reduced_degree']) for v in records],
                   marker=marker,c=color,s=75,alpha=.7,label=f'{role} ({len(records)})',linewidths=1.3)
    ax.set_title(f'F{p}, base size {b}'); ax.set_xlabel('Recorded mean initial rank gain / query')
    ax.set_ylabel('Recorded mean reduced matrix degree'); ax.grid(alpha=.2)
    ax.legend(fontsize=7,loc='best')
fig.suptitle('Uniform S3: all 96 arms; coincident records overlap',fontsize=13)
for ext in ['svg','pdf']: fig.savefig(root/f'profiles.{ext}')

# Normalize renderer whitespace only; recorded data and SVG geometry are unchanged.
p=root/'profiles.svg'
p.write_text('\n'.join(line.rstrip() for line in p.read_text().splitlines())+'\n')
