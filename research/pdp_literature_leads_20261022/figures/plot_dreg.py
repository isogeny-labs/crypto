#!/usr/bin/env python3
"""Plot recorded degree values for Weil-descended summation-polynomial systems.

Lays out values recorded in figures/dreg_records.csv (literature and
repository records, each with its source); computes nothing new. Rendering
only (research-visuals skill): SVG plus a 150 dpi PNG from the same source.
"""
import csv
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "dreg_records.csv")


def load():
    rows = []
    with open(DATA, newline="") as f:
        for r in csv.DictReader(f):
            r["x"] = float(r["x"])
            r["y"] = float(r["y"])
            r["lb"] = r["y_is_lower_bound"] == "1"
            rows.append(r)
    return rows


def series(rows, panel):
    out = {}
    for r in rows:
        if r["panel"] != panel:
            continue
        out.setdefault(r["series"], []).append(r)
    for k in out:
        out[k].sort(key=lambda r: r["x"])
    return out


def main():
    rows = load()
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12.5, 4.8))

    # Panel 1: degree vs n for the descended S_3 and S_4 systems.
    s = series(rows, "dreg_vs_n")
    markers = ["o", "s", "^", "D", "v", "P", "X"]
    for i, (name, pts) in enumerate(s.items()):
        xs = [p["x"] for p in pts]
        ys = [p["y"] for p in pts]
        if name.endswith("(step)"):
            ax1.step(xs, ys, where="post", lw=2, label=name, color="tab:red")
        else:
            ax1.plot(xs, ys, marker=markers[i % len(markers)], ls="--" if len(xs) > 1 else "none",
                     lw=1, ms=7, label=name)
        for p in pts:
            if p["lb"]:
                ax1.annotate("≥", (p["x"], p["y"]), textcoords="offset points", xytext=(6, -4),
                             fontsize=9)
    ax1.set_xlabel("n (field F_{2^n}); XL rows placed at 5·ℓ for display")
    ax1.set_ylabel("degree (D_reg, closure, or max F4 step degree)")
    ax1.set_title("Recorded degrees of descended summation systems vs n")
    ax1.set_ylim(1.5, 8)
    ax1.set_yticks(range(2, 9))
    ax1.grid(alpha=0.3)
    ax1.legend(fontsize=6.5, loc="lower right")

    # Panel 2: first-fall bounds vs m.
    s2 = series(rows, "ffd_vs_m")
    for i, (name, pts) in enumerate(s2.items()):
        xs = [p["x"] for p in pts]
        ys = [p["y"] for p in pts]
        ax2.plot(xs, ys, marker=markers[i % len(markers)], ls="-" if len(xs) > 2 else ("--" if len(xs) == 2 else "none"),
                 lw=1.2, ms=7, label=name)
    ax2.set_xlabel("m (summands; the (m+1)-th summation polynomial)")
    ax2.set_ylabel("first-fall degree (bound or recorded)")
    ax2.set_title("First-fall bounds and recorded values vs m")
    ax2.set_xticks([2, 3, 4, 5])
    ax2.grid(alpha=0.3)
    ax2.legend(fontsize=6.5, loc="upper left")

    fig.suptitle("Literature and repository records only (sources in figures/dreg_records.csv); "
                 "no new computation. Study: research/pdp_literature_leads_20261022", fontsize=8.5)
    fig.tight_layout(rect=(0, 0, 1, 0.95))
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(os.path.join(HERE, f"dreg_records.{ext}"), **kw)
    print("wrote dreg_records.svg and dreg_records.png")


if __name__ == "__main__":
    sys.exit(main())
