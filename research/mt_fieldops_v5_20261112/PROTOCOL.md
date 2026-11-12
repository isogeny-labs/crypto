# Multi-target IC v5: the stronger arms on both sides, with rho's native work counted (protocol, frozen before data)

Follows `../mt_fieldops_v4_20261112/` (PR #3959, claim PCC-20261112-830272 [pcc:unverified]). Its validator found
no ranking established: the v4 decision rule priced h on hash-table operations only, while rho's distinguished-point
walk canonicalises and hashes every point (section 5a prices such native work in the band); and each side carried
a handicap (IC: ic5ks retained along one shared arithmetic progression of offsets; rho: rhodp's steps included
duplicates, and rho2xks's best kappa sat on the grid floor). This study pre-registers the stronger arms the
validator built as sensitivity arms and counts the native work, so the comparison can be decided.

## Question

Is the multi-target IC (best of four variants) cheaper or dearer per target than the cheapest multi-target rho of
this harness (Kuhn-Struik retention rho or distinguished-point rho with distinct steps), in counted field
operations with every arm's native work priced across the section 5a band, on fresh curves?

## Harness change (one binary: `examples/mt_fieldops.rs` at the commit that adds this file)

A new counter N ("native work", printed as `"N"`) counts work that is neither a field operation nor a hash-table
operation, one per:
- step-index computation in every r-adding rho walk (`rho`, `rho2`, `rho2x`, `rho2xks`, `rhoseq`);
- generated point of the distinguished-point walks (`rhodp`, `rhodpu`): 2 (one canonicalisation and one hash of
  x; the step index and the distinguished-point test are bits of that one hash and are not counted again);
- offset-step index of the IC's random-walk offsets (`ic5w`, `ic5rw`).
The IC's trials and table use only field operations and hash-table operations (H). Existing counters I, M, S, H, L
are unchanged for every existing arm (checked by replaying v4 rows).

## Arms

- IC (k = 4, mu = 1.0 frozen in `../mt_fieldops_v3_20261111/frozen_ic5.json`): `ic5`; `ic5ks` (retention);
  `ic5w` (offsets W_(k+1) = W_k + D_(j(k)), D_j 32 random known-log points, j(k) from splitmix64(k): no
  retention); `ic5rw` (the same offsets with retention). `ic5w` and `ic5rw` are the v4 validator's sensitivity arms
  (`../mt_fieldops_v4_20261112/validation/scripts/patch_ic5rw.py`), now pre-registered.
- Rho: `rho2xks` (retention rho, x-only sibling walks), kappa in {0.05, 0.1, 0.25, 0.5}; `rhodpu`
  (distinguished-point rho with 1024 distinct steps, the v4 validator's `patch_rhodpu.py`), precomputation kappa in
  {0, 0.25} x d offset in {-1, 0, 1}.
- `bsgs2`, mf in {0.7, 1.0, 1.4} (distance remaining).
- Analysis only: `icbest` = the cheapest of the 4 IC runs per configuration and price; `rhobest` = the cheapest of
  the 10 rho runs per configuration and price.

## Instances (fresh)

`instances/`: 33 prime-order Weierstrass curves, `../ic_e222_m221_20261106/gen_instances.py --families W --bits
24,28,32 --per 11 --seed mtfo-v5-20261112` (indices 0..10), 0 overlap with the 1261 curve records under
`research/` and `docs/curves`.

## Holdout (no tune phase; every grid point is run and the best per configuration and price is taken)

Seeds 41 and 42; cells {24, 28, 32} bits x T {1024, 4096, 16384}: 9 cells x 22 paired configurations x 17 runs =
3366 runs. Every target of every run must verify; failures are reported, not dropped.

## Cost unit and decision rule

Primary (section 5a): cost per target in GAE = [(c_I I + M + S)/(c_I + 3) + h (H + L + N)] / T, c_I in {3, 10, 30},
h in {0, 0.1, 1}. Per (cell, c_I), X vs Y: **cheaper** if the bootstrap 95% upper bound of the median
per-configuration ratio X/Y is below 1 at all three h; **dearer** if the lower bound is above 1 at all three;
otherwise **unranked**. Comparisons: icbest, ic5 and ic5rw vs rhobest, rho2xks, rhodpu and bsgs2.
Secondary (the v4 rule, reported for continuity): the same with h on H + L only.

## Predictions (from one smoke run per arm at 24 b, T = 1024 and 32 b, T = 16384; not data)

- rho2xks at small kappa is the cheapest rho at h = 0 and 0.1; at h = 1 the cheapest rho is rho2xks or rhodpu.
- icbest vs rhobest: rho cheaper at h = 0 (IC about 1.1x rho), IC cheaper at h = 1
  (about 0.7-0.75x), close to 1 at h = 0.1: unranked in most or all 27 decisions; no cell cheaper.
- ic5rw is the cheapest IC at h = 0; ic5 or ic5w at h = 1.
- icbest dearer than bsgs2 everywhere.

## Limits known in advance

Group orders about 2^24 to 2^32; counted, not timed; memory unbounded for every arm; local host (no AWS identity,
so not isolab-eks); the rho arms are this harness's; pricing one native operation at h, the same as a hash-table
operation, is the section 5a convention, not a measured cost; the verification constant (about 30 to 50 GAE per
target) is shared by every arm and compresses ratios toward 1 (reported with and without it).
