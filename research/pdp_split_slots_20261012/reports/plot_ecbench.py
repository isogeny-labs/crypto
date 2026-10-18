"""Lay out the ecbench rows of an ANALYSIS.json (native `pdp_split_analyze
--ecbench`) as SVG figures. Layout only: every number is read from the
analysis file, which the native tool computed from the session records.

    python plot_ecbench.py ANALYSIS.json OUT_DIR [TITLE_PREFIX]
"""

import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("svg")
import matplotlib.pyplot as plt  # noqa: E402

# Reference palette slots 1-3 (dataviz skill; validated all-pairs, light).
BASE_COLOR = {"std": "#2a78d6", "clo": "#eb6834", "inv": "#1baf7a", "pdp": "#1baf7a"}
INK, INK2, GRID = "#0b0b0b", "#52514e", "#e4e3df"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK,
                     "xtick.color": INK2, "ytick.color": INK2, "axes.grid": True,
                     "grid.color": GRID, "grid.linewidth": 0.6, "svg.fonttype": "none"})


def arm_style(arm):
    base = arm.split("-")[0]
    color = BASE_COLOR.get(base, INK2)
    if "mitm" in arm:
        marker = "P"
    elif "chain" in arm:
        marker = "s"
    elif "g8" in arm or "g4" in arm or "unsplit" in arm or arm.startswith("pdp"):
        marker = "o"
    else:
        marker = "D"
    return color, marker


def main():
    analysis = json.loads(Path(sys.argv[1]).read_text())
    out = Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    prefix = sys.argv[3] if len(sys.argv) > 3 else ""
    rows = analysis["ecbench"]["rows"]
    curves = sorted({(r["n"], r["curve_slug"]) for r in rows})
    arms = sorted({r["arm"] for r in rows if not r["arm"].startswith("rho")})
    fig, axes = plt.subplots(1, 2, figsize=(12, 4.6), sharey=False)
    for ax, key, title in ((axes[0], "median_s_recorded", "recorded S (F4 arms: lower bound, XORs uncharged)"),
                           (axes[1], "median_priced_s", "priced S (F4 XORs at the pinned ratio; a sensitivity)")):
        for xi, (n, slug) in enumerate(curves):
            byarm = {r["arm"]: r for r in rows if r["n"] == n}
            for rho, ls in (("rho-strong", "-"), ("rho-frob", (0, (2, 2)))):
                if rho in byarm:
                    ax.hlines(byarm[rho]["median_s_recorded"], xi - 0.42, xi + 0.42,
                              color=INK2, linewidth=2, linestyle=ls)
            for k, arm in enumerate(arms):
                r = byarm.get(arm)
                if not r or r.get(key) is None:
                    continue
                color, marker = arm_style(arm)
                vals = [w["s"] if key == "median_s_recorded" else w["priced_s"] for w in r["per_workload"]]
                xs = [xi + (k - (len(arms) - 1) / 2) * 0.11] * len(vals)
                ax.scatter(xs, vals, s=14, color=color, marker=marker, alpha=0.35, linewidth=0)
                ax.scatter(xs[0], r[key], s=46, color=color, marker=marker, edgecolor="white",
                           linewidth=1.0, zorder=3)
        ax.set_yscale("log")
        ax.set_xticks(range(len(curves)))
        ax.set_xticklabels([f"{slug}\nn = {n}" for n, slug in curves], fontsize=7)
        ax.set_title(title, fontsize=9, color=INK)
    axes[0].set_ylabel("S = charged group-addition equivalents / √r (log)")
    handles = [plt.Line2D([], [], color=arm_style(a)[0], marker=arm_style(a)[1], linestyle="", label=a)
               for a in arms]
    handles += [plt.Line2D([], [], color=INK2, linewidth=2, label="strong rho (measured)"),
                plt.Line2D([], [], color=INK2, linewidth=2, linestyle=(0, (2, 2)), label="signed-Frobenius rho (measured)")]
    fig.legend(handles=handles, loc="lower center", ncol=min(6, len(handles)), frameon=False, fontsize=7)
    fig.suptitle(f"{prefix}ecbench arms per curve: medians (large) and per-workload values (small), "
                 "8 public targets", fontsize=9, color=INK)
    fig.tight_layout(rect=(0, 0.12, 1, 0.95))
    fig.savefig(out / "fig-ecbench-s.svg")
    plt.close(fig)

    ladder = analysis.get("ladder", {}).get("rows", [])
    if ladder:
        fig, ax = plt.subplots(figsize=(7, 4.2))
        series = sorted({(r["n"], r["plan"]) for r in ladder})
        for (n, plan), color in zip(series, ["#2a78d6", "#eb6834", "#1baf7a"] * 3):
            pts = sorted((r["residual_unknowns"], r["mean_xors_per_branch"]) for r in ladder
                         if r["n"] == n and r["plan"] == plan)
            ax.plot([p[0] for p in pts], [p[1] for p in pts], marker="o", color=color,
                    linewidth=2, markersize=6, label=f"n = {n}, {plan}, F4 (measured)")
        ks = sorted({r["residual_unknowns"] for r in ladder})
        k0, c0 = ks[0], min(r["mean_xors_per_branch"] for r in ladder if r["residual_unknowns"] == ks[0])
        ax.plot(ks, [c0 * 2 ** (k - k0) for k in ks], color=INK2, linestyle=(0, (2, 2)), linewidth=1.5,
                label="2× per unknown (enumeration), anchored at the first point")
        ax.set_yscale("log")
        ax.set_xlabel("residual Boolean unknowns after guessing, k")
        ax.set_ylabel("mean F4 word XORs per branch (log)")
        ax.legend(fontsize=7, frameon=False)
        ax.set_title(f"{prefix}F4 cost per branch against residual unknowns (recorded values)", fontsize=9)
        fig.tight_layout()
        fig.savefig(out / "fig-ladder.svg")
        plt.close(fig)


if __name__ == "__main__":
    main()
