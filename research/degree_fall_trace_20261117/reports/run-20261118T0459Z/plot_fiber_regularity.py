#!/usr/bin/env python3
"""Plot measured regularity by survivor fiber size."""
import csv
from io import BytesIO, StringIO
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from PIL import Image

HERE = Path(__file__).resolve().parent


def main():
    with (HERE / "fiber_regularity.csv").open(newline="") as source:
        rows = list(csv.DictReader(source))
    controls = np.full((6, 5), np.nan)
    for row in rows:
        if row["cohort"] == "control":
            controls[int(row["second"]), int(row["largest"]) - 1] = int(row["reference"])
    witness = {
        int(row["s"]): row for row in rows
        if row["cohort"] == "target" and row["fixture"] == "Consecutive"
        and row["t"] == "3" and row["s"] in ("0", "2")
    }

    fig, (degrees, fibers) = plt.subplots(1, 2, figsize=(10.5, 4.4), layout="constrained")
    image = degrees.imshow(controls, origin="lower", cmap="Blues", vmin=1, vmax=6,
                           aspect="equal")
    for (row, column), value in np.ndenumerate(controls):
        if np.isfinite(value):
            degrees.text(column, row, f"{value:.0f}", ha="center", va="center",
                         color="white" if value >= 4 else "#152436")
    degrees.set_xticks(range(5), range(1, 6))
    degrees.set_yticks(range(6))
    degrees.set_xlabel("Larger fiber size")
    degrees.set_ylabel("Smaller fiber size (0 = absent)")
    degrees.set_title("Degree from independent point-set controls")
    fig.colorbar(image, ax=degrees, label="Observed degree", fraction=0.05, pad=0.03)

    positions = np.arange(3)
    for index, (s, color) in enumerate(((0, "#805ba1"), (2, "#147655"))):
        values = [int(witness[s][field]) for field in ("largest", "second", "reference")]
        bars = fibers.bar(positions + (index - 0.5) * 0.35, values, 0.32,
                          color=color, label=f"(t, s) = (3, {s})")
        fibers.bar_label(bars, padding=2)
    fibers.set_xticks(positions, ("Larger fiber", "Smaller fiber", "Observed degree"))
    fibers.set_ylim(0, 5.5)
    fibers.set_yticks(range(1, 6))
    fibers.set_title("Same base and sum, five survivors each")
    fibers.legend(loc="upper right", frameon=False)
    fibers.grid(axis="y", alpha=0.2)

    fig.suptitle("The distribution of survivors determines degree on two fibers",
                 fontsize=14, fontweight="bold")
    with StringIO() as vector:
        fig.savefig(vector, format="svg", metadata={"Date": None})
        lines = vector.getvalue().splitlines()
    (HERE / "fiber_regularity.svg").write_text(
        "\n".join(line.rstrip() for line in lines) + "\n", encoding="utf-8"
    )
    with BytesIO() as raster:
        fig.savefig(raster, format="png", dpi=150)
        raster.seek(0)
        with Image.open(raster) as image:
            image.convert("RGB").quantize(colors=48).save(
                HERE / "fiber_regularity.png", dpi=(150, 150), optimize=True
            )


if __name__ == "__main__":
    plt.rcParams["svg.hashsalt"] = "toy-fiber-regularity"
    main()
