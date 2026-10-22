---
title: "P-224 class-group 2-rank: null experiment"
date: "2026-10-22"
---

Is a 2-rank of 6 (`omega(D_K) = 7`) unusual for the CM discriminant of a
prime-order curve of this size? **No: within the control distribution**
(P-224 at the 91.5th mid-rank percentile of random 224-bit primes, upper tail
12.3%, 95% CI 10.3 to 14.5%; fixed P-224 prime 90.0%, 14.5%). The result and its
limits are in [`report.pdf`](report.pdf) (source `report.typ`).

* Frozen protocol: [`PROTOCOL.md`](PROTOCOL.md) (committed before the run).
* Raw rows: [`run/rows.jsonl`](run/rows.jsonl); recomputed outputs
  `run/summary.json`, `run/summary.md`, graph `run/omega_null.svg`; run log
  `run/run.log`.
* Code: `src/cryptanalysis/two_rank_null.rs`, `examples/p224_two_rank_null.rs`
  (reuses `p224_hypotheses::factor` and `class_number_estimate::erfc`).
* Reproduce: `cargo run --release --example p224_two_rank_null -- run --out
  research/p224_two_rank_null_20261022/run --n 1000 --n-big 100 --n-pc 300`, then
  `... -- analyze --dir research/p224_two_rank_null_20261022/run`.

Not an ECDLP cost measurement: no rho ratio, no scoreboard row, no `benchmarks/`
entry owed. The null is a Hasse-interval trace model, not point-counted curves
(`PROTOCOL.md`).
