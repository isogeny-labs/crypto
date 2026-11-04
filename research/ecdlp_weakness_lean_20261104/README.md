# Lean attacks on two weak points of the ECDLP hardness graph

Date: 2026-11-04. Targets from
[`docs/ecdlp-hardness-corpus/WEAKNESS_TRIAGE.md`](../../docs/ecdlp-hardness-corpus/WEAKNESS_TRIAGE.md)
W2 (relation rank) and W7 (xedni waiting time). Lean project:
[`lean/`](lean/) (Lean v4.34.1, Mathlib v4.34.1, same pins as
`research/jksst_xedni_audit_20261101/lean`).

## Results

| module | theorem | what it settles |
|---|---|---|
| `LeftKernel` | `left_kernel_identity`, `left_kernel_recovers` | Any left-kernel vector `λ` of the relation matrix gives `Σλa + x·Σλb = 0`, so `x = −Σλa/Σλb` when `Σλb ≠ 0`. |
| `LeftKernel` | `exists_left_kernel` | With more relations than factor-base columns a nonzero `λ` exists for **every** matrix: no rank hypothesis. |
| `LeftKernel` | `card_dot_ker_mul`, `card_bad_pairs` | If `λ` is any nonzero-valued function of the relation points only (their logs `a + b·x`), exactly `q^(2n−1)` of the `q^(2n)` hidden pairs `(a, b)` give `Σλb = 0`: failure density exactly `1/q`. |
| `HazardSeparation` | `survival_ge_of_marginals` | Union bound: if each trial `t ≤ N` succeeds with marginal mass `≤ ε` (correlated or adaptive trials allowed), survival after `n` trials is `≥ 1 − nε`. |
| `HazardSeparation` | `truncated_expectation_ge_half_horizon_of_marginals` | With `Nε ≤ 1/2`, `E[min τ N] ≥ N/2`, the conclusion of `JKSSTExpectedTrials.truncated_expectation_ge_half_horizon` **without** its conditional-hazard recurrence. |
| `HazardSeparation` | `marginals_U`, `marginals_do_not_give_hazard`, `wait_U` | Separation: three equally likely outcomes, outcome `k` succeeding at trial `k+1`. Every trial's marginal is `≤ 1/3`, the recurrence `(1−1/3)·s(n) ≤ s(n+1)` fails at `n = 1` (`4/9 > 1/3`), and the wait is still `E[min τ 3] = 2`. |

## What changes in the corpus

- **`HEUR-RELATION-RANK` (W2).** The left-kernel method needs no rank
  condition. The finite core is now kernel-checked: existence of `λ` for any
  matrix, recovery, and the exact `1/q` failure count with `λ` depending only on
  the relation points. The heuristic stays load-bearing only where relations
  are reused across targets (right-kernel amortisation, multi-target batches,
  PKM's Δ surplus). Not formalised here: the group-theoretic step "a
  decomposition of `R_i` gives the row `M_i`" (by definition of the factor-base
  logs) and the reduction from a uniform `(a_i, b_i)` to the counting statement
  (the counting theorem is the uniform-measure statement itself).
- **`ASM-XEDNI-HAZARD-RECURRENCE` (W7).** The linear waiting time behind
  `OBSTRUCTION-XEDNI-HEIGHT` follows from per-trial marginal bounds. A
  history-uniform hazard bound is not needed, and marginal bounds do not imply
  it. The open xedni bridge is therefore a **per-trial marginal bound for the
  adaptive trial actually run**, a weaker and more natural premise. This
  strengthens the obstruction; it does not reopen xedni.

Both results are recorded in
[`docs/ecdlp-hardness-corpus/lean-weakness-20261104.json`](../../docs/ecdlp-hardness-corpus/lean-weakness-20261104.json).

## Verification

```sh
cd research/ecdlp_weakness_lean_20261104/lean
lake exe cache get && lake build      # what CI's lean-proof-gate runs
lake env lean AxiomAudit.lean         # #print axioms for every theorem above
tools/lean/proof-gate.sh research/ecdlp_weakness_lean_20261104/lean   # from the repo root
```

The local check compiled each module with `lake env lean -o` against the
Mathlib v4.34.1 cache. A full local `lake build` was not possible because the
session's disk allowance could not hold Mathlib's rebuild outputs. CI's
`lean-proof-gate` runs the full build and the transitive axiom audit. No `sorry`,
`native_decide` or user `axiom` occurs in either module.
