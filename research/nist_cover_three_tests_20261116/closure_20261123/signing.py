"""Render recorded workflow states only; no native or curve computation."""
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

root=Path(__file__).resolve().parent
plt.rcParams["svg.fonttype"]="none"
fig,ax=plt.subplots(figsize=(8.5,1.7),layout="constrained")
ax.set(xlim=(0,10),ylim=(0,2));ax.axis("off")
for x,label,color in [(0.1,"Fixed replay\nPASS", "#e1f0e8"),(3.5,"Raw/report ZIPs\nDigests + CRCs match", "#e1f0e8"),(6.9,"Existing signer\nUNSET SECRET", "#f7e6dc")]:
 ax.add_patch(FancyBboxPatch((x,0.35),2.9,1.1,boxstyle="round,pad=0.03",facecolor=color,edgecolor="#355b75"))
 ax.text(x+1.45,0.9,label,ha="center",va="center",fontsize=10)
for x in (3.05,6.45):ax.annotate("",xy=(x+0.4,0.9),xytext=(x,0.9),arrowprops={"arrowstyle":"->"})
ax.set_title("Main workflow 45221063854 · fixed public fixtures · one execution",fontsize=11)
fig.savefig(root/"signing.svg");fig.savefig(root/"signing.png",dpi=100)
