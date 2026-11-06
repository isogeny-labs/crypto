# ML-aided SAT for the PDP: ceiling test before any model is built

Status: **preregistered, frozen before data** (2026-10-27). Toy stage
diagnostic. Class if it fires: engineering lead for the oracle constant.
No leaderboard or `benchmarks/` row can come from this run (local
development host, not an admitted Iso Lab measurement).

## Question

The SAT/SMT survey (`research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md`
§6) closed "ML-guided SAT" without a round. Its reason (no ECC benchmarks,
size generalisation) is about published models, not about whether the PDP
search *carries a learnable signal at all*. This run measures the ceiling
that any ML component can reach, per lever, with simple learners standing in
for the model and the exact labels standing in for a perfect one. If the
ceiling is ≈ 1×, no GNN, RL branching policy or learned restart schedule can
help, and the lead closes with a measurement instead of an opinion.

Three levers, each a place a learned model could sit in a SAT-based PDP oracle:

- **L1, instance triage (predict UNSAT, skip).** Most targets do not
  decompose and refuting them is the expensive part. A model scoring the
  target before (or early in) the solve can skip some. Features, each group
  fitted separately so the signal source is isolated:
  - `F-x`: the `n` bits of `x_r` (linear model);
  - `F-xx`: `F-x` plus all pairwise products (quadratic model in `x_r`);
  - `F-tel(k)`: solver telemetry after `k ∈ {64, 512}` conflicts
    (decisions, propagations, XOR propagations, XOR conflicts, restarts,
    learnt clauses, mean kept learnt size, mean conflict level, max level,
    root-fixed variables), normalised per conflict.
- **L2, cutoff/restart policy.** Stop each solve after `c` conflicts. The
  best fixed `c` is chosen on the training split from the empirical
  distributions, no ML needed; it is the baseline an ML stopping rule must beat.
- **L3, branching order.** The solver's `set_branch_priority` stands in for a
  learned static order: (a0) default VSIDS; (a1) all `3ℓ` x-bits first;
  (a2) `X₁, X₂` bits first (`2ℓ`, the pairs-then-propagate order the best
  enumerator uses); (a3) e-variables first; (a4) "learned": the top `2ℓ`
  variables by mean final VSIDS activity on training refutations.

## Instrument and population

- Native solver `crypto_lib::cryptanalysis::sat` (CDCL + Gauss–Jordan XOR),
  symmetrised `S₄` encoding with native XOR and `X₁ ≤ X₂ ≤ X₃` symmetry
  breaking (`encode_semaev_s4_with`, defaults), `m = 3`.
- Families (curve, field, subspace exactly as the frozen Trimoska corpus):
  `n15l5`, `n17l6`. Targets: `x_r` uniform nonzero in `F_{2^n}`, SplitMix64
  from seed `20261027 ⊕ family-id`. N = 3000 (`n15l5`), 1200 (`n17l6`).
- Labels: complete SAT solve; **every** label cross-checked by exhaustive
  sorted-triple search (`decide_exhaustively`); any disagreement voids the run.
- Split: deterministic by index, even = train, odd = test. Second A/A split:
  index mod 4 ∈ {0,3} train.
- Cost unit: solver conflicts (machine-independent); `word_ops` and wall
  time are reported alongside, not substituted.

## Controls

- Negative: each L1 model refit on train with labels permuted (seeded);
  test AUC must be within the bootstrap noise of 0.5.
- Positive: a synthetic feature equal to the label flipped with probability
  0.25 appended to `F-x`; the learner must reach test AUC ≥ 0.70 or the
  learner is broken and L1 is void.
- A/A: the second split; reported AUCs must agree within their CIs.
- Size generalisation: `F-tel(k)` model trained on `n15l5`, scored on `n17l6`.

## Metrics and decision rules (frozen)

- L1: test AUC with 1000-sample bootstrap 95 % CI. Policy "continue only if
  score ≥ τ" with `τ` fitted on train; **gain** = (test SAT found / test
  conflicts charged, including the `k` telemetry conflicts on every target)
  ÷ (same for solve-everything). Ceiling: same with exact labels at `k`.
- L2: gain of the train-optimal fixed cutoff on test, same ratio.
- L3: median and summed conflicts per UNSAT refutation on 60 (`n15l5`) and
  30 (`n17l6`) test-split UNSAT targets per arm; ratio to a0; conflicts per
  sorted triple and per pair `2^{2ℓ}`.
- **Fires (lead opens)** if any lever's held-out gain ≥ 1.5× with the
  bootstrap CI lower bound > 1.2×, on both families.
- **Closes** if every lever's held-out gain CI includes ≤ 1.2× and the L1
  models' AUC CIs include ≤ 0.6.
- Anything between is "inconclusive" and names the next measurement.

## Predictions (written before data)

- P1: `F-x`, `F-xx` AUC in [0.45, 0.60] — decomposability of a uniform
  target is a pseudo-random function of `x_r` at this scale.
- P2: `F-tel(k)` AUC ≤ 0.65 for `k ≪` median refutation conflicts.
- P3: L2 gain ≤ 1.3× (a complete solver here behaves as an enumerator, so
  the time-to-witness is close to uniform over the refutation length).
- P4: every L3 arm within 0.7–1.4× of a0 in summed refutation conflicts,
  none below 0.5 conflicts per sorted triple.

## What this cannot show

Nothing about `n = 131`, ECC2K-130 cost, or rho. A gain here is a constant
on a toy oracle that the product law (`relations × targets × oracle ≈ m·2^n`)
caps; it is not an index-calculus improvement until carried into an
end-to-end solve with failed targets and rho charged.

## Amendment 1 (2026-10-27, before any L1/L2/L3 result was seen)

- The first full run was stopped by the session's 2-hour background limit
  during `n17l6` L3; its L1/L2/L3 results lived only in memory and were never
  written or read. Its per-target rows (`run1/rows_*.jsonl`, labels
  cross-checked by exhaustive search on every target) are kept and the
  analysis is re-run from them (`--from-rows`, which regenerates the seeded
  target sequence and asserts it matches). The rows were produced after the
  solver fix in commit "Fix unsound SAT resume after a conflict-budget stop".
- L3 solves are capped at 10× the family's median default-order refutation
  conflicts. A capped solve counts at the cap, so a capped arm's sums are
  lower bounds and its ratio to a0 is a lower bound; the capped count is reported.
- Each stage's output is written as soon as it exists.
