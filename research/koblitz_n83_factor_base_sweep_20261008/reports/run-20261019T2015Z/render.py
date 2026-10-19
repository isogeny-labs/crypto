"""Plot frozen native natural-trial values; no solver, replay or result math."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
p=Path(__file__).resolve().parent
d=json.loads((p/'figure-data.json').read_text())
rows=d['rows']
fig,axes=plt.subplots(1,2,figsize=(11,4))
for ax,key,title in zip(axes,['natural_solver_ms','outer_wall_ms'],['Native solver interval','Supervised outer wall']):
    available=[r for r in rows if r.get(key) is not None]
    labels=[f"K{r['columns']} / {r['conflict_cap']}" for r in available]
    bars=ax.bar(labels,[r[key] for r in available],color=['#23798e' if r['columns']==2048 else '#9c661f' for r in available])
    for bar,r in zip(bars,available):
        if "CENSORED" in r["status"]:bar.set_hatch("///")
    ax.set_ylabel('Milliseconds');ax.set_title(title);ax.tick_params(axis='x',labelrotation=45)
fig.suptitle('Audited bounded observations only | n=1 admitted per K and cap; failed K4096 attempt preserved separately')
fig.tight_layout()
for ext in ['svg','png']:fig.savefig(p/f'cap-continuation.{ext}',dpi=150)
plt.close(fig)
fig,ax=plt.subplots(figsize=(10,2.8));ax.set_xlim(0,10);ax.set_ylim(0,3);ax.axis('off')
boxes=[(.1,'Same public query\na=0 N83, seed2026100801\nm5, identity mask0'),(3.5,'K2048 then K4096\n400000 conflicts / 2400 s each\nNative indexed worker'),(6.9,'Native phase/query replay\nPreserve complete raw records\nFresh S3 GET and hash checks')]
from matplotlib.patches import FancyBboxPatch
for x,label in boxes:
 ax.add_patch(FancyBboxPatch((x,.6),2.9,1.5,boxstyle='round,pad=.04',fc='#edf3f7',ec='#23798e'))
 ax.text(x+1.45,1.35,label,ha='center',va='center',fontsize=9)
ax.text(5,.12,"Fresh v10 cards reserve full core14/30; prior rounds remain immutable observations.",ha="center",fontsize=8)
ax.set_title('Frozen higher-cap continuation | prior trials are separate observations')
fig.tight_layout()
for ext in ['svg','png']:fig.savefig(p/f'execution.{ext}',dpi=150)
plt.close(fig)
lines=['#table(columns:(1.5fr,1fr,3fr,1.5fr),inset:4pt,[K / cap],[Conflicts],[Native audit],[Solver ms],']
for r in rows:
 if r.get("columns") is None:continue
 counts=r.get('terminal_counters') or {}
 ms=r.get('natural_solver_ms')
 lines.append(f"[K{r.get('columns','?')} / {r.get('conflict_cap','?')}], [{counts.get('conflicts','missing')}], [{r['status'].replace('_',' ')}], ["+('missing' if ms is None else f'{ms:.6f}')+'],')
lines.append(')')
(p/'runs.typ').write_text('\n'.join(lines)+'\n')
fig,ax=plt.subplots(figsize=(10,2.8));ax.set_xlim(0,10);ax.set_ylim(0,3);ax.axis('off')
for x,label,color in [(.1,'v10 worker-reported output\n400000 conflicts; zero yield\nFinal Docker state missing','#edf3f7'),(3.5,'Launcher inspect failed\nDocker exit status omitted\nNo terminal audit admitted','#fff0e5'),(6.9,'Assessed corrective v11 card\nRead-only inspect wait 60 s\nFresh complete audit required','#edf3f7')]:
 ax.add_patch(FancyBboxPatch((x,.6),2.9,1.5,boxstyle='round,pad=.04',fc=color,ec='#9c661f'))
 ax.text(x+1.45,1.35,label,ha='center',va='center',fontsize=9)
ax.set_title('K4096 orchestration failure and separately frozen repeat')
ax.text(5,.12,'Original failure and cost retained; no pooling or replacement of its records.',ha='center',fontsize=8)
fig.tight_layout()
for ext in ['svg','png']:fig.savefig(p/f'inspection-failure.{ext}',dpi=150)
plt.close(fig)
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'),font_paths=[str(Path(matplotlib.__file__).parent/'mpl-data/fonts/ttf')])
