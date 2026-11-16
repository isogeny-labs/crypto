"""Summarize runs/pm_p*_s*.json. Cost of a run = M + S_W*S + I_W*I field multiplications,
reported at I_W in {20, 80}, S_W = 1 (S_W = 0.8 kept in summary.json). Per p: mean over 3 curves of per-curve means."""
import json, glob, os, statistics as st, math
here=os.path.dirname(os.path.abspath(__file__)); PS=[10007,100003,1000003]; WS=(1,16,64); MS=(0,1,4); NS=(1,2,4)
def cost(o,I,S=1.0): return o['M']+S*o['S']+I*o['I']
ic_arms=['base','pm']+[f'{m}{W}' for W in WS for m in ('bb','pmb')]
def ic_arm(x): return x['mode'] if x['mode'] in ('base','pm') else f"{x['mode']}{x['W']}"
summary={'unit':'F_p multiplications M; S counted as 1 M (S0.8 keys: 0.8 M); I at 20 M and 80 M','primes':{}}
nsolved=0
def stats(groups):
    row={}
    for I in (20,80):
        for S in (1.0,0.8):
            vals=[[cost(x['ops'],I,S) for x in g] for g in groups]
            row[f'I{I}_S{S}']=st.mean(st.mean(v) for v in vals)
            allv=sum(vals,[]); row[f'I{I}_S{S}_se']=st.stdev(allv)/math.sqrt(len(allv))
    row['runs']=sum(len(g) for g in groups)
    for k in 'MSI': row[k]=st.mean(st.mean(x['ops'][k] for x in g) for g in groups)
    return row
for p in PS:
    files=sorted(glob.glob(f'{here}/runs/pm_p{p}_s*.json')); ds=[json.load(open(f)) for f in files]
    P={'files':[os.path.relpath(f,here) for f in files],'sqrt_r':st.mean(math.isqrt(d['r']) for d in ds),'rho':{},'ic':{}}
    for d in ds:
        nsolved+=len(d['rho'])+len(d['ic'])
        assert all(x['log_ok'] for x in d['ic']+d['rho'])
    for W in WS:
        for m in MS:
            g=[[x for x in d['rho'] if x['W']==W and x['m']==m] for d in ds]
            row=stats(g); row['points']=st.mean(st.mean(x['points']+x['side_points'] for x in gg) for gg in g)
            P['rho'][f'W{W}_m{m}']=row
    for n in NS:
        for a in ic_arms:
            g=[[x for x in d['ic'] if x['n']==n and ic_arm(x)==a] for d in ds]
            row=stats(g); row['targets']=st.mean(st.mean(x['targets'] for x in gg) for gg in g)
            P['ic'][f'n{n}_{a}']=row
    summary['primes'][str(p)]=P
summary['logs_checked']=nsolved
json.dump(summary,open(f'{here}/summary.json','w'),indent=1)
def bic(P,a,k): return min((P['ic'][f'n{n}_{a}'][k],n) for n in NS)
def brho(P,W,k,ms=MS): return min((P['rho'][f'W{W}_m{m}'][k],m) for m in ms)
L=[]
for I in (20,80):
    k=f'I{I}_S1.0'
    L.append(f'\n#### Field-op cost per solved log, I = {I} M, S = 1 M (mean of 3 curves; IC at its best n in 1,2,4, shown in parentheses)\n')
    L.append('| p | sqrt r | rho W=1 m=0 | rho W=64 m=0 | rho W=64 best m (m) | IC base | IC pm | IC bb64 | IC pmb64 | IC pmb16 |')
    L.append('|'+'---|'*10)
    for p in PS:
        P=summary['primes'][str(p)]; f=lambda t:f'{t[0]:.0f} ({t[1]})'
        L.append(f'| {p} | {P["sqrt_r"]:.0f} | {P["rho"]["W1_m0"][k]:.0f} | {P["rho"]["W64_m0"][k]:.0f} | {f(brho(P,64,k))} | '
                 +' | '.join(f(bic(P,a,k)) for a in ('base','pm','bb64','pmb64','pmb16'))+' |')
    L.append(f'\n#### Gap ratios (IC / rho; > 1 means IC costs more), I = {I} M\n')
    L.append('| p | base / rho W1 m0 (ic_mt setting) | pm / rho W1 m0 | bb64 / rho W64 m0 | pmb64 / rho W64 m0 | pmb64 / rho W64 m1 | best IC / best rho (all arms) |')
    L.append('|'+'---|'*7)
    for p in PS:
        P=summary['primes'][str(p)]
        bI=min((bic(P,a,k)[0],a) for a in ic_arms); bR=min((P['rho'][x][k],x) for x in P['rho'])
        L.append(f'| {p} | {bic(P,"base",k)[0]/P["rho"]["W1_m0"][k]:.2f} | {bic(P,"pm",k)[0]/P["rho"]["W1_m0"][k]:.2f} | '
                 f'{bic(P,"bb64",k)[0]/P["rho"]["W64_m0"][k]:.2f} | {bic(P,"pmb64",k)[0]/P["rho"]["W64_m0"][k]:.2f} | '
                 f'{bic(P,"pmb64",k)[0]/P["rho"]["W64_m1"][k]:.2f} | {bI[0]/bR[0]:.2f} ({bI[1]} vs rho {bR[1]}) |')
L.append('\n#### Per-arm detail at p = 1000003 (mean field ops per solved log, mean of 3 curves)\n')
L.append('| arm | M | S | I | stored points (rho) / targets (IC) | cost I=20 | cost I=80 |')
L.append('|---|---|---|---|---|---|---|')
P=summary['primes']['1000003']
for W in WS:
    for m in MS:
        R=P['rho'][f'W{W}_m{m}']; L.append(f'| rho_store_neg W={W} m={m} | {R["M"]:.0f} | {R["S"]:.0f} | {R["I"]:.0f} | {R["points"]:.0f} | {R["I20_S1.0"]:.0f} | {R["I80_S1.0"]:.0f} |')
for n in NS:
    for a in ic_arms:
        R=P['ic'][f'n{n}_{a}']; L.append(f'| LP-IC {a} n={n} | {R["M"]:.0f} | {R["S"]:.0f} | {R["I"]:.0f} | {R["targets"]:.0f} | {R["I20_S1.0"]:.0f} | {R["I80_S1.0"]:.0f} |')
L.append('\n#### Standard errors (pooled over 3 curves) of the I = 80 M cost\n')
L.append('| p | rho W1 m0 | rho W64 m0 | rho W64 m1 | IC base n1 | IC pm n1 | IC bb64 n1 | IC pmb64 n1 |'); L.append('|'+'---|'*8)
for p in PS:
    P=summary['primes'][str(p)]; k='I80_S1.0_se'
    L.append(f'| {p} | '+' | '.join(f'{P["rho"][x][k]:.0f}' for x in ('W1_m0','W64_m0','W64_m1'))+' | '
             +' | '.join(f'{P["ic"]["n1_"+a][k]:.0f}' for a in ('base','pm','bb64','pmb64'))+' |')
L.append(f'\nLogs solved and checked against the true value: {nsolved} (every run asserts its log; a wrong log aborts the run).')
open(f'{here}/tables.md','w').write('\n'.join(L)+'\n'); print('\n'.join(L))
import matplotlib; matplotlib.use('Agg'); import matplotlib.pyplot as plt
fig,axs=plt.subplots(1,2,figsize=(10,4),sharey=True)
for ax,I in zip(axs,(20,80)):
    k=f'I{I}_S1.0'
    for lab,f,sty in [('rho_store_neg W=1 (unbatched)',lambda P:P['rho']['W1_m0'][k],'s-'),
                      ('rho_store_neg W=64 batched',lambda P:P['rho']['W64_m0'][k],'s--'),
                      ('rho_store_neg W=64 batched, +- side points m=1',lambda P:P['rho']['W64_m1'][k],'s:'),
                      ('LP-IC base (ic_mt.py), best n',lambda P:bic(P,'base',k)[0],'o-'),
                      ('LP-IC +- x-only, unbatched, best n',lambda P:bic(P,'pm',k)[0],'^-'),
                      ('LP-IC batched W=64 without +-, best n',lambda P:bic(P,'bb64',k)[0],'v--'),
                      ('LP-IC batched W=64 with +-, best n',lambda P:bic(P,'pmb64',k)[0],'^--')]:
        ax.loglog(PS,[f(summary['primes'][str(p)]) for p in PS],sty,label=lab)
    ax.set_title(f'I = {I} M, S = 1 M',fontsize=9); ax.set_xlabel('p')
axs[0].set_ylabel('F_p cost per solved log (M)'); axs[0].legend(fontsize=6)
fig.suptitle('Large-prime IC vs rho_store_neg in F_p operations (3 curves per p, 200-400 runs per arm per curve)',fontsize=9); fig.tight_layout()
for ext in ('png','svg','pdf'): fig.savefig(f'{here}/pm_vs_rho.{ext}',dpi=150)
