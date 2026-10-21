# Matched factor-base sweep and source rho

- Run ID: `20261021T1546Z-factor-base-matched`.
- Source: commits `c62b02ff4`, `248c0258c`, and `149fc6b20`; the published evidence uses the final matched analyzer.
- Host: Intel Xeon 6975P-C, 32 visible CPUs, Linux 6.18.54-121.166.amzn2023.x86_64; L0 concurrent single-thread processes. Wall readings are diagnostics only.
- Input: 21 exact-order models, 60 fixed public target workloads each; archive `sessions_projective_chart_v2.tar.zst` SHA-256 `550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.
- Command: `bash research/four_followups_20261021/run_factor_base.sh` with `FOUR_FOLLOWUPS_OUTPUT_DIR=research/four_followups_20261021/results/matched`, followed by the matched analyzer and independent native validator.
- Output: five arms by seven bit sizes, 35 raw JSONL files. All 6,300 candidate and 6,300 same-process rho scalars were verified by source multiplication. Zero failures or cap exhaustions.
- Replay: `bash research/four_followups_20261021/replay_matched.sh` forces the committed input archive. Its 35 file comparisons are recorded in `results/matched/replay.log` and the benchmark replay certificate.
- Accounting: the full 36-cell price grid and all 6,300 joined target rows are in `results/matched/`; the separate validator recomputed 15 headline values directly from raw JSONL.

The [report](../../REPORT.md), [PDF](report.pdf), [factor-base graph](factor-base-grid.svg), and [Kummer gate diagram](kummer-ambiguity.svg) are copies of the study's reviewer-facing record. The complete genus-2 law, full pencil collector, and differential Kummer walk remain unexecuted and receive no end-to-end verdict here.
