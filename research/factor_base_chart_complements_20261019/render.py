#!/usr/bin/env python3
"""Render recorded native counts only. No field/group/rank/result calculation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import MaxNLocator
root=Path(__file__).resolve().parent
summary=json.loads((root/"summary.json").read_text())
rows={row["id"]:row for row in summary["plot_examples"]}
plt.rcParams.update({"font.size":9,"svg.fonttype":"none"})
fig,axes=plt.subplots(1,2,figsize=(9,4.6),gridspec_kw={"width_ratios":[1.3,1]})
examples=[("B029","Coordinate"),("B031","GP seed17"),("B035","GP seed29"),("B027+B033","Two-column graft")]
for j,(key,label) in enumerate(examples):
    row=rows[key]
    axes[0].barh(j,row["support"],color="#ac632c" if "+" in key else "#365e87")
    axes[0].text(row["support"]+3,j,str(row["support"]),va="center")
axes[0].set_yticks(range(4),[f'{label} ({rows[key]["columns"]} cols)' for key,label in examples])
axes[0].invert_yaxis();axes[0].set_xlim(0,285);axes[0].set_xlabel("Supported nonidentity targets /2,002")
axes[0].set_title("Small graft: finite support")
examples2=[("B031","GP17"),("B041","GP43"),("B027+B033","Graft"),("B031+B041","Large union")]
for key,label in examples2:
    row=rows[key]
    axes[1].scatter(row["columns"],row["support"],s=50,color="#ac632c" if "+" in key else "#365e87")
    axes[1].annotate(label,(row["columns"],row["support"]),xytext={"B031":(6,-14),"B041":(6,6),"B027+B033":(-12,12),"B031+B041":(-65,5)}[key],textcoords="offset points",fontsize=8)
axes[1].set_xlim(10,43);axes[1].set_ylim(0,1150);axes[1].set_xlabel("Factor-base columns (count)")
axes[1].set_ylabel("Supported targets (count)");axes[1].set_title("Size remains an explicit cost")
for ax in axes:
    ax.spines[["top","right"]].set_visible(False);ax.grid(axis="x",alpha=.15);ax.set_axisbelow(True)
    ax.xaxis.set_major_locator(MaxNLocator(integer=True))
fig.suptitle("icv1-f2m13-t181-515ee569 · exact structural census",fontsize=11)
fig.text(.02,.025,"42 distinct nonempty bases; 313 unions; all pair memberships checked natively.\nPost-hoc graft example; chart branches, successful F4 work, full pricing and regularity are unmeasured.",fontsize=8)
fig.tight_layout(rect=(0,.14,1,.94))
fig.savefig(root/"chart-support.svg");fig.savefig(root/"chart-support.png",dpi=150)
p=root/"chart-support.svg";p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
