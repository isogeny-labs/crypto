#!/usr/bin/env python3
"""Document layout only: plot already-recorded native fields, with no research arithmetic."""
import json
import pathlib
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

raw = json.loads(pathlib.Path(sys.argv[1]).read_text())
out = pathlib.Path(__file__).resolve().parent
colours = {"prime": "#1f77b4", "binary": "#d62728", "koblitz": "#2ca02c",
           "subfield": "#9467bd", "extension": "#ff7f0e"}
fig, ax = plt.subplots(figsize=(9.0, 4.5), constrained_layout=True)
for family, colour in colours.items():
    rows = [c for c in raw["curves"] if c["family"] == family and c["best_route"] is not None]
    ax.scatter([c["subgroup"]["bits"] for c in rows],
               [c["routes"][c["best_route"]]["cost"]["ratio_log2"] for c in rows],
               s=9, alpha=.5, c=colour, label=family, linewidths=0, rasterized=True)
ax.axhline(0, color="#475569", linewidth=.8, linestyle="--", label="formula parity")
ax.axhline(raw["cost_model"]["advantage_margin_bits"], color="#9ca3af", linewidth=.7,
           linestyle=":", label="model advantage margin")
ax.set(xlabel="Recorded subgroup size (bits)",
       ylabel="Recorded formula log2(rho / route)",
       title="Jacobian formula inventory: 4,493 registered models")
ax.grid(alpha=.18)
ax.legend(loc="upper right", fontsize=8, ncol=2, frameon=True)
for ext in ("svg", "pdf", "png"):
    fig.savefig(out / f"jacobian-readable.{ext}", dpi=150)
