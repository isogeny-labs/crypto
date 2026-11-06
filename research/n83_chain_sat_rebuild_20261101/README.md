# Chained-S3 on N83, m = 5…8: planted witnesses rebuilt from the lookup hit

Protocol `PROTOCOL.md` (frozen `e4662c05f1`); run 2026-11-01 06:42:02Z –
07:06:28Z, L0 host. Report: `reports/run-20261101T0642Z/report.pdf`.

| m | d | control (lookup + SAT): planted verified | rebuild: planted verified | rebuild planted median s | public refuted (both) |
|---|---|---|---|---|---|
| 5 | 3 | 0/5 | **5/5**, 0 solver calls | 0.91 | 3/3 |
| 6 | 3 | 0/5 | **5/5**, 0 solver calls | 1.65 | 3/3 |
| 7 | 4 | 0/5 | **5/5**, 0 solver calls | 10.8 | 3/3 |
| 8 | 4 | 0/5 | **5/5**, 0 solver calls | 81.5 | 3/3 |

64 records, 0 soundness flags. Decomposition stage only; no leaderboard or
`benchmarks/` row (validation gate). Reproduce: `run_shard.sh
target/release/examples/n83_chain_sat_bench raw/m<m> <m> <d>`; `cargo run
--release --example n83_chain_sat_analyze -- <this dir> all-summands+t2.2+meet3`;
`python3 plot.py ANALYSIS.json reports/run-20261101T0642Z/`. Raw records:
`raw.tar.xz` (sha256 in `raw.tar.xz.sha256`).
