# N83 K1120 padd rank stage: frozen protocol

Frozen 2026-10-19, before any timing of this change on the measurement
runner. The only local runs are the bit-identity check on sealed point 1
(`identity_check.py`) and a one-point harness smoke at K = 896; neither
is timing evidence and neither sets a threshold below.

## Question

Does computing each rank-stage candidate's two partner roots by point
addition (`KIC_RANK_SOLVE=padd`, ported from `aburan28/cryptanalysis`
PR #970) lower the end-to-end cold one-target time of crypto's frontier
pipeline, K1120/root11 dynamic (`research/n83_e2e_holdout_20261016`)?

## Baseline named before optimising

The baseline is the same binary with `KIC_RANK_SOLVE` unset: the K1120
producer of the current tree, whose `s3` path is the code of
`scan_state128` unchanged. Both arms use the identical environment of
`research/n83_root11_dynamic_20261016/run.py::environment`
(`KIC_ROOT_ONLY11=1`, `KIC_RANK_THREADS=4`, symmetric S3, sparse
canonical, rank seed 29, base `construct:83:1:1120`). One binary means
the switch is the only variable; a binary built from main without the
change is used only for the bit-identity check.

## Arms and design

- `cur_a`, `cur_b`: baseline twice (the A/A pair); `padd`: the change.
- 20 points: the five sealed holdout points (point-file SHA-256
  `722bc60c...f93d26`) plus 15 new (`generate.py`, domain
  `n83-padd-rank-fifteen-20261019-v1`), every one on the curve, in the
  order-r subgroup, disjoint from every committed N83 point file and the
  1000 derived wide3 points. Earlier archived fixtures that are not in
  the repository could not be replayed; the collision probability with
  them is about 1e-12 and is a stated limit, not a checked fact.
- Per point: three fresh IC processes in a rotating Latin order (six
  orders cycled over points), and three fresh same-point rho processes
  (config `research/n83_dense_root_20261013/rho_config.json`, the strong
  rho of the holdout), seeds `2026101900 + 10 * point + draw`, run before
  the IC arms on even points and after on odd points. 60 rho draws.
- One GitHub `ubuntu-24.04` runner (4 vCPU), the tier of the holdout.
  Caps: 480 s per IC child, 120 s per rho child.

## Gates (any failure makes the run INVALID)

1. Every IC arm recovers the sealed scalar of its point; every rho draw
   recovers it and is replayed generically.
2. Per point, identical across the three IC arms: the target relation
   (point indices, probes), rank attempts, relations, failures, rows
   without gain, rank, mean rank probes, and the SHA-256 of the dumped
   factor base.
3. `padd`: `rank_solve = padd`, zero fallback states, zero irregular
   inputs, and one resolved hit per rank relation. Baseline arms:
   `s3`.
4. A/A: the ratio `cur_a / cur_b` of mean cold time and of mean rank
   stage must each lie in [0.97, 1.03].
5. `cargo test` of the example passes, including the padd tests (padd,
   padd_check and order-swapped emission equal the S3 scan's relation,
   intermediates and probes).

## Primary and secondary quantities

- **Primary**: paired cold saving per point, mean of the two baseline
  arms minus `padd`, in seconds of child wall time (start of the fresh
  process to its exit: construction, rank stage, one target, output).
  Two-sided 95% interval by the percentile bootstrap over points
  (10,000 draws, seed 20261019); the Student-t interval (t = 2.0930,
  df = 19) is reported beside it.
  - `IMPROVEMENT` iff the A/A gate passes and the bootstrap lower bound
    is above 0 s; `REGRESSION` iff the upper bound is below 0 s;
    otherwise `NOT_SEPARATED`.
- **Rank-stage speedup**: ratio of mean baseline rank stage to mean
  `padd` rank stage with a point-resampled bootstrap interval.
- **Kill condition (pre-registered)**: if the point estimate of the
  rank-stage speedup is below **1.2**, record that the idea does not
  transfer to this pipeline, whatever the cold interval says.
- **Cold IC/rho**: mean cold time of each IC arm over the mean rho
  target-online time (all 60 draws), interval by resampling points and,
  inside each, rho draws. Also reported against rho child wall time and
  on the five sealed points alone (comparable to the earlier 5-draw
  figure of about 55).
- Secondary: setup, per-target online, peak RSS, position effects.

## Section 5a (pricing)

Every arm's unpriced native work is listed and priced at 0, 0.1 and 1
group-addition equivalents per operation: IC probes, canonicalisations
(one per probe), table queries (one per probe), root solves (one per two
probes); rho canonicalisations, partition hashes, table queries and
inserts from its receipt. Two arms are ranked only where the order holds
at every weight. The two IC arms have identical native counts, so the
change is an implementation (wall-clock) gain and is reported as such,
not as a change in counted S.

## Limit on extrapolation

Every complete N83 result is on the a = 1 curve (53-bit prime subgroup,
r = 8569786107849059). The a = 0 curve (81-bit subgroup) is the
ECC2K-130 analogue and has no complete index-calculus result. Nothing
here is evidence about ECC2K-130 or about any asymptotic exponent.
Wall-clock is a practicality measurement, not the headline metric.
