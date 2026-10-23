"""Lay out recorded values from results/summary.json and the raw rows as SVG
and PNG graphs. Rendering only (AGENTS.md "Implementation language: no
Python"): every plotted number is read from a file the Rust harness wrote.
Medians, ratios, slopes and verdicts come from `summary.json`
(`prime_smallness_graph_probe --arm s`); raw per-run points come from the
jsonl rows unchanged.

Usage: python plot_results.py RESULTS_DIR OUT_DIR
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

SERIES = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]
INK, INK2, GRID, SURFACE = "#0b0b0b", "#52514e", "#e4e3df", "#fcfcfb"
plt.rcParams.update({
    "font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK, "xtick.color": INK2,
    "ytick.color": INK2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.6,
    "axes.spines.top": False, "axes.spines.right": False, "figure.facecolor": SURFACE,
    "axes.facecolor": SURFACE, "legend.frameon": False, "lines.linewidth": 2,
})


def rows(path):
    out = []
    for line in Path(path).read_text().splitlines():
        v = json.loads(line)
        if "meta" not in v:
            out.append(v)
    return out


def save(fig, out, name):
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


def plot_g(summary, raw, out, run_id):
    g = summary["G"]
    sizes = sorted({c["bits"] for c in g["cells"]})
    fbs = sorted({c["fb_size"] for c in g["cells"]})
    rho = {r["bits"]: r["summary"] for r in g["rho"]}
    fig, axes = plt.subplots(1, len(sizes), figsize=(3.0 * len(sizes), 3.2), sharey=True)
    if len(sizes) == 1:
        axes = [axes]
    for ax, bits in zip(axes, sizes):
        for i, b in enumerate(fbs):
            cells = [c for c in g["cells"] if c["bits"] == bits and c["fb_size"] == b
                     and c["lp_shift"] is not None and c["valid"]]
            cells.sort(key=lambda c: c["lp_set_size"])
            xs = [c["lp_set_size"] for c in cells]
            ys = [c["median_S_w0"] for c in cells]
            if xs:
                ax.plot(xs, ys, color=SERIES[i], marker="o", markersize=4, label=f"B = {b}")
            # raw per-target points
            pts = [(r["lp_set_size"], r["S_w0"]) for r in raw
                   if r.get("arm") == "G" and not r.get("skipped") and r["bits"] == bits
                   and r["fb_size"] == b and r["lp_shift"] is not None and r["log_correct"]]
            if pts:
                ax.scatter(*zip(*pts), s=8, color=SERIES[i], alpha=0.35, linewidths=0)
            base = [c for c in g["cells"] if c["bits"] == bits and c["fb_size"] == b
                    and c["lp_shift"] is None and c["valid"]]
            if base:
                ax.axhline(base[0]["median_S_w0"], color=SERIES[i], linestyle=":", linewidth=1.2)
        if bits in rho:
            ax.axhline(rho[bits]["S_w0"], color=INK, linestyle="--", linewidth=1.2, label="matched rho")
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_title(f"{bits}-bit curve", color=INK)
        ax.set_xlabel("large-prime set size ℓ (x-values)")
    axes[0].set_ylabel("S = GAE / √n  (w = 0, median of 3)")
    axes[-1].legend(loc="upper right", fontsize=8)
    fig.suptitle("Arm G: large-prime graph IC-2 vs matched rho "
                 "(dotted: same-B baseline without large primes)", color=INK, fontsize=10)
    fig.text(0.01, 0.005, f"run {run_id}; faint dots = individual targets; all plotted runs log-verified",
             color=INK2, fontsize=7)
    fig.tight_layout(rect=(0, 0.03, 1, 0.95))
    save(fig, out, "arm_g_cost_vs_large_prime_set")


def plot_l(summary, out, run_id):
    series = summary["L"]["series"]
    keys = sorted({(s["bits"], s["level"]) for s in series})
    fig, axes = plt.subplots(1, len(keys), figsize=(2.6 * len(keys), 3.0), sharey=True)
    if len(keys) == 1:
        axes = [axes]
    for ax, (bits, level) in zip(axes, keys):
        for i, kind in enumerate(["Interval", "Fraction"]):
            s = [x for x in series if x["bits"] == bits and x["level"] == level and x["kind"] == kind]
            if not s:
                continue
            cells = sorted(s[0]["cells"], key=lambda c: c["delta_eff"])
            xs = [c["delta_eff"] for c in cells]
            ys = [c["successes"] / max(c["trials"], 1) for c in cells]
            ax.plot(xs, ys, color=SERIES[i], marker="o", markersize=4,
                    linestyle="-" if kind == "Interval" else "--", label=f"{kind} x-set")
        ax.axhline(0.5, color=INK2, linewidth=0.8, linestyle=":")
        ax.set_title(f"{bits}-bit, level {level}", color=INK)
        ax.set_xlabel("δ_eff = log_p |factor-base x-set|")
        ax.set_ylim(-0.05, 1.05)
    axes[0].set_ylabel("fraction of 16 trials with ≥ 2\nreduced rows vanishing at the root")
    axes[-1].legend(loc="lower left", fontsize=8)
    fig.suptitle(f"Arm L: Coppersmith reach on S₃, interval vs fraction factor base — verdict: "
                 f"{summary['L']['verdict']}", color=INK, fontsize=10)
    fig.text(0.01, 0.005, f"run {run_id}", color=INK2, fontsize=7)
    fig.tight_layout(rect=(0, 0.03, 1, 0.93))
    save(fig, out, "arm_l_reach")


def plot_f(summary, raw, out, run_id):
    fams = summary["F"]["families"]
    fig, axes = plt.subplots(1, 2, figsize=(7.6, 3.2))
    for i, fam in enumerate(["Interval", "Subgroup", "Fraction"]):
        cells = sorted(fams.get(fam, []), key=lambda c: c["B"])
        cells = [c for c in cells if c["completed"] > 0]
        xs = [c["B"] for c in cells]
        axes[0].plot(xs, [c["median_field_ops"] for c in cells], color=SERIES[i], marker="o",
                     markersize=4, label=fam)
        axes[1].plot(xs, [c["median_ratio_to_direct"] for c in cells], color=SERIES[i], marker="o",
                     markersize=4, label=fam)
    ds = sorted({(r["B"], r["direct_subtraction_mults"]) for r in raw if r.get("arm") == "F"})
    by_b = {}
    for b, d in ds:
        by_b.setdefault(b, []).append(d)
    if by_b:
        axes[0].plot(sorted(by_b), [min(by_b[b]) for b in sorted(by_b)], color=INK, linestyle="--",
                     linewidth=1.2, label="direct subtraction (derived)")
    axes[1].axhline(1.0, color=INK, linestyle="--", linewidth=1.2)
    for ax in axes:
        ax.set_xscale("log", base=2)
        ax.set_yscale("log")
        ax.set_xlabel("factor-base size B")
    axes[0].set_ylabel("F_p multiplications (median of 3)")
    axes[1].set_ylabel("F4 / direct subtraction")
    axes[0].legend(fontsize=8)
    fig.suptitle("Arm F: owner's F4 on ⟨S₃, L(x₁), L(x₂)⟩ for size-B fake subfields", color=INK, fontsize=10)
    fig.text(0.01, 0.005, f"run {run_id}; timed-out runs excluded from medians and listed in the report",
             color=INK2, fontsize=7)
    fig.tight_layout(rect=(0, 0.03, 1, 0.93))
    save(fig, out, "arm_f_f4_cost")


def main():
    res, out = Path(sys.argv[1]), Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    run_id = out.name
    summary = json.loads((res / "summary.json").read_text())
    if (res / "arm_g.jsonl").exists():
        plot_g(summary, rows(res / "arm_g.jsonl"), out, run_id)
    if (res / "arm_l.jsonl").exists():
        plot_l(summary, out, run_id)
    if (res / "arm_f.jsonl").exists():
        plot_f(summary, rows(res / "arm_f.jsonl"), out, run_id)


if __name__ == "__main__":
    main()
