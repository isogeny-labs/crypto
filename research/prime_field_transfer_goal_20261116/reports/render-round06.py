"""Plot existing recorded counts only; no cryptographic algorithms or calls."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
base=Path(__file__).resolve().parent
source=base.parent/'receipts/round-06/public-aggregate-01.json'
data=json.loads(source.read_text())
plt.rcParams.update({'svg.fonttype':'none','font.family':'DejaVu Sans'})
fig,axes=plt.subplots(1,2,figsize=(10,2.55),constrained_layout=True)
for ax,fixture in zip(axes,data['fixtures']):
    ns=[16,32,64,128]
    for i,n in enumerate(ns):
        row=fixture['by_precision'].get(str(n))
        if row:
            ax.bar(i,row['accepted'],color='#167961',width=.58)
            ax.text(i,row['accepted']+.10,str(row['accepted'])+'/4',ha='center',fontsize=9)
        else:
            ax.text(i,1.8,'No completed row\n4 timeouts',ha='center',fontsize=8,color='#9d542d')
    ax.set_xticks(range(4),labels=[str(n) for n in ns])
    ax.set_ylim(0,4.8);ax.set_yticks([0,1,2,3,4])
    ax.set_title('Order 61: four fixed public inputs' if fixture['u']==5 else 'Order 127: four fixed public inputs',fontsize=10)
    ax.set_xlabel('Source precision N (p-adic digits)',fontsize=9)
    ax.set_ylabel('Accepted completed inputs (count)',fontsize=9)
    ax.spines[['top','right']].set_visible(False)
fig.savefig(base/'round-06-coverage.svg')
(base/'round-06-coverage-data.json').write_text(json.dumps({'source':str(source.relative_to(base.parent)),
    'population':'Four fixed inputs per fixture, not random sampling or a statistical interval',
    'recorded':data},indent=2)+'\n')
