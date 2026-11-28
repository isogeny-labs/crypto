"""Plot only already-recorded synthetic native results."""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
record=json.loads((root/"evidence/qualification-v2/result.json").read_text())
labels=["Private\ncolumns","Shared\nalias","Signed\nalias","Unsigned\ntriangle","Signed\ntriangle","Canceled\nnew part"]
values=[case["proof"]["gain"] for case in record["cases"]]
plt.rcParams["svg.fonttype"]="none"
fig,ax=plt.subplots(figsize=(8.1,2.4),layout="constrained")
bars=ax.bar(labels,values,color="#316e8c")
ax.bar_label(bars,padding=3)
ax.set_ylim(0,1.4)
ax.set_yticks([0,1])
ax.set_ylabel("Additional closed rank")
ax.set_title("Synthetic coefficient controls; no curve collection executed",fontsize=10)
ax.spines[["top","right"]].set_visible(False)
fig.savefig(root/"controls.svg")
fig.savefig(root/"controls.png",dpi=100)
p=root/"controls.svg"
p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
