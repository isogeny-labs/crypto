# Exact-curve bounded BSGS measurement

**Frozen before execution, 2026-10-21.** Measure the release `ecdlp-nist solve` BSGS path on the published Certicom ECCp-131 and ECC2K-130 groups using the same planted scalar and two bounded intervals. This is a measurement of the interval algorithm on exact group operations, not a solution-cost estimate for either published challenge point.

## Arms and controls

- Command per arm: `ecdlp-nist solve CURVE --secret 12345 --interval-bits W --method bsgs --threads 1 --seed 131 --json`, with `CURVE` in `{ECCp-131,ECC2K-130}` and `W` in `{16,20}`. The planted scalar lies within both intervals.
- Run three rounds. In each round, run all four curve/width cells once and an identical A/A repeat of each cell; rotate cell order by round and put the repeat on the other side of the cells in alternate rounds. Each invocation is a fresh process. The driver records the actual order.
- Require exit zero, `planted_recovered=true`, `verified=true`, `report.scalar=12345`, method `bsgs`, the expected exact curve name, and a nonzero table size. A failed check stops the matrix, preserving every attempt.
- Cap each process at 120 seconds and stop if host memory or disk pressure becomes unsafe. Do not adapt rounds, widths, scalar, or command after seeing results.

## Measurements and scope

- Primary: exact `group_ops` and `table_size` from the native JSON, paired with correctness status. Secondary: native `elapsed_ms`, external process wall and peak RSS from `/usr/bin/time -l`; macOS has no CPU affinity and this host is concurrently busy, so all wall values are descriptive L0 observations.
- A/A repeats quantify measurement drift within each curve/width cell. Do not divide prime and binary wall times into a performance claim; arithmetic and curves differ. Neither interval has a matched whole-group rho reference; do not promote an end-to-end attack cost or a leaderboard row.
- Retain exact command, executable and source hashes, host/tool record, stdout JSON, stderr/resource stream, exit code, and driver summary under `bsgs_bench_20261021/`. Write no raw path twice. Report all completed and failed attempts, and add a source-linked graph and PDF.
