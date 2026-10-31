"""Plot the complete native enumeration; no new mathematical measurements."""
from pathlib import Path
from collections import Counter
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
h=Path(__file__).resolve().parent;data=json.loads((h/'cycle-plot-data.json').read_text())
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':11,'svg.fonttype':'none','svg.hashsalt':'koblitz-r51-cycle'})
labels=[]
for row in data['rows']:
 labels.append('$'+r'\;'.join(str(k)+(f'^{{{v}}}' if v!=1 else '') for k,v in sorted(Counter(row['cycles']).items()))+'$')
values=[row['count'] for row in data['rows']]
fig,ax=plt.subplots(figsize=(10,4.65));fig.subplots_adjust(left=.18,right=.94,bottom=.18,top=.81)
y=list(range(len(values)));bars=ax.barh(y,values,color='#2b718f',height=.65)
ax.set_yticks(y,labels);ax.invert_yaxis();ax.set_xlim(0,94);ax.set_xticks([0,14,28,42,56,70,84])
ax.set_xlabel('Matrices in SL₂(F₇) — complete count');ax.set_ylabel('Cycle type on 16 QR classes')
ax.bar_label(bars,padding=5);ax.set_axisbelow(True);ax.grid(axis='x',alpha=.2)
for side in ['top','right']:ax.spines[side].set_visible(False)
fig.suptitle('No 16-cycle in the complete QR₇ action',fontsize=16,fontweight='bold',y=.98)
fig.text(.18,.86,'336 SL₂ matrices; all 1,008 square-determinant normalizations checked.',fontsize=11)
fig.text(.18,.035,'Exact enumeration; longest cycle 14. Source: fiber51-01/native.log.',fontsize=10)
fig.savefig(h/'cycles51.svg',metadata={'Date':None});plt.close(fig)
