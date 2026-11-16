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
# Recorded exhaustive toy correctness results; no curve arithmetic or result generation.
recoveries=[json.loads((root/f'evidence/toy-v3-{p}/result.json').read_text()) for p in (109,1009)]
fig,ax=plt.subplots(figsize=(8.2,3.2),layout='constrained')
labels=[f'F{p}: subgroup order {r["prime_subgroup_order"]}' for p,r in zip((109,1009),recoveries)]
values=[r['source_verified_targets'] for r in recoveries]
ax.barh(labels,values,color='#207143',height=.48)
for i,r in enumerate(recoveries):ax.text(r['source_verified_targets']+1,i,f'{r["source_verified_targets"]} / {r["prime_subgroup_order"]}',va='center')
ax.set(xlabel='Recovered scalars checked on the source curve',xlim=(0,125),title='Working IC prototype: every tiny-subgroup target recovered')
ax.invert_yaxis();ax.grid(axis='x',alpha=.18);ax.set_axisbelow(True)
fig.supxlabel('Two fixed public correctness fixtures; complete nonzero projected bases.\nSource: evidence/toy-v3-{109,1009}/result.json. Producer-unverified; no timing or P-192 result.',fontsize=8)
for ext in ['pdf','svg']:fig.savefig(root/f'figures/recovery.{ext}')
plt.close(fig)
# Updated canonical route: executable coset image; geometric model still open.
fig,ax=plt.subplots(figsize=(10,4));ax.set(xlim=(0,12.1),ylim=(0,5));ax.axis('off')
def stage(x,y,w,h,text,color):
    ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=.07',facecolor=color,edgecolor='#34475a'))
    ax.text(x+w/2,y+h/2,text,ha='center',va='center',fontsize=9)
stage(.15,2.1,1.7,1.15,'Source E\npublic points\nprime subgroup','#e4f1e8')
stage(2.75,2.1,2.0,1.15,'J(H), genus 3\nexplicit divisors\nactual 3-torsion','#e4f1e8')
stage(5.65,2.1,2.0,1.15,'Kernel cosets\nquotient image\nexecuted dual','#e4f1e8')
stage(8.65,2.1,3.0,1.15,'Projected principal-function IC\nsmall controls: r = 11, 109\nscalar recovery + source check','#e4f1e8')
for start,end,label in [(1.95,2.65,'pullback'),(4.85,5.55,'degree 27'),(7.75,8.55,'toy solver')]:
    ax.annotate('',xy=(end,2.65),xytext=(start,2.65),arrowprops={'arrowstyle':'->','color':'#207143'})
    ax.text((start+end)/2,3.0,label,ha='center',fontsize=8)
stage(5.4,3.95,3.5,.55,'Plane-quartic geometry: open','#fff0d4')
ax.annotate('',xy=(6.65,3.85),xytext=(6.65,3.35),arrowprops={'arrowstyle':'->','linestyle':'--','color':'#a56b12'})
ax.annotate('',xy=(1.05,1.95),xytext=(6.65,1.95),arrowprops={'arrowstyle':'->','color':'#207143','connectionstyle':'arc3,rad=-.22'})
ax.text(3.85,.65,'Checked transfer composition TF = [6]\nInverse 6 recovers the source point',ha='center',fontsize=10)
ax.text(6.0,4.85,'Executed development pipeline and remaining target geometry',ha='center',fontsize=13)
fig.text(.5,.035,'P-192: transfer checks on all 8 kernels. Small controls: complete scalar recovery.\nProducer-unverified; no P-192 DLP recovery, reconstructed quartic or measured speed result.',ha='center',fontsize=8)
for ext in ['pdf','svg']:fig.savefig(root/f'figures/route.{ext}',bbox_inches='tight')
plt.close(fig)
# Normalize rendering whitespace; this does not change plotted observations.
for path in (root/'figures').glob('*.svg'):
    path.write_text('\n'.join(line.rstrip() for line in path.read_text().splitlines())+'\n')
