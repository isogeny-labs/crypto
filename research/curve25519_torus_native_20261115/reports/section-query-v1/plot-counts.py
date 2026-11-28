"""Layout only: render recorded native counters; no research computation."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
rows=list(csv.DictReader((root/"counts.csv").open()))
fig,ax=plt.subplots(figsize=(7.5,2.7))
values=[int(r["new_fp2_multiplications"]) for r in rows]
ax.bar([r["case"] for r in rows],values,color=["#6488aa","#3c7199","#21577f"])
ax.set_yscale("log")
ax.set_ylabel("New-arithmetic Fp2 mul calls\n(log scale; partial counters)")
for i,value in enumerate(values): ax.text(i,value,str(value),ha="center",va="bottom",fontsize=9)
ax.set_ylim(10000,4000000)
ax.set_title("One supplied-section control per case; no timing or full cold price")
ax.spines[["top","right"]].set_visible(False)
fig.tight_layout()
for suffix in ["svg","pdf"]: fig.savefig(root/f"counts.{suffix}")
plt.close(fig)
