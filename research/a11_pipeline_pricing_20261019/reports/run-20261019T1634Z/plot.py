import csv, math, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
rows=list(csv.DictReader(open("data.csv")))
fig,ax=plt.subplots(figsize=(5.5,3.6))
for d,c,mk in (("A0","#1f77b4","o"),("A11","#d95f02","s")):
    r=[x for x in rows if x["design"]==d]
    ax.plot([int(x["t"]) for x in r],[math.log2(int(x["field_ops"])) for x in r],marker=mk,color=c,label=d+(" (tower, incumbent presentation)" if d=="A0" else " (norm chain)"))
ax.set_xlabel("tower height t (m = 3, p1 = 786433)"); ax.set_ylabel("log2 F4 field multiplications")
ax.set_xticks([2,3,4]); ax.legend(fontsize=7); ax.set_title("Recorded F4 field_ops per system (stage diagnostic, not S)",fontsize=8)
fig.tight_layout(); fig.savefig("graph.svg"); fig.savefig("graph.png",dpi=160)
