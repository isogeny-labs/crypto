"""Render the archived K4405 host samples; no experiment calculation occurs here."""

import hashlib
import json
from pathlib import Path

import matplotlib.pyplot as plt


REPORT = Path(__file__).resolve().parent
EVIDENCE = REPORT.parents[1] / "evidence-01"
receipt = json.loads((EVIDENCE / "receipt.json").read_text())
raw = (EVIDENCE / "samples.json").read_bytes()
assert hashlib.sha256(raw).hexdigest() == receipt["samples_sha256"]
samples = json.loads(raw)
assert len(samples) == 1613

minutes = [row["elapsed_seconds"] / 60 for row in samples]
rss_minutes = [x for x, row in zip(minutes, samples) if row["rss_bytes"] is not None]
rss_gib = [row["rss_bytes"] / (1024**3) for row in samples if row["rss_bytes"] is not None]
free_minutes = [x for x, row in zip(minutes, samples) if row["host_free_percent"] is not None]
free_pct = [row["host_free_percent"] for row in samples if row["host_free_percent"] is not None]

plt.rcParams.update({"font.size": 10, "axes.spines.top": False,
                     "axes.spines.right": False, "svg.hashsalt": "n83-k4405-20261026"})
fig, (ax_rss, ax_free) = plt.subplots(2, 1, sharex=True, figsize=(9, 5.5))
ax_rss.plot(rss_minutes, rss_gib, color="#136fba", linewidth=1.25)
ax_rss.axhline(24, color="#a74d39", linestyle="--", linewidth=1, label="24 GiB child RSS cap")
ax_rss.set_ylabel("Child RSS (GiB)")
ax_rss.set_ylim(0, 26)
ax_rss.legend(loc="upper left", frameon=False)
ax_rss.grid(axis="y", color="#d8dfe6", linewidth=0.5)

ax_free.plot(free_minutes, free_pct, color="#2b855d", linewidth=1.25, marker=".", markersize=2)
ax_free.axhline(10, color="#a74d39", linestyle="--", linewidth=1, label="10% host free gate")
ax_free.set_ylabel("Host free memory (%)")
ax_free.set_xlabel("Elapsed time (minutes)")
ax_free.set_ylim(0, 60)
ax_free.legend(loc="upper right", frameon=False)
ax_free.grid(axis="y", color="#d8dfe6", linewidth=0.5)
ax_free.set_xlim(0, receipt["wall_seconds"] / 60)

fig.suptitle("N83 K4405 current-source host control | 1,613 recorded samples", fontweight="bold")
fig.text(0.5, 0.01, "Run evidence-01 · 2026-10-26 · manual SIGTERM after host swap pressure",
         ha="center", color="#485563", fontsize=9)
fig.tight_layout(rect=(0, 0.04, 1, 0.96))
svg = REPORT / "resource_trace.svg"
fig.savefig(svg, metadata={"Date": None})
# Matplotlib writes whitespace inside many SVG path strings; strip line ends so
# the committed vector source passes the repository's whitespace gate.
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(REPORT / "resource_trace.png", dpi=150)
