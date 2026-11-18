"""Plot recorded coverage only; no cryptographic computations or executions."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
base=Path(__file__).resolve().parent
plt.rcParams.update({'svg.fonttype':'none','font.family':'DejaVu Sans'})
fig,axes=plt.subplots(1,2,figsize=(10,2.5),constrained_layout=True)
data=[]
for ax,u,total in zip(axes,[3,5],[18,4]):
    for offset,version,label,color in [(-.18,'04','Euclidean coefficients','#93a5ad'),(.18,'05','Eisenstein coefficients','#167961')]:
        directory=base.parent/'receipts'/('round-'+version)
        files=[f'qualify-{version}-u3-all.jsonl'] if u==3 else [f'qualify-{version}-u5-public-{i}.jsonl' for i in range(4)]
        counts={}
        for name in files:
            for line in (directory/name).read_text().splitlines():
                row=json.loads(line)
                if row['kind']=='precision_summary':
                    n=row['precision'];counts[n]=counts.get(n,0)+row['accepted']
        ns=sorted(counts)
        ax.bar([i+offset for i in range(len(ns))],[counts[n] for n in ns],width=.34,label=label,color=color)
        data.append({'u':u,'version':version,'files':files,'counts':counts})
    ax.set_xticks(range(len(ns)),labels=[str(n) for n in ns])
    ax.set_ylim(0,total+1)
    ax.set_yticks([0,6,12,18] if u==3 else [0,1,2,3,4])
    ax.set_title('Order19: all18 nonidentity inputs' if u==3 else 'Order61: four fixed public inputs',fontsize=10)
    ax.set_xlabel('Source precision N (p-adic digits)',fontsize=9)
    ax.set_ylabel('Accepted inputs (count)',fontsize=9)
    ax.spines[['top','right']].set_visible(False)
axes[0].legend(fontsize=7,loc='upper left',frameon=False)
fig.savefig(base/'round-05-coverage.svg')
(base/'round-05-coverage-data.json').write_text(json.dumps(data,indent=2)+'\n')
