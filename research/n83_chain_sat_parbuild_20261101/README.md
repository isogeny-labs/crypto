# Chained-S3 on N83, m = 9: parallel build of the depth-5 fingerprint set

Protocol `PROTOCOL.md` (frozen `27ef5fa874`); run 2026-11-01 22:58:33Z –
23:49:48Z, L0 4-vCPU host, processes S1, P1, S2, P2 one at a time. Report:
`reports/run-20261101T2258Z/report.pdf`.

| depth | fingerprints | sequential build s (S1; S2) | parallel build s, 4 threads (P1; P2) | ratio of means |
|---|---|---|---|---|
| 5 | 372,406,144 | 1,006.8; 993.4 | 287.7; 283.3 | 3.50× |
| 4 | 12,976,303 | 27.0; 25.2 | 7.5; 7.9 | 3.4× |

Identical sets (counts) in all processes; every process decides all 8 m = 9
queries (32 records, 0 soundness flags). Wall-clock speed-up, same work.
Decomposition stage only; no leaderboard or `benchmarks/` row.

Reproduce: `run_all.sh target/release/examples/n83_chain_sat_bench raw`;
`cargo run --release --example n83_chain_sat_analyze -- <this dir> all-summands+t2.2+meet5+rebuild+indexed+parallel`;
`python3 plot.py ANALYSIS.json reports/run-20261101T2258Z/`. Raw records:
`raw.tar.xz` (sha256 in `raw.tar.xz.sha256`).
