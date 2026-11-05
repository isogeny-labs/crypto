from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent;d=json.loads((p/"plot-data.json").read_text());r=d["rows"]
fig,ax=plt.subplots(figsize=(8,3.3));m=[x["m"] for x in r];lo=[x["lower"] for x in r];hi=[x["upper"] for x in r]
ax.fill_between(m,lo,hi,color="#dbeaf3",label="Unresolved interval")
ax.plot(m,lo,"o-",color="#176583",label="Certified lower bound")
ax.plot(m,hi,"s--",color="#ad641b",label="Certified upper bound")
ax.set_xticks(m);ax.set_xlabel("Decomposition length m");ax.set_ylabel("Original top-form degree")
ax.legend(frameon=False,loc="upper left");ax.spines[["top","right"]].set_visible(False)
ax.set_title("Koblitz and general binary: same interval in all paired cases")
fig.tight_layout();fig.savefig(p/"bounds.svg");plt.close(fig)
f=p/"bounds.svg";f.write_text("\n".join(x.rstrip() for x in f.read_text().splitlines())+"\n")
