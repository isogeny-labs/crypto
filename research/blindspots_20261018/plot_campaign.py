"""Campaign graph: verdicts (rows) by level (columns); each square is an item.
Plots the verdict and level recorded in each item's README; no computation."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle

ITEMS = {  # item: (verdict, level) as recorded in the item README
 1:("REGRESSES","constant"), 2:("ADVANCES","exponent"), 3:("INCONCLUSIVE","none"),
 4:("ADVANCES","primitive"), 5:("ADVANCES","constant"), 6:("REGRESSES","constant"),
 7:("BLOCKED","primitive"), 8:("MATCHES","constant"), 9:("MATCHES","constant"),
 10:("INCONCLUSIVE","constant"), 11:("MATCHES","primitive"), 12:("MATCHES","primitive"),
 13:("TRADES","constant"), 14:("REGRESSES","primitive"), 15:("ADVANCES","primitive"),
 16:("ADVANCES","constant"), 17:("TRADES","constant"), 18:("INCONCLUSIVE","none"),
 19:("DELIVERED","none"), 20:("INCONCLUSIVE","constant")}
VERD = ["ADVANCES","TRADES","MATCHES","REGRESSES","INCONCLUSIVE","BLOCKED","DELIVERED"]
LEV = ["exponent","constant","primitive","none"]
COL = {"ADVANCES":"#2a7f4f","TRADES":"#b8860b","MATCHES":"#5b6f8a","REGRESSES":"#b03a2e",
       "INCONCLUSIVE":"#7f7f7f","BLOCKED":"#444444","DELIVERED":"#2f6f9f"}
fig, ax = plt.subplots(figsize=(8.6,4.8))
for vi,v in enumerate(VERD):
    for li,l in enumerate(LEV):
        ax.add_patch(Rectangle((li,len(VERD)-1-vi),1,1,fc="#f4f4f4",ec="#cccccc"))
        its=[i for i,(vv,ll) in ITEMS.items() if vv==v and ll==l]
        for k,i in enumerate(sorted(its)):
            x=li+0.06+(k%4)*0.23; y=len(VERD)-1-vi+0.55-(k//4)*0.4
            ax.add_patch(Rectangle((x,y),0.21,0.34,fc=COL[v],ec="white"))
            ax.text(x+0.105,y+0.17,f"{i:02d}",ha="center",va="center",color="white",fontsize=8,fontweight="bold")
ax.set_xlim(0,len(LEV)); ax.set_ylim(0,len(VERD))
ax.set_xticks([i+.5 for i in range(len(LEV))]); ax.set_xticklabels(LEV)
ax.set_yticks([len(VERD)-1-i+.5 for i in range(len(VERD))]); ax.set_yticklabels(VERD)
ax.xaxis.tick_top(); ax.tick_params(length=0)
for s in ax.spines.values(): s.set_visible(False)
ax.set_title("Blind-spot campaign 2026-10-18: verdict by level (20 items; one square per item)",fontsize=10,pad=28)
fig.text(0.01,0.01,"Source: each item README verdict and level. Item 02 is a derived model; 03 and 18 state no level; "
         "all other levels are toy-scale stage diagnostics.",fontsize=6.5)
fig.tight_layout(rect=(0,0.03,1,1))
fig.savefig("campaign_verdicts.svg"); fig.savefig("campaign_verdicts.png",dpi=150)
