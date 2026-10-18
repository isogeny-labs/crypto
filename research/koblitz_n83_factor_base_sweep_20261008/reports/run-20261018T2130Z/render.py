"""Render recorded values only; no research computation or experiment execution."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
p=Path(__file__).resolve().parent
d=json.loads((p/'figure-data.json').read_text())
fig,axes=plt.subplots(1,2,figsize=(10,4))
for ax,key,title,cap in zip(axes,['sat_variables','domain_clauses'],['Installed SAT variables (count)','Domain clauses (count)'],[150000,2000000]):
    bars=ax.bar(['K2048','K4096'],[a[key] for a in d['capacity']],color=['#276c8e','#d47a28'])
    ax.axhline(cap,color='#a43333',linestyle='--',label=f'Frozen cap {cap:,}')
    ax.bar_label(bars,labels=[f"{a[key]:,}" for a in d['capacity']],padding=4)
    ax.set_ylim(0,cap*1.22); ax.set_title(title); ax.legend(fontsize=8); ax.ticklabel_format(axis='y',style='plain')
fig.suptitle('N83 guarded model construction | n=1 per arm; exact recorded counts')
fig.text(.5,.01,d['run']+' | no uncertainty interval recorded; not a speed comparison',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.04,1,.92))
for ext in ['svg','png']: fig.savefig(p/f'capacity.{ext}',dpi=150)
plt.close(fig)
fig,ax=plt.subplots(figsize=(10,4));ax.set_xlim(0,10);ax.set_ylim(0,4);ax.axis('off')
boxes=[(.15,1.3,2,1.3,'Public fixtures\nNative Rust worker\nHistorical supervisor'),(2.7,1.3,2,1.3,'K2048 / K4096\nModel construction\nPASS / PASS'),(5.3,2.15,4.3,1.35,'Long searches: 600 s cap each\nBoth wall-censored\nTerminal counters unknown'),(5.3,.25,4.3,1.35,'Short controls: 180 s cap each\nK2048: 1000 conflicts; 0 relations\nK4096: wall-censored; counters unknown')]
for x,y,w,h,label in boxes:
    ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=.08',fc='#edf4f7',ec='#276c8e'))
    ax.text(x+w/2,y+h/2,label,ha='center',va='center',fontsize=10)
for xy,xytext in [((2.6,1.95),(2.25,1.95)),((5.2,2.8),(4.8,2.2)),((5.2,.9),(4.8,1.7))]:ax.annotate('',xy=xy,xytext=xytext,arrowprops={'arrowstyle':'->'})
ax.set_title('Observed execution and terminal evidence | one trial per arm')
fig.text(.5,.01,'Identical fixture files; paired terminal query identity unverified for K4096.',ha='center',fontsize=9)
fig.tight_layout()
for ext in ['svg','png']:fig.savefig(p/f'execution.{ext}',dpi=150)
plt.close(fig)
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'),font_paths=[str(Path(matplotlib.__file__).parent/'mpl-data/fonts/ttf')])
