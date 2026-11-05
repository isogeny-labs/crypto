import csv
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
rows=list(csv.DictReader(open("ratios.csv")))
q=[float(r["q"]) for r in rows[1:]]; g=[float(r["geomean_vs_q0"]) for r in rows[1:]]
lo=[float(r["ci_lo"]) for r in rows[1:]]; hi=[float(r["ci_hi"]) for r in rows[1:]]
fig,ax=plt.subplots(figsize=(6.5,4))
ax.errorbar(q,g,yerr=[[a-b for a,b in zip(g,lo)],[b-a for a,b in zip(g,hi)]],fmt="o-",capsize=3)
ax.axhline(1,color="gray",ls="--",label="q=0 (pure m=2 cycle stop)")
ax.set_xscale("log");ax.set_yscale("log")
ax.set_xlabel("q (fraction of 2+1 source-B trials used)");ax.set_ylabel("calls to stop / q=0 (paired geomean, 95% CI)")
ax.set_title("Mixed m=2 / m=3 relations, a=0 n=37 d=12 (6 seeds)");ax.legend()
fig.tight_layout();fig.savefig("mix_ratio.svg");fig.savefig("mix_ratio.png",dpi=150)
