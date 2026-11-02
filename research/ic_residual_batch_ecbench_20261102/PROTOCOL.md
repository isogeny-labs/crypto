# Index-calculus structure in the batch vs multi-target rho, in ecbench (preregistered before data)

Date: 2026-11-02. Branch `claude/klp-ladder-20261102`. Follows the prototype parity result of `research/ic_klp_ladder_20261102` (Amendments 1, 4, 5).

## Methods (both registered in ecbench, `src/cryptanalysis/ecbench/methods.rs`)
- `rho.multitarget_pooled` (existing): Kuhn-Struik batch, the record's target plus L - 1 public companions (hashed to <G>, logs unknown), negation map, tuned walker, one shared distinguished-point table, jumps in G.
- `ic.residual_batch` (new, `ic_boundary::ic_residual_batch_with`): the same batch, companions, walker, table and charging, plus one public base point P0 of unknown log (hashed by the same law) in every relation, the residual collector's B = 1 column. First target: two independent equations (d_1 and log P0); later targets: one collision. log P0 and every d_i checked in the run.
Both report S = whole batch's GAE / sqrt(r).

## Design
Curve p36 (`p = 68719476731, a = -3, b = 33, r = 68719022539`). Arms: `rho` (rho.negation, reference, single target), `pool16`/`res16` (batch 16), `pool64`/`res64` (batch 64).
16 workloads (target seed 20261901), measure seed 20261902, 3 rounds + 1 warm-up, alternate_measured, local L0 (counts deterministic; no wall-clock claim; L2 replay owed before any benchmarks row).

## Claims and decision rule (frozen)
- P (parity, confirmatory): for L in {16, 64}, the paired ratio S(resL)/S(poolL) (`ecbench compare`, same workloads and seeds) has a 95% interval containing 1 and inside [0.9, 1.1] at w = 0, 0.1 and 1. Holds at L iff all three weights hold.
- D (descriptive): per-target cost S(batch)/L against single-target rho S, at w = 0, 0.1, 1.
- All records verified (the runner checks the record's target; companions and log P0 checked inside the run); audits OK; else void.
No threshold is tuned after data.

## Amendment 1 (2026-11-02, before any valid data): dispatch fix, rerun from the start
Attempt 1 (kept unmodified in `aborted-attempt1/`, 74 records) crashed on every `ic.residual_batch` record (panic "no entry found for key" at
the IC pipeline's parameter lookup): the new method's dispatch arm sat after the generic `ic` family arm, so it never ran. The rho and pooled
arms verified. Fix, code only: the arm moved ahead of the family arm, and the method's family set to `rho` (its solve is one rho-style batch phase,
so the rho online-window mapping describes it; the id and summary still name the index-calculus column). Design, seeds, claims and rule unchanged; rerun from the start.

## Amendment 2 (2026-11-02, before any valid data): start-point fix, rerun from the start
Attempt 2 (kept unmodified in `aborted-attempt2/`) ran but every `ic.residual_batch` record was exhausted with log P0 never solved:
with one stride, walk w's point was a multiple of [t_b]Q + P0, so every equation had the same Q : P0 coefficient ratio and the two-equation
solve was always singular. Fix, code only (`ic_residual_batch_with`): starts alternate between the precomputed `stride` and `stride + P0`
(still one addition a walk), so the P0 coefficient grows at half the rate of the Q coefficient and equations are independent.
Design, seeds, claims and rule unchanged; rerun from the start.
