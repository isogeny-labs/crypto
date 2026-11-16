# N83 retained-base exact rank-query census

Run `N83-RQC-20261115-v1`, 2026-11-15 UTC. **Analytic necessary-bound screen**, not a timed factor-base or end-to-end index-calculus comparison. Protocol: [PROTOCOL.md](../../native_rank_query_census/PROTOCOL.md). Raw exact rows: [results.json](../../native_rank_query_census/results.json). Native implementation: [src/main.rs](../../native_rank_query_census/src/main.rs). [Graph](rank-query-plot.svg) plots selected primary-curve size classes on a logarithmic query-count axis; its source is [plot.rs](../../native_rank_query_census/src/bin/plot.rs).

## Fixed inputs and model

The input is the October 77-object S3 manifest census (`verification/live-s3-manifest-census-20261025/evidence-01/catalog.json`, SHA-256 `1117ead0dfcc9164a73d83d5ca709ec538528f7042e070ca8d72a5e188487ade`) plus the later primary-curve full-D18 export (`verification/d18-full-orbit-20261025/manifest.json`, SHA-256 `1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce`). The primary curve is `icv1-f2m83-tm6151469093347-debefd74`, subgroup order `2417851639230796216685689`. The `a1` subgroup order `8569786107849059` is a separate diagnostic.

For a base claiming `B` distinct points and `K` orbit columns, the exact number of unordered `m`-point multisets is `M=binomial(B+m-1,m)`. If each query target is uniform over the whole subgroup and the oracle returns at most one independent full-smooth row, the hit probability is at most `min(M,r)/r`. By the first-moment bound, the probability of rank `K` after `q` queries is at most `min(1,q min(M,r)/(rK))`, and zero for `q<K`. The reported `q_min=max(K,ceil(Kr/(2 min(M,r))))` is necessary before that ceiling can reach one half. Query outcomes need not be independent. The declared `B` and `K` values are manifest claims, not a new semantic validation of all stored point sets.

## Derived results

All 78 source-claimed objects yielded 390 exact `(object,m)` rows for `m=2..6`: 51 primary objects (26 distinct declared `(B,K)` sizes) and 27 diagnostic objects (3 distinct declared sizes). The native screen reproduced all 30 frozen historical six-base cases, with exact threshold checks immediately before and at each `q_min`.

| Primary declared base | `K` | `B` | `m=4` necessary queries | `m=5` | `m=6` |
| --- | ---: | ---: | ---: | ---: | ---: |
| Retained orbit base | 64 | 10,624 | 145,677,800,576 | 68,534,909 | 38,688 |
| Retained orbit base | 256 | 42,496 | 2,277,179,841 | 267,904 | 256 |
| Retained orbit base | 600 | 99,600 | 176,888,106 | 8,880 | 600 |
| Full direct-subgroup D18 export | 32,643 | 5,418,738 | 32,643 | 32,643 | 32,643 |

These are lower bounds on query counts under the fixed oracle model; they are not achieved query counts, wall time, or a measured gain. The large full-D18 base makes the combinatorial bound vacuous at these arities: the condition collapses to `q>=K`, while its construction, decomposition, and linear algebra costs remain unpriced. The primary and diagnostic rows are never pooled.

## Validation and claim boundary

Local host: Darwin 27.2.0 arm64, macOS 27.2. The nested native crate used Homebrew Rust 1.93.1 in release mode. Native Rust release tests: 3 passed, 0 failed. They check small exact multiset arithmetic, the threshold, pinned source digests with a changed-byte rejection, 77+1 object counts, and all 30 historical cases. The runner also rejects duplicate object keys and zero sizes. The native release binary wrote `results.json` once; repeat execution checks byte identity. The graph renderer reads only that file and also enforces write-once identity. Run commands:

```sh
cargo test --manifest-path research/koblitz_n83_factor_base_sweep_20261008/native_rank_query_census/Cargo.toml --release --locked --offline
cargo run --manifest-path research/koblitz_n83_factor_base_sweep_20261008/native_rank_query_census/Cargo.toml --release --locked --offline --bin n83-native-rank-query-census
cargo run --manifest-path research/koblitz_n83_factor_base_sweep_20261008/native_rank_query_census/Cargo.toml --release --locked --offline --bin plot
```

A base-size screen cannot select the factor base that minimizes fully charged single-target IC runtime. Required empirical next steps are same-instance cold construction/import, natural relation yield and rank, decomposition/solver costs (including WDSat/FES and large-prime variants where implemented), descent, verification, and matched rho. The frozen K600 `m=6` hosted run remains gated on a qualified host, exact input access, and its own budget authorization. New-bucket publication of existing artifacts remains pending exact writer/destination confirmation. No canonical performance graph changes: this run measured no runtime. Checked `docs/ic/BOUNDARY_TARGETS.md` and the study's `PAIRED_FRONTIER_V5.md`; neither receives a new measured row from this screen.
