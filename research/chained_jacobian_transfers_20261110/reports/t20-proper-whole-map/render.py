"""Lay out recorded counts and the report; no cryptographic computation."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst
p=Path(__file__).resolve().parent
subprocess.run(["dot","-Tsvg",str(p/"chain.dot"),"-o",str(p/"chain.svg")],check=True)
d=json.loads((p/"counts.json").read_text())
plt.rcParams.update({"svg.fonttype":"none","font.family":"DejaVu Sans","font.size":9})
fig,axes=plt.subplots(1,2,figsize=(7.7,2.0),layout="constrained")
for ax,key,value,title,unit in [
    (axes[0],"two_torsion","fixed_by_frobenius_cube","Frobenius³ fixed two-torsion","Classes (complete16 each)"),
    (axes[1],"omitted_basepoint","disagreements","Omitted-basepoint mutant","Disagreements (all points)")]:
    rows=d[key]
    bars=ax.bar([r["name"] for r in rows],[r[value] for r in rows],color=["#4f8b6b","#bf8740"])
    ax.bar_label(bars,padding=3)
    ax.set_title(title,fontsize=10)
    ax.set_ylabel(unit)
    ax.set_ylim(0,20 if key=="two_torsion" else 150)
    ax.spines[["top","right"]].set_visible(False)
fig.savefig(p/"counts.svg")
plt.close(fig)
for f in p.glob("*.svg"):
    f.write_text("\n".join(x.rstrip() for x in f.read_text().splitlines()).rstrip()+"\n")
typst.compile(str(p/"report.typ"),output=str(p/"report.pdf"))
