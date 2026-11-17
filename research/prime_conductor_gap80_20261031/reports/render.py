"""Document rendering only: plot frozen receipts; never execute research code."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
data = json.loads((HERE.parent/'historical-summary.json').read_text())
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10})
fig, axes = plt.subplots(1,3,figsize=(12,3.6),layout='constrained')
for ax, row in zip(axes[:2],data['microbenchmarks']):
    vals=[row['baseline_median_seconds'],row['optimized_median_seconds']]
    ax.bar(['Baseline','Candidate'],vals,color=['#57677e','#177e89'])
    ax.set_title(f"Native helper · degree {row['degree']:,}\n7 pairs, unpinned")
    for i,v in enumerate(vals):ax.text(i,v+max(vals)*.025,f'{v:.6f}',ha='center',fontsize=9)
    ax.set_ylim(0,max(vals)*1.25)
    ax.set_ylabel('Seconds per helper call (median)')
tile=data['actual_target_tile']
ax=axes[2]
for i,key in enumerate(['baseline_seconds','optimized_seconds']):
    vals=[r[key] for r in tile['paired_receipts'] if r['clean_pinned_pair']]
    median=tile[['baseline_median_seconds','optimized_median_seconds'][i]]
    ax.scatter([i-.04,i,i+.04],vals,color=['#57677e','#177e89'][i],s=35)
    ax.hlines(median,i-.17,i+.17,color='black',linewidth=2)
    ax.text(i,max(vals)+.04,f'{median:.6f}',ha='center',fontsize=9)
ax.set_xticks([0,1],['Baseline','Candidate'])
ax.set_ylim(3.1,4.0)
ax.set_title('Same 80-bit partial tile · block 4,096\n3 retained pairs, CPU 0; Sage harness')
ax.set_ylabel('Seconds per partial tile (points; bar = median)')
fig.suptitle('Historical gap80 observations — documentary, outside admitted benchmark table',fontsize=12)
for ax in axes:ax.spines[['top','right']].set_visible(False)
fig.savefig(HERE/'historical-timings.svg')
fig.savefig(HERE/'historical-timings.png',dpi=150)
plt.close(fig)

fig,ax=plt.subplots(figsize=(11,2.0),layout='constrained')
ax.set_xlim(0,11);ax.set_ylim(0,2);ax.axis('off')
labels=['Validate monic h\nand parse jets','Shared reversed-\nmodulus inverse','Log-jet coefficients\nusing modular products','Shared trace weights\nand coefficient dot products']
xs=[1.25,4.05,6.85,9.65]
for x,label in zip(xs,labels):
    ax.text(x,1,label,ha='center',va='center',bbox={'boxstyle':'round,pad=.6','facecolor':'#e7f0f3','edgecolor':'#177e89'},fontsize=9)
for a,b in zip(xs,xs[1:]):ax.annotate('',xy=(b-1.25,1),xytext=(a+1.18,1),arrowprops={'arrowstyle':'->','color':'#57677e'})
fig.suptitle('Native helper data flow — no complete isogeny implied',fontsize=12)
fig.savefig(HERE/'helper-flow.svg')
fig.savefig(HERE/'helper-flow.png',dpi=150)
plt.close(fig)

# Normalize Matplotlib's path-data whitespace without changing geometry.
for svg in HERE.glob('*.svg'):
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
