"""Render recorded native summaries only; no statistical/result computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
from matplotlib.ticker import EngFormatter

HERE=Path(__file__).resolve().parent
DATA=HERE/'summary.json'
if not DATA.exists(): DATA=HERE/'attempt-2'/'summary.json'
s=json.loads(DATA.read_text())
plt.rcParams.update({'font.family':'DejaVu Sans','svg.fonttype':'none','font.size':10})
figures=[]
def save(fig,name):
    fig.savefig(HERE/(name+'.svg'),bbox_inches='tight')
    p=HERE/(name+'.svg');p.write_text('\n'.join(x.rstrip() for x in p.read_text().splitlines())+'\n')
    fig.savefig(HERE/(name+'.png'),dpi=150,bbox_inches='tight')
    figures.append(fig)

fig,axes=plt.subplots(1,2,figsize=(11.7,8.3));fig.subplots_adjust(top=.80,bottom=.43,wspace=.35)
fig.suptitle('Runpod: native trace and source-coordinate root finding',fontsize=18,fontweight='bold',y=.96)
fig.text(.065,.88,'Source c0176f942 · 8 vCPU / 16 GB · one pinned measurement CPU · shared host, no L2',fontsize=10)
arms=['dense','trace','pullback','dense-isogeny','trace-isogeny','pullback-isogeny','flint']
values=[s['paper_root_per_call'][a]['cpu_seconds']['median'] for a in arms]
axes[0].bar(range(len(arms)),values,color=['#e39836','#4778a8','#2f966d','#dfbd8a','#91acc8','#89baa4','#7a6b9d'])
axes[0].set_xticks(range(len(arms)),['Dense','Trace','Pullback','Dense\n+isog','Trace\n+isog','Pullback\n+isog','FLINT'],rotation=45,ha='right')
axes[0].yaxis.set_major_formatter(EngFormatter(unit='s'));axes[0].set_ylabel('Median CPU time per root call');axes[0].set_title('Fixed paper polynomial: F83, degree 10')
for k,a in enumerate(arms):
    d=s['paper_root_per_call'][a]['cpu_seconds'];axes[0].vlines(k,d['minimum'],d['maximum'],color='#182c3d')
cases=['generic','collision','exception_minus','exception_plus']
ratios=[s['derived_translation_cpu_ratio_of_medians_dense_over_trace']['256/'+a] for a in cases]
axes[1].bar(range(4),ratios,color='#4778a8');axes[1].set_xticks(range(4),['Generic','Collisions','Exception −','Exception +'],rotation=35,ha='right')
axes[1].axhline(1,color='#777',lw=1);axes[1].set_ylabel('Dense CPU / trace CPU (ratio of medians)');axes[1].set_title('Complete degree-256 translation')
fig.text(.065,.17,'Root panel: five rounds, 32 full calls per row; literal datum supplied and validated each call.\nIsogeny arms additionally construct and validate the full toy codomain datum. FLINT uses no datum.\nTranslation panel: five rounds on each of two fields; collisions and exceptional handling included.\nThe frozen point-support audit is recorded separately from the translation timer.',fontsize=10,linespacing=1.5)
fig.text(.065,.045,'Unsigned descriptive measurements. No fitted exponent, fastest-tool claim or security-tier promotion.\nSources: attempt-2/summary.json, immutable raw archive 0b27d27910807fc9…7b7f1cb06be81fa. [pcc:unverified]',fontsize=8.5,color='#52687b')
save(fig,'root-and-translation')

fig,axes=plt.subplots(2,2,figsize=(11.7,8.3));fig.subplots_adjust(top=.86,bottom=.22,hspace=.45,wspace=.3)
fig.suptitle('Complete native divisor translation — recorded scaling',fontsize=18,fontweight='bold',y=.96)
for ax,case in zip(axes.flat,cases):
    for arm,color in [('dense','#d68b2d'),('trace','#3f75aa')]:
        degrees=[8,16,32,64,128,256]
        ys=[s['translation'][str(d)+'/'+case+'/'+arm]['cpu_seconds']['median'] for d in degrees]
        ax.plot(degrees,ys,'o-',label=arm,color=color)
    ax.set_xscale('log',base=2);ax.set_yscale('log');ax.set_xticks(degrees,degrees);ax.yaxis.set_major_formatter(EngFormatter(unit='s'));ax.set_title(case.replace('_',' '));ax.set_xlabel('Input degree');ax.set_ylabel('Median CPU time');ax.grid(alpha=.2);ax.legend()
fig.text(.065,.055,'Same native FLINT backend and identical frozen inputs for both arms; p=1,000,003 and 1,000,033.\nEach point summarizes ten calls: five rounds in each field. These are descriptive repeats, not confidence bounds.\nTrace loses at some small degrees and retains a quadratic recurrence. [pcc:unverified]',fontsize=9,color='#52687b',linespacing=1.4)
save(fig,'translation-scaling-native')

fig,axes=plt.subplots(1,2,figsize=(11.7,8.3));fig.subplots_adjust(top=.80,bottom=.45,wspace=.32)
fig.suptitle('Complete random-labelled target recovery and rho',fontsize=18,fontweight='bold',y=.96)
fig.text(.065,.88,'60 public targets · B=8…128 · r approximately 8 B³ · fresh charged pools, up to four attempts\nFive measured rounds after a warmup; finite failures/censoring retained.',fontsize=10)
Bs=[8,16,32,64,128]
for arm,color in [('known','#777777'),('pairs','#cf8832'),('pullback','#3979ad'),('pullback-isogeny','#369a76')]:
    axes[0].plot(Bs,[s['incidence_bounded_target_work'][str(b)+'/'+arm]['wall_seconds']['median'] for b in Bs],'o-',label=arm,color=color)
    axes[1].plot(Bs,[s['paired_arm_over_rho_wall_completed_only'][str(b)+'/'+arm]['median'] for b in Bs],'o-',label=arm,color=color)
axes[0].plot(Bs,[s['rho_adapter_by_B'][str(b).zfill(3)]['wall_seconds']['median'] for b in Bs],'o-',label='rho',color='#7a5c96')
for ax in axes:
    ax.set_xscale('log',base=2);ax.set_xticks(Bs,Bs);ax.set_yscale('log');ax.set_xlabel('Pool size B');ax.grid(alpha=.2)
axes[0].yaxis.set_major_formatter(EngFormatter(unit='s'));axes[0].set_ylabel('Median bounded target work, wall time');axes[0].legend(fontsize=8)
axes[1].set_ylabel('Paired IC / rho wall-time ratio');axes[1].axhline(1,color='#777',lw=1);axes[1].set_title('Jointly completed targets only')
lines=['B       pullback(s)    pairs(s)       rho(s)       IC completed / measured target units']
for b in Bs:
    a=s['incidence_bounded_target_work'][str(b)+'/pullback'];p=s['incidence_bounded_target_work'][str(b)+'/pairs'];r=s['rho_adapter_by_B'][str(b).zfill(3)]
    lines.append(f"{b:<7} {a['wall_seconds']['median']:<14.6g} {p['wall_seconds']['median']:<14.6g} {r['wall_seconds']['median']:<13.6g} {a['completed']} / {a['rows']}")
fig.text(.065,.30,'\n'.join(lines),fontfamily='DejaVu Sans Mono',fontsize=9,va='top',linespacing=1.5)
fig.text(.065,.04,'Time includes public validation/generation/rejections, prior misses, lazy datum construction/validation,\nnorm/GCD, roots, witness recovery and original-curve scalar checks. All four misses mean censored, not solved.\nRho: prime_fast, negation, one thread, table_bits=4, walkers=8, dp_bits=0; no optimal-tuning claim.\nProcess startup/output and parent/child RSS are separately retained. No L2 or signed replay. [pcc:unverified]',fontsize=9,color='#52687b',linespacing=1.4)
save(fig,'complete-target-costs')
with PdfPages(HERE/'RUNPOD_REPORT.pdf') as pdf:
    for figure in figures:pdf.savefig(figure,bbox_inches='tight')
for figure in figures:plt.close(figure)
