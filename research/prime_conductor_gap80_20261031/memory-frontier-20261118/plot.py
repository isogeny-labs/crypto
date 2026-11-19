"""Plot recorded model metadata only; no research arithmetic or timers."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
s=json.loads((root/"summary.json").read_text());t=s["target_cases"][0]
fig,(ax,bx)=plt.subplots(1,2,figsize=(9,3.6),constrained_layout=True)
values=[float(t["cap_bytes"]),float(t["all_window_single_rectangle_minimum_bytes"])]
ax.bar(["Example cap","Best one-rectangle\nnecessary bound"],[v/1e12 for v in values],color=["#78858c","#427a9c"])
ax.set_ylabel("TB (decimal) · coefficient-slot model only")
ax.set_title("Single rectangle · every baby window")
for i,v in enumerate(values):ax.text(i,v/1e12+0.12,f"{v/1e12:.3f}",ha="center")
contexts=[float(t["all_window_contexts_minimum_under_cap"])]+[float(r["coordinate_contexts"]) for r in s["target_cases"]]
bars=bx.bar(["Lower bound\nunder cap","Balanced\nblock 6M","Zero leftover\nblock 6M"],contexts,color=["#78858c","#427a9c","#419274"])
bars[0].set_hatch("//")
bx.set_yscale("log");bx.set_ylabel("Loop coordinate contexts · log scale")
bx.set_title("Current traversal · no time conversion")
for a in (ax,bx):a.spines[["top","right"]].set_visible(False)
fig.suptitle("Pinned-source necessary bounds · not RSS, timings or universal algorithm limits")
reports=root/"reports";reports.mkdir(exist_ok=True)
for ext in ("svg",):
    p=reports/("frontier."+ext);fig.savefig(p,dpi=150)
    if ext=="svg":p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines())+"\n")
