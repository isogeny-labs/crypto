"""Layout of recorded summaries; jq computes the summaries, Python plots fields only."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
here=Path(__file__).resolve().parent
rows=json.loads((here/'../../followup-run/summary.json').read_text())
lp=json.loads((here/'../../followup-run/lp_summary.json').read_text())
fig,axs=plt.subplots(1,2,figsize=(11,4))
for ax,key,title in [(axs[0],'hybrid_solve_ratio','Hybrid / Fast median solve time'),(axs[1],'oracle_ratio','Direct / reference median oracle time')]:
    ax.bar(range(len(rows)),[r[key] for r in rows],color=['#4477aa' if r['g']==2 else '#228833' if r['g']==3 else '#aa3377' for r in rows])
    ax.axhline(1,color='black',lw=1)
    ax.axhline(.8 if key.startswith('hybrid') else .9,color='#bb0000',ls='--',lw=1)
    ax.set_xticks(range(len(rows)),[f"g{r['g']}\n{r['p']}" for r in rows],fontsize=8)
    ax.set_ylabel('ratio (lower is better)');ax.set_title(title,fontsize=10)
fig.suptitle('20 seeds per instance; local L0 host; record 20261114T2210Z-hec-followups',fontsize=10)
fig.tight_layout();fig.savefig(here/'ratios.svg');fig.savefig(here/'ratios.png',dpi=140);plt.close(fig)
fig,ax=plt.subplots(figsize=(9,4))
ax.bar(range(len(lp)),[r['trials_median'] for r in lp],color=['#999999' if r['lp']==0 else '#4477aa' if r['lp']==1 else '#228833' for r in lp])
ax.set_xticks(range(len(lp)),[f"p{r['p']}\nm{r['m']}\n{r['lp']} LP" for r in lp],fontsize=8)
ax.set_yscale('log');ax.set_ylabel('median walk trials (log scale)')
ax.set_title('Genus 4 baseline: all 36 runs verify; 3 seeds per configuration',fontsize=10)
fig.tight_layout();fig.savefig(here/'large_primes.svg');fig.savefig(here/'large_primes.png',dpi=140)
