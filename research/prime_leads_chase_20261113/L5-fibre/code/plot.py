import json, sys, matplotlib
matplotlib.use('Agg'); import matplotlib.pyplot as plt
rows=[json.loads(l) for f in sys.argv[2:] for l in open(f)]
fig,axs=plt.subplots(1,2,figsize=(10,4))
cols={'A1_fibre_randS':'#1f77b4','A2_fibre_intS':'#2ca02c','I_interval_x':'#9467bd','P_AP_logs':'#d62728','N_random_s0':'#7f7f7f'}
for ax,j in zip(axs,('W2','W3')):
    for arm,c in cols.items():
        pts=[(r['B'],r[j]) for r in rows if r['arm']==arm]
        pts.sort(); ax.loglog([p[0] for p in pts],[p[1] for p in pts],'o',color=c,label=arm,ms=4,alpha=.8)
    Bs=sorted({r['B'] for r in rows}); k=int(j[1])
    ax.loglog(Bs,[b**k/(2 if k==2 else 6.0) for b in Bs],'k--',lw=.8,label=f'B^{k}/{2 if k==2 else 6}')
    ax.set_xlabel('|F| = B'); ax.set_ylabel(f'|{k}F| (target-free message width)'); ax.set_title(f'{j}: distinct {k}-sums, 12-26 bit toy curves')
axs[0].legend(fontsize=7)
fig.tight_layout(); fig.savefig(sys.argv[1]); print('ok')
