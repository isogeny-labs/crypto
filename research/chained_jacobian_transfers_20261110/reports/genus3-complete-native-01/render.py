from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent
data=[x["value"] for x in json.loads((p/"records.json").read_text()) if x["kind"]=="G3_FIBRES"]
plt.rcParams.update({"svg.fonttype":"none","font.family":"DejaVu Sans","font.size":9})
fig,ax=plt.subplots(figsize=(7.2,2.6),layout="constrained")
for offset,key,label,color in [(-.25,"all_points","All source fibres","#245b86"),(0,"nonsplit","Nonsplit over K","#38a29d"),(.25,"repeated_or_overlapping","Repeated/overlapping support","#d0873c")]:
    bars=ax.bar([i+offset for i in range(len(data))],[x[key] for x in data],width=.24,label=label,color=color)
    ax.bar_label(bars,padding=2,fontsize=8)
ax.set_xticks(range(len(data)),["q="+str(x["q"]) for x in data]);ax.set_ylabel("Recorded source-point count");ax.set_ylim(0,1600);ax.legend(frameon=False,loc="upper left",fontsize=8);ax.spines[["top","right"]].set_visible(False)
ax.set_title("Complete fixed-panel fibre checks · unit03 c162f3 · unsigned",fontsize=10)
fig.savefig(p/"counts.svg");fig.savefig(p/"counts.png",dpi=150)
