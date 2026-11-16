"""Plots recorded values only (runs/SCORE.tsv, runs/AMENDMENT1_SCORE.txt). Computes nothing new."""
import csv, re, sys
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
rows=[r for r in csv.DictReader([l for l in open("runs/SCORE.tsv") if not l.startswith(("SKIPPED","OUTCOME"))],delimiter="\t") if r.get("arm") and r["n"].isdigit()]
arms=["poly","frob","rsub","pmix","pint"]; col={"poly":"#1f77b4","frob":"#2ca02c","rsub":"#7f7f7f","pmix":"#ff7f0e","pint":"#d62728"}
fig,ax=plt.subplots(1,2,figsize=(12,4.8),gridspec_kw={"width_ratios":[3,2]})
xs={17:0,19:1,23:2}
for i,a in enumerate(arms):
    rr=[r for r in rows if r["arm"]==a]
    x=[xs[int(r["n"])]+(i-2)*0.13 for r in rr]; y=[float(r["Z"])/float(r["null_max"]) for r in rr]
    ax[0].scatter(x,y,s=18,c=col[a],label=a,alpha=.8,edgecolors="none")
ax[0].axhline(1,ls="--",c="k",lw=.8); ax[0].set_xticks([0,1,2],["n=17","n=19","n=23"])
ax[0].set_yscale("log"); ax[0].set_ylabel("Z_arm / max Z over matched nulls (log)")
ax[0].set_title("Primary run: one cell per point; above dashed line = beats every null")
ax[0].legend(fontsize=8,ncol=5,loc="upper left")
t=open("runs/AMENDMENT1_SCORE.txt").read()
r2=[(int(m[0]),float(m[1]),float(m[2])) for m in re.findall(r"R2 rsub seed (\d+) fb=\d+ Z=([\d.]+) null_max=([\d.]+)",t)]
ax[1].scatter([s for s,_,_ in r2],[z/m for _,z,m in r2],c="#7f7f7f",s=22,label="fresh rsub seeds 6-25 (R2)")
zf=float(re.search(r"flagged Z=([\d.]+)",t)[1]); mx=float(re.search(r"max=([\d.]+)",t)[1])
ax[1].scatter([5],[zf/mx],c="#9467bd",marker="*",s=120,label="flagged rsub 5 vs 200 fresh nulls (R1)")
ax[1].axhline(1,ls="--",c="k",lw=.8); ax[1].set_xlabel("rsub seed (n=23, K1)"); ax[1].set_ylabel("Z / null max")
ax[1].set_title("Amendment 1 (R2): 0/20 non-flat"); ax[1].legend(fontsize=8)
fig.tight_layout()
for e in ("svg","png","pdf"): fig.savefig(f"reports/run-20261116/spectrum.{e}",dpi=150)
