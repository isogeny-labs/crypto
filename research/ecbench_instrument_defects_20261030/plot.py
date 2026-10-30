#!/usr/bin/env python3
"""Figures for research/ecbench_instrument_defects_20261030.

Reads committed files only:
- research/f4_pricing_jump_20261018/leaderboard_rerun/f4_f5_f6_cross_family_20261011-K13/records.jsonl
  (the K13 spec re-run on main, 2026-10-18)
- research/f4_f5_f6_cross_family_20261011/ANALYSIS.json (the archived 2026-10-11 medians)
- d1_census.json (this directory)

Writes k13_correction.{svg,png,pdf} and d1_links.{svg,png,pdf}.
"""
import json
import statistics
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
RERUN = REPO / "research/f4_pricing_jump_20261018/leaderboard_rerun/f4_f5_f6_cross_family_20261011-K13/records.jsonl"
ANALYSIS = REPO / "research/f4_f5_f6_cross_family_20261011/ANALYSIS.json"


def save(fig, stem):
    for ext in ("svg", "png", "pdf"):
        fig.savefig(HERE / f"{stem}.{ext}", dpi=150, bbox_inches="tight")
    plt.close(fig)


def k13():
    s = {}
    for line in RERUN.read_text().splitlines():
        r = json.loads(line)
        if not r["warmup"] and r["outcome"]["status"] == "verified":
            s.setdefault(r["arm"], []).append(r["cost"]["s"])
    cell = json.loads(ANALYSIS.read_text())["sessions"]["K13"]["cells"][0]["arms"]
    arms = ["rho-negation", "rho-strong", "inherited-f4", "f6-ic", "matrix-f5", "matrix-f4"]
    fig, ax = plt.subplots(figsize=(7.5, 4))
    for k, arm in enumerate(arms):
        v = sorted(s[arm])
        ax.scatter([k + 0.12] * len(v), v, s=12, color="#1f6fb4", alpha=0.6,
                   label="main 2026-10-18 (each target)" if k == 0 else None)
        ax.plot([k + 0.02, k + 0.22], [statistics.median(v)] * 2, color="#1f6fb4", lw=2)
        old = (cell.get(arm) or {}).get("s")
        if old:
            ax.scatter([k - 0.12], [old["median"]], marker="D", s=30, color="#c0392b",
                       label="archived 2026-10-11 median (lower bound)" if k == 0 else None)
    ax.set_yscale("log")
    ax.set_xticks(range(len(arms)), arms, rotation=15)
    ax.set_ylabel("S = total GAE / sqrt(r)  (log)")
    ax.set_title("K13 (icv1-f2m13-t181-515ee569): word XORs charged since PR #1774\n"
                 "inherited F4 16.285 (lower bound) -> 630.859 [492.4, 806.4]; rho rows identical")
    ax.legend(loc="upper left", fontsize=8)
    ax.grid(axis="y", alpha=0.3)
    save(fig, "k13_correction")


def d1():
    c = json.loads((HERE / "d1_census.json").read_text())
    h = {int(k): v for k, v in c["trees_per_inode_histogram"].items()}
    fig, ax = plt.subplots(figsize=(7.5, 3.5))
    ax.bar(list(h), list(h.values()), color="#7d3c98")
    ax.set_xlabel("worktrees sharing one inode")
    ax.set_ylabel("inodes")
    ax.set_title(f"D1: {c['paths_with_link_count_gt_1']:,} paths on {c['distinct_inodes']:,} inodes "
                 f"across {c['trees_involved']} worktrees\n(jdupes -r -L, 2026-10-24 18:11-19:11)")
    ax.grid(axis="y", alpha=0.3)
    save(fig, "d1_links")


if __name__ == "__main__":
    k13()
    d1()
