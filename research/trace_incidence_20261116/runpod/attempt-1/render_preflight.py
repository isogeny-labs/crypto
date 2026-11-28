"""Render-only layout of already recorded execution metadata; no result calculation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
from matplotlib.backends.backend_pdf import PdfPages

root=Path(__file__).resolve().parent
data=json.loads((root/'data.json').read_text())
plt.rcParams.update({'font.family':'DejaVu Sans','svg.fonttype':'none'})
fig,ax=plt.subplots(figsize=(11.7,6.5))
fig.patch.set_facecolor('#f5f7fa');ax.set_facecolor('#f5f7fa');ax.axis('off');ax.set_xlim(0,1);ax.set_ylim(0,1)
ax.text(.035,.95,'Runpod preflight — preserved before timing',fontsize=20,fontweight='bold',color='#142d45',va='top')
ax.text(.035,.87,'Attempt 1 · source c7b146a05 · pod 8yoqn5n838rcfp · 2026-11-16',fontsize=10,color='#52687b')
boxes=[(.035,'Source + S3 checkpoint','Passed','#e1f1e9'),(.285,'Native qualification','Passed','#e1f1e9'),(.535,'Rho source hydration','Exit 129','#f8e0db'),(.785,'Timing matrix','Not started','#e4e9ef')]
for x,title,state,color in boxes:
    ax.add_patch(FancyBboxPatch((x,.61),.19,.16,boxstyle='round,pad=0.012',facecolor=color,edgecolor='#8296a8'))
    ax.text(x+.095,.72,title,fontsize=10,ha='center',va='center',fontweight='bold')
    ax.text(x+.095,.65,state,fontsize=12,ha='center',va='center')
for x in (.235,.485,.735):
    ax.annotate('',xy=(x+.035,.69),xytext=(x,.69),arrowprops={'arrowstyle':'->','color':'#52687b','lw':1.4})
ax.text(.035,.51,'What the preserved record establishes',fontsize=13,fontweight='bold',color='#142d45')
ax.text(.035,.45,'Native translation, root, incidence/isogeny, CLI and characteristic-p wrapper checks passed.\nThe additional driver reported 22 no-timer checks. Assertions remain unsigned.',fontsize=10.5,va='top',linespacing=1.5)
ax.text(.035,.33,'The setup stopped on an unsupported Git option before any timing row.\nThe raw archive was hash-checked, uploaded/read back in approved S3, and promoted to CAS.\nThe pod was deleted with HTTP 204. Separate temporary development-qualifier directories\nwere not archived; wrapper outputs and the main benchmark build were retained.',fontsize=10.5,va='top',linespacing=1.5)
ax.text(.035,.16,f"Recorded compute interval: {data['previous_compute_seconds']} seconds at USD 0.28/hour.\nEstimated compute fee: USD {data['previous_compute_cost_estimate_usd']:.5f}; storage/transfer separate.",fontsize=10.5,va='top',linespacing=1.5)
ax.text(.035,.045,'No speed ratio: there are no measured rows. Raw archive SHA-256: db2d28bd5543fc42…27dd3d1817fd6\nSources: attempt-1/{deployment,completion,cleanup,custody}.json; AMENDMENT_A2.json. [pcc:unverified]',fontsize=8.3,color='#52687b',va='top')
fig.savefig(root/'preflight.svg',bbox_inches='tight')
svg=root/'preflight.svg';svg.write_text('\n'.join(s.rstrip() for s in svg.read_text().splitlines())+'\n')
fig.savefig(root/'preflight.png',dpi=150,bbox_inches='tight')
with PdfPages(root/'PREFLIGHT_REPORT.pdf') as pdf: pdf.savefig(fig,bbox_inches='tight')
plt.close(fig)
