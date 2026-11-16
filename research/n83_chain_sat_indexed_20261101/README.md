# Chained-S3 on N83, m = 8, 9: indexed lookup and rebuild by peeling

Protocol `PROTOCOL.md` (frozen `7304b82d81`); run 2026-11-01 17:53:15Z –
18:33:36Z, L0 host. Report: `reports/run-20261101T1753Z/report.pdf`.

| m | d | A5 merged lookup + rebuild: public / planted | A6 indexed + peel: public / planted | median s A5 → A6 (public; planted) |
|---|---|---|---|---|
| 8 | 4 | 3/3, 5/5 | 3/3, 5/5 | 78.4 → 39.7; 83.1 → 40.9 |
| 9 | 5 | 0/3, 0/5 (cap) | **3/3, 5/5** | cap → 54.1; cap → 54.7 |

32 records, 0 soundness flags, no solver calls on any decided target.
Depth-5 set: 372,406,144 fingerprints, 999 s once per process. Decomposition
stage only; no leaderboard or `benchmarks/` row (validation gate).

Reproduce: `run_shard.sh target/release/examples/n83_chain_sat_bench raw/m<m> <m> <d>`;
`cargo run --release --example n83_chain_sat_analyze -- <this dir> all-summands+t2.2+meet4+rebuild`;
`python3 plot.py ANALYSIS.json reports/run-20261101T1753Z/`. Raw records:
`raw.tar.xz` (sha256 in `raw.tar.xz.sha256`).
