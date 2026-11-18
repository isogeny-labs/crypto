#!/usr/bin/env python3
"""Render joint survivor retention from the recorded toy summary."""
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
STAGES = ("membership", "sum", "mixed")


def main():
    with (HERE / "survivor_summary.csv").open(newline="") as source:
        rows = list(csv.DictReader(source))
    records = {
        (row["base"], row["stage"], row["source"], int(row["pin"])): (
            int(row["pairs_after"]), int(row["pairs_before"])
        )
        for row in rows
    }
    pins = sorted({pin for _, _, _, pin in records})
    columns = [(variable, pin) for variable in "xyz" for pin in pins]

    fig, axes = plt.subplots(3, 1, figsize=(10.6, 5.9), layout="constrained")
    for base, axis in zip(BASES, axes):
        counts = [[records[base, stage, variable, pin] for variable, pin in columns]
                  for stage in STAGES]
        fractions = [[after / before if before else np.nan for after, before in row]
                     for row in counts]
        image = axis.imshow(fractions, cmap="Blues", vmin=0, vmax=1, aspect="auto")
        for (row, column), fraction in np.ndenumerate(fractions):
            after, before = counts[row][column]
            label = f"{after}/{before}" if before else "—"
            axis.text(column, row, label, ha="center", va="center", fontsize=9,
                      color="white" if fraction > 0.65 else "#203248")
        for boundary in (len(pins) - 0.5, 2 * len(pins) - 0.5):
            axis.axvline(boundary, color="white", linewidth=2)
        axis.set_yticks(range(len(STAGES)), ("Membership", "Sum", "Mixed"))
        axis.set_title(base, loc="left", fontsize=11, weight="bold")
        axis.set_xticks(range(len(columns)), [f"{var} = {pin}" for var, pin in columns])
        axis.tick_params(length=0)

    fig.colorbar(image, ax=axes, label="Fraction of unpinned pairs retained",
                 fraction=0.04, pad=0.025)
    fig.suptitle("Which pairs survive when one variable is pinned?",
                 fontsize=15, fontweight="bold")
    with StringIO() as vector:
        fig.savefig(vector, format="svg", metadata={"Date": None})
        lines = vector.getvalue().splitlines()
    (HERE / "survivor_retention.svg").write_text(
        "\n".join(line.rstrip() for line in lines) + "\n", encoding="utf-8"
    )
    with BytesIO() as raster:
        fig.savefig(raster, format="png", dpi=150)
        raster.seek(0)
        with Image.open(raster) as image:
            image.convert("RGB").quantize(colors=48).save(
                HERE / "survivor_retention.png", dpi=(150, 150), optimize=True
            )


if __name__ == "__main__":
    plt.rcParams["svg.hashsalt"] = "toy-survivor-retention"
    main()
