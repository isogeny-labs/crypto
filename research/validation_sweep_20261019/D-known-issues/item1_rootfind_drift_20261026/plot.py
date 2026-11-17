# Lays out compare_native.out only (recorded values); computes nothing new.
import json, matplotlib; matplotlib.use('svg')
import matplotlib.pyplot as plt
rows=[json.loads(l) for l in open('compare_native.out')]
phases=['factor_base','oracle_setup','relations','linear_algebra','verify']
fig,axs=plt.subplots(1,2,figsize=(10.5,3.8),sharey=True)
for ax,arm in zip(axs,['ic-rootfind-m2','ic-rootfind-m3']):
    for i,ph in enumerate(phases):
        r=next(x for x in rows if x['arm']==arm and x['phase']==ph)
        vals=[r['pairs'] if r['only_in_rerun'] else 0, r['pairs'] if r['only_in_committed'] else 0,
              r['pairs'] if r['value_changed'] else 0, r['pairs'] if r['arithmetic_changed'] else 0]
        for j,(v,c) in enumerate(zip(vals,['#c0504d','#8fa8c8','#e0a030','#6b9a5b'])):
            ax.bar(i+(j-1.5)*0.2,v,0.2,color=c,label=['extra keys in re-run','keys only in committed','a value differs','adds/doubles/mults differ'][j] if i==0 else None)
    ax.set_xticks(range(5)); ax.set_xticklabels(phases,fontsize=7,rotation=15); ax.set_title(arm,fontsize=9); ax.set_ylim(0,40)
axs[0].set_ylabel('records (of 32) that differ')
h,l=axs[0].get_legend_handles_labels(); fig.legend(h,l,fontsize=7,loc='lower center',ncol=4,frameon=False)
fig.suptitle('claim-n13 root-finder rows: committed (2245b75) vs re-run on main, per phase',fontsize=9)
fig.tight_layout(rect=(0,0.07,1,1)); fig.savefig('drift.svg'); fig.savefig('drift.png',dpi=150)
