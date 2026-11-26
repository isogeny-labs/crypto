#!/usr/bin/env python3
"""Render only values retained in control_data.csv."""

import csv
from pathlib import Path

import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
with (HERE / "control_data.csv").open(newline="", encoding="utf-8") as handle:
    rows = list(csv.DictReader(handle))

labels = [row["fixture"] for row in rows]
matched = [int(row["matched_coefficients"]) for row in rows]
wrong_scale = [int(row["wrong_scaling_first_difference"]) for row in rows]
wrong_parameter = [int(row["wrong_parameter_first_difference"]) for row in rows]
positions = list(range(len(rows)))
width = 0.24

figure, axis = plt.subplots(figsize=(8.2, 4.5))
axis.bar([x - width for x in positions], matched, width, label="correct scaling: coefficients matched")
axis.bar(positions, wrong_scale, width, label="wrong scaling: first difference")
axis.bar([x + width for x in positions], wrong_parameter, width, label="wrong parameter: first difference")
axis.set_xticks(positions, labels, rotation=18, ha="right")
axis.set_ylim(0, 26)
axis.set_ylabel("coefficient count or first-difference index")
axis.set_title("H116-CURVE-COORDINATE-001 · five cells · precision 24")
axis.grid(axis="y", alpha=0.25)
axis.legend(fontsize=8, loc="upper right")
figure.tight_layout()
figure.savefig(HERE / "controls.svg")
figure.savefig(HERE / "controls.png", dpi=150)

