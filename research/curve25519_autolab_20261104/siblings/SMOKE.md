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

# Amendment of 2026-11-10: exact charging, product-set pool, second smoke (written before any new run)

Status: [pcc:unverified]. The section above stays as the record of the first smoke, whose pre-registered bit-op ratio of 0.62 was missed (measured 2.299) [pcc:unverified]. This amendment is committed before the second smoke is run.

## Reasons [pcc:unverified]

1. The first charging priced a pair one multiplication above what it performs. Reason: the group addition's `y` product is not performed by a non-continuing pair, and the price must equal the field work [pcc:unverified].
2. The pool's set-up was 1,587 additions a run for 256 jumps at k = 20 (about 6 a jump), which is 4.4 sqrt(r) and more than the whole `tw2cj-lazy` walk. Reason: a product set `J = A_i + K_j` needs only about `a * popcount + b + a * b` additions for `a * b` jumps [pcc:unverified].

## Changes [pcc:unverified]

- Charging: a pair performs 1 inversion, 2 multiplications, 2 squarings and 8 additions (`dx`, `dy`, two for `x+`; the sum of the y values, its negation, two for `x-`). It is charged as one group addition (1 inv, 2 mul, 1 sqr, 6 add) plus one `sibling_second_points` priced as 1 sqr + 2 add, so the `bitop/v1` price of the pair equals the price of its field work (unit test `sibling_pair_bitop_price_equals_its_field_work`). The continuing child's `y` is a second counter, `sibling_lift_products` (1 mul + 2 add), once a parent. A pair whose minus child is skipped is charged the addition alone, one multiplication and two additions above what it performs (at most two such pairs a parent).
- Pool: product set with `a = 4`, `b = 16` for M = 64 and `a = 8`, `b = 32` for M = 256 (`a = 4` below 256, so `pool` must be a multiple of `a`); the `K_j` chain `K_j = K_(j-1) + K_(j-2)` as `cj=1`; coefficient vectors follow.

## Pre-registered, second smoke [pcc:unverified]

Specs `spec-smoke2-k20.json` (same curve and seeds as the first smoke; arms `rho-neg`, `tw2cj-lazy`, `tw2cj-lazy-sib8-p64`, `tw2cj-lazy-sib8-p256`) and `spec-smoke2-k28.json` (first curve of `snc-panel/spec-k28.json`; arms `rho-neg`, `tw2cj-lazy`, `tw2cj-lazy-sib8-p64`), 12 targets, 2 rounds each [pcc:unverified].

- Bit-op ratio `tw2cj-lazy-sib8-p64` / `tw2cj-lazy`, total including set-up: 0.70 [0.62, 0.80] at k = 20 and 0.64 [0.58, 0.72] at k = 28 [pcc:unverified].
- Stored points per sqrt(r) within 5 percent of `tw2cj-lazy` [pcc:unverified].
- Every record verified and 4 replays reproduced a session [pcc:unverified].
- Reported separately: the walk-only ratio and the set-up share of each arm's `oracle_setup` phase [pcc:unverified]. No pre-registration for `p256`, which is reported as measured [pcc:unverified].

## Results of the second smoke [pcc:unverified]

Sessions `smoke2-k20` (144 records) and `smoke2-k28` (108 records), every record verified, `verify --replay 4` reproduced 4 replays with 0 problems in each, and the k24 panel session replays with the new binary (`--replay 8`: 2,700 records, 8 reproduced, 0 problems) [pcc:unverified]. 24 measured runs an arm, paired bootstrap 95 percent intervals, `smoke_stats2.py` writing `smoke2-stats-k20.json` and `smoke2-stats-k28.json`, bands in `band-smoke2-k*.txt` [pcc:unverified].

Price per point after the fix, one pair over two points against one group addition, from the unit test [pcc:unverified]:

| field bits | pair, bit-ops | per point / group addition |
|--:|--:|--:|
| 20 | 31,920 | 0.6014 |
| 24 | 45,408 | 0.6006 |
| 28 | 61,264 | 0.6001 |
| 32 | 79,488 | 0.5997 |

The note's 0.59 counted 6 additions a pair; the pair performs 8 (two more for the negated sum of the y values), which gives 0.60 [pcc:unverified].

Set-up additions a run (means), `tw_setup_adds` with doubles in brackets [pcc:unverified]:

| configuration | k = 20 | k = 28 |
|:--|--:|--:|
| `tw2cj-lazy` (16 chained jumps) | 127.9 (66) | 173.0 (94) |
| sib8, pool 64 (a = 4, b = 16) | 126.2 (68) | 149.4 (100) |
| sib8, pool 256 (a = 8, b = 32) | 350.2 (68) | not run |
| first smoke, pool 256, 6 a jump | 1,587 | not run |

Per sqrt(r), means [pcc:unverified]:

| size | arm | stored points | S (w = 0) | S (w = 0.1) | S (w = 1) | bit-ops |
|:--|:--|--:|--:|--:|--:|--:|
| k = 20 | rho-neg | 0.857 steps | 1.337 | 1.428 | 2.243 | 37,710 |
| k = 20 | tw2cj-lazy | 1.745 | 2.534 | 2.719 | 4.384 | 71,081 |
| k = 20 | sib8 p64 | 1.796 | 1.725 | 2.017 | 4.641 | 55,113 |
| k = 20 | sib8 p256 | 1.847 | 2.361 | 2.661 | 5.361 | 72,174 |
| k = 28 | rho-neg | 0.870 steps | 0.948 | 1.036 | 1.831 | 50,248 |
| k = 28 | tw2cj-lazy | 1.769 | 1.839 | 2.017 | 3.616 | 94,705 |
| k = 28 | sib8 p64 | 1.932 | 1.050 | 1.352 | 4.069 | 65,459 |

Against the pre-registered intervals, ratio of means with paired bootstrap 95 percent interval [pcc:unverified]:

| quantity | measured | pre-registered | inside |
|:--|--:|:--|:--|
| k = 20, bit-ops p64 / tw2cj-lazy | 0.775 [0.713, 0.846] | 0.70 [0.62, 0.80] | point yes, interval end 0.846 above 0.80 |
| k = 28, bit-ops p64 / tw2cj-lazy | 0.691 [0.606, 0.793] | 0.64 [0.58, 0.72] | point yes, interval inside |
| k = 20, stored points p64 / tw2cj-lazy | 1.029 [0.895, 1.183] | within 5 percent | point yes, interval wide |
| k = 28, stored points p64 / tw2cj-lazy | 1.092 [0.944, 1.269] | within 5 percent | point no (9.2 percent above), interval holds 1 |
| verified, replays | 144 and 108 of 144 and 108; 4 and 4 reproduced | all, 4 | yes |

Other ratios [pcc:unverified]: k = 20, p256 / tw2cj-lazy 1.015 [0.917, 1.123] in bit-ops; p64 / rho-neg 1.461 [1.280, 1.682] at k = 20 and 1.303 [1.085, 1.595] at k = 28; tw2cj-lazy / rho-neg 1.885 at both sizes.

Walk-only ratio (every part of `oracle_setup` except the set-up, priced at the records' own prices) and set-up share, post hoc [pcc:unverified]:

| size | arm | walk-only / tw2cj-lazy | set-up share of `oracle_setup` |
|:--|:--|--:|--:|
| k = 20 | tw2cj-lazy | 1 | 0.246 |
| k = 20 | sib8 p64 | 0.655 [0.570, 0.752] | 0.334 |
| k = 20 | sib8 p256 | 0.669 [0.556, 0.807] | 0.505 |
| k = 28 | tw2cj-lazy | 1 | 0.027 |
| k = 28 | sib8 p64 | 0.678 [0.588, 0.785] | 0.037 |

## Reading, post hoc [pcc:unverified]

- The walk-only ratio is 0.66 to 0.68 where the per-point price alone says 0.60: the rest is the lift products, the sign resolutions at collisions, the table inserts and a stored-point count 3 to 9 percent above `tw2cj-lazy`'s [pcc:unverified].
- The product-set pool removes the first smoke's miss: the p64 set-up is as cheap as the 16-jump chain's (126 against 128 additions at k = 20), so the total ratio falls from 2.30 to 0.775 at k = 20 and is 0.691 at k = 28 [pcc:unverified].
- In the band the mechanism wins at w = 0 and loses at w = 1: the second points and lifts are native operations, and at w = 1 `sib8 p64` is 4.64 against 4.38 at k = 20 and 4.07 against 3.62 at k = 28 [pcc:unverified]. The band ranks `tw2cj-lazy` against `sib8 p64` as unresolved inside the band at both sizes [pcc:unverified].
- Neither sib8 arm reaches `rho-neg` in bit-ops: 1.46 and 1.30 times it [pcc:unverified].
