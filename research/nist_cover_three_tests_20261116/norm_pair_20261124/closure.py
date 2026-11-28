"""Render dimensions from already-recorded native audit JSON; no arithmetic research."""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
raw=json.loads((root/"evidence/aggregate-v1/result.json").read_text())
old=json.loads((root/"evidence/qualification-v1/input/baseline.json").read_text())
plt.rcParams["svg.fonttype"]="none"
fig,(counts,closed)=plt.subplots(1,2,figsize=(8.7,2.8),layout="constrained",gridspec_kw={"width_ratios":[1.45,1]})
x=[0,1]
bars=counts.bar([-0.18,0.82],[raw["old_columns"],raw["total_columns"]],width=.32,label="Columns",color="#879ba9")
counts.bar_label(bars,padding=2)
bars=counts.bar([.18,1.18],[old["rank_mod_r"],raw["full_rank"]],width=.32,label="Full rank",color="#2e6c8e")
counts.bar_label(bars,padding=2)
counts.set_xticks(x,["Original","Whole traces"])
counts.set_ylim(0,540)
counts.set_ylabel("Matrix dimensions")
counts.legend(frameon=False,fontsize=8)
bars=closed.bar(["Original","Whole traces"],[old["rank_mod_r"],raw["closed_rank"]],color="#2e6c8e")
closed.bar_label(bars,padding=3)
closed.set_ylim(0,8)
closed.set_ylabel("Closed old-base rank")
for ax in (counts,closed):ax.spines[["top","right"]].set_visible(False)
fig.savefig(root/"closure.svg")
fig.savefig(root/"closure.png",dpi=100)
p=root/"closure.svg"
p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
