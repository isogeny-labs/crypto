"""Figures from results/summary.json (editable source for the rendered SVG/PNG in figures/).

    python3 make_figures.py

Palette: first six categorical slots of the validated reference palette (light mode),
checked with the dataviz skill's validate_palette.js (adjacent CVD dE 9.1, normal-vision
dE 19.6; three slots under 3:1 contrast on the surface, so every bar carries a legend,
direct labels, and the numbers are in the tables of RESULTS.md).
"""
import hashlib
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, Patch

import analysis

HERE = Path(__file__).resolve().parent
OUT = HERE / "figures"
SUMMARY = HERE / "results" / "summary.json"
S = json.loads(SUMMARY.read_text())

SURFACE, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e3e2de"
SLOT = {"fb": "#2a78d6", "eq": "#eb6834", "solver": "#1baf7a", "lvr": "#eda100", "load": "#e87ba4", "input": "#008300"}
LABEL = {"fb": "factor-base setup", "eq": "equation build", "solver": "solver stage (setup + solve + extraction)",
         "lvr": "lifting + verification + rank update", "load": "engine library load (outside A)",
         "input": "input generation (outside A)"}
ENG = analysis.ENGINES
EL = analysis.ENGINE_LABEL
plt.rcParams.update({"svg.fonttype": "none", "font.family": "DejaVu Sans", "font.size": 9,
                     "figure.facecolor": SURFACE, "axes.facecolor": SURFACE, "savefig.facecolor": SURFACE,
                     "text.color": INK, "axes.labelcolor": INK2, "xtick.color": INK2, "ytick.color": INK2,
                     "axes.edgecolor": GRID, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": False, "hatch.linewidth": 0.8})


def med(group, engine, key):
    return S["w1"]["groups"][group]["engines"][engine]["median"][key]


def parts(engine, group="W1a"):
    g = lambda s: med(group, engine, f"cpu_s.per_attempt.stage.{s}") * 1000
    return {"fb": g("fb_setup"), "eq": g("eq_build"),
            "solver": g("solver_setup") + g("solve") + g("model_extraction"),
            "lvr": g("lifting") + g("verification") + g("rank_update"),
            "load": g("engine_load"), "input": g("input_gen")}


def style(ax, xgrid=True):
    ax.tick_params(length=0)
    if xgrid:
        ax.xaxis.grid(True, color=GRID, linewidth=0.8)
        ax.set_axisbelow(True)


def save(fig, name):
    fig.savefig(OUT / f"{name}.svg", format="svg")
    fig.savefig(OUT / f"{name}.png", dpi=150)
    plt.close(fig)


def fig_w1a():
    order = ["solver", "fb", "eq", "lvr", "load", "input"]
    ys = list(range(len(ENG)))[::-1]
    fig, (a, b) = plt.subplots(1, 2, figsize=(11.5, 4.9), gridspec_kw={"wspace": 0.08, "width_ratios": [1.15, 1]})
    for ax in (a, b):
        style(ax)
    for y, e in zip(ys, ENG):
        p = parts(e)
        left = 0.0
        for k in order:
            hatch = "////" if k in ("load", "input") else None
            a.barh(y, p[k], left=left, height=0.38, color=SLOT[k], edgecolor=SURFACE, linewidth=1.5,
                   hatch=hatch)
            left += p[k]
        a.text(left + 3, y, f"{left:.0f} ms incl. hatched", va="center", color=INK2, fontsize=8.5)
        core = {k: v for k, v in p.items() if k not in ("load", "input")}
        total = sum(core.values())
        left = 0.0
        for k in order[:4]:
            b.barh(y, core[k] / total * 100, left=left, height=0.38, color=SLOT[k], edgecolor=SURFACE, linewidth=1.5)
            left += core[k] / total * 100
        b.text(101.5, y, f"f_s {med('W1a', e, 'cpu_s.f_s') * 100:.1f}%", va="center", color=INK2, fontsize=8.5)
    a.set_yticks(ys, [EL[e] for e in ENG])
    b.set_yticks(ys, [])
    a.set_xlabel("mean CPU ms per input, accounting A (solid) + outside A (hatched)")
    b.set_xlabel("share of accounting-A total cost (%)")
    b.set_xlim(0, 125)
    b.set_xticks([0, 25, 50, 75, 100])
    a.set_xlim(0, 340)
    a.set_title("Cost per input (absolute)", loc="left", fontsize=10, color=INK)
    b.set_title("Where the cost goes (share of T_core)", loc="left", fontsize=10, color=INK)
    handles = [Patch(facecolor=SLOT[k], edgecolor=SURFACE, hatch=("////" if k in ("load", "input") else None),
                     label=LABEL[k]) for k in order]
    fig.legend(handles=handles, loc="lower center", ncol=3, frameon=False, fontsize=8.5, bbox_to_anchor=(0.5, 0.045))
    fig.suptitle("W1a fixed-phase S4 workload, m=83: CPU cost per input by stage "
                 "(12 inputs, median over 3 repetitions; 8 natural + 4 planted)", x=0.01, ha="left",
                 fontsize=10.5, color=INK)
    fig.text(0.01, 0.012, "Bars use per-stage medians over repetitions; f_s is the median of per-repetition solver "
             "shares (table in RESULTS.md section 2), so bar proportions can differ from it by under 1 point.",
             fontsize=7.2, color=INK2)
    fig.subplots_adjust(left=0.2, right=0.97, top=0.87, bottom=0.31)
    save(fig, "cost_decomposition_w1a")


def fig_accounting():
    keys = [("cpu_s.f_s", "A: cold factor-base setup per input (preregistered)", SLOT["fb"]),
            ("cpu_s.f_s_B", "B: setup amortised over the inputs sharing it", SLOT["eq"]),
            ("cpu_s.f_s_C", "C: fully loaded (+ library load + input generation)", SLOT["lvr"]),
            ("cpu_s.ss.f_s", "post hoc steady state: natural attempts only, no setup", SLOT["solver"])]
    fig, ax = plt.subplots(figsize=(8.6, 4.7))
    style(ax)
    ys = list(range(len(ENG)))[::-1]
    for y, e in zip(ys, ENG):
        vals = [med("W1a", e, k) * 100 for k, _, _ in keys]
        ax.plot([min(vals), max(vals)], [y, y], color=GRID, linewidth=2, zorder=1)
    for k, lab, col in keys:
        ax.scatter([med("W1a", e, k) * 100 for e in ENG], ys, s=64, color=col, edgecolor=SURFACE, linewidth=2,
                   zorder=3, label=lab)
    ax.set_yticks(ys, [EL[e] for e in ENG])
    ax.set_xlim(-2, 102)
    ax.set_xlabel("solver stage as % of total cost (CPU), W1a, median over 3 repetitions")
    ax.set_title("The solver share depends on what is charged and amortised",
                 loc="left", fontsize=10, color=INK)
    ax.legend(loc="upper center", frameon=False, fontsize=8, ncol=2, bbox_to_anchor=(0.45, -0.17))
    fig.subplots_adjust(left=0.25, right=0.97, top=0.9, bottom=0.27)
    save(fig, "solver_share_by_accounting")


def fig_w2():
    if "w2" not in S:
        return
    fig, axes = plt.subplots(1, 2, figsize=(11, 3.8), gridspec_kw={"wspace": 0.25})
    for ax, e in zip(axes, ("sat-cms-xor", "sat-z3")):
        style(ax)
        rows = S["w2"]["engines"][e]["rows"]
        for i, r in enumerate(rows):
            st = r["stages"]
            s_solver = sum(st[k]["cpu_s"] for k in ("solver_setup", "solve", "model_extraction"))
            rest = {"fb": st["fb_setup"]["cpu_s"], "eq": st["eq_build"]["cpu_s"],
                    "lvr": sum(st[k]["cpu_s"] for k in ("lifting", "verification", "rank_update"))}
            left = 0.0
            for k, v in (("solver", s_solver), ("fb", rest["fb"]), ("eq", rest["eq"]), ("lvr", rest["lvr"])):
                ax.barh(i, v, left=left, height=0.4, color=SLOT[k], edgecolor=SURFACE, linewidth=1.5)
                left += v
            tag = r["result"] or r["status"]
            ax.text(max(left + 1.5, 122.5), i, f"{left:.1f} s, {tag}", va="center", color=INK2, fontsize=8.5)
        ax.set_yticks(range(len(rows)), [f"target {r['index']}" for r in rows])
        ax.invert_yaxis()
        ax.axvline(120, color=INK2, linewidth=1, linestyle=(0, (4, 3)))
        ax.text(121.5, -0.42, "120 s deadline", fontsize=7.5, color=INK2, va="center")
        ax.set_xlim(0, 160)
        ax.set_xlabel("CPU s per target (accounting A)")
        ax.set_title(EL[e], loc="left", fontsize=10, color=INK)
    handles = [Patch(facecolor=SLOT[k], edgecolor=SURFACE, label=LABEL[k]) for k in ("solver", "fb", "eq", "lvr")]
    fig.legend(handles=handles, loc="lower center", ncol=4, frameon=False, fontsize=8.5, bbox_to_anchor=(0.5, -0.02))
    fig.suptitle("W2 n=13 phase-aware subgroup_xor system, four natural targets, 120 s solver deadline",
                 x=0.01, ha="left", fontsize=10.5, color=INK)
    fig.subplots_adjust(left=0.08, right=0.97, top=0.82, bottom=0.27)
    save(fig, "cost_decomposition_w2")


def fig_diagram():
    p = parts("sat-cms-xor")
    stages = [("fb", "factor-base\nsetup", p["fb"]), ("input", "input\ngeneration*", p["input"]),
              ("eq", "equation\nbuild", p["eq"]), ("load", "engine\nload*", p["load"]),
              ("solver", "SOLVER STAGE\nsetup, solve,\nextraction", p["solver"]),
              ("lvr", "lift, verify,\nrank update", p["lvr"])]
    fig, ax = plt.subplots(figsize=(11.5, 4.4))
    ax.set_xlim(0, 100)
    ax.set_ylim(0, 46)
    ax.axis("off")
    x, w, gap = 1.0, 11.2, 2.4
    centers = []
    for k, lab, ms in stages:
        hatched = k in ("input", "load")
        ax.add_patch(FancyBboxPatch((x, 21), w, 12, boxstyle="round,pad=0.2,rounding_size=1.0", linewidth=1.2,
                                    edgecolor=INK2 if hatched else SURFACE, facecolor=SLOT[k],
                                    hatch="////" if hatched else None, linestyle="--" if hatched else "-"))
        ax.text(x + w / 2, 27, lab, ha="center", va="center", fontsize=8.2, color=INK,
                bbox=dict(facecolor=SURFACE, edgecolor="none", alpha=0.85, pad=1.5))
        ax.text(x + w / 2, 18.2, f"{ms:.1f} ms", ha="center", va="center", fontsize=9, color=INK)
        centers.append(x + w / 2)
        if k != "lvr":
            ax.annotate("", xy=(x + w + gap - 0.2, 27), xytext=(x + w + 0.2, 27),
                        arrowprops=dict(arrowstyle="-|>", color=INK2, lw=1))
        x += w + gap
    ax.text(1, 43.5, "CryptoMiniSat on W1a: mean CPU per input, stage by stage (measured, median of 3 repetitions)",
            fontsize=10, color=INK, va="center")
    ax.text(1, 39.5, "solid boxes: measured, inside accounting A.   dashed + hatched (*): measured, outside A.",
            fontsize=8, color=INK2, va="center")
    share = med("W1a", "sat-cms-xor", "cpu_s.f_s") * 100
    ax.plot([centers[4] - 5.6, centers[4] + 5.6], [16.2, 16.2], color=SLOT["solver"], lw=2)
    ax.text(centers[4], 13.6, f"solver stage = {share:.1f}% of accounting-A cost", ha="center", fontsize=8.8, color=INK2)
    rate = med("W1a", "sat-cms-xor", "natural.new_independent")
    n = med("W1a", "sat-cms-xor", "n.natural_completed")
    expect = med("W1a", "sat-cms-xor", "natural.expected_cpu_s_per_relation_derived")
    ax.add_patch(FancyBboxPatch((1, 1.0), 98, 9.0, boxstyle="round,pad=0.2,rounding_size=1.0", linewidth=1,
                                edgecolor=INK2, facecolor=SURFACE))
    ax.text(3, 7.2, f"Yield (measured): {int(rate)} independent verified natural relations in {int(n)} natural "
            f"attempts, so cost per relation is UNDEFINED.", fontsize=8.8, color=INK, va="center")
    ax.text(3, 3.6, f"Derived (not a measurement): under uniform targets, at least {expect:.1e} CPU s per relation.",
            fontsize=8.8, color=INK2, va="center")
    save(fig, "pipeline_diagram")


if __name__ == "__main__":
    OUT.mkdir(exist_ok=True)
    fig_w1a()
    fig_accounting()
    fig_w2()
    fig_diagram()
    (OUT / "figures_manifest.json").write_text(json.dumps({
        "summary_sha256": hashlib.sha256(SUMMARY.read_bytes()).hexdigest(),
        "files": sorted(p.name for p in OUT.glob("*.svg")),
        "source": "make_figures.py", "palette_validated": "dataviz validate_palette.js, light mode, 6 slots"},
        indent=1) + "\n")
    print("figures written:", sorted(p.name for p in OUT.iterdir()))
