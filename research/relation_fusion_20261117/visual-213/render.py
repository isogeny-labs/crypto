"""Layout only: all experimental statistics come from frozen Rust analysis211."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
s=Path(__file__).resolve().parents[1]
a=json.loads((s/'PACKED_ANALYSIS_211.json').read_text())
fig,ax=plt.subplots(figsize=(9.5,3.8),layout='constrained')
labels=[]
for i,c in enumerate(a['cases']):
 b=c['primary_packed_dag_487_vs_strongest'];aa=c['aa_symmetric_max_ratio']
 ax.plot([1/aa,aa],[i,i],lw=12,color='#d8dfe5',solid_capstyle='butt',label='Observed symmetric A/A envelope' if i==0 else None)
 ax.plot([b['pointwise95_low'],b['pointwise95_high']],[i,i],lw=2.5,color='#a34700',label='Paired bootstrap pointwise 95% interval' if i==0 else None)
 ax.plot(b['median'],i,'o',color='#a34700',ms=5,label='Primary median' if i==0 else None)
 labels.append(f"p31, n{c['n']}" if c['prime']==2147483647 else f"p53, n{c['n']}")
ax.axvline(1,color='#68747e',lw=1)
ax.axvline(15,color='#872c30',lw=1.6,ls='--',label='15x target')
ax.set_xscale('log');ax.set_xlim(0.1,18);ax.set_ylim(5.7,-0.8)
ax.set_yticks(range(6),labels);ax.set_xticks([0.1,0.2,0.5,1,2,5,10,15],['0.1','0.2','0.5','1','2','5','10','15'])
ax.set_xlabel('Full parent control / candidate ratio (log scale; larger favors candidate)')
ax.set_title('Packed constructor200: primary remains inside observed A/A spread',loc='left',fontweight='bold',fontsize=11)
ax.spines[['top','right']].set_visible(False)
ax.legend(loc='upper left',bbox_to_anchor=(0,-0.22),ncol=2,frameon=False,fontsize=8)
fig.savefig(s/'visual-213/primary-vs-aa.svg')
fig.savefig(s/'visual-213/primary-vs-aa.png',dpi=140)

# Normalize generated SVG line endings for repository whitespace checks.
p=s/'visual-213/primary-vs-aa.svg'
p.write_text('\n'.join(line.rstrip() for line in p.read_text().splitlines())+'\n')
