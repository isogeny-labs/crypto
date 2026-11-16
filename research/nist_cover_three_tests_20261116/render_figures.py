"""Layout recorded evidence only; no cryptographic/result computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
root=Path(__file__).parent
(root/'figures').mkdir(exist_ok=True)
rows=[json.loads(s) for s in (root/'evidence/final-kernel-check/kernels.jsonl').read_text().splitlines()]
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10,'svg.fonttype':'none'})
fig,ax=plt.subplots(figsize=(8.2,3.5),layout='constrained')
x=[r['kernel'] for r in rows]
ax.bar(x,[r['point_field_degree'] for r in rows],color='#d4e5f3',label='Full point-field degree')
ax.scatter(x,[r['rational_elements'] for r in rows],color='#ad4b16',zorder=3,label='Base-field rational elements (includes identity)')
ax.set(xticks=x,xlabel='P-192 d7 kernel (RREF index)',ylabel='Recorded exact integer',ylim=(0,10),title='Eight stable kernels; individual points need degree 8')
ax.legend(loc='upper left',fontsize=8)
ax.grid(axis='y',alpha=.2)
fig.supxlabel('Source: evidence/final-kernel-check/kernels.jsonl. Producer-unverified; exact finite-module replay.',fontsize=8)
for ext in ['pdf','svg']:fig.savefig(root/f'figures/kernels.{ext}')
plt.close(fig)
fig,ax=plt.subplots(figsize=(8.2,3.4));ax.set(xlim=(0,10),ylim=(0,4));ax.axis('off')
def box(x,y,w,h,text,color):
    ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=.08',facecolor=color,edgecolor='#34475a'))
    ax.text(x+w/2,y+h/2,text,ha='center',va='center',fontsize=9)
box(.1,1.5,2.2,1.4,'NIST P-192\nE / Fp\npublic generator/order','#e4f1e8')
box(3.5,1.5,2.6,1.4,'J(H), genus 3\nmonic degree 7 model\n729 actual 3-torsion classes','#e4f1e8')
box(7.3,1.5,2.3,1.4,'A = J(H) / K\nquotient + dual\nnot constructed','#fff0d4')
ax.annotate('',xy=(3.4,2.5),xytext=(2.4,2.5),arrowprops={'arrowstyle':'->','color':'#207143'})
ax.text(2.9,2.8,'pullback',ha='center',fontsize=8)
ax.annotate('',xy=(2.4,1.85),xytext=(3.4,1.85),arrowprops={'arrowstyle':'->','color':'#207143'})
ax.text(2.9,1.5,'norm',ha='center',fontsize=8)
ax.annotate('',xy=(7.2,2.2),xytext=(6.2,2.2),arrowprops={'arrowstyle':'->','linestyle':'--','color':'#a56b12'})
ax.text(6.7,2.6,'degree 27',ha='center',fontsize=8)
ax.text(4.9,.65,'Executed: source maps, actual kernels and corruption controls\nOpen: quotient, descent, dual and projected relation solver',ha='center',fontsize=10)
ax.text(4.9,3.6,'Native development checks and the remaining construction',ha='center',fontsize=13)
fig.text(.5,.02,'Solid arrows: fixture-tested producer implementation. Dashed arrow: proposed next construction.',ha='center',fontsize=8)
for ext in ['pdf','svg']:fig.savefig(root/f'figures/route.{ext}',bbox_inches='tight')
plt.close(fig)
