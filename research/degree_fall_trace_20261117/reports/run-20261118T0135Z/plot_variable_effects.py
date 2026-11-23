#!/usr/bin/env python3
"""Render the recorded toy variable effects without rerunning the algebra."""
import csv
from io import BytesIO, StringIO
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from PIL import Image

HERE = Path(__file__).resolve().parent
VARIABLES = ("x", "y", "z")
PAIRS = [(source, target) for source in VARIABLES for target in VARIABLES
         if source != target]


def annotate(axis, labels, values):
    for (row, column), value in np.ndenumerate(values):
        axis.text(column, row, labels[row][column], ha="center", va="center",
                  color="white" if value > 0.6 else "#192230", fontsize=10)


def main():
    with (HERE / "variable_effects.csv").open(newline="") as source:
        records = {(row["fixture"], int(row["relations"]), row["variable"]): row
                   for row in csv.DictReader(source)}
    fixtures = list(dict.fromkeys((fixture, count) for fixture, count, _ in records))
    row_labels = [f"{count} dense quadrics" if fixture == "dense"
                  else f"{fixture.capitalize()} control" for fixture, count in fixtures]

    shifts, shift_labels, fractions, fraction_labels = [], [], [], []
    for fixture, count in fixtures:
        rows = {variable: records[fixture, count, variable] for variable in VARIABLES}
        baseline = int(rows["x"]["baseline"])
        pinned = [int(rows[variable]["pinned"]) for variable in VARIABLES]
        shifts.append([degree - baseline for degree in pinned])
        shift_labels.append([f"{baseline} → {degree}" for degree in pinned])
        hits = [int(rows[source][f"to_{target}"]) for source, target in PAIRS]
        fractions.append([hit / count for hit in hits])
        fraction_labels.append([f"{hit}/{count}" for hit in hits])

    fig, (regularity, diffusion) = plt.subplots(
        1, 2, figsize=(11.3, 4.7), width_ratios=(3, 5), layout="constrained"
    )
    scale = max(1, np.max(np.abs(shifts)))
    shift_image = regularity.imshow(shifts, cmap="PuOr_r", vmin=-scale, vmax=scale,
                                    aspect="auto")
    coupling_image = diffusion.imshow(fractions, cmap="Blues", vmin=0, vmax=1,
                                      aspect="auto")
    annotate(regularity, shift_labels, np.abs(shifts) / scale)
    annotate(diffusion, fraction_labels, fractions)
    regularity.set_xticks(range(len(VARIABLES)), VARIABLES)
    diffusion.set_xticks(range(len(PAIRS)), [f"{a} → {b}" for a, b in PAIRS])
    regularity.set_yticks(range(len(fixtures)), row_labels)
    diffusion.set_yticks(range(len(fixtures)), [""] * len(fixtures))
    for axis in (regularity, diffusion):
        axis.tick_params(length=0)
        axis.axhline(sum(fixture == "dense" for fixture, _ in fixtures) - 0.5,
                     color="white", linewidth=2)
    regularity.set_title("Degree of regularity before → after")
    regularity.set_xlabel("Variable pinned to 1")
    diffusion.set_title("Direct diffusion between variables")
    diffusion.set_xlabel("Perturbed variable → affected variable")
    fig.colorbar(shift_image, ax=regularity, label="Change in degree")
    fig.colorbar(coupling_image, ax=diffusion, label="Fraction of relations")
    fig.suptitle("How individual variables affect a toy polynomial system",
                 fontsize=14, fontweight="bold")

    with StringIO() as vector:
        fig.savefig(vector, format="svg", metadata={"Date": None})
        lines = vector.getvalue().splitlines()
    (HERE / "variable_effects.svg").write_text(
        "\n".join(line.rstrip() for line in lines) + "\n", encoding="utf-8"
    )
    # The published PNG uses a palette to keep the 150 dpi preview compact.
    with BytesIO() as raster:
        fig.savefig(raster, format="png", dpi=150)
        raster.seek(0)
        with Image.open(raster) as image:
            image.convert("RGB").quantize(colors=48).save(
                HERE / "variable_effects.png", dpi=(150, 150), optimize=True
            )


if __name__ == "__main__":
    plt.rcParams["svg.hashsalt"] = "ecbench-variable-effects"
    main()
