"""Layout only: recorded native Rust summary/profile values; no research computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter
import typst
P=Path(__file__).resolve().parent
plt.rcParams.update({'font.size':10,'svg.fonttype':'none','axes.spines.top':False,'axes.spines.right':False})
def load(name): return json.loads((P/'data'/name).read_text())
def save(fig,name):
    fig.savefig(P/(name+'.svg'),bbox_inches='tight')
    fig.savefig(P/(name+'.png'),dpi=150,bbox_inches='tight')
    plt.close(fig)
rows=load('summary.json')['rows']
fig,ax=plt.subplots(figsize=(8,3.5))
ax.bar([-.18,.82],[r['natural_solver_ms'] for r in rows],width=.34,label='Native solver wall',color='#176b93')
ax.bar([.18,1.18],[r['outer_wall_ms'] for r in rows],width=.34,label='Supervised outer wall',color='#59a9a0')
ax.set_xticks([0,1],['K2048-v20','K4096-v20'])
ax.set_ylabel('Recorded wall time (ms)'); ax.ticklabel_format(axis='y',style='plain'); ax.legend(loc='lower right')
ax.set_title('1M conflicts, zero models / relations / rank in each arm')
fig.text(.12,-.03,'n=1 per base; descriptive clocks, no timing ranking or uncertainty estimate.',fontsize=9)
save(fig,'completed-clocks')
fig,ax=plt.subplots(figsize=(8,4.2))
for name,label,color,style,key in [
 ('k4096-v16.memory-validation-v18.json','K4096-v16: rejected swap profile','#999999','--','observations'),
 ('k2048-v16.memory-validation-v18.json','K2048-v16: interrupted prefix','#ba9e60',':','samples'),
 ('k2048-v19.memory-validation.json','K2048-v19: stopped on swap drift','#b55151','--','observations'),
 ('k2048-v20.memory-summary.json','K2048-v20: compliant samples','#176b93','-','samples'),
 ('k4096-v20.memory-summary.json','K4096-v20: compliant samples','#228476','-','samples')]:
    data=load(name)[key]
    ax.plot([s['elapsed_ms'] for s in data],[s['current_bytes'] for s in data],label=label,color=color,linestyle=style)
    if key=='observations':
        bad=[s for s in data if s['swap_max_raw']!='0']
        ax.scatter([s['elapsed_ms'] for s in bad],[s['current_bytes'] for s in bad],color=color,s=9,marker='x')
ax.axhline(8589934592,color='#343434',linestyle=':',label='8 GiB hard limit')
ax.set_xticks([0,600000,1200000,1800000,2400000,3000000,3600000])
ax.set_yticks([0,2147483648,4294967296,6442450944,8589934592])
ax.xaxis.set_major_formatter(FuncFormatter(lambda x,_:f'{x/60000:g}'))
ax.yaxis.set_major_formatter(FuncFormatter(lambda y,_:f'{y/1073741824:g}'))
ax.set_xlabel('Profile elapsed time (minutes; display conversion from native ms)')
ax.set_ylabel('Cgroup memory.current (GiB; from bytes)')
ax.set_title('Container memory observations; earlier attempts remain unadmitted')
ax.legend(fontsize=8,loc='upper left')
fig.text(.12,-.03,'x = sampled unlimited swap; sampled charges are not full-run peak or worker RSS.',fontsize=8)
save(fig,'memory-profiles')
fig,ax=plt.subplots(figsize=(8,4.3)); ax.axis('off')
items=[('K2048-v15','Preflight refusal; no worker', '#eeeeee'),('K4096-v16','1M terminal; memory profile rejected','#f6dddd'),('K2048-v16','Host interruption; terminal and cleanup proof missing','#f6dddd'),('K2048-v18','Lock-open refusal; no worker','#eeeeee'),('K2048-v19','Stopped at nonterminal 420K; swap drift','#f6dddd'),('K2048-v20','1M terminal / replay + profile PASS','#dcefe8'),('K4096-v20','1M terminal / replay + profile PASS','#dcefe8')]
for y,(name,status,color) in zip([.90,.77,.64,.51,.38,.25,.12],items):
    ax.text(.01,y,name,weight='bold',va='center')
    ax.text(.25,y,status,va='center',bbox={'boxstyle':'round,pad=.35','facecolor':color,'edgecolor':'none'})
ax.set_title('Seven actual attempts; five earlier outcomes preserved',loc='left')
fig.text(.12,-.01,'K4096-v18/v19 were superseded without execution. No timing pooling across attempts.',fontsize=9)
save(fig,'attempt-custody')
typst.compile(str(P/'report.typ'),output=str(P/'report.pdf'))
