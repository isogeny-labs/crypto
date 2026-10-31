"""Layout only: draws the study's figures from the recorded CSVs (computes no result).
usage: plot.py <a-run-dir> <b-run-dir> <out-dir>"""
import csv, glob, math, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

arun, brun, out = sys.argv[1:4]
os.makedirs(out, exist_ok=True)
COL = {"prime_with_kernel": "#d95f02", "prime_no_kernel": "#1b9e77", "composite": "#9e9e9e", "prime_other": "#7570b3"}
LAB = {"prime_with_kernel": "prime order, Tian kernel (positive)", "prime_no_kernel": "prime order, no kernel (negative control)", "composite": "composite order", "prime_other": "prime order, other"}

# ---- (a) hits against t3
qs = [7, 11, 13, 17, 19]
fig, axs = plt.subplots(len(qs), 1, figsize=(8.4, 9.6), sharex=False)
for ax, q in zip(axs, qs):
    rows = list(csv.DictReader(open(f"{arun}/census_q{q}.csv")))
    for r in rows:
        if r["realizable"] != "1" or r["constant_type"] == "1":
            continue
        t3 = int(r["t3"]); h = float(r["hits_per_1e6"])
        prime = r["prime"] == "1"
        g = "composite"
        if prime:
            st = r["round2_status"]
            if st == "na" and q > 13:
                st = {"kernel": "all", "none": "none"}.get(r["own_class"], "na")
            g = "prime_with_kernel" if st in ("all", "some") else ("prime_no_kernel" if st == "none" else "prime_other")
        ax.scatter(t3, max(h, 0.3), s=14 if prime else 6, c=COL[g], marker="o" if prime else ".", zorder=3 if prime else 1,
                   edgecolors="k" if (prime and h == 0) else "none", linewidths=0.6)
    ax.set_yscale("log"); ax.set_ylabel(f"q = {q}\nhits / 10$^6$")
    ax.grid(alpha=.3)
axs[-1].set_xlabel("trace t3 of E / F_{q^3}   (points at the floor have 0 hits)")
hs = [plt.Line2D([], [], marker="o", ls="", c=COL[k], label=LAB[k]) for k in ("prime_with_kernel", "prime_no_kernel", "composite")]
axs[0].legend(handles=hs, fontsize=7, loc="lower center", ncol=3)
fig.suptitle("Smooth plane quartics with L-polynomial of Res E, per trace t3 (random-sample census)", fontsize=10)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(f"{out}/fig_a_hits.{ext}", dpi=140)
plt.close(fig)

# ---- (a) ratio to local expectation by group
det = list(csv.DictReader(open(f"{arun}/summary_detail.csv")))
fig, ax = plt.subplots(figsize=(7.4, 3.6))
groups = ["prime_with_kernel", "prime_no_kernel", "composite"]
import random
random.seed(1)
for gi, g in enumerate(groups):
    for qi, q in enumerate(qs):
        vs = [float(r["ratio"]) for r in det if r["group"] == g and int(r["q"]) == q and r["ratio"] not in ("nan", "inf")]
        xs = [gi * 6 + qi + random.uniform(-.15, .15) for _ in vs]
        ax.scatter(xs, vs, s=8, c=COL[g], alpha=.7)
        if vs:
            ax.plot([gi * 6 + qi - .3, gi * 6 + qi + .3], [sum(vs) / len(vs)] * 2, c="k", lw=1.2)
ax.axhline(1, c="k", lw=.5, ls=":")
ax.set_xticks([gi * 6 + qi for gi in range(3) for qi in range(len(qs))])
ax.set_xticklabels([str(q) for _ in range(3) for q in qs], fontsize=7)
for gi, g in enumerate(groups):
    ax.text(gi * 6 + 2, ax.get_ylim()[1] * .93, LAB[g].split(" (")[0], ha="center", fontsize=7)
ax.set_ylabel("hits / local expectation"); ax.set_xlabel("q within group")
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(f"{out}/fig_a_ratio.{ext}", dpi=140)
plt.close(fig)

# ---- (b) costs
if brun and os.path.exists(f"{brun}/summary_cells.csv"):
    cells = list(csv.DictReader(open(f"{brun}/summary_cells.csv")))
    arms = {"ic": ("#d95f02", "index calculus"), "rho": ("#1b9e77", "rho"), "rhoneg": ("#7570b3", "rho + negation")}
    fig, axs = plt.subplots(1, 3, figsize=(10, 3.4), sharey=True)
    for ax, w in zip(axs, ("0.0", "0.1", "1.0")):
        for arm, (c, lab) in arms.items():
            pts = sorted((int(r["q"]), float(r["mean_cost_GAE"])) for r in cells if r["arm"] == arm and r["w"] == w and r["cI_rule"] == "3")
            if pts:
                ax.plot([p[0] for p in pts], [p[1] for p in pts], "o-", c=c, label=lab)
        ax.set_xscale("log"); ax.set_yscale("log"); ax.set_title(f"native op = {w} GAE (I = 3 M)", fontsize=9); ax.grid(alpha=.3)
        ax.set_xlabel("q")
    axs[0].set_ylabel("mean cost, group additions (GAE)")
    axs[0].legend(fontsize=7)
    fig.tight_layout()
    for ext in ("svg", "png"):
        fig.savefig(f"{out}/fig_b_cost.{ext}", dpi=140)
    plt.close(fig)
    rk = list(csv.DictReader(open(f"{brun}/summary_rank.csv")))
    fig, ax = plt.subplots(figsize=(6, 3.4))
    cs = ["w0_cI3", "w0.1_cI3", "w1_cI3", "w0_cI2log", "w0.1_cI2log", "w1_cI2log"]
    for i, cname in enumerate(cs):
        pts = sorted((int(r["q"]), float(r["ic_over_rho"])) for r in rk if r["cell"] == cname)
        ax.plot([p[0] for p in pts], [p[1] for p in pts], "o-", label=cname, ls="-" if "cI3" in cname else "--", c=plt.cm.viridis(i / 6))
    ax.axhline(1, c="k", lw=.6); ax.set_xscale("log"); ax.set_yscale("log")
    ax.set_xlabel("q"); ax.set_ylabel("IC cost / rho cost"); ax.legend(fontsize=6, ncol=2); ax.grid(alpha=.3)
    fig.tight_layout()
    for ext in ("svg", "png"):
        fig.savefig(f"{out}/fig_b_ratio.{ext}", dpi=140)
    plt.close(fig)
print("figures written to", out)
