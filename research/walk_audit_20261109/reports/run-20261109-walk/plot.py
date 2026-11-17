"""Plot native recorded summaries only; no census or statistical inference."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.ticker import ScalarFormatter

here=Path(__file__).resolve().parent
def rows(name):
    return [json.loads(s) for s in (here/'../../evidence/runs'/name).read_text().splitlines()]
palette={2:'#34688c',3:'#c48126',4:'#43866a',6:'#775a9e'}
mix=[r for r in rows('mix-0.stderr') if r.get('kind')=='mix_population']
fig,ax=plt.subplots(figsize=(7,3.3),layout='constrained')
for count in [2,3,4,6]:
    rs=[r for r in mix if r['r']==count]
    means=[r['completed_step_sum']/r['completed'] for r in rs]
    lo=[m-r['completed_min'] for m,r in zip(means,rs)]
    hi=[r['completed_max']-m for m,r in zip(means,rs)]
    ax.errorbar([r['p'] for r in rs],means,yerr=[lo,hi],marker='o',capsize=3,
                color=palette[count],label=f'{count} increments')
ax.set_xticks([5,7,11,13])
ax.set_xlabel('Prime modulus p')
ax.set_ylabel('First quarter-TV step')
ax.set_yscale('log',base=2)
ax.set_yticks([1,2,4,8,16,32])
ax.yaxis.set_major_formatter(ScalarFormatter())
ax.grid(alpha=.15)
ax.legend(ncol=2,fontsize=9,frameon=False)
ax.spines[['top','right']].set_visible(False)
fig.savefig(here/'mixing.svg')
fig.savefig(here/'mixing.pdf')

rho=[r for r in rows('rho-0.stderr') if r.get('kind')=='rho_population']
fig,axes=plt.subplots(1,2,figsize=(7,3.1),layout='constrained',sharey=True)
for ax,count in zip(axes,[2,3]):
    for stratum,label,style in [('all','All supports',':'),('no_zero','No zero increment','-')]:
        rs=[r for r in rho if r['r']==count and r['stratum']==stratum and r['maps']]
        ax.plot([r['p'] for r in rs],[int(r['relative_num'])/int(r['relative_den']) for r in rs],
                marker='o',linestyle=style,color=palette[count],label=label)
    reference=next(r for r in rho if r['r']==count and r['maps'])
    penalty=(reference['heuristic_penalty_squared_num']/reference['heuristic_penalty_squared_den'])**.5
    ax.axhline(penalty,color='#444444',linestyle='--',label='Heuristic penalty')
    ax.set_title(f'{count} increments')
    ax.set_xticks([3,5,7])
    ax.set_xlabel('Prime modulus p')
    ax.set_ylim(.94,1.53)
    ax.grid(alpha=.15)
    ax.spines[['top','right']].set_visible(False)
    ax.legend(fontsize=8,frameon=True,facecolor='white',framealpha=.95,loc='upper left')
axes[0].set_ylabel('Mean / mapping baseline')
fig.savefig(here/'collisions.svg')
fig.savefig(here/'collisions.pdf')
