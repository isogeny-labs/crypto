#!/usr/bin/env python3
"""Plot the recorded candidate coverage and shared-target flow."""
import csv
from io import BytesIO, StringIO
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from PIL import Image

HERE = Path(__file__).resolve().parent
BASES = ("Centered", "Consecutive", "Spread")
COLORS = ("#805ba1", "#27778f", "#147655")
STAGES = ("membership", "sum", "mixed")
STAGE_LABELS = ("Membership", "Sum t = 3", "Mixed s = 2")


def main():
    with (HERE / "base_flow.csv").open(newline="") as source:
        records = list(csv.DictReader(source))
    shared = {
        (row["base"], row["stage"]): row
        for row in records
        if row["stage"] == "membership"
        or (row["t"] == "3" and row["s"] in ("", "2"))
    }
    fig, ((coverage, flow), (regularity, diffusion)) = plt.subplots(
        2, 2, figsize=(11.5, 6.2), layout="constrained"
    )

    positions = np.arange(3)
    for offset, stage in enumerate(STAGES[1:]):
        counts = [sum(row["base"] == base and row["stage"] == stage for row in records)
                  for base in BASES]
        bars = coverage.bar(positions + (offset - 0.5) * 0.34, counts, 0.32,
                            color=("#94b8c8", "#215c71")[offset],
                            label=("Sum targets", "Full target pairs")[offset])
        coverage.bar_label(bars, padding=2, fontsize=9)
    coverage.set_xticks(positions, BASES)
    coverage.set_ylim(0, 73)
    coverage.set_ylabel("Distinct targets reached")
    coverage.set_title("Coverage across the entire grid")
    coverage.legend(loc="upper left", frameon=False, ncols=2)

    for index, (base, color) in enumerate(zip(BASES, COLORS)):
        counts = [int(shared[base, stage]["candidates"]) for stage in STAGES]
        flow.plot(positions + (index - 1) * 0.065, counts, "o-",
                  label=base, color=color, linewidth=1.8)
        flow.annotate(str(counts[1]), (1 + (index - 1) * 0.065, counts[1]),
                      xytext=((index - 1) * 3, 7 + (index % 2) * 10),
                      textcoords="offset points", color=color, ha="center", fontsize=9)
        flow.annotate(str(counts[2]), (2 + (index - 1) * 0.065, counts[2]),
                      xytext=(10, (index - 1) * 11),
                      textcoords="offset points", color=color, fontsize=9)
    flow.set_xticks(positions, STAGE_LABELS)
    flow.set_xlim(-0.2, 2.35)
    flow.set_yscale("log")
    flow.set_ylim(3, 180)
    flow.set_yticks([3, 5, 10, 25, 50, 125], [3, 5, 10, 25, 50, 125])
    flow.set_ylabel("Ordered triples (log scale)")
    flow.set_title("Survivors at the shared target")
    flow.legend(loc="upper right", frameon=False)
    flow.grid(axis="y", alpha=0.2)

    degrees = np.array([
        [int(shared[base, stage]["dreg"]) for stage in STAGES]
        + [int(shared[base, "mixed"][f"pin_{variable}"]) for variable in "xyz"]
        for base in BASES
    ])
    regularity.imshow(degrees, cmap="Purples", vmin=1, vmax=13, aspect="auto")
    for (row, column), value in np.ndenumerate(degrees):
        regularity.text(column, row, str(value), ha="center", va="center",
                        color="white" if value > 8 else "#252039")
    regularity.axvline(2.5, color="white", linewidth=3)
    regularity.set_xticks(range(6), ("Members", "Sum", "Mixed", "Pin x", "Pin y", "Pin z"))
    regularity.set_yticks(positions, BASES)
    regularity.set_title("Observed degree of regularity at the shared target")

    pairs = ("xy", "xz", "yz")
    coupling = [[shared[base, stage][pair] for pair in pairs]
                for base in BASES for stage in STAGES]
    # The three bases share the same input-equation coupling at each stage.
    if coupling[:3] != coupling[3:6] or coupling[:3] != coupling[6:]:
        raise ValueError("Direct coupling differs by base")
    diffusion.imshow(np.asarray(coupling[:3], dtype=int), cmap="Blues",
                     vmin=0, vmax=1.5, aspect="auto")
    for row, stage in enumerate(STAGES):
        relation_count = shared[BASES[0], stage]["relations"]
        for column, count in enumerate(coupling[row]):
            diffusion.text(column, row, f"{count}/{relation_count}", ha="center", va="center",
                           color="white" if count == "1" else "#1a2b38")
    diffusion.set_xticks(positions, ("x ↔ y", "x ↔ z", "y ↔ z"))
    diffusion.set_yticks(positions, ("Membership", "Sum", "Mixed"))
    diffusion.set_title("Direct diffusion, same for all bases")

    for axis in (coverage, flow, regularity, diffusion):
        axis.tick_params(length=0)
    fig.suptitle("How the candidate base changes the toy relation flow",
                 fontsize=15, fontweight="bold")
    with StringIO() as vector:
        fig.savefig(vector, format="svg", metadata={"Date": None})
        lines = vector.getvalue().splitlines()
    (HERE / "base_flow.svg").write_text(
        "\n".join(line.rstrip() for line in lines) + "\n", encoding="utf-8"
    )
    with BytesIO() as raster:
        fig.savefig(raster, format="png", dpi=150)
        raster.seek(0)
        with Image.open(raster) as image:
            image.convert("RGB").quantize(colors=48).save(
                HERE / "base_flow.png", dpi=(150, 150), optimize=True
            )


if __name__ == "__main__":
    plt.rcParams["svg.hashsalt"] = "toy-candidate-base-flow"
    main()
