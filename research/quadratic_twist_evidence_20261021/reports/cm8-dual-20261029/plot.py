# Presentation of recorded request counts only; no cryptographic computations.
from pathlib import Path
import os,json
os.environ.setdefault('MPLCONFIGDIR','/tmp/twist-dual-mpl')
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent
v=json.loads((p/'plot-data.json').read_text());rows=v['pullbacks'];x=list(range(len(rows)))
fig,axes=plt.subplots(1,3,figsize=(10,3.6),constrained_layout=True)
for ax,key,title in zip(axes,['scalar','phi','dual'],['Scalar requests','Phi requests','D requests']):
 for shift,route,color in [(-.23,'reference_requests','#27649b'),(0,'cached_requests','#55986c'),(.23,'candidate_requests','#c45d2c')]:
  ax.bar([i+shift for i in x],[z[route][key] for z in rows],.23,label=route.split('_')[0],color=color)
 ax.set_xticks(x,[str(z['policy'])+':'+str(z['column']) for z in rows]);ax.set_title(title);ax.set_xlabel('Policy : base column');ax.set_ylabel('Recorded kernel requests / query');ax.grid(axis='y',alpha=.15)
axes[0].legend(fontsize=7)
fig.suptitle('All six public-base pullbacks; cache construction and common boundary separate')
for e in ['svg','pdf']:fig.savefig(p/f'requests.{e}')
f=p/'requests.svg';f.write_text('\n'.join(t.rstrip() for t in f.read_text().splitlines())+'\n')
