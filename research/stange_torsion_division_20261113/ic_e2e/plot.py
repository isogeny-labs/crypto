import json,glob,statistics as st,os,matplotlib
matplotlib.use('Agg'); import matplotlib.pyplot as plt
here=os.path.dirname(os.path.abspath(__file__)); ps=[10007,100003,1000003]; rho=[];ic=[];lp=[]
for p in ps:
    rs=[json.load(open(f)) for f in sorted(glob.glob(f'{here}/runs/e2e_p{p}_s*.json'))]
    rho.append(st.mean(o['rho_ops_mean'] for o in rs))
    ic.append(min(st.mean(o['ic'][i]['total_ops'] for o in rs) for i in range(len(rs[0]['ic']))))
    ls=[json.load(open(f)) for f in sorted(glob.glob(f'{here}/runs_lp/lp_p{p}_s*.json'))]
    lp.append(min(st.mean(o['ic'][i]['total_ops'] for o in ls) for i in range(len(ls[0]['ic']))))
fig,ax=plt.subplots(figsize=(6,4))
ax.loglog(ps,ic,'o-',label='index calculus, best n (hash-table 3-decomposition)')
ax.loglog(ps,lp,'^-',label='index calculus + large-prime collisions, best n')
ax.loglog(ps,rho,'s-',label='Pollard rho (r-adding walk, Brent)')
ax.loglog(ps,[p**(2/3) for p in ps],':',color='gray',label='p^(2/3)'); ax.loglog(ps,[p**.5 for p in ps],'--',color='gray',label='p^(1/2)')
ax.set_xlabel('p'); ax.set_ylabel('group operations (point additions)'); ax.set_title('Toy IC vs rho, same prime-order curves (3 per p)')
ax.legend(fontsize=7); fig.tight_layout()
for ext in ('png','svg','pdf'): fig.savefig(f'{here}/ic_vs_rho.{ext}',dpi=150)
