"""Plot recorded JSON values and render the report; no research execution."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
p=Path(__file__).resolve().parent
d=json.loads((p/'figure-data.json').read_text())
fig,axes=plt.subplots(1,2,figsize=(10,4.4))
for ax,arm in zip(axes,d['arms']):
    phases=arm.get('phases',[])
    ax.barh([v['phase'].replace('_',' ') for v in phases],[v['elapsed_ms'] for v in phases],color='#286d8a')
    ax.invert_yaxis();ax.set_xscale('log');ax.set_xlabel('Elapsed ms from closure entry (log axis)')
    ax.set_title('K'+str(arm['columns'])+' | n=1')
    ax.tick_params(axis='y',labelsize=8)
fig.suptitle('Recorded phase checkpoints | native corrected v3 arms')
fig.text(.5,.01,'solver enter includes preprocessing; absence of a return leaves final counters unknown',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.05,1,.92))
for ext in ['svg','png']:fig.savefig(p/f'phases.{ext}',dpi=150)
plt.close(fig)
fig,ax=plt.subplots(figsize=(10,3.7))
labels=[a['label'] for a in d['outer']]
values=[a['outer_wall_ms'] for a in d['outer']]
colors=['#c17d34' if a['wall_censored'] else '#286d8a' for a in d['outer']]
bars=ax.barh(labels,values,color=colors);ax.invert_yaxis()
ax.bar_label(bars,labels=[str(a['outer_wall_ms'])+' ms'+(' [censored]' if a['wall_censored'] else '') for a in d['outer']],padding=4,fontsize=8)
ax.set_xlim(0,max(values)*1.65);ax.set_xlabel('Observed outer wall (ms), L0 diagnostic')
ax.set_title('Distinct workloads and launch failures | n=1 each; no uncertainty recorded')
fig.text(.5,.01,'Different controls and failed launches are not performance comparisons; tests/builds excluded',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.05,1,1))
for ext in ['svg','png']:fig.savefig(p/f'outer-wall.{ext}',dpi=150)
plt.close(fig)
fig,ax=plt.subplots(figsize=(10,2.5));ax.set_xlim(0,10);ax.set_ylim(0,4);ax.axis('off')
boxes=[(.1,1.1,2.05,1.5,'Exact retained bases\npublic_x_hash\nK2048 / K4096'),(2.65,1.1,2.1,1.5,'Native preflight\nOriginal-group query\nPhase checkpoints'),(5.25,1.1,2.1,1.5,'Unpinned natural SAT\n1000 conflicts\n600 s wall'),(7.85,1.1,2.05,1.5,'Terminal receipt\nor wall censoring\nNo cold winner')]
for x,y,w,h,label in boxes:
    ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=.04',fc='#edf4f7',ec='#286d8a'))
    ax.text(x+w/2,y+h/2,label,ha='center',va='center',fontsize=9)
for x in [2.2,4.8,7.4]:ax.annotate('',xy=(x+.4,1.85),xytext=(x,1.85),arrowprops={'arrowstyle':'->'})
ax.set_title('Corrected v3 execution; earlier v1/v2 outcomes preserved separately')
fig.text(.5,.08,'Fully specified selector witness checks encoding consistency; free-selector N83 control remains censored',ha='center',fontsize=8)
fig.tight_layout()
for ext in ['svg','png']:fig.savefig(p/f'execution.{ext}',dpi=150)
plt.close(fig)
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'),font_paths=[str(Path(matplotlib.__file__).parent/'mpl-data/fonts/ttf')])
