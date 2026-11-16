# N12 model-held-out factor-base cost report

This directory copies the [study report](../../research/factor_base_heldout_20261025/REPORT.md), its [PDF](results-report.pdf), and both source-linked SVG/PNG graphs for browsing. The study retains the raw JSONL, standardized CSV/JSON, native validator, and 36-cell grids. The measured unit is modeled cold source-addition equivalents on a shared host; no isolated wall-speed claim or leaderboard row is proposed. The repository validation gate keeps this packet outside the admitted `benchmarks/README.md` results table until an independent replay certificate and isolated matched reference are available.

The PDF and PNGs are in the repository artifact store. From the repository
root, run `python3 tools/artifacts/artifacts.py pull
benchmarks/20261025-factor-base-heldout` to hydrate them.
