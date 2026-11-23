# Plots recorded values from raw/fit.txt only (card 04 validation, n=23..29, 16 instances).
import re,sys,matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
D="research/blindspots_20261018/08-learned-sat-branching"
rows=[]
for l in open(f"{D}/raw/fit.txt"):
    m=re.match(r"fit: validation mean log2 cost: (.*)",l)
    if not m: continue
    s=m.group(1)
    if s.startswith("cube"):
        mm=re.match(r"cube (\S+) (dcut=\d) (k=\d) (\S+)",s); rows.append((f"cube {mm[1]} {mm[2]} {mm[3]}",float(mm[4])))
    else:
        for a,b in re.findall(r"(\w+)\s+([\d.]+)",s): rows.append((a,float(b)))
fig,ax=plt.subplots(figsize=(7,5))
ax.barh([r[0] for r in rows][::-1],[r[1] for r in rows][::-1],color="#4c78a8")
inc=[v for n,v in rows if n=="identity"][0]; ax.axvline(inc,color="k",ls="--",label=f"identity (incumbent) {inc:.2f}")
ax.set_xlim(19,21); ax.set_xlabel("validation mean log2 cost (unitless search-tree nodes), 16 instances, n=23-29")
ax.set_title("Blind spot 08: exploratory validation only (card 04); primary test n=31-37 NOT run",fontsize=9)
ax.legend(); fig.tight_layout(); fig.savefig(f"{D}/graph-validation.svg"); fig.savefig(f"{D}/graph-validation.png",dpi=150)
