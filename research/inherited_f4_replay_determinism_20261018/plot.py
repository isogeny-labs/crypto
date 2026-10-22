import json,sys
import matplotlib; matplotlib.use('svg')
import matplotlib.pyplot as plt
d=json.load(open('counters.json'))
fig,ax=plt.subplots(1,2,figsize=(11,4))
for a,arm in zip(ax,('ic-f4-m2','ic-f4-m3')):
    ks=sorted(k for k in d['apple'] if k.startswith(arm+'|'))
    x=range(len(ks))
    a.plot(x,[d['apple'][k] for k in ks],'s',mfc='none',color='C3',ms=9,label='Apple M4 (archived, commit 2245b75)')
    for n,m,c in (('t1','o','C0'),('t4','x','C2'),('t4b','+','C1')):
        a.plot(x,[d[n][k] for k in ks],m,color=c,ms=5,label={'t1':'x86-64, RAYON 1','t4':'x86-64, RAYON 4','t4b':'x86-64, RAYON 4 (2nd process)'}[n])
    a.set_title(arm+' solver word XORs per measured run (24 rows)')
    a.set_xlabel('row (target, round) index'); a.set_ylabel('solver_ops (word XORs)')
    a.set_yscale('log')
ax[0].legend(fontsize=7)
fig.suptitle('Session ECBS1hfd5bf1885227 spec re-run on x86-64 (3 sessions) vs archived Apple record; recorded values')
fig.tight_layout(); fig.savefig('counters.svg')
