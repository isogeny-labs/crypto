# Protocol: the F = 1 target-in-walk arm at k = 28 and 32, single-target ecbench (frozen 2026-11-08)

Frozen before any measured data of this study. Nothing below changes after the first session
runs; failed predictions stay failed and are reported as failed. Before freezing, the only
executions were `ecbench plan` (no run) and the pre-data model `scripts/model.py`, which uses
the measured phases of the k = 20 and 24 studies and no data of this study.

## 1. Question

The target-in-walk arm `twcj` (`wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,seed=run`, one-abscissa base)
has been measured at k = 20 and 24 only: 1.40x / 1.41x rho-neg at w = 0
(`research/curve25519_f1_targetwalk_20261108`) and 1.36x / 1.34x on fresh seeds
(`research/curve25519_tw_jumps_20261108`). At those sizes rho-neg carries a set-up overhead of
0.38 and 0.10 sqrt(r) that vanishes by k = 32 (rho-neg S 1.01 there, autolab OBSTRUCTION.md),
while the twcj arm's own set-up and verification shrink with 1/sqrt(r) too. Does the distance
to rho hold near 1.4x at k = 28 and 32, where both arms are close to their walk floors
(0.886 and 1.329 sqrt(r))? And does the jump count 32, whose shorter walk (-2.5 % and -1.5 % of
steps at k = 20, 24) was outweighed by its set-up at those sizes, pay at k = 28 and 32 where
the set-up is 0.01 sqrt(r) or less?

## 2. Implementation under test

No code change. Binary `/home/user/ctarget-f1/release/ecbench` (SHA-256 recorded per size in
`data/binary-k{28,32}.sha256`), the same as the k = 20, 24 studies. Both band readings
(committed, and `--price-table-work`) are reported.

## 3. Design (frozen)

- Curves: the family miniatures at k = 28 (`p = 268,435,157`, `r = 33,557,809`) and k = 32
  (`p = 4,294,967,197`, `r = 536,884,639`), as in the autolab holdout specs
  (`research/curve25519_autolab_20261104/round-033/specs/holdout-k{28,32}.json`).
- Sessions: 8 per k, `targets_per_curve = 1`, own `target_seed` and measurement `seed`
  (`scripts/make_specs.py`), disjoint from the other studies' seeds. 64 measured rounds plus
  1 warm-up per arm, `order = alternate`; warm-up records excluded from every pooled figure.
- Arms (pipeline arms use `prime-projected-abscissa:size=1,scan_cap=32`,
  `targets=stride-generator`):
  - `rho-neg` (reference): `rho.negation`.
  - `f1-x07` (baseline, table then probes): `wwalk:x=0.7,g=1,grow=2,seed=run`.
  - `twcj-j16` (**primary** candidate, the current arm): `wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,jumps=16,seed=run`.
  - `twcj-j32`, `twcj-j64` (candidates): the same with `jumps=32`, `jumps=64`.
- Counted unit: ecbench GAE, `S = cost/sqrt(r)`, every comparison at w = 0, 0.1, 1.
- Audit: `ecbench verify --replay 1` per session. Pooling: `scripts/pooled_compare.py`
  (unchanged copy; warm-up skipped), ratio of per-session sums, linearised t, df = 7.
- Analysis: `scripts/analyse.py` -> `data/pooled_measured.json`.
- Runtime cap: 30 minutes in total.

## 4. Model (derived, before data; `scripts/model.py` -> `data/model.json`)

| k | arm | model S (w = 0) | ratio |
|--:|:--|--:|:--|
| 28 | twcj-j16 | 1.435 | / rho-neg 1.38; / f1-x07 0.89 |
| 28 | twcj-j32 | 1.423 | / j16 0.991 |
| 28 | twcj-j64 | 1.438 | / j16 1.002 |
| 32 | twcj-j16 | 1.388 | / rho-neg 1.375; / f1-x07 0.874 |
| 32 | twcj-j32 | 1.367 | / j16 0.984 |
| 32 | twcj-j64 | 1.364 | / j16 0.982 |

Walk lengths 1.37 (j16), 1.345 (j32) and 1.335 (j64, extrapolated) sqrt(r); set-up from
`TargetWalk::cheap_jumps` (about 3.8 GAE per jump plus the doubling chains and K_0, K_1);
verification and fixed phases scaled from k = 20, 24; rho-neg 1.04 and 1.01.

## 5. Predictions and thresholds (frozen)

- **P1 (distance to rho).** At both sizes the w = 0 pooled ratio S(twcj-j16) / S(rho-neg) has
  its point estimate in [1.30, 1.50], and its lower end > 1.00 at w = 0 and, under
  `--price-table-work`, at w = 0, 0.1 and 1 (no parity).
- **P2 (vs the table-then-probe arm).** At both sizes S(twcj-j16) / S(f1-x07) at w = 0 has its
  upper end < 1.00; point estimate in [0.82, 0.93].
- **P3 (walk floor).** The mean `tw_steps` / sqrt(r) of twcj-j16 is in [1.30, 1.42] at both
  sizes, and twcj-j16's mean S at w = 0 is in [1.36, 1.50] at k = 32.
- **P4 (32 jumps).** S(twcj-j32) / S(twcj-j16) at w = 0 has its point estimate in
  [0.95, 1.00] at both sizes. **P4b** (scored separately): at k = 32 its upper end < 1.00.
- **P5 (64 jumps).** S(twcj-j64) / S(twcj-j16) at w = 0 has its point estimate in
  [0.95, 1.01] at both sizes.
- **P6 (rank-2 stop).** At least 99.5 % of twcj records solve at `tw_rank = 2` with zero probe
  hits on the first trial, at every jump count.
- Every record must be `verified`, and every session's `ecbench verify --replay 1` must
  reproduce. A failed audit excludes the session; nothing is rerun.

## 6. Reporting

README.md tabulates S per arm and weight (source file), the pooled ratios under both band
readings, the status of P1-P6, phase breakdowns, walk diagnostics, and the distance to
rho-neg at every weight. Scope as in the earlier studies: the F = 1 target-in-walk arm is one
unknown away from rho (P1 := G makes it rho with a stored walk); it is not decomposition-based
index calculus and proposes no leaderboard row. Every result sentence ends with
[pcc:unverified].
