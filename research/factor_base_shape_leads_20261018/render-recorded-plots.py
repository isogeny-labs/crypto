#!/usr/bin/env python3
"""Lay out recorded native values only; no cryptographic/result computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import MaxNLocator
root=Path(__file__).resolve().parent
obs=json.loads((root/"observations.json").read_text())
coverage=json.loads((root/"exact-coverage.json").read_text())
cov={r["id"]:r for r in coverage["rows"]}
plt.rcParams.update({"font.size":9,"svg.fonttype":"none"})
def save(fig,name):
    fig.savefig(root/(name+".svg"))
    fig.savefig(root/(name+".png"),dpi=150)
    p=root/(name+".svg")
    p.write_text("\n".join(x.rstrip() for x in p.read_text().splitlines())+"\n")
def clean(ax):
    for sp in ["top","right"]:ax.spines[sp].set_visible(False)
    ax.grid(axis="x",alpha=.18);ax.set_axisbelow(True)
    ax.xaxis.set_major_locator(MaxNLocator(integer=True))
def label(r):
    c=r["configuration"]
    name=(c.get("shape") or r["label"].split("-")[-1]).replace("tracezero-","tz-").replace("frobenius","Frob")
    return f'{name} d{c["dimension"]} s{c["seed"]} · {r["columns"]} cols'
rows=[r for r in obs["candidates"] if r["panel"]=="primary"]
fig,ax=plt.subplots(figsize=(8.2,4.4))
for j,r in enumerate(rows):
    value=cov[r["id"]]["prime_target_coverage"]
    ax.barh(j,value,color="#335f91")
    ax.text(value+.03,j,str(value),va="center")
ax.set_yticks(range(len(rows)),[r["label"].replace("primary-","") for r in rows])
ax.invert_yaxis();ax.set_xlim(0,2.7)
ax.set_xlabel("Exhaustive supported nonidentity subgroup targets (count / 1,439,392)")
ax.set_title("Primary: icv1-f2m31-tm90707-c95f16f5",fontsize=11)
clean(ax)
fig.text(.02,.025,"5 sealed candidates; every old whole-curve control passed; 0/24 random successes per candidate.\nCoverage is a finite census, not a cheap oracle, regularity certificate or speed result.",fontsize=8)
fig.tight_layout(rect=(0,.14,1,1));save(fig,"primary")
fig,axes=plt.subplots(3,1,figsize=(8.2,13),gridspec_kw={"height_ratios":[4,4,1.2]})
groups=[("icv1-f2m7-tm13-ac10a42c",lambda r:r["configuration"]["curve"]["n"]==7),
        ("icv1-f2m13-t181-515ee569",lambda r:r["configuration"]["curve"]["n"]==13),
        ("icv1-f2m9-tm5-4a3ea183 · composite control",lambda r:r["configuration"]["curve"]["n"]==9)]
for ax,(title,pred) in zip(axes,groups):
    cases=[r for r in obs["candidates"] if r["panel"]=="secondary" and pred(r)]
    for j,r in enumerate(cases):
        if r["outcome"]=="completed":
            ax.barh(j-.16,r["holdout"]["verified"],height=.3,color="#335f91",label="Holdout /32" if j==0 else None)
            ax.barh(j+.16,r["training_independent_rank"],height=.3,color="#cb892a",label="Train rank" if j==0 else None)
        else:ax.text(.2,j,"FAILED: empty inventory",va="center",color="#a33434",fontsize=8)
    ax.set_yticks(range(len(cases)),[label(r) for r in cases]);ax.invert_yaxis()
    ax.set_xlim(0,34);ax.set_xlabel("Count");ax.set_title(title,fontsize=10)
    clean(ax)
axes[0].legend(loc="lower right",fontsize=8)
fig.text(.02,.015,"Screen: target seed 2026101801; 16 train/32 holdout per completed case. Repeated coordinate sets are not new bases.\nDifferent cardinalities are explicit. Counts are descriptive; no uncertainty interval, regularity or speed claim.",fontsize=8)
fig.tight_layout(rect=(0,.055,1,1));save(fig,"secondary")
rows=[r for r in obs["candidates"] if r["panel"]=="confirmation"]
fig,axes=plt.subplots(1,2,figsize=(9,5.4),sharey=True)
for j,r in enumerate(rows):
    axes[0].barh(j,r["holdout"]["verified"],color="#335f91")
    axes[0].text(r["holdout"]["verified"]+.4,j,str(r["holdout"]["verified"]),va="center")
    value=cov[r["id"]]["prime_target_coverage"]
    axes[1].barh(j,value,color="#39785b")
    axes[1].text(value+5,j,str(value),va="center")
axes[0].set_yticks(range(len(rows)),[label(r) for r in rows]);axes[0].invert_yaxis()
axes[0].set_xlim(0,35);axes[1].set_xlim(0,550)
axes[0].set_xlabel("Verified holdout draws /128")
axes[1].set_xlabel("Exact subgroup coverage /2,002")
for ax in axes:clean(ax)
fig.suptitle("Fresh confirmation: icv1-f2m13-t181-515ee569",fontsize=12)
fig.text(.02,.025,"Target seed 2026101802; 122 distinct holdout targets per case. GP seeds17/29 have15 columns; seed43 has21.\nExact support is independently enumerated with a second native arithmetic implementation.\nObserved successful solving degree remains3. This is yield/support evidence, not cheap solving or an end-to-end win.",fontsize=8)
fig.tight_layout(rect=(0,.18,1,.95));save(fig,"confirmation")
comparisons=obs["representation_comparisons"]
fig,axes=plt.subplots(2,1,figsize=(8.2,6.8))
for ax,(name,prefix,count) in zip(axes,[("icv1-f2m7-tm13-ac10a42c","representation-a1-n7",83),
                                    ("icv1-f2m13-t181-515ee569","representation-a0-n13",5)]):
    pairs=[r for r in comparisons if r["baseline"].startswith(prefix)]
    values=[pairs[0]["baseline_successful_f4_multiplications"]]+[r["candidate_successful_f4_multiplications"] for r in pairs]
    for j,(name2,value) in enumerate(zip(["natural","reversed","triangular"],values)):
        ax.barh(j,value,color="#335f91" if j==0 else "#ba7b45")
        ax.text(value*1.01,j,f"{value:,}",va="center",fontsize=8)
    ax.set_yticks(range(3),["natural","reversed","triangular"]);ax.invert_yaxis()
    ax.set_xlim(0,max(values)*1.25);ax.ticklabel_format(style="plain",axis="x")
    ax.set_title(f"{name}: 96 matched random probes, {count} successful",fontsize=10)
    ax.set_xlabel("F4 reduction multiplications on the same successful probes")
    clean(ax)
fig.text(.02,.02,"Exact signed point sets and target outcomes match; positive-control work is excluded.\nAll reversed/triangular aggregates exceed the natural basis. One reversed basis raises training rank10→11.\nNeither changed basis passes the preregistered strict per-success work-improvement filter. Total pricing is incomplete.",fontsize=8)
fig.tight_layout(rect=(0,.14,1,1));save(fig,"representation")
