#!/usr/bin/env python3
"""Reach of the JV weak class: class-group walk vs rational-root walk, by prime (Wilson 95%)."""
import json
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
a=json.load(open("analysis.json"))
primes=["37","53","1009","1511","1823"]
fig,(ax,bx)=plt.subplots(1,2,figsize=(10,4.3),gridspec_kw={"width_ratios":[1.5,1]})
x=np.arange(len(primes)); w=0.36
def bars(ax,key,dx,color,label,hatch=None):
    ys,los,his,xs=[],[],[],[]
    for i,p in enumerate(primes):
        d=a["primes"][p].get(key)
        if not d or d.get("walked",0)==0: continue
        ys.append(d["frac"]); los.append(d["frac"]-d["ci"][0]); his.append(d["ci"][1]-d["frac"]); xs.append(i+dx)
    ax.bar(xs,ys,w,color=color,label=label,hatch=hatch,edgecolor="white")
    ax.errorbar(xs,ys,yerr=[los,his],fmt="none",ecolor="#333",capsize=3,lw=1)
def bars2(ax,dx,color,label,getter):
    ys,los,his,xs=[],[],[],[]
    for i,p in enumerate(primes):
        d=getter(p)
        if not d or d.get("walked",0)==0: continue
        ys.append(d["frac"]); los.append(d["frac"]-d["ci"][0]); his.append(d["ci"][1]-d["frac"]); xs.append(i+dx)
    ax.bar(xs,ys,w,color=color,label=label,edgecolor="white")
    ax.errorbar(xs,ys,yerr=[los,his],fmt="none",ecolor="#333",capsize=3,lw=1)
# small primes: cap 10k / 60 walks (apples-to-apples with the published RR); large primes: paired cap 3k
cg_get=lambda p: a["primes"][p].get("cg")
rr_get=lambda p: a["primes"][p].get("rr_published") if p in ("37","53") else a["primes"][p].get("rr_paired")
bars2(ax,-w/2,"#c0392b","class-group walk (this study)",cg_get)
bars2(ax,+w/2,"#7f8c8d","rational-root walk",rr_get)
ax.axvline(1.5,color="#999",ls=":",lw=1)
ax.set_xticks(x); ax.set_xticklabels([f"p={p}" for p in primes])
ax.set_ylabel("admitted classes reaching a weak curve (Wilson 95%)")
ax.set_ylim(0,1.0); ax.grid(alpha=0.3,axis="y"); ax.legend(fontsize=8,loc="upper right")
ax.set_title("Weak-class reach: class-group vs rational-root odd isogenies",fontsize=9)
ax.text(0.02,0.96,"p=37,53: cap 10,000, 60 walks",transform=ax.transAxes,fontsize=7,va="top",color="#555")
ax.text(0.62,0.96,"p≥1009: paired, cap 3,000\n(weak locus beyond feasible cap)",
        transform=ax.transAxes,fontsize=7,va="top",color="#555")
# right: odd kernel-polynomial steps are what reaches the weak curve
od=[]; lab=[]
for p in ("37","53"):
    cg=a["primes"][p]["cg"]; od.append(cg["odd_used"]/max(1,cg["found"])); lab.append(f"p={p}")
bx.bar(range(len(od)),od,0.5,color="#2471a3")
bx.set_xticks(range(len(od))); bx.set_xticklabels(lab); bx.set_ylim(0,1.05)
bx.set_ylabel("found walks using ≥1 odd kernel-polynomial step")
bx.grid(alpha=0.3,axis="y"); bx.set_title("The new isogenies do the work",fontsize=9)
for i,v in enumerate(od): bx.text(i,v+0.02,f"{v:.0%}",ha="center",fontsize=9)
fig.tight_layout(); fig.savefig("reach.svg"); fig.savefig("reach.png",dpi=200)
