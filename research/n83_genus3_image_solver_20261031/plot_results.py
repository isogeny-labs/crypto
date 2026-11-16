#!/usr/bin/env python3
"""Render the sealed genus-3 image native-cost and descriptive-wall figures."""

from __future__ import annotations

import csv
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


ROOT = Path(__file__).resolve().parent
SUMMARY = ROOT / "NATIVE_SUMMARY.json"
OUT = ROOT / "figures"


def rows_for(rows: list[dict], arm: str) -> list[dict]:
    return sorted(
        [r for r in rows if r["arm"] == arm and r["verified"] > 0],
        key=lambda r: r["r"],
    )


def slope(rows: list[dict], key: str) -> float:
    x = np.log2([r["r"] for r in rows])
    y = np.log2([r[key] for r in rows])
    return float(np.polyfit(x, y, 1)[0])


def main() -> None:
    payload = json.loads(SUMMARY.read_text())
    rows = payload["rows"]
    bsgs = rows_for(rows, "destination-bsgs")
    rho = rows_for(rows, "destination-rho3")
    source = rows_for(rows, "source-rho-strong")
    successful = [r for r in bsgs if r["n"] in {9, 13, 19, 23}]
    rho_successful = [r for r in rho if r["n"] in {9, 13, 19, 23}]

    OUT.mkdir(parents=True, exist_ok=True)
    plt.rcParams.update(
        {
            "font.size": 10,
            "axes.titlesize": 11,
            "axes.labelsize": 10,
            "legend.fontsize": 9,
            "svg.fonttype": "none",
        }
    )
    fig, axes = plt.subplots(1, 2, figsize=(11.2, 4.4), constrained_layout=True)

    ax = axes[0]
    for data, label, marker, color in [
        (rho, "destination rho3", "o", "#9b4f0f"),
        (bsgs, "destination BSGS", "s", "#1768ac"),
    ]:
        ax.plot(
            [r["r"] for r in data],
            [r["mean_jacobian_group_operations"] for r in data],
            marker=marker,
            color=color,
            linewidth=1.8,
            label=label,
        )
    ax.set_xscale("log", base=2)
    ax.set_yscale("log", base=2)
    ax.set_xlabel("prime image order r (log₂ scale)")
    ax.set_ylabel("mean native Jacobian group operations")
    ax.set_title("Destination-only native work")
    ax.grid(True, which="both", alpha=0.25)
    ax.legend(frameon=False)
    ax.text(
        0.03,
        0.97,
        f"successful-size log slopes\nBSGS {slope(successful, 'mean_jacobian_group_operations'):.3f}\n"
        f"rho3  {slope(rho_successful, 'mean_jacobian_group_operations'):.3f}",
        transform=ax.transAxes,
        va="top",
        bbox={"facecolor": "white", "alpha": 0.85, "edgecolor": "none"},
    )

    ax = axes[1]
    for data, label, marker, color in [
        (source, "source strong rho", "^", "#238b45"),
        (rho, "destination rho3", "o", "#9b4f0f"),
        (bsgs, "destination BSGS", "s", "#1768ac"),
    ]:
        ax.plot(
            [r["r"] for r in data],
            [r["median_solve_wall_ms"] for r in data],
            marker=marker,
            color=color,
            linewidth=1.8,
            label=label,
        )
    ax.set_xscale("log", base=2)
    ax.set_yscale("log", base=2)
    ax.set_xlabel("prime subgroup order r (log₂ scale)")
    ax.set_ylabel("median solve wall time (ms)")
    ax.set_title("Cold one-target wall time (L0 descriptive)")
    ax.grid(True, which="both", alpha=0.25)
    ax.legend(frameon=False)
    ax.text(
        0.62,
        0.25,
        "n=7 source exhausted;\nfailure retained in data",
        transform=ax.transAxes,
        ha="center",
        va="center",
        bbox={"facecolor": "white", "alpha": 0.85, "edgecolor": "none"},
    )

    fig.suptitle(
        "Same-field genus-3 image solvers: valid transfer, no common net-cost unit",
        fontsize=13,
    )
    stem = OUT / "native-cost-and-wall"
    svg_path = stem.with_suffix(".svg")
    fig.savefig(svg_path)
    svg_path.write_text(
        "\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n"
    )
    fig.savefig(stem.with_suffix(".pdf"))
    fig.savefig(stem.with_suffix(".png"), dpi=150)
    plt.close(fig)

    columns = [
        "n",
        "slug",
        "r",
        "arm",
        "attempts",
        "verified",
        "exhausted",
        "mean_source_gae",
        "mean_jacobian_group_operations",
        "median_solve_wall_ms",
    ]
    with (OUT / "plot-data.csv").open("w", newline="") as handle:
        writer = csv.DictWriter(
            handle, fieldnames=columns, extrasaction="ignore", lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(rows)

    report = {
        "schema": "crypto.n83-genus3-plot-summary/v1",
        "input": str(SUMMARY.relative_to(ROOT.parent.parent)),
        "successful_sizes": [9, 13, 19, 23],
        "destination_native_log2_slopes": {
            "jacobian_image.bsgs": slope(successful, "mean_jacobian_group_operations"),
            "jacobian_image.rho3": slope(rho_successful, "mean_jacobian_group_operations"),
        },
        "warning": payload["warning"],
    }
    (OUT / "plot-summary.json").write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
