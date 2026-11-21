#!/usr/bin/env python3
"""Plot the recorded d_F ladders (data/dF_rows.json) against N with the alpha* slope
lines of README section 3. Plots recorded values only."""
import json, sys, os, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
sys.path.insert(0, os.path.dirname(__file__))
from thresholds import Hinv
here=os.path.dirname(os.path.abspath(__file__)); root=os.path.dirname(here)
d=json.load(open(os.path.join(root,"data","dF_rows.json")))
fig,ax=plt.subplots(figsize=(8.2,5.2))
colors=["#1f5fbf","#c0392b","#2e8b57"]
for lad,col in zip(d["ladders"],colors):
    xs=[]; ys=[]; cx=[]; cy=[]
    for v,dF,cens in lad["points"]:
        N = 2*v if lad["x"]=="t" else lad["t"]*v
        (cx if cens else xs).append(N); (cy if cens else ys).append(dF)
    ax.plot(xs,ys,"o-",color=col,label=lad["name"],lw=1.6,ms=5)
    if cx:
        ax.plot(cx,cy,"^",color=col,mfc="none",ms=8,label=f"  censored lower bound (d_F >= value)")
# C-step line
ts=list(range(1,15)); ax.plot([2*t for t in ts],[4+(t-2)//4 for t in ts],":",color="#1f5fbf",lw=1,label="C-step 4+floor((t-2)/4) (m = 2, Amendment 6)")
# alpha* lines from N=4,d=4 anchor
for m,w,ls in ((4,2,"--"),(6,2,"-."),(10,1,(0,(1,1)))):
    a=Hinv((1-3/m)/w)
    ax.plot([4,30],[4,4+a*26],linestyle=ls,color="#555",lw=1.2,label=f"parity slope alpha* at m={m}, omega'={w}: {a:.3f}/bit")
ax.set_xlabel("N = m*t tower unknowns (one bit each; chain unknowns excluded)")
ax.set_ylabel("last fall degree d_F (lower median of 2 draws)")
ax.set_title("PKM Kummer towers at P1: recorded d_F against N, and the slopes parity would need\n(data: research/pkm_falldegree_unblock_20261107 RESULTS tables; thresholds: README section 3, derived)",fontsize=10)
ax.set_xlim(2,30); ax.set_ylim(2.5,9.5); ax.grid(alpha=.3)
ax.legend(fontsize=7.5,loc="upper left")
fig.tight_layout()
fig.savefig(os.path.join(root,"fig_slope.svg")); fig.savefig(os.path.join(root,"fig_slope.png"),dpi=150)
print("wrote fig_slope.svg/png")
