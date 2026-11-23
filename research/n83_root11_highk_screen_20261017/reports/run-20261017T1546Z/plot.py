from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent
s=json.loads((p/"raw/summary.json").read_text()); a=s["rows"][0]
b=json.loads((p/"raw/baseline_k3072.json").read_text())
fig,axes=plt.subplots(1,2,figsize=(9.1,3.0))
colors=["#476a89","#b7772e"]
values=[b["online_geometric_mean_ms"],a["geometric_target_ms"]]
axes[0].bar(["K3072\nprior run","K3584"],values,color=colors)
axes[0].set_ylabel("Geometric target interval (ms)");axes[0].set_ylim(0,23)
for i,v in enumerate(values):axes[0].text(i,v+.35,f"{v:.3f}",ha="center")
values=[b["cold_process_ms"]/1000,a["cold_process_seconds"]]
axes[1].bar(["K3072\nprior run","K3584"],values,color=colors)
axes[1].set_ylabel("Shared-setup process\n(seconds)");axes[1].set_ylim(0,1000)
for i,v in enumerate(values):axes[1].text(i,v+14,f"{v:.3f}",ha="center")
for ax in axes:ax.spines[["top","right"]].set_visible(False);ax.grid(axis="y",alpha=.15);ax.set_axisbelow(True)
fig.suptitle("N83 root11 high-K screen | 128 points per completed process",fontsize=11)
fig.text(.5,.01,"Historical shared-host observations; no A/A control or admitted speedup. Failed and unrun arms are not zeros.",ha="center",fontsize=8)
fig.tight_layout(rect=(0,.07,1,.95))
for ext in ["svg","png"]:fig.savefig(p/("observations."+ext),dpi=150,bbox_inches="tight")
svg=p/"observations.svg";svg.write_text("\n".join(x.rstrip() for x in svg.read_text().splitlines())+"\n")
