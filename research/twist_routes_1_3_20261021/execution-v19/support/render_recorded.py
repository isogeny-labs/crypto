#!/usr/bin/env python3
"""Lay out already recorded native statistics. No arithmetic/research analysis."""
import hashlib
import json
import os
from pathlib import Path

os.environ.setdefault("MPLCONFIGDIR", "/workspace/shared/twist-report-mpl-20261030")
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import MaxNLocator, NullFormatter, ScalarFormatter

ROOT = Path("research/twist_routes_1_3_20261021")
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "svg.fonttype": "none"})

def digest(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def save(fig, directory, stem, source):
    for ext in ("svg", "pdf", "png"):
        fig.savefig(directory / (stem + "." + ext), dpi=150, bbox_inches="tight", facecolor="white")
    plt.close(fig)
    (directory / (stem + ".SOURCES.json")).write_text(json.dumps({
        "schema": "twist.recorded-plot-sources/v1", "source": str(source),
        "source_sha256": digest(source), "renderer_sha256": digest(__file__),
        "operations": "Layout of native recorded counts, medians and range endpoints only; no new aggregation, inference or solver work.",
        "png_dpi": 150,
        "files": {ext: digest(directory / (stem + "." + ext)) for ext in ("svg", "pdf", "png")}
    }, indent=2) + "\n")

def outcomes(data, directory, source, key, title, subtitle):
    values = data[key]
    names = list(values)
    fig, ax = plt.subplots(figsize=(10.4, max(4.3, len(names) * .31)))
    colors = {"recovered": "#217b68", "ambiguous": "#7a8698", "underfilled": "#be7b21", "unsupported-alpha": "#a95748", "capped": "#b1393c"}
    ax.barh(names, [values[k] for k in names], color=[colors.get(k.split("|")[-1], "#4d7193") for k in names])
    ax.invert_yaxis()
    for i, k in enumerate(names):
        ax.text(values[k], i, "  " + str(values[k]), va="center", fontsize=9)
    ax.set_xlabel("Recorded executions (count)")
    ax.set_title(title + "\n" + subtitle, loc="left", fontsize=13, pad=16)
    ax.spines[["top", "right"]].set_visible(False)
    ax.grid(axis="x", alpha=.18)
    ax.set_axisbelow(True)
    fig.tight_layout()
    save(fig, directory, "OUTCOMES", source)

def ranges(ax, entries, title, label, linear=False):
    shown = [(name, v) for name, v in entries if v is not None]
    for i, (name, v) in enumerate(shown):
        ax.plot([v["min"], v["max"]], [i, i], color="#51728e", lw=2)
        ax.plot(v["median"], i, "o", color="#153d60", ms=6)
        ax.text(.99, i, "n=" + str(v["n"]), transform=ax.get_yaxis_transform(), ha="right", va="bottom", fontsize=8)
    ax.set_yticks(list(range(len(shown))), [x[0] for x in shown])
    ax.invert_yaxis()
    ax.axvline(1, color="#ae5b41", ls="--", lw=1)
    if linear:
        ax.xaxis.set_major_locator(MaxNLocator(5))
        ax.xaxis.set_major_formatter(ScalarFormatter(useOffset=True))
    else:
        ax.set_xscale("log")
        ax.xaxis.set_minor_formatter(NullFormatter())
    ax.set_xlabel(label)
    ax.set_title(title, loc="left", fontsize=12)
    ax.spines[["top", "right"]].set_visible(False)
    ax.grid(axis="x", alpha=.18)

def main_plot(number):
    names = {16: "cold-sizing-20261029", 17: "complete-cold-compression-20261029", 18: "larger-heldout-cost-20261029", 19: "sparse-compression-20261030"}
    wave = ROOT / ("execution-v" + str(number))
    source = wave / ("analysis-v2/SUMMARY.json" if number in (17, 18) else "analysis-v1/SUMMARY.json")
    data = json.loads(source.read_text())
    directory = ROOT / "reports" / names[number]
    directory.mkdir(parents=True, exist_ok=True)
    if number == 19:
        outcomes(data, directory, source, "status_by_arm", "Sparse substitution: all planned outcomes", "612 profiles; 24 fresh public targets and 12 Q2 controls; existing 12 toy models")
        r = data["fresh_target_ratios"]
        labels = [(k.replace("-over-", " / "), v) for k, v in r.items()]
        fig, ax = plt.subplots(1, 2, figsize=(12.2, 5.1))
        ranges(ax[0], [(k, v["cold"]) for k, v in labels], "Cold software cost on fresh targets", "B/A guest instructions Ir (lower is cheaper B)", linear=True)
        ranges(ax[1], [(k, v["matrix"]) for k, v in labels], "Matrix phase separately", "B/A matrix-phase Ir")
        fig.suptitle("Observed case ranges and medians; no confidence intervals", fontsize=13)
        fig.tight_layout()
        save(fig, directory, "RATIOS", source)
    else:
        outcomes(data, directory, source, "status_by_cohort", "Cold twist profiling: all planned outcomes", "Exact finite diagnostic; configurations and process repeats are correlated")
        if number != 16:
            fig, ax = plt.subplots(1, 2, figsize=(12.0, 4.4))
            ranges(ax[0], [("Full / compressed: cold", data["compression_total_ratio"]), ("Full / compressed: matrix", data["compression_matrix_ratio"]), ("Ordinary / twist: cold", data["ordinary_over_twist"]), ("Map / scalar: cold", data["map_over_scalar"])], "Matched successful recoveries", "Recorded numerator/denominator Ir")
            ranges(ax[1], [("IC / best measured reference", data["cold_over_reference"])], "Strongest completed of five references", "Cold Ir ratio (all admitted scalar recoveries)")
            fig.suptitle("Observed ranges and medians; not confidence intervals or calibrated GAE", fontsize=13)
            fig.tight_layout()
            save(fig, directory, "RATIOS", source)
        else:
            fig, ax = plt.subplots(figsize=(9.0, 3.2))
            ranges(ax, list(data["aa_ir"].items()), "Three fresh processes per identical positive case", "Counted guest instructions Ir", linear=True)
            fig.tight_layout()
            save(fig, directory, "AA", source)
    selected = {k: data[k] for k in ("status_by_arm", "fresh_target_ratios", "status_by_cohort", "compression_total_ratio", "compression_matrix_ratio", "ordinary_over_twist", "map_over_scalar", "cold_over_reference", "aa_ir") if k in data}
    (directory / "PLOT_DATA.json").write_text(json.dumps(selected, indent=2) + "\n")

def input_plot():
    source = ROOT / "execution-v15/reference-v2/ADMISSION.json"
    a = json.loads(source.read_text())
    directory = ROOT / "reports/cold-inputs-20261029"
    directory.mkdir(parents=True, exist_ok=True)
    fig, ax = plt.subplots(figsize=(9.2, 3.7))
    ax.barh(["Same native input set", "Corrected independent GP check"], [a["verified_models"], a["verified_models"]], color=["#51728e", "#217b68"])
    ax.set_xlim(0, 34)
    for i in (0, 1):
        ax.text(a["verified_models"] + .4, i, str(a["verified_models"]), va="center")
    ax.set_xlabel("Models in the same fixed population (not separate targets)")
    ax.set_title("Input validation: native data reused by corrected reference\nOriginal independent checker remains rejected and preserved", loc="left", pad=16)
    ax.spines[["top", "right"]].set_visible(False)
    fig.tight_layout()
    save(fig, directory, "INPUTS", source)

if __name__ == "__main__":
    import sys
    if sys.argv[1] == "inputs":
        input_plot()
    else:
        main_plot(int(sys.argv[1]))
