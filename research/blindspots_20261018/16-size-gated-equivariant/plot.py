# Lays out values recorded in raw/analysis-g16.jsonl (output of examples/blindspots_16_gate.rs). No computation of results.
import json, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
rows=[json.loads(l) for l in open("raw/analysis-g16.jsonl")]
sizes=[5,7,11,13]
def get(label,n):
    for r in rows:
        if r["kind"]=="ratio" and r["label"]==label and r["n"]==n: return r
series=[("held a/c (gated vs incumbent)","gated / incumbent, CPU (rule arm)","#1f77b4"),
        ("held a/b (always-equivariant vs incumbent)","always-equivariant / incumbent, CPU","#d62728"),
        ("held F/C (attribution, not in the rule)","F/C (fall rows only vs equivariant)","#2ca02c"),
        ("held a/a2 (A/A)","A/A (I vs I2), noise floor","#7f7f7f"),
        ("held counted a/c, GF product = 1 XOR","gated / incumbent, counted XORs (w=1)","#9467bd")]
fig,ax=plt.subplots(figsize=(8,4.8))
for k,(lab,name,c) in enumerate(series):
    xs=[i+(k-2)*0.12 for i in range(4)]
    g=[get(lab,n)["gm"] for n in sizes]; lo=[get(lab,n)["gm"]-get(lab,n)["lo"] for n in sizes]; hi=[get(lab,n)["hi"]-get(lab,n)["gm"] for n in sizes]
    ax.errorbar(xs,g,yerr=[lo,hi],fmt="o",color=c,label=name,capsize=3)
ax.axhline(1,color="k",lw=.8); ax.set_yscale("log")
ax.set_xticks(range(4)); ax.set_xticklabels([f"n={n}\n{5*n+6} barrel unknowns\n{'gate off (I)' if n<11 else 'gate on (C)'}" for n in sizes])
ax.set_ylabel("geometric-mean ratio (>1: first arm costs more)")
ax.set_title("Item 16, held-out targets t=12..19 (8 per size), N0=61 tuned on training t=0..11\nbarrel gauge-fixed oracle, toy sizes; 95% bootstrap CI",fontsize=9)
ax.legend(fontsize=7,loc="upper left"); fig.tight_layout()
fig.savefig("graph.svg"); fig.savefig("graph.png",dpi=150)
