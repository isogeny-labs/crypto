# Chained-S3 SAT on N83, m = 5…9: meet-in-the-middle lookup

Protocol `PROTOCOL.md` (frozen `0704c3b008`, m = 5…7) and amendment
`PROTOCOL_m8_9.md` (m = 8, 9, frozen before its data). Report:
`reports/run-20261101T0150Z/report.pdf`, graphs beside it.

| m | d | lookup build (once) | control A2: public, planted | A4 lookup: public refuted | A4 median public s |
|---|---|---|---|---|---|
| 5 | 3 | 0.68 s | 0/3, 0/2 | **3/3**, 0 conflicts | 0.80 |
| 6 | 3 | 0.68 s | 0/3, 0/2 | **3/3**, 0 conflicts | 1.39 |
| 7 | 4 | 26.1 s | 0/3, 0/2 | **3/3**, 0 conflicts | 2.54 |
| 8 | 4 | 27.2 s | 0/3, 0/2 | **3/3**, 0 conflicts | 76.7 |
| 9 | 5 | 958.8 s | 0/3, 0/2 | 0/3 (cap) | — |

- 50 records, 0 soundness flags. Planted: 0/2 for both arms at every m; the
  lookup domain is non-empty on every planted target (10–250) and empty on
  every public one.
- A meet-in-the-middle in front of the SAT instance: per-query cost ~|B|^(m−d)
  after a |B|^d build. Decomposition stage only; no leaderboard or
  `benchmarks/` row (validation gate).

Reproduce: `cargo build --release --example n83_chain_sat_bench`;
`run_shard.sh target/release/examples/n83_chain_sat_bench raw/m<m> <m> <d>`;
`cargo run --release --example n83_chain_sat_analyze -- <this dir> all-summands+t2.2`;
`python3 plot.py ANALYSIS.json reports/run-20261101T0150Z/`. Raw records:
`raw.tar.xz` (sha256 in `raw.tar.xz.sha256`).
