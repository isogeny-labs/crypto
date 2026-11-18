"""Plot recorded counts only, no construction arithmetic or timing."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
s=json.loads((root/"summary-final.json").read_text())
rows=[r for r in s["rows"] if r["projective_batch_cap"]==0]
labels=[r["case_group"] for r in rows]
fig,ax=plt.subplots(figsize=(8.2,3.3),constrained_layout=True)
y=list(range(len(rows)))
ax.barh([i-0.18 for i in y],[int(r["evaluations"]) for r in rows],height=0.36,label="Norm evaluations",color="#427a9c")
ax.barh([i+0.18 for i in y],[int(r["preparations"]) for r in rows],height=0.36,label="Cached h preparations",color="#419274")
ax.set_yticks(y,labels);ax.invert_yaxis();ax.set_xlabel("Recorded call counts; different operations, no runtime ratio")
ax.set_title("Prepared context reuse · affine development controls")
ax.spines[["top","right"]].set_visible(False);ax.legend()
reports=root/"reports";reports.mkdir(exist_ok=True)
for ext in ("svg","png"):
    p=reports/("reuse."+ext);fig.savefig(p,dpi=160)
    if ext=="svg":p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
