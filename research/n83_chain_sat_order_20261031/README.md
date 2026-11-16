# Chained-S3 SAT on N83, m = 2…9: summand ordering and root-finder tables

Run 2026-10-31 21:36:33Z – 22:56:35Z on one 4-vCPU cloud VM (L0), protocol
`PROTOCOL.md` frozen at `8951a4a1cf`; binary sha256 in `run/SHA256SUMS`.
Report: `reports/run-20261031T2136Z/report.pdf` (graphs beside it).

| m | A0 unordered (pub/3, plant/2) | A1 all-summand order | A2 order + tables | A3 A2 affine-only |
|---|---|---|---|---|
| 2 | 3, 2 | 3, 2 | 3, 2 | 3, 2 |
| 3 | 0, 0 | 0, 0 | **3, 2** | 3, 2 |
| 4 | 0, 0 | 0, 0 | **3, 0** | 3, 0 |
| 5–9 | 0, 0 | 0, 0 | 0, 0 | 0, 0 |

- 160 records, 0 soundness flags (no planted target refuted; every witness
  re-adds in the group).
- m = 2: all-summand ordering uses fewer conflicts on every paired target
  (ratios 0.50–0.60 public, 0.07–0.20 planted); median public 13,461 → 7,195
  conflicts, 12.8 → 6.2 s.
- m = 3: root-finder tables refute every public target with 0 conflicts
  (0.38 s) and find both planted witnesses (median 8,735 conflicts); the
  baseline decides none in 120 s.
- m = 4: tables refute every public target (median 156 conflicts, 0.98 s);
  no arm finds a planted witness within 120 s.
- m ≥ 5: every arm is wall-capped on its first identity pattern.

Decomposition-stage measurement only: no complete DLP, no leaderboard or
`benchmarks/` row (validation gate: no replay certificate or matched-rho
`VALIDATION.json`). Matched rho on this curve: 1.51 s mean online
(`research/n83_msweep_20261024`).

Reproduce: `cargo build --release --example n83_chain_sat_bench`, then
`run_shard.sh target/release/examples/n83_chain_sat_bench raw/shardK <ms>`
per shard; `cargo run --release --example n83_chain_sat_analyze -- <this dir>`;
`python3 plot.py ANALYSIS.json reports/run-20261031T2136Z/`. Raw records:
`raw.tar.xz` (sha256 in `raw.tar.xz.sha256`; moved to the artifact store by
`tools/artifacts`).
