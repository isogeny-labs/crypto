"""Plot recorded native storage quantities. No experiment/result computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter
import typst
P=Path(__file__).resolve().parent
plt.rcParams.update({'font.size':10,'svg.fonttype':'none','axes.spines.top':False,'axes.spines.right':False})
profiles=[json.loads((P/'data'/f'{arm}.storage.json').read_text()) for arm in ('k2048-p22','k4096-p22')]
labels=['K2048-p22','K4096-p22']
def save(fig,name):
    fig.savefig(P/f'{name}.svg',bbox_inches='tight')
    fig.savefig(P/f'{name}.png',dpi=150,bbox_inches='tight')
    plt.close(fig)
fig,axes=plt.subplots(2,1,figsize=(8.2,7.2),sharex=True)
for ax,p,label in zip(axes,profiles,labels):
    rows=p['snapshots']; x=[r['conflicts'] for r in rows]
    for field,title,color,style in [('arena_capacity_bytes','Arena requested capacity','#b94a48','--'),('arena_used_bytes','Arena used words × 4','#df9b39','-'),('worker_rss_bytes','Actual worker RSS','#246c9a','-'),('allocator_in_use_and_mmap_bytes','Process allocator in-use + mmap','#718b49',':')]:
        ax.plot(x,[r[field] for r in rows],label=title,color=color,linestyle=style)
    ax.set_title(label+' · n=1; native snapshots every 10K conflicts')
    ax.set_ylabel('Bytes (decimal GB ticks)')
    ax.yaxis.set_major_formatter(FuncFormatter(lambda v,pos:f'{v/1e9:g}'))
    ax.grid(alpha=.2)
axes[0].legend(fontsize=8,loc='upper left')
axes[-1].set_xlabel('Cumulative conflicts; 450K diagnostic cap')
fig.tight_layout(); save(fig,'storage-growth')
fig,axes=plt.subplots(2,1,figsize=(8.2,7),sharex=True)
for ax,p,label in zip(axes,profiles,labels):
    rows=p['snapshots']; x=[r['conflicts'] for r in rows]
    ax.stackplot(x,*[[r[field] for r in rows] for field in ['original_clause_bytes','active_learnt_clause_bytes','detached_learnt_clause_bytes']],labels=['Original clauses','Active learned clauses','Detached learned clauses'],colors=['#246c9a','#5a9b78','#b94a48'])
    ax.set_title(label+' · logical arena storage; capacity slack excluded')
    ax.set_ylabel('Bytes (decimal GB ticks)')
    ax.yaxis.set_major_formatter(FuncFormatter(lambda v,pos:f'{v/1e9:g}'))
    ax.grid(alpha=.2)
axes[0].legend(fontsize=8,loc='upper left')
axes[-1].set_xlabel('Cumulative conflicts; 450K diagnostic cap')
fig.tight_layout();save(fig,'arena-partition')
fig,axes=plt.subplots(1,2,figsize=(9,4.1))
for ax,p,label in zip(axes,profiles,labels):
    rows=p['terminal_components_descending'][:5]
    ax.barh([r[0] for r in rows],[r[1] for r in rows],color='#246c9a')
    ax.invert_yaxis();ax.set_title(label+' · terminal top five owners')
    ax.xaxis.set_major_formatter(FuncFormatter(lambda v,pos:f'{v/1e9:g}'))
    ax.set_xlabel('Requested capacity (decimal GB)');ax.grid(axis='x',alpha=.2)
fig.tight_layout();save(fig,'terminal-owners')
typst.compile(str(P/'report.typ'),output=str(P/'report.pdf'))
