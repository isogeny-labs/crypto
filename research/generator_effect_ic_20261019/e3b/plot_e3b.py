"""Plot E3b (E3b-PROTOCOL.md) from aggregated values only.

Reads e3b/summary.json (generator_effect_e3_summary on the ladder sessions),
../e3/summary.json (E3's sessions, for the t = 2..5 points) and
e3b/f4check.json (generator_effect_e3b_f4check).  Computes nothing beyond
selecting and laying out recorded or aggregated numbers.
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
L = json.load(open(os.path.join(HERE, "summary.json")))
E3 = json.load(open(os.path.join(HERE, "..", "e3", "summary.json")))
F = json.load(open(os.path.join(HERE, "f4check.json")))

NAMES = {
    10398721: "e3-c4", 15052801: "e3-c5", 15161857: "e3-c6",
    9061313: "e3-c1", 13040257: "e3-c2", 11223809: "e3-c3",
}
MARK = {"e3-c4": "o", "e3-c5": "s", "e3-c6": "^",
        "e3-c1": "o", "e3-c2": "s", "e3-c3": "^"}
GRID, KL, RHO, F4C = "#1f77b4", "#d62728", "#555555", "#9467bd"


def ladder_fig():
    fig, axes = plt.subplots(2, 3, figsize=(15, 8.5), sharex=True,
                             gridspec_kw={"height_ratios": [3, 2]})
    for wi, w in enumerate(L["weights"]):
        ax, axr = axes[0][wi], axes[1][wi]
        for src, faint in ((E3, True), (L, False)):
            for c in src["curves"]:
                name = NAMES.get(c["p"])
                if name is None:
                    continue
                m, a = MARK[name], (0.35 if faint else 1.0)
                rows = c["rows"]
                bs = [r["B"] for r in rows]
                for key, col in (("grid", GRID), ("kl", KL)):
                    ys = [r[key]["by_w"][wi]["mean_s"] for r in rows]
                    lo = [r[key]["by_w"][wi]["ci90"][0] for r in rows]
                    hi = [r[key]["by_w"][wi]["ci90"][1] for r in rows]
                    ax.errorbar(bs, ys, yerr=[[y - l for y, l in zip(ys, lo)],
                                              [h - y for y, h in zip(ys, hi)]],
                                marker=m, color=col, alpha=a, ms=4, lw=1,
                                capsize=2)
                if faint:
                    f4 = [(r["B"], f["by_w"][wi]["mean_s"]) for r in rows
                          for f in r["f4"] if f["presentation"] == "chain"]
                    ax.plot([b for b, _ in f4], [s for _, s in f4], ls="--",
                            marker=m, color=F4C, alpha=a, ms=4, lw=1)
                rho = c["rho"]["by_w"][wi]["mean_s"]
                ax.hlines(rho, min(bs), max(bs), colors=RHO, linestyles=":",
                          alpha=a)
                rat = [r["grid_vs_kl"][wi]["grid_over_kl"] for r in rows]
                axr.errorbar(bs, [x["ratio"] for x in rat],
                             yerr=[[x["ratio"] - x["ci90"][0] for x in rat],
                                   [x["ci90"][1] - x["ratio"] for x in rat]],
                             marker=m, color=GRID, alpha=a, ms=4, lw=1,
                             capsize=2)
        ax.set_yscale("log")
        ax.set_xscale("log")
        ax.set_title(f"w = {w} GAE per unpriced native op")
        ax.grid(alpha=0.3, which="both")
        axr.axhline(1, color="k", lw=0.8)
        axr.set_yscale("log")
        axr.set_xscale("log")
        axr.set_xlabel("B (columns of prime-kummer:t, = known-log size)")
        axr.grid(alpha=0.3, which="both")
    axes[0][0].set_ylabel("S = GAE / sqrt(r), full solve (mean of 8)")
    axes[1][0].set_ylabel("grid / known-log, paired")
    handles = [
        plt.Line2D([], [], color=GRID, marker="o", label="grid reference (measured)"),
        plt.Line2D([], [], color=KL, marker="o", label="known-log MITM m=3 (measured)"),
        plt.Line2D([], [], color=F4C, ls="--", marker="o",
                   label="F4 chain, E3 t=2..5 (substituted, not run)"),
        plt.Line2D([], [], color=RHO, ls=":", label="rho-neg (measured, per curve)"),
        plt.Line2D([], [], color="k", alpha=0.35, marker="o",
                   label="faint: E3 curves c1-c3, t=2..5"),
        plt.Line2D([], [], color="k", marker="o",
                   label="solid: E3b ladder c4-c6, t=4..9 (o/s/^ = c4/c5/c6)"),
    ]
    fig.legend(handles=handles, loc="lower center", ncol=3, fontsize=8.5,
               frameon=False)
    sess = ", ".join(c["session"] for c in L["curves"])
    fig.suptitle(f"E3b ladder: grid reference vs known-log MITM at equal B "
                 f"(sessions {sess})", fontsize=11)
    fig.tight_layout(rect=(0, 0.07, 1, 0.97))
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(os.path.join(HERE, f"e3b_ladder.{ext}"), **kw)


def f4_fig():
    runs = F["runs"]
    fig, (a1, a2) = plt.subplots(1, 2, figsize=(12, 4.5))
    labels = [f"{r['arm']}\ntarget {r['target_index']}" for r in runs]
    x = range(len(runs))
    for wi, (w, col) in enumerate(zip(F["weights"], ("#999999", F4C, "#2ca02c"))):
        a1.bar([i + (wi - 1) * 0.27 for i in x],
               [r["by_w"][wi]["ratio"] for r in runs], width=0.27, color=col,
               label=f"w = {w}")
    a1.axhspan(0.99, 1.01, color="#2ca02c", alpha=0.15, label="F2 band [0.99, 1.01]")
    a1.set_xticks(list(x))
    a1.set_xticklabels(labels, fontsize=7)
    a1.set_ylabel("S measured / S substituted (E3's formula)")
    lo = min(r["by_w"][wi]["ratio"] for r in runs for wi in range(3))
    hi = max(r["by_w"][wi]["ratio"] for r in runs for wi in range(3))
    a1.set_ylim(min(0.97, lo - 0.005), max(1.03, hi + 0.005))
    a1.legend(fontsize=8)
    a1.set_title("F4 run to full solves vs E3's substitution")
    a2.bar(list(x), [r["f4_muladds_per_call"] for r in runs], color=F4C,
           label="measured, this run")
    a2.scatter(list(x), [r["e3_capped_mean_per_call"] for r in runs],
               color="k", marker="x", zorder=3, label="E3 capped mean")
    a2.set_yscale("log")
    a2.set_xticks(list(x))
    a2.set_xticklabels(labels, fontsize=7)
    a2.set_ylabel("F4 multiply-adds per call")
    a2.legend(fontsize=8)
    a2.set_title("F4 cost per call")
    nv = sum(r["status"] == "verified" for r in runs)
    fig.suptitle(f"E3b Part F: e3-c1, F4 on every trial; {nv} of {len(runs)} "
                 f"runs verified (sessions in f4check.json)", fontsize=10)
    fig.tight_layout()
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(os.path.join(HERE, f"e3b_f4.{ext}"), **kw)


if __name__ == "__main__":
    ladder_fig()
    f4_fig()
