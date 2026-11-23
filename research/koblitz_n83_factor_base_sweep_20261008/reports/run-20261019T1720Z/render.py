"""Render frozen native observations only; no research calculations."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
p=Path(__file__).resolve().parent
d=json.loads((p/'figure-data.json').read_text())
rows=d['rows']
verified=[r for r in rows if r['status']=='PASS_KNOWN_WITNESS_MODEL_AND_INDEPENDENT_GROUP_SUM']
fig,axes=plt.subplots(1,3,figsize=(12,4))
for ax,arm,key,title in zip(axes,['A','B','E'],['solver_ms','solver_ms','external_algorithm_cpu_ms'],['Dense native solve wall','Indexed native solve wall','CMS solve/simplify CPU']):
    rs=[r for r in verified if r['arm']==arm]
    ax.bar([r['id'] for r in rs],[r[key] for r in rs],color={'A':'#a6611a','B':'#23798e','E':'#6b558c'}[arm])
    ax.set_ylabel('Milliseconds');ax.set_title(title+' (n=7)');ax.tick_params(axis='x',labelrotation=70)
fig.suptitle('N83 fixed-coordinate, free-selector control | recorded clocks differ by panel')
fig.tight_layout()
for ext in ['svg','png']:fig.savefig(p/f'solver-times.{ext}',dpi=150)
plt.close(fig)
fig,ax=plt.subplots(figsize=(8,3))
rs=[next(r for r in verified if r['arm']==arm) for arm in ['A','B']]
ax.bar(['Dense word updates','Indexed word updates'],[r['actual_word_updates'] for r in rs],color=['#a6611a','#23798e'])
ax.set_yscale('log');ax.set_ylabel('64-bit word XOR updates (log)');ax.set_title('Same 3,493,683 row operations; seven identical counts per native arm')
fig.tight_layout()
for ext in ['svg','png']:fig.savefig(p/f'word-updates.{ext}',dpi=150)
plt.close(fig)
fig,ax=plt.subplots(figsize=(9,3.2))
rs=[r for r in rows if r.get('natural_solver_ms') is not None]
ax.bar([r['id'] for r in rs],[r['natural_solver_ms'] for r in rs],color='#23798e')
ax.set_ylabel('Native solver interval, milliseconds');ax.set_title('One retained natural query per K and cap; zero-relation outcomes are bounded')
ax.tick_params(axis='x',labelrotation=15);fig.tight_layout()
for ext in ['svg','png']:fig.savefig(p/f'natural-trials.{ext}',dpi=150)
plt.close(fig)
fig,ax=plt.subplots(figsize=(10,2.5));ax.set_xlim(0,10);ax.set_ylim(0,3);ax.axis('off')
for x,label in [(0.1,'Original 180 s profile\nWall censored; counters null'),(3.5,'One-conflict profile\n971,224,218 ns in XOR'),(6.9,'External attempts\n5K unknown; v3 preflight refused\nv4 SAT models independently verified')]:
    ax.add_patch(FancyBboxPatch((x,.6),2.9,1.55,boxstyle='round,pad=.04',fc='#edf3f7',ec='#23798e'))
    ax.text(x+1.45,1.375,label,ha='center',va='center',fontsize=9)
ax.set_title('Preserved diagnostics and failures | separate frozen protocols')
fig.tight_layout()
for ext in ['svg','png']:fig.savefig(p/f'diagnostics.{ext}',dpi=150)
plt.close(fig)
lines=['#table(columns: (1.7fr, 3fr, 1.2fr), inset: 4pt, [Run], [Recorded disposition], [Solver ms],']
for r in rows:
    ms=r.get('natural_solver_ms') or r.get('solver_ms') or r.get('external_algorithm_cpu_ms')
    lines.append('['+r['id']+'], ['+r['status'].replace('_',' ')+'], ['+('missing' if ms is None else f'{ms:.3f}')+'],')
lines.append(')')
(p/'runs.typ').write_text('\n'.join(lines)+'\n')
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'),font_paths=[str(Path(matplotlib.__file__).parent/'mpl-data/fonts/ttf')])
