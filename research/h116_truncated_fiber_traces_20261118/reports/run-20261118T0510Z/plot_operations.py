#!/usr/bin/env python3
"""Plot only retained H116 OOS operation counts from operations.csv."""

import csv
from pathlib import Path

import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
with (HERE / "operations.csv").open(newline="", encoding="utf-8") as handle:
    rows = list(csv.DictReader(handle))

r = [int(row["r"]) for row in rows]
candidate_add = [int(row["candidate_additions"]) for row in rows]
candidate_mul = [int(row["candidate_multiplications"]) for row in rows]
oracle_add = [int(row["oracle_base_additions"]) for row in rows]
oracle_mul = [int(row["oracle_base_multiplications"]) for row in rows]

fig, axis = plt.subplots(figsize=(8.2, 4.6))
axis.plot(r, candidate_add, "o-", label="formal-scalar candidate additions")
axis.plot(r, candidate_mul, "s-", label="formal-scalar candidate multiplications")
axis.plot(r, oracle_add, "o--", label="full-map oracle base additions")
axis.plot(r, oracle_mul, "s--", label="full-map oracle base multiplications")
axis.set_yscale("log")
axis.set_xlabel("isogeny degree r")
axis.set_ylabel("recorded field operations (log scale)")
axis.set_title("H116 OOS construction counts · 4 completed cells\nDifferent obligations; no timing or speed ratio")
axis.grid(True, which="both", alpha=0.25)
axis.legend(fontsize=8)
fig.tight_layout()
fig.savefig(HERE / "operations.svg")
fig.savefig(HERE / "operations.png", dpi=150)
