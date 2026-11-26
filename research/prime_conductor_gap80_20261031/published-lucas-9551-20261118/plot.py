"""Plot recorded certificate status and sizes; no mathematical search or timing."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
s=json.loads((root/"summary.json").read_text())
prior=json.loads((root.parent/"published-lucas-20261118/summary.json").read_text())
status="certificate replayed" if s["certificate_replayed"] else "certificate incomplete"
labels=["Frozen crossing target", "Saved certified n=7351", "Fixed published n=9551"]
values=[80,prior["conductor_bits"],s["conductor_bits"]]
fig,ax=plt.subplots(figsize=(8.2,3.1),constrained_layout=True)
bars=ax.barh(labels,values,color=["#49779a","#49779a","#388a75"])
for bar,value in zip(bars,values):
    ax.text(value+40,bar.get_y()+bar.get_height()/2,str(value),va="center")
ax.set_xlim(0,max(values)*1.13)
ax.set_xlabel("Recorded bit length; no runtime or evaluated-crossing comparison")
ax.set_title("Fixed conductor arithmetic: "+status)
ax.spines[["top","right"]].set_visible(False)
ax.invert_yaxis()
reports=root/"reports";reports.mkdir(exist_ok=True)
for ext in ("svg","png"):
    p=reports/("sizes."+ext);fig.savefig(p,dpi=170)
    if ext=="svg":
        p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
