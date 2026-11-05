# Chained-S3 on N83, m = 8, 9: parallel back enumeration

Protocol `PROTOCOL.md` (frozen `3045ec4d33`); run 2026-11-01 20:46:09Z –
21:20:29Z, L0 4-vCPU host, m = 8 then m = 9. Report:
`reports/run-20261101T2046Z/report.pdf`.

| m | d | verdicts (both arms) | median wall s, 1 thread → 4 threads: public | planted |
|---|---|---|---|---|
| 8 | 4 | 3/3 refuted, 5/5 verified | 41.7 → 11.4 | 41.8 → 11.7 |
| 9 | 5 | 3/3 refuted, 5/5 verified | 57.0 → 13.7 | 55.2 → 13.9 |

32 records, 0 soundness flags, identical verdicts. Wall-clock speed-up on
4 vCPUs, same work. Decomposition stage only; no leaderboard or `benchmarks/`
row (validation gate).

Reproduce: `run_shard.sh target/release/examples/n83_chain_sat_bench raw/m<m> <m> <d>`
(m = 8, d = 4; m = 9, d = 5, one after the other);
`cargo run --release --example n83_chain_sat_analyze -- <this dir> all-summands+t2.2+meet4+rebuild+indexed`;
`python3 plot.py ANALYSIS.json reports/run-20261101T2046Z/`. Raw records:
`raw.tar.xz` (sha256 in `raw.tar.xz.sha256`).
