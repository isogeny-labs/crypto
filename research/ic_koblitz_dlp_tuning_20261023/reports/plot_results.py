"""Plot recorded/derived values only (research-visuals: no computation of results here).

Inputs: stage1/paired-n*.json, holdout/paired-n*.json (outputs of examples/ic_paired_band.rs),
selection.json. Outputs: reports/fig_*.svg and .png.
"""
import json, os, sys, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
NS = [37, 41, 43, 47]
W = [0.0, 0.1, 1.0]
COL = {"base": "#4c78a8", "dlp": "#e45756", "ret": "#54a24b", "rho": "#7f7f7f"}


def load(p):
    p = os.path.join(ROOT, p)
    return json.load(open(p)) if os.path.exists(p) else None


def save(fig, name):
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(os.path.join(HERE, f"{name}.{ext}"), bbox_inches="tight", **kw)
    plt.close(fig)


# Figure 1: holdout paired ratios with 95% CI, per curve and weight.
def fig_ratio():
    fig, axes = plt.subplots(1, 4, figsize=(13, 3.6), sharey=True)
    for ax, n in zip(axes, NS):
        d = load(f"holdout/paired-n{n}.json")
        ax.axhline(1.0, color="k", lw=0.8)
        ax.set_title(f"n = {n}")
        ax.set_xticks(range(3)); ax.set_xticklabels([f"w={w:g}" for w in W])
        ax.set_yscale("log")
        if d is None:
            ax.text(0.5, 0.5, "not run", transform=ax.transAxes, ha="center"); continue
        for off, (a, b, c, lab) in enumerate([("base-frozen", "dlp-selected", COL["dlp"], "dlp-selected / base-frozen"),
                                              ("base-frozen", "base-retuned", COL["ret"], "base-retuned / base-frozen")]):
            for p in d["pairs"]:
                if p["a"] == a and p["b"] == b:
                    for i, cell in enumerate(p["weights"]):
                        if cell.get("ratio_b_over_a") is None: continue
                        r, ci = cell["ratio_b_over_a"], cell["ci95"]
                        x = i + (off - 0.5) * 0.2
                        ax.errorbar([x], [r], yerr=[[r - ci[0]], [ci[1] - r]] if ci else None, fmt="o", color=c, capsize=3,
                                    label=lab if i == 0 else None)
        if n == NS[0]:
            ax.set_ylabel("paired ratio of summed S (<1: candidate cheaper)")
        ax.legend(fontsize=6, loc="best")
    fig.suptitle("Holdout: paired S ratio to the frozen baseline, 95% cluster-bootstrap CI (counts, L0)", y=1.05)
    save(fig, "fig_holdout_ratio")


# Figure 2: distance to matched rho (S / S_rho-frob) per arm.
def fig_rho():
    fig, axes = plt.subplots(1, 4, figsize=(13, 3.6), sharey=True)
    names = ["base-frozen", "base-retuned", "dlp-selected"]
    cols = [COL["base"], COL["ret"], COL["dlp"]]
    for ax, n in zip(axes, NS):
        d = load(f"holdout/paired-n{n}.json")
        ax.set_title(f"n = {n}"); ax.set_yscale("log")
        ax.set_xticks(range(3)); ax.set_xticklabels([f"w={w:g}" for w in W])
        if d is None:
            ax.text(0.5, 0.5, "not run", transform=ax.transAxes, ha="center"); continue
        arms = {a["arm"]: a for a in d["arms"]}
        for j, (nm, c) in enumerate(zip(names, cols)):
            if nm not in arms or "rho-frob" not in arms: continue
            ys = []
            for i in range(3):
                x, y = arms[nm]["mean_priced_s"][i], arms["rho-frob"]["mean_priced_s"][i]
                ys.append(x / y if x and y else float("nan"))
            ax.bar([i + (j - 1) * 0.27 for i in range(3)], ys, width=0.26, color=c, label=nm)
        ax.axhline(1.0, color="k", lw=0.8)
        if n == NS[0]: ax.set_ylabel("S(arm) / S(rho-frob), same weight")
        ax.legend(fontsize=6)
    fig.suptitle("Distance remaining to the matched rho (rho = 1), holdout means, priced at w = 0, 0.1, 1", y=1.05)
    save(fig, "fig_distance_to_rho")


# Figure 3: stage-1 minimax objective over the (K, c) grid.
def fig_grid():
    fig, axes = plt.subplots(1, 4, figsize=(13, 3.6))
    import re
    for ax, n in zip(axes, NS):
        d = load(f"stage1/paired-n{n}.json")
        ax.set_title(f"n = {n}")
        if d is None:
            ax.text(0.5, 0.5, "not run", transform=ax.transAxes, ha="center"); continue
        Ks = sorted({int(re.match(r"dlp-k(\d+)-c", a["arm"]).group(1)) for a in d["arms"] if a["arm"].startswith("dlp-k")})
        Cs = sorted({float(re.match(r"dlp-k\d+-c([\d.]+)", a["arm"]).group(1)) for a in d["arms"] if a["arm"].startswith("dlp-k")})
        grid = [[float("nan")] * len(Cs) for _ in Ks]
        for a in d["arms"]:
            m = re.match(r"dlp-k(\d+)-c([\d.]+)", a["arm"])
            if m and a.get("minimax_vs_base"):
                grid[Ks.index(int(m.group(1)))][Cs.index(float(m.group(2)))] = math.log10(a["minimax_vs_base"])
        im = ax.imshow(grid, aspect="auto", cmap="RdYlGn_r", vmin=-0.3, vmax=1.5)
        ax.set_xticks(range(len(Cs))); ax.set_xticklabels([f"{c:g}" for c in Cs], fontsize=6)
        ax.set_yticks(range(len(Ks))); ax.set_yticklabels(Ks, fontsize=7)
        ax.set_xlabel("c = |D|/|F|"); ax.set_ylabel("K columns")
    fig.colorbar(im, ax=axes, label="log10 minimax ratio to base (stage 1, 8 targets)", shrink=0.8)
    fig.suptitle("Stage 1 (prune): worst band ratio of each double-LP cell to the frozen baseline (0 = parity)", y=1.05)
    save(fig, "fig_stage1_grid")


for f in (fig_ratio, fig_rho, fig_grid):
    f()
print("figures written")
