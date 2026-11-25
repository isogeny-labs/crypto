"""Layout of native-recorded bounds only; no research arithmetic."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
matplotlib.rcParams['svg.fonttype']='none'
matplotlib.rcParams['pdf.fonttype']=42
root=Path(__file__).resolve().parent
with (root/'evidence/constraints.tsv').open() as f:
    rows=[r for r in csv.DictReader(f,delimiter='\t') if r['kind']=='gap' and r['bits']=='239']
fig,ax=plt.subplots(figsize=(7.4,2.5))
for y,r,color in zip([1,0],rows,['#245a81','#b05520']):
    low=int(r['min_extra_seed_blocks']); high=int(r['max_extra_seed_blocks'])
    ax.plot([low,high],[y,y],color=color,lw=8,solid_capstyle='butt')
    ax.text(low,y+.12,str(low),ha='center',fontsize=10)
    ax.text(high,y+.12,str(high),ha='center',fontsize=10)
ax.set_yticks([1,0],['3 point attempts\nafter index2185','2 point attempts\nafter index2320'])
ax.set_xlabel('Required extra20-call seed blocks q (exact integer constraints)')
ax.set_xlim(-30,790);ax.set_ylim(-.35,1.5)
ax.set_title('More point attempts would require fewer RNG calls')
ax.grid(axis='x',alpha=.2);ax.spines[['top','right','left']].set_visible(False)
fig.text(.5,.015,'Two recorded gaps in the239bit run; no sampling interval. Producer-unverified.',ha='center',fontsize=8)
fig.tight_layout(rect=[0,.06,1,1])
fig.savefig(root/'point-cost.svg')
fig.savefig(root/'point-cost.pdf')
