#!/usr/bin/env python3
"""Recompute the exact-curve bounded BSGS summary and plots from raw records."""

import hashlib
import json
from pathlib import Path
import re
import statistics

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1] / "bsgs_bench_20261021"
RAW = STUDY / "raw"
ROWS = []
for path in sorted(RAW.glob("*.json")):
    if path.name.endswith(".command.json"):
        continue
    tag = path.stem
    match = re.fullmatch(r"(\d+)-r(\d+)-(eccp-131|ecc2k-130)-w(16|20)-([ab])", tag)
    assert match, tag
    data = json.loads(path.read_text())
    result = data["report"]
    assert data["planted_recovered"] and result["verified"] and result["scalar"] == "12345"
    assert result["method"] == "bsgs" and (RAW / f"{tag}.exit").read_text().strip() == "0"
    command = json.loads((RAW / f"{tag}.command.json").read_text())
    assert command["tag"] == tag and command["argv"][2] == result["curve"]
    stderr = (RAW / f"{tag}.stderr").read_text()
    peak = re.search(r"^\s*(\d+)\s+maximum resident set size$", stderr, re.MULTILINE)
    assert peak, tag
    ROWS.append({
        "tag": tag,
        "seq": int(match[1]),
        "round": int(match[2]),
        "curve": result["curve"],
        "width": int(match[4]),
        "repeat": match[5],
        "group_ops": result["group_ops"],
        "table_size": result["table_size"],
        "native_ms": result["elapsed_ms"],
        "process_ms": float((RAW / f"{tag}.wall-ms").read_text()),
        "rss_bytes": int(peak[1]),
        "json_sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
    })
assert len(ROWS) == 24 and (STUDY / "COMPLETE").read_text().strip() == "24 verified process records"
summary = {"source_head": json.loads((STUDY / "manifest.json").read_text())["source_head"], "records": ROWS, "cells": []}
for curve in ("ECCp-131", "ECC2K-130"):
    for width in (16, 20):
        cell = [r for r in ROWS if r["curve"] == curve and r["width"] == width]
        assert len(cell) == 6
        pairs = []
        for round_index in (1, 2, 3):
            a, b = sorted((r for r in cell if r["round"] == round_index), key=lambda r: r["repeat"])
            assert (a["repeat"], b["repeat"]) == ("a", "b")
            pairs.append(abs(a["process_ms"] - b["process_ms"]))
        summary["cells"].append({
            "curve": curve, "interval_bits": width, "n": 6,
            "group_ops": sorted({r["group_ops"] for r in cell}),
            "table_sizes": sorted({r["table_size"] for r in cell}),
            "native_ms_median": statistics.median(r["native_ms"] for r in cell),
            "process_ms_median": statistics.median(r["process_ms"] for r in cell),
            "process_ms_range": [min(r["process_ms"] for r in cell), max(r["process_ms"] for r in cell)],
            "aa_process_abs_diff_ms": pairs,
            "max_rss_bytes_range": [min(r["rss_bytes"] for r in cell), max(r["rss_bytes"] for r in cell)],
        })
(HERE / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")

fig, ax = plt.subplots(figsize=(8.2, 3.3))
colors = {"ECCp-131": "#174a7e", "ECC2K-130": "#a14c28"}
for group_index, (curve, width) in enumerate((c, w) for c in colors for w in (16, 20)):
    cell = [r for r in ROWS if r["curve"] == curve and r["width"] == width]
    for r in cell:
        ax.scatter(group_index + (r["round"] - 2) * 0.12 + (-0.03 if r["repeat"] == "a" else 0.03),
                   r["process_ms"], color=colors[curve], marker="o" if r["repeat"] == "a" else "s", s=36)
    ax.hlines(statistics.median(r["process_ms"] for r in cell), group_index - 0.25, group_index + 0.25,
              color="black", linewidth=1.5)
ax.set_xticks(range(4), ["prime 16", "prime 20", "binary 16", "binary 20"])
ax.set_ylabel("Fresh process wall (ms), L0")
ax.set_title("Bounded BSGS on the two exact challenge groups")
ax.grid(axis="y", alpha=0.2)
fig.tight_layout()
fig.savefig(HERE / "bsgs_process.svg")
fig.savefig(HERE / "bsgs_process.png", dpi=180)
