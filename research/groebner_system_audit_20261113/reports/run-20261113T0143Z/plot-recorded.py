"""Presentation only: plot the literal native initial-ideal Hilbert arrays.
No research counting, fitting, statistics or extrapolation is performed here.
"""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

here=Path(__file__).resolve().parent
records=[json.loads(x)for x in (here.parent.parent/'analysis/analysis.jsonl').read_text().splitlines()]
names={'edwards-homogeneous':'Edwards homogenized',
       'jv-empty-homogeneous':'JV empty homogenized',
       'jv-split-homogeneous':'JV split homogenized'}
rows=[x for x in records if x.get('replicate')==0 and x.get('fixture')in names]
(here/'plot-data.json').write_text(json.dumps(rows,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(7.0,3.4),layout='constrained')
for r,color,marker in zip(rows,['#174a7e','#b64926','#527f35'],['o','s','^']):
 values=r['hilbert_0_to_12'][:7]
 ax.plot(range(len(values)),values,label=names[r['fixture']],color=color,marker=marker,lw=1.8,ms=5)
ax.set(xlabel='Homogeneous degree',ylabel='Initial-ideal Hilbert function',
       title='Exact native Hilbert values; displayed degrees 0–6',xticks=range(7),yticks=range(5),ylim=(-.2,4.7))
ax.grid(alpha=.2);ax.legend(loc='upper right',frameon=False,fontsize=8)
for ext in ['svg','pdf']:fig.savefig(here/('hilbert.'+ext),metadata={'Title':'Exact source-system Hilbert values'})
p=here/'hilbert.svg';p.write_text('\n'.join(line.rstrip()for line in p.read_text().splitlines())+'\n')
