"""Layout of exact native comparison fields only; no research arithmetic."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle, FancyArrowPatch, FancyBboxPatch
plt.rcParams.update({'pdf.use14corefonts':True,'font.family':'sans-serif','font.size':9})
p=Path(__file__).parent
data=json.loads((p/'graph-data.json').read_text())
fig,ax=plt.subplots(figsize=(9.8,4.6))
for row,v in enumerate(data['comparisons']):
    y=8-(row//6);x=v['n']-1
    ax.add_patch(Rectangle((x-.5,y-.5),1,1,facecolor='#cdebd8' if v['reduced']['joint_gain'] else '#f4f6f8',edgecolor='white'))
    ax.text(x,y+.16,f"{v['main_useful']}/{v['twist_useful']}",ha='center',va='center')
    ax.text(x,y-.18,f"{v['reduced']['main_degree']}/{v['reduced']['twist_degree']}",ha='center',va='center',color='#285574')
ax.set(xlim=(-.5,5.5),ylim=(-.5,8.5),xticks=range(6),xticklabels=range(1,7),yticks=range(9),yticklabels=['M2/T2','M2/T1','M2/T0','M1/T2','M1/T1','M1/T0','M0/T2','M0/T1','M0/T0'],xlabel='Public synthetic source target nP (separate records)',ylabel='Canonical base policy pair')
ax.tick_params(length=0);ax.set_title('F13 reduced S3: useful rank main/twist (black), operational degree main/twist (blue)')
fig.text(.5,.005,'cm8-transfer-20261025: all54 target comparisons; green = native joint higher useful rank/lower degree. No timing or regularity claim.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.035,1,1));fig.savefig(p/'comparisons.pdf');fig.savefig(p/'comparisons.svg');plt.close(fig)
fig,ax=plt.subplots(figsize=(9.8,1.2));ax.set(xlim=(0,1),ylim=(0,1));ax.axis('off')
labels=[('E/F13: a1,b4\nP=(0,2); r7',.02),('Actual Phi\nKernel (10,0)',.28),('T/F13: a4,b6\nU=(8,11)',.54),('S3 relations + logs\nVerify source scalar',.80)]
for label,x in labels:
    ax.add_patch(FancyBboxPatch((x,.25),.18,.5,boxstyle='round,pad=.01',facecolor='#eef3f7',edgecolor='#35566e'))
    ax.text(x+.09,.5,label,ha='center',va='center',fontsize=9)
for a,b in [(.21,.27),(.47,.53),(.73,.79)]:ax.add_patch(FancyArrowPatch((a,.5),(b,.5),arrowstyle='->',mutation_scale=12))
fig.tight_layout(pad=.1);fig.savefig(p/'bridge.pdf');fig.savefig(p/'bridge.svg');plt.close(fig)

# Normalize exporter formatting without altering vector geometry.
for svg in p.glob("*.svg"):
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines())+"\n")
