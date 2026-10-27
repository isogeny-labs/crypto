"""Render existing upstream values only; no cryptographic computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
p=Path(__file__).resolve().parent
d=json.loads((p/'figure-data.json').read_text())
fig,ax=plt.subplots(figsize=(10,4))
a=d['phases']
ax.barh([v['phase'].replace('_',' ') for v in a],[v['elapsed_ms'] for v in a],color='#6d6597')
ax.invert_yaxis();ax.set_xscale('log');ax.set_xlabel('Reported elapsed ms from decomposition entry (log axis)')
ax.set_title('Separate c7i.xlarge K4096 trace | n=1; no uncertainty interval')
fig.text(.5,.01,d['run']+' | source8856f2eb; not a matched host comparison',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.05,1,1))
for e in ['svg','png']:fig.savefig(p/f'phases.{e}',dpi=150)
plt.close(fig)
fig,ax=plt.subplots(figsize=(10,2.4));ax.set_xlim(0,10);ax.set_ylim(0,3);ax.axis('off')
for x,w,t in [(.15,2.8,'Frozen upstream inputs/source\nNative Rust worker\nBash + Python checks'),(3.6,2.8,'c7i.xlarge, Docker 1 CPU\n4 GiB, no swap/network\nOne natural K4096 trial'),(7.05,2.8,'1000 conflicts, unknown\n0 models / relations / rank\nSeparate host and source')]:
    ax.add_patch(FancyBboxPatch((x,1),w,1.2,boxstyle='round,pad=.04',fc='#f0edf7',ec='#6d6597'))
    ax.text(x+w/2,1.6,t,ha='center',va='center',fontsize=10)
for x in [3,6.45]:ax.annotate('',xy=(x+.5,1.6),xytext=(x,1.6),arrowprops={'arrowstyle':'->'})
ax.set_title('Preserved peer receipt; no new run or audit recertification')
fig.tight_layout()
for e in ['svg','png']:fig.savefig(p/f'execution.{e}',dpi=150)
plt.close(fig)
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'),font_paths=[str(Path(matplotlib.__file__).parent/'mpl-data/fonts/ttf')])
