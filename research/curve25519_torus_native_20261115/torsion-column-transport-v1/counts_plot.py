"""Layout only: values are projected from the qualified native records."""
import csv,json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent
data=json.loads((p/'RESULTS.json').read_text())
panels=[data['toy_tagged_panel'],data['actual_tagged_panel']]
with (p/'counts.csv').open('w',newline='') as out:
    w=csv.writer(out,lineterminator='\n');w.writerow(['panel','rows','unknown_columns','rank_M','rank_M_b','kernel_vectors','informative_witnesses'])
    for name,r in zip(['toy','actual'],panels):w.writerow([name,r['rows'],r['unknown_columns'],r['rank_M'],r['rank_M_b'],r['left_kernel_vectors'],r['informative_witnesses']])
plt.rcParams.update({'svg.fonttype':'none','font.size':10})
fig,axes=plt.subplots(1,2,figsize=(7.2,2.6),layout='constrained')
for ax,keys,labels,ticks in [(axes[0],['rank_M','rank_M_b'],['rank M','rank [M | b]'],[0,8,16,24]),(axes[1],['left_kernel_vectors','informative_witnesses'],['Kernel basis','Informative'],[0,4,8,12])]:
    for offset,key,label,color in zip([-0.17,0.17],keys,labels,['#28658d','#e89947']):ax.bar([offset,1+offset],[r[key] for r in panels],0.32,label=label,color=color)
    ax.set_xticks([0,1],['Toy panel','Actual panel']);ax.set_yticks(ticks);ax.set_ylabel('Recorded count');ax.legend(frameon=False,fontsize=9,loc='upper left');ax.spines[['top','right']].set_visible(False)
axes[0].set_ylim(0,31);axes[1].set_ylim(0,16)
axes[0].set_title('Rank gain: 4→5 versus 24→24',fontsize=11)
axes[1].set_title('Kernel size does not imply recovery',fontsize=11)
for name in ['counts.pdf','counts.svg']:fig.savefig(p/name)
fig.savefig(p/'counts.png',dpi=150)
svg=p/'counts.svg';svg.write_text('\n'.join(s.rstrip() for s in svg.read_text().splitlines())+'\n')
