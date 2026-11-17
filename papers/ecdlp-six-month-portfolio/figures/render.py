"""Render recorded values only; does not run cryptographic experiments."""
from pathlib import Path
import json
import os
import tempfile
os.environ.setdefault('MPLCONFIGDIR',str(Path(tempfile.gettempdir())/'ecdlp-paper-matplotlib'))
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.patches import FancyBboxPatch

ROOT=Path(__file__).resolve().parent
d=json.loads((ROOT/'recorded-data.json').read_text())
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'axes.spines.top':False,
                     'axes.spines.right':False,'axes.titlesize':11,'axes.labelsize':9,
                     'svg.fonttype':'none','pdf.fonttype':42,'figure.facecolor':'white'})
COLORS=['#204b70','#278d8e','#ca8841','#a35464']

def save(fig,name):
    fig.savefig(ROOT/(name+'.pdf'),bbox_inches='tight')
    fig.savefig(ROOT/(name+'.svg'),bbox_inches='tight')
    svg=ROOT/(name+'.svg')
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
    fig.savefig(ROOT/(name+'.png'),bbox_inches='tight',dpi=150)
    plt.close(fig)

months=['2026-04','2026-05','2026-06','2026-07','2026-08','2026-09','2026-10']
fig,ax=plt.subplots(figsize=(8.1,3.1),layout='constrained')
bottom=np.zeros(len(months))
for repo,color in zip(['crypto','cryptanalysis','crypto-autoresearcher'],COLORS):
    vals=[sum(int(r['reachable_commits']) for r in d['monthly'] if r['repo']==repo and r['month']==m) for m in months]
    ax.bar(range(7),vals,bottom=bottom,label=repo,color=color,width=.65)
    bottom+=vals
ax.set(xticks=range(7),xticklabels=['Apr','May','Jun','Jul','Aug','Sep','Oct*'],
       ylabel='Reachable commits in pinned history',title='Recorded activity, not independent experiments')
ax.legend(frameon=False,loc='upper left');ax.grid(axis='y',alpha=.15);save(fig,'activity')

fig,axs=plt.subplots(1,2,figsize=(8.1,3.4),layout='constrained')
for ax,prefix,arm,title in [(axs[0],'icv1-f2m','ic-compact','Koblitz compact-orbit IC'),
                            (axs[1],'icv1-fp','ic-pair','Prime pair-table IC')]:
    rows=[r for r in d['frontier'] if r['curve'].startswith(prefix) and r['arm']==arm]
    assert len(rows)==(8 if prefix=='icv1-f2m' else 6), 'Unexpected campaign panel'
    rows=sorted(rows,key=lambda r:float(r['log2_r']))
    x=[float(r['log2_r']) for r in rows]
    mid=[float(r['ratio_w0.1']) for r in rows]
    lo=[float(r['ratio_w0']) for r in rows];hi=[float(r['ratio_w1']) for r in rows]
    mins=np.minimum(lo,hi);maxs=np.maximum(lo,hi)
    ax.errorbar(x,mid,yerr=[np.array(mid)-mins,maxs-np.array(mid)],fmt='o',color=COLORS[0],
                ecolor=COLORS[1],capsize=3,markersize=4)
    ax.axhline(1,color='black',ls='--',lw=.8)
    ax.set(yscale='log',xlabel='log2 prime subgroup order r',ylabel='Cold IC / matched rho',title=title)
    ax.grid(axis='y',alpha=.16)
save(fig,'frontier')

n=d['n83'];s=n['pipeline_summaries'];keys=['k1104','k1108','k1120','k1152']
fig,axs=plt.subplots(1,2,figsize=(8.1,3.0),layout='constrained')
values=[s[k]['mean_cold_process_ms']/1000 for k in keys]
axs[0].bar(range(4),values,color=[COLORS[0],COLORS[0],COLORS[1],COLORS[2]],width=.65)
axs[0].set(xticks=range(4),xticklabels=['1104','1108','1120','1152'],ylim=(0,138),
           xlabel='Folded columns',ylabel='Mean cold process seconds',title='Five-point cold means')
for i,v in enumerate(values):axs[0].text(i,v+2,f'{v:.3f}',ha='center',fontsize=8)
specs=[('k1104|k1152','1104 vs 1152'),('k1120|k1152','1120 vs 1152'),('k1120|k1104','1120 vs 1104')]
means=[];low=[];high=[]
for key,label in specs:
    a=n['paired_intervals'][key];means.append(a['mean_ms']/1000);low.append(a['lower_95_ms']/1000);high.append(a['upper_95_ms']/1000)
y=np.arange(3)
axs[1].errorbar(means,y,xerr=[np.array(means)-low,np.array(high)-means],fmt='o',capsize=4,color=COLORS[1])
axs[1].axvline(0,color='black',ls='--',lw=.8)
axs[1].set(yticks=y,yticklabels=[v for k,v in specs],xlabel='Paired seconds saved (95% interval)',
           title='No unique winner between 1104 and 1120');axs[1].invert_yaxis()
save(fig,'n83')

fig,axs=plt.subplots(1,2,figsize=(8.1,2.8),layout='constrained')
for ax,labels,vals,title in [(axs[0],['Host','RTX 5090'],[425.8,5.70],'One large matrix: build + eliminate'),
                            (axs[1],['Inherited CPU','GPU decider'],[1.25,5.57],'Small-system oracle panel')]:
    ax.bar(labels,vals,color=[COLORS[0],COLORS[1]],width=.6);ax.set(ylabel='Historical seconds',title=title)
    for i,v in enumerate(vals):ax.text(i,v*1.03,f'{v:g}',ha='center',fontsize=9)
    ax.set_ylim(0,max(vals)*1.2)
save(fig,'gpu-stages')

v=d['volcano'];coverage=v['campaigns'][0]['coverage'];graphs=['mixed','three','two']
fig,axs=plt.subplots(1,2,figsize=(8.1,3.0),layout='constrained')
audited=[sum(x['vertices'] for x in coverage if x['graph']==g and x['state']=='audited') for g in graphs]
totals=[sum(x['vertices'] for x in coverage if x['graph']==g) for g in graphs]
axs[0].barh(graphs,audited,color=COLORS[1],label='Audited')
axs[0].barh(graphs,np.array(totals)-audited,left=audited,color='#dce4e8',label='Remaining / interrupted')
for i,(a,t) in enumerate(zip(audited,totals)):axs[0].text(t+70,i,f'{a:,} / {t:,}',va='center',fontsize=8)
axs[0].set(xlabel='Vertices',xlim=(0,13500),title='Actual measured coverage');axs[0].legend(frameon=False,fontsize=7,loc='lower right')
labels=['Verified','Exhausted','Timeout'];vals=[v['outcome_totals'][k] for k in ['verified','exhausted','timeout']]
axs[1].barh(labels,vals,color=[COLORS[1],COLORS[2],COLORS[3]])
for i,a in enumerate(vals):axs[1].text(a+450,i,f'{a:,}',va='center',fontsize=8)
axs[1].set(xlabel='Attempt records',xlim=(0,37500),title='Unsuccessful attempts are retained')
save(fig,'volcano-coverage')

fig,axs=plt.subplots(1,2,figsize=(8.1,2.8),layout='constrained')
for ax,labels,vals,title in [(axs[0],['Full model','Ceiling baseline'],[.884,.885],'Blind global plain-rho cost'),
                            (axs[1],['Planted-safe model','Oracle'],[.914,.918],'Within-class planted rho signal')]:
    ax.bar(labels,vals,color=[COLORS[0],COLORS[1]],width=.6);ax.set(ylabel='Held-out R-squared',ylim=(0,1.05),title=title)
    for i,x in enumerate(vals):ax.text(i,x+.03,f'{x:.3f}',ha='center')
save(fig,'ml-controls')

fig,ax=plt.subplots(figsize=(8.1,2.5),layout='constrained')
labels=['Free relations: production Koblitz IC','First dependency: extension-field D4 IC',
        'Structured elimination + Wiedemann: genus 3']
vals=np.array([.960,.979,.653]);lo=np.array([.946,.973,.647]);hi=np.array([.976,.986,.660]);y=np.arange(3)
ax.errorbar(vals,y,xerr=[vals-lo,hi-vals],fmt='o',color=COLORS[1],capsize=4)
ax.axvline(1,color='black',ls='--',lw=.8);ax.set(yticks=y,yticklabels=labels,xlabel='Candidate / its own incumbent counted cost',xlim=(.60,1.025))
ax.invert_yaxis();save(fig,'incumbent-improvements')

fig,ax=plt.subplots(figsize=(8.1,4.7));ax.set_xlim(0,10);ax.set_ylim(0,6);ax.axis('off')
def box(x,y,w,h,text,color=COLORS[0]):
    ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=.05',facecolor=color,edgecolor='none'))
    ax.text(x+w/2,y+h/2,text,ha='center',va='center',color='white',fontsize=9,linespacing=1.5)
def arrow(a,b):ax.annotate('',xy=b,xytext=a,arrowprops={'arrowstyle':'->','color':'#526373','lw':1.4})
box(.15,4.7,2.75,.85,'Mathematical leads\nfields / bases / maps')
box(3.55,4.7,2.75,.85,'Executable components\nRust / C / Sage / CUDA')
box(7.05,4.7,2.75,.85,'Complete workloads\nsetup to verified answer')
arrow((2.95,5.1),(3.5,5.1));arrow((6.35,5.1),(7,5.1))
box(.15,2.7,2.75,1.0,'Controls and identities\nsubgroups / signs / lifting',COLORS[1])
box(3.55,2.7,2.75,1.0,'Sealed records\ncounts / failures / hashes',COLORS[1])
box(7.05,2.7,2.75,1.0,'Matched comparisons\ncold / online / advice',COLORS[1])
arrow((1.5,4.65),(1.5,3.75));arrow((4.9,4.65),(4.9,3.75));arrow((8.4,4.65),(8.4,3.75))
box(.15,.55,4.35,1.1,'Proof obligations\nalgebra + concrete-to-abstract bridge',COLORS[2])
box(5.45,.55,4.35,1.1,'Independent review and replay\nscoped result / correction / next test',COLORS[2])
arrow((1.5,2.65),(2.3,1.7));arrow((4.9,2.65),(6.5,1.7));arrow((8.4,2.65),(7.6,1.7))
fig.subplots_adjust(left=.02,right=.98,top=.98,bottom=.02);save(fig,'research-map')
print('Rendered eight figure sets from recorded values.')
