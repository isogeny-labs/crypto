# Sibling points, Stage 1 smoke measurement (k = 20, one curve)

Status: [pcc:unverified]. A smoke run, not the panel. No pcc claim id. Every number below is an unverified assertion [pcc:unverified].

## What was built [pcc:unverified]

`wwalk:...,tw=1|2,...,sib=m,pool=M` (m in 1..16, M in m..512, default off) in `src/cryptanalysis/ic_boundary.rs` (`TargetWalk`, `build_ext_sib`), `ic_framework/plugins.rs` (parameters `sib`, `pool`), `ecbench/work.rs` (rule for `sibling_second_points`) and `ecbench/band.rs` (`SIBLING_WORK_COUNTERS`, so the band prices the counter at w). A pool of M chained jumps is built as `cj=1` builds them. Each parent draws its own random m-subset, forms one inversion per chosen jump and the two x-only children from it, keys all of them with coefficient vectors `c_P +- c_J`, continues from `P + J_h` (h the first of the subset, y by one more multiplication) and skips the two known-copy minus children. A key collision on an x-only point resolves its sign bit at that moment, charged as one addition.

## Charging rule as implemented [pcc:unverified]

- `field_ops` are tallied as performed and never through `add`: per pair one inversion, per child one multiplication and one squaring, per parent one more multiplication. Per parent that is m inversions, 2m + 1 multiplications and 2m squarings, less one multiplication and one squaring per skipped minus child.
- The group ledger charges a pair as one group addition and the second x-only point as one unit of the native counter `sibling_second_points`, priced in `bitop/v1` as 1 field_mul + 1 field_sqr + 3 field_add and by the band at w = 0, 0.1, 1.
- The group addition also contains a y multiplication that a non-continuing pair does not perform, so the group-ledger price is one multiplication above `field_ops` for those pairs (conservative). The pair costs 37,040 bit-ops at b = 20 for two points, 0.698 of a rho step per point, not the note's 0.592 [pcc:unverified].
- The `ic.pipeline` record carries no `field_ops` block, so the smoke reads the walk's tallies from the counters `tw_walk_invs`, `tw_walk_muls`, `tw_walk_sqrs`. At m = 8 the means are 304.3 inversions, 645.4 multiplications and 607.4 squarings over 38.04 parents, which is the formula with 1.3 skipped minus children a parent set [pcc:unverified].

## Tests and replay [pcc:unverified]

- `cargo test --release --lib sibling`: 2 new tests pass. The first has 128 builds (tw = 1 and 2 through one and two seeds, verified and lazy, (m, M) = (4, 32) and (8, 64)) reaching full rank 128 of 128, recovering the true log every time, with zero lazy failures, and checks the field-op formula, the ledger sums and that sib off reproduces `build_ext_cv` (rows, collisions, ledgers, counters). The second runs `sib` through the parameter parser and the pipeline's `[d]G = Q` check. [pcc:unverified]
- `cargo test --release --lib -- wwalk target_walk lazy_removes band:: work::`: 207 tests, 203 passed first time and the 4 failures were band tests whose committed `records.jsonl` was not hydrated; after `artifacts.py pull` the band tests pass (8 of 8).
- Replay with the new binary: `ecbench verify --dir snc-panel/sessions/k24 --replay 8`: 2,700 records, 2,700 verified, 8 replays reproduced, 0 problems.
- Smoke session `verify --replay 4`: 108 records, 108 verified, 4 replays reproduced, 0 problems.
- rustfmt on the four touched Rust files only; clippy result in the session report.

## Smoke design [pcc:unverified]

`spec-smoke-k20.json`: spec-k20 with arms `rho-neg`, `tw2cj-lazy` and `tw2cj-lazy-sib8` (the `tw2cj-lazy` oracle plus `,sib=8,pool=256`), one curve (p = 1,047,989, r = 131,111, sqrt(r) = 362.09), 12 targets, 2 rounds (round 0 is the warm-up, so 24 measured runs an arm), `--cpus auto --allow-busy`, L1. 108 records, all verified. Statistics: `smoke_stats.py`, output `smoke-stats.json`; band `band-smoke-k20.txt`. Intervals are paired bootstrap 95 percent (4,000 resamples over the 24 target-round pairs). [pcc:unverified]

## Results [pcc:unverified]

Per sqrt(r), means of 24 runs [pcc:unverified]:

| arm | stored points | GAE S (w = 0) | S (w = 0.1) | S (w = 1) | bitop/v1 per sqrt(r) |
|:--|--:|--:|--:|--:|--:|
| rho-neg | 0.857 walk steps (not stored points) | 1.337 | 1.428 | 2.243 | 37,710 |
| tw2cj-lazy | 1.745 (SE 0.100) | 2.534 | 2.719 | 4.384 | 71,081 |
| tw2cj-lazy-sib8 | 1.664 (SE 0.101) | 5.681 | 5.942 | 8.287 | 163,419 |

Stored points are `table_insert_attempts` for both walk arms. The band ranks rho-neg < tw2cj-lazy < tw2cj-lazy-sib8 at every w [pcc:unverified].

Ratios in bit-ops per sqrt(r) [pcc:unverified]:

| ratio | point | 95 percent interval | pre-registered |
|:--|--:|--:|--:|
| sib8 / tw2cj-lazy | 2.299 | [2.131, 2.470] | 0.62 [0.56, 0.70] |
| sib8 / rho-neg | 4.334 | [3.790, 4.977] | not registered |
| tw2cj-lazy / rho-neg | 1.885 | [1.597, 2.225] | not registered |
| stored points, sib8 / tw2cj-lazy | 0.954 | [0.802, 1.115] | within 5 percent |

## Against the pre-registered expectation [pcc:unverified]

- Bit-ops sib8 / tw2cj-lazy: the expectation 0.62 [0.56, 0.70] is not met at k = 20, the measured ratio is 2.299 [2.131, 2.470] [pcc:unverified].
- Stored points: the point estimate 0.954 is inside 5 percent of tw2cj-lazy's, but the interval [0.802, 1.115] is wide at 24 runs, so the criterion is not tested at this size [pcc:unverified]. Stage 0 gave 0.96 to 0.99 of D0 for (8, 256) [pcc:unverified].
- The whole miss is the pool's set-up (post hoc, not pre-registered). `tw_setup_adds` is 1,587 additions a run for the 256-jump pool against 128 for the 16-jump chain, while the walk is 304 pair additions against 632 additions.

Post hoc decomposition of the `oracle_setup` phase in bit-ops per sqrt(r), priced at b = 20 (group_add 26,540, group_double 31,920, mul = sqr 4,860, add 260, hash_probe 80, read off the records' own primitive counts) [pcc:unverified]:

| part | tw2cj-lazy | tw2cj-lazy-sib8 |
|:--|--:|--:|
| jump or pool set-up | 15,191 | 122,309 |
| walk (pairs or additions) | 46,302 | 22,306 |
| second points | 0 | 8,788 |
| sign resolutions at collisions (6 a run) | 0 | 440 |
| table inserts | 140 | 133 |
| walk total | 46,441 | 31,667 |

The walk-only ratio is 0.682 [pcc:unverified]. That is the near-top-of-interval value the charging rule predicts (0.698 a point against a rho step, less the walk's saving), and it is a post hoc number, not a claim of the mechanism's total cost. At k = 20 the pool set-up (M = 256) is about four times the whole `tw2cj-lazy` walk, so the mechanism cannot pay here; whether it pays when the set-up is a small share (k = 28, where sqrt(r) is 16,384 and the set-up is a fixed 1,600 additions) is not measured by this smoke [pcc:unverified].

Nothing was tuned after the data: `pool` and the spec are as first written [pcc:unverified]. All 36 sib8 records (24 measured, 12 warm-up) verified with no lazy failure [pcc:unverified].
