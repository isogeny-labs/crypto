# Protocol: the `ic.pipeline` oracle set-up, scalar against batched against cache-blocked pair table

Frozen 2026-10-20, before any recorded session. Changes after data go under
"Deviations" in the README, never edited in here.

Follows `research/ic_linalg_pipeline_20261019`, whose result named the oracle's
|F|² pair-table set-up as the bottleneck (84–89% of a run's wall time at
|F| ≥ 8,192). This protocol measures a new builder for that table,
`mitm-prime-blocked`, against the two builders that already exist.

## Question

On a 32-bit prime curve, at |F| = 1,024 to 16,384 abscissae, how much
oracle set-up wall time does the cache-blocked builder save against the
batched builder (`mitm-prime-batch`) and the scalar builder
(`mitm:negation_folded=1`), with everything else in the run unchanged?

## What the three builders are

All three build the same negation-folded pair table: the same keys, the same
witness for every key (the first pair to reach it), the same probe, and the
same counted operations (one abstract addition per folded pair, about |F|²).

- `mitm:negation_folded=1` (scalar): one affine addition, with its own
  inversion, per pair; one large hash map.
- `mitm-prime-batch:m=2` (batched, `research/ecbench_rootfinding_batchtable_20261013`):
  one batched Montgomery inversion per abscissa row, and only the sum's x;
  the same single map.
- `mitm-prime-blocked:m=2` (cache-blocked, this study): the batched
  builder's arithmetic and order. When p < 2³² and the base has at most 65,536
  signed points, an entry is one 64-bit word (key and witness). The table is
  split into regions of about 2¹⁴ entries by a second hash of the key; each
  region's sums are gathered in order, then inserted into an open-addressed
  region of one array in one pass. Larger fields fall back to the same split
  over small hash maps. The unit test
  `prime_blocked_table_matches_batch_table_across_blocks_and_parts` checks both
  layouts against the batched table on every key of a 20-bit field.

## Boundary and reference

- Curve: `icv1-fp32-t7319-90da43e9` (registered; p = 2^32 − 5, a = −3, b = 31,
  prime order r = 4294959973, A = 2), `prime_explicit`, as in the previous study.
- Reference arm: `rho.negation`.
- Every IC arm: `factor_base=prime-abscissa:size=N`, `linalg=incremental-gauss`
  (the default, target-pinned stop rule, which the leaderboard's prime ladder
  uses), `targets=walk`. Only the oracle differs between a size's arms.

## Arms

| arm | oracle | N |
|:--|:--|:--|
| `rho-neg` (reference) | — (`rho.negation`) | — |
| `scalar-N` (baseline) | `mitm:negation_folded=1` | 1,024 · 2,048 · 4,096 · 8,192 |
| `batch-N` (baseline) | `mitm-prime-batch:m=2` | same |
| `blocked-N` (candidate) | `mitm-prime-blocked:m=2` | same |
| `blocked-4096-aa` (control) | repeat of `blocked-4096` | 4,096 |

Session A (`specs/A-1k-8k.json`): the arms above, 8 targets (seed 20261019,
the previous study's), 5 rounds, 1 warm-up, `order: alternate`,
`isolation_required: L2`, timeout 3,000 s per run.
Session B (`specs/B-16k.json`): `rho-neg`, `scalar-16384`, `batch-16384`,
`blocked-16384`; the first four targets, 2 rounds, 1 warm-up. Its wall times
are descriptive only (fewer than 5 rounds); its counts are deterministic.

Both sessions run on one host from one committed, clean-tree binary.

## Measures

1. Equality: per (workload, round), the three IC arms of a size must agree on
   the recovered logarithm, `total_gae`, every phase's counted operations,
   relations, trials and table entries. Checked by `scripts/analyze.jq`.
2. Oracle set-up wall time (`oracle_setup` phase `wall_ns`) per (workload,
   round) pair: ratio of totals Σ blocked / Σ batch and Σ blocked / Σ scalar per
   N, with the range of per-pair ratios; the A/A spread from `blocked-4096-aa`
   against `blocked-4096`.
3. Whole-run wall: `ecbench compare` of `blocked-N` against `batch-N` and
   `scalar-N` (its wall block and admission status), and the whole-run Σ
   ratio from the records.
4. Peak resident memory (`time.max_rss_kib`) per arm, descriptive.
5. Pricing band (AGENTS.md §5a): `ecbench band` at 0, 0.1 and 1 GAE per
   unpriced native operation, every arm including rho.
6. Gap to the matched rho per N (`S_IC / S_rho`, `ecbench compare` cluster
   intervals), as distance remaining.

## Predictions (pilot-informed)

Unrecorded pilots on this curve (one target, L0) informed these.

- P1. Every equality in measure 1 holds exactly: `S` is identical across
  the three oracles at every N, at every weight of the band.
- P2. blocked/batch oracle set-up wall ≤ 0.5 at N ≥ 4,096.
- P3. blocked/scalar oracle set-up wall ≤ 0.25 at N ≥ 4,096.
- P4. At N = 1,024 the gain is smaller: blocked/batch ≤ 1.0.
- P5. Peak memory of `blocked-N` is at most that of `batch-N` at N ≥ 4,096.
- P6. The gap to rho does not move (no counted change); the smallest gap is
  at N = 1,024.

## Decision rule

- A set-up wall saving is reported as measured where the IC runs are
  `admitted` and the per-pair ratios lie outside the A/A range; otherwise it is
  descriptive, with the isolation level stated.
- Any equality failure in measure 1 voids the comparison at that N and is
  reported as a defect of the builder.
- This is a component (phase) speed-up, not an advance (AGENTS.md §3), unless
  `S` moves, which P1 says it cannot. It goes to the leaderboard as its own
  component section, outside tables A–C.
- Failures, timeouts and unverified runs are reported, never dropped.

## Inadmissible

Changing the curve, targets, seeds, sizes, arms, unit or accounting after seeing
a recorded session; dropping a run; pooling sizes into a scaling claim; quoting
a set-up wall ratio without its isolation level.
