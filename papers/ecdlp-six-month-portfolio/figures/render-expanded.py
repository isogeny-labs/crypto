"""Plot copied historical data only; no solver, statistical fit or replay."""
from pathlib import Path
import json
import os
import tempfile
os.environ.setdefault('MPLCONFIGDIR', str(Path(tempfile.gettempdir())/'ecdlp-paper-matplotlib'))
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np

ROOT = Path(__file__).resolve().parent
d = json.loads((ROOT/'recorded-data.json').read_text())
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,
                     'axes.spines.top':False,'axes.spines.right':False,
                     'axes.titlesize':10,'svg.fonttype':'none','pdf.fonttype':42})
def save(fig, stem):
    for extension in ['pdf','svg','png']:
        fig.savefig(ROOT/(stem+'.'+extension), bbox_inches='tight', dpi=160)
    svg = ROOT/(stem+'.svg')
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
    plt.close(fig)

c = d['conductor_controls']
fig = plt.figure(figsize=(7.6,4.5), layout='constrained')
grid = fig.add_gridspec(2,2,width_ratios=[1.1,1],height_ratios=[1,1.7])
axes = [fig.add_subplot(grid[:,0]),fig.add_subplot(grid[0,1]),fig.add_subplot(grid[1,1])]
panels = [('replication','Degree 37/41: no detected shift','#278d8e'),
          ('matched51','Degree 51: matched action','#204b70'),
          ('deep51_diagnostic','Deeper degree 51: inference withheld','#8b9299')]
for ax,(key,title,color) in zip(axes,panels):
    rows = c[key]
    mid = np.array([r['effect'] for r in rows])
    low = np.array([r['low'] for r in rows]); high = np.array([r['high'] for r in rows])
    y = np.arange(len(rows))
    ax.errorbar(mid,y,xerr=[mid-low,high-mid],fmt='o',capsize=3,color=color,markersize=4)
    ax.axvline(0,color='black',lw=.8,ls='--')
    ax.set(yticks=y,yticklabels=[r['label'] for r in rows],title=title,
           xlabel='log2 walk-count contrast',xlim=(-.105,.105))
    ax.invert_yaxis();ax.grid(axis='x',alpha=.12)
    ax.tick_params(labelsize=8)
axes[1].set_yticklabels(['Floor / crater'])
axes[2].set_yticklabels(['A / crater','P / crater','Bottom / crater'])
save(fig,'conductor-controls')

v = d['volcano_v7']; coverage = v['campaigns'][0]['coverage']
graphs = ['mixed','three','two']
audited = [sum(r['vertices'] for r in coverage if r['graph']==g and r['state']=='audited') for g in graphs]
totals = [sum(r['vertices'] for r in coverage if r['graph']==g) for g in graphs]
fig,axs = plt.subplots(1,2,figsize=(8.1,3.1),layout='constrained')
axs[0].barh(graphs,audited,color='#278d8e',label='Audited')
axs[0].barh(graphs,np.array(totals)-audited,left=audited,color='#dce4e8',label='Remaining')
for i,(a,t) in enumerate(zip(audited,totals)):
    axs[0].text(t+100,i,f'{a:,} / {t:,}',va='center',fontsize=8)
axs[0].set(xlim=(0,14200),xlabel='Vertices',title='Later frozen checkpoint: 7,937 vertices')
axs[0].legend(frameon=False,fontsize=8,loc='lower right',bbox_to_anchor=(1,-.30),ncol=2)
labels = ['Verified','Exhausted','Timeout']
values = [v['outcome_totals'][k] for k in ['verified','exhausted','timeout']]
axs[1].barh(labels,values,color=['#278d8e','#ca8841','#a35464'])
for i,value in enumerate(values):
    axs[1].text(value+1200,i,f'{value:,}',va='center',fontsize=8)
axs[1].set(xlim=(0,105000),xlabel='Accepted attempt records',title='126,992 attempts across repeated arms')
save(fig,'volcano-latest')
print('Rendered two additional figure sets from copied recorded values.')
