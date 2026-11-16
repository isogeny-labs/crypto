#!/usr/bin/env python3
"""Layout only: plot imported scores; perform no curve or search computation."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

ROOT=Path(__file__).parent
rows=list(csv.DictReader((ROOT/'data/scores.csv').open()))
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10,'svg.fonttype':'none'})
fig,axes=plt.subplots(6,1,figsize=(9.4,12),gridspec_kw={'height_ratios':[5,4,2.5,8,1.8,1.8]},sharex=True,layout='constrained')
for ax,family in zip(axes,['192','224','256','239','384','521']):
    panel=[r for r in rows if r['family']==family]
    for i,r in enumerate(panel):
        published=r['status']=='published'
        ax.scatter(float(r['S']),i,color='#1764a3' if published else '#bf5700',marker='o' if published else 'x',s=45,zorder=3)
        ax.text(float(r['S'])+3,i,r['S'],va='center',fontsize=9)
    labels=[r['label'] if r['status']=='published' else f"omitted #{r['candidate']}" for r in panel]
    ax.set_yticks(range(len(panel)),labels);ax.invert_yaxis();ax.set_xlim(0,230)
    ax.grid(axis='x',color='#d9e1e8',linewidth=.6);ax.set_axisbelow(True)
    ax.set_title(f"{family}-bit stream"+(' (ANSI)' if family=='239' else ''),loc='left',fontsize=11,fontweight='bold')
    for side in ['top','right','left']:ax.spines[side].set_visible(False)
    if family in ['384','521']:ax.text(3,0,'No eligible omission control',fontsize=8,color='#59636d',va='center')
axes[-1].set_xlabel('S: sum of eight full Jacobian 3-torsion field degrees')
fig.suptitle('Reported hyperelliptic-cover ranking\nBlue circles: published | Orange crosses: eligible omissions',fontsize=14)
fig.supxlabel('21 curves; fixed d = 0,...,7. Deterministic producer observations, [pcc:unverified].\nNo sampling uncertainty interval; no common cross-field cutoff.',fontsize=9)
for suffix in ['svg','pdf']:fig.savefig(ROOT/f'figures/ranking.{suffix}')
plt.close(fig)

fig,ax=plt.subplots(figsize=(9.4,3.9));ax.set_xlim(0,1);ax.set_ylim(0,1);ax.axis('off')
boxes=[(.04,.66,.36,.2,r'$H_{b,d}$'+'\nGenus-3 hyperelliptic cover'),(.60,.66,.36,.2,r'$E_b$'+'\nElliptic quotient'),(.04,.22,.36,.2,r'$J(H_{b,d})$'+'\nDimension 3'),(.60,.22,.36,.2,r'$E_b\times J(D_{b,d})$'+'\nDimensions 1 + 2')]
for x,y,w,h,label in boxes:
    ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=0.012',facecolor='#edf4fa',edgecolor='#38688c',linewidth=1.2))
    ax.text(x+w/2,y+h/2,label,ha='center',va='center',fontsize=11)
for y,label in [(.76,'Degree 2'),(.32,'Degree 8')]:
    ax.annotate('',xy=(.585,y),xytext=(.415,y),arrowprops={'arrowstyle':'->','color':'#294b65','lw':1.5})
    ax.text(.50,y+.065,label,ha='center',fontsize=9)
ax.text(.22,.54,'Take Jacobian',ha='center',fontsize=9,color='#59636d')
ax.plot([.22,.22],[.64,.44],linestyle=':',color='#59636d')
ax.text(.5,.075,r'On 3-torsion: direct sum of Frobenius modules; $m_H=\mathrm{lcm}(m_E,m_D)$.',ha='center',fontsize=10)
ax.set_title('Objects used in the reported score [pcc:unverified]',fontsize=12)
for suffix in ['svg','pdf']:fig.savefig(ROOT/f'figures/cover-flow.{suffix}',bbox_inches='tight')
plt.close(fig)

# Keep generated SVG line formatting clean without changing the plotted data.
for path in (ROOT/'figures').glob('*.svg'):
    path.write_text('\n'.join(line.rstrip() for line in path.read_text().splitlines())+'\n')
