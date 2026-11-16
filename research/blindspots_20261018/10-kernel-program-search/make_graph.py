#!/usr/bin/env python3
"""Plots recorded training-stage values of item 10 (no computation of results beyond ratios of recorded Q)."""
import json, math, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
A = {('31','b'):'raw-r2/state/n31-b.jsonl', ('37','b'):'raw-r2/state/n37-b.jsonl', ('41','b'):'raw-r2/state/n41-b.jsonl',
     ('31','c'):'raw-r2/state/n31-c.jsonl', ('37','c'):'raw-r2/state/n37-c.jsonl', ('41','c'):'raw-r2/state/n41-c.jsonl',
     ('31','p'):'raw-r2/state/n31-p.jsonl'}
def load(f): return [json.loads(l) for l in open(f)]
def q(r,k='q01'):
    v=r[k]; return math.inf if v is None else v
D={k:load(f) for k,f in A.items()}
inc={n:D[(n,'b')][0] for n in ('31','37','41')}
S={}
for (n,a),L in D.items():
    best=min(L,key=lambda r:(q(r),r['idx']))
    S[f'n{n}-{a}']=dict(evals=len(L),best_idx=best['idx'],best=best['name'],best_phi=best['phi'],
      Q_best={w:q(best,k) for w,k in (('0','q0'),('0.1','q01'),('1','q1'))},
      Q_incumbent={w:q(inc[n],k) for w,k in (('0','q0'),('0.1','q01'),('1','q1'))},
      ratio_best_over_incumbent={w:q(best,k)/q(inc[n],k) for w,k in (('0','q0'),('0.1','q01'),('1','q1'))},
      dominated=sum(r['dominated'] for r in L), q_inf=sum(q(r)==math.inf for r in L),
      t_eff_best=best['t_eff'], t_eff_incumbent=inc[n]['t_eff'])
json.dump(S,open('raw-r2/training-stage-summary.json','w'),indent=1)
col={'b':'#1f77b4','c':'#d62728','p':'#2ca02c'}
fig,ax=plt.subplots(1,2,figsize=(11,4.2))
for (n,a),L in D.items():
    rb=[];m=math.inf
    for r in L: m=min(m,q(r)); rb.append(m)
    ls={'31':'-','37':'--','41':':'}[n]
    ax[0].step(range(len(rb)),[v if v<math.inf else float('nan') for v in rb],ls,color=col[a],label=f'n={n} arm {a}',where='post')
ax[0].set_yscale('log'); ax[0].set_xlabel('evaluation index'); ax[0].set_ylabel('running-best training Q (w=0.1)')
ax[0].set_title('Training running best (6 natural + 32 planted)',fontsize=9); ax[0].legend(fontsize=6,ncol=2)
cells=['31','37','41']; x=range(3)
rb_=[S[f'n{n}-b']['ratio_best_over_incumbent']['0.1'] for n in cells]
rc_=[S[f'n{n}-c']['ratio_best_over_incumbent']['0.1'] for n in cells]
ax[1].bar([i-0.2 for i in x],rb_,0.4,color=col['b'],label='b (search)')
ax[1].bar([i+0.2 for i in x],rc_,0.4,color=col['c'],label='c (random)')
ax[1].set_xticks(list(x)); ax[1].set_xticklabels([f'n={n}\nb:{S[f"n{n}-b"]["evals"]} c:{S[f"n{n}-c"]["evals"]} evals' for n in cells],fontsize=8)
ax[1].set_ylabel('best training Q / incumbent training Q (w=0.1)'); ax[1].set_ylim(0,0.2)
ax[1].set_title('TRAINING ratios only: no held-out data, no verdict\nselected from the same targets (winner\'s curse)',fontsize=9); ax[1].legend(fontsize=8)
fig.suptitle('Item 10 kernel program search, stage 2 of 4 complete: all seven searches at 256 evaluations',fontsize=9)
fig.tight_layout()
fig.savefig('graph.svg'); fig.savefig('graph.png',dpi=150)
with PdfPages('REPORT.pdf') as pdf:
    import textwrap
    t=open('README.md').read()
    lines=[]
    for ln in t.splitlines(): lines+= (textwrap.wrap(ln,105) or [''])
    per=62
    pages=[lines[i:i+per] for i in range(0,len(lines),per)]
    pf=plt.figure(figsize=(8.27,11.69)); pf.text(0.5,0.5,'',ha='center'); 
    pdf.savefig(fig)
    plt.close(pf)
    for p in pages:
        f=plt.figure(figsize=(8.27,11.69)); f.text(0.06,0.97,'\n'.join(p),va='top',family='monospace',fontsize=6.6); pdf.savefig(f); plt.close(f)
print(json.dumps(S,indent=1)[:200])
