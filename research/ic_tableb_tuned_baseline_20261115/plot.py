import json, matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt, numpy as np
rows=[x for x in json.load(open('table.json')) if x['w']=='0']
lab=[x['slug'].split('-')[1] for x in rows]
series=[('Table B recorded IC/rho (other harness)',[x['rec_ratio'] for x in rows],'#999999','//'),
 ('inc-tb / rho-neg (in-session, Table B recipe base)',[x['inc_tb']/x['rho'] for x in rows],'#0072B2',None),
 ('best tuned incumbent / rho-neg',[x['best']/x['rho'] for x in rows],'#E69F00',None),
 ('ksum5 / rho-neg',[x['ksum5']/x['rho'] for x in rows],'#009E73',None)]
fig,ax=plt.subplots(figsize=(8,4),dpi=150); n=len(series); wd=0.2; xs=np.arange(len(rows))
for i,(nm,v,c,h) in enumerate(series):
    ax.bar(xs+(i-1.5)*wd,v,wd,label=nm,color=c,hatch=h,edgecolor='white')
ax.set_xticks(xs); ax.set_xticklabels(lab); ax.set_ylabel('S_IC / S_rho  (w = 0)'); ax.axhline(1,color='k',lw=.8)
ax.set_title('Table B IC/rho gap: recorded vs in-session tuned baseline [pcc:unverified]',fontsize=9)
ax.legend(fontsize=7,frameon=False); ax.spines[['top','right']].set_visible(False)
fig.tight_layout(); fig.savefig('baseline-gap.png'); fig.savefig('baseline-gap.svg')
