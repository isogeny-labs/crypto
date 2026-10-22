# Sign-safe Kummer relation labels for a genus-2 collector

- **Status:** `open`
- **Kind:** implementation and proof gap, with a proved ordinary-addition obstruction
- **Affected work:** [four-follow-up protocol](../research/four_followups_20261021/PROTOCOL.md), Kummer walk arithmetic
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-22

## Problem

No differential Kummer walk with a checked sign/lift recovery rule currently
preserves the signed relation labels required by the arbitrary-jump genus-2
collector. The requested completion is a 1,260-target source-verified full
collector priced against matched rho, not merely fast scalar multiplication.

## Evidence and uncertainty

- **Observed:** [The model gate](../research/four_followups_20261021/results/kummer-model-gate.jsonl)
  exhaustively found and reconstructed five distinct rational roots of each
  exact quintic on 21/21 models, so the rational two-torsion field-of-definition
  condition for Gaudry's fast model is met. [The ambiguity gate](../research/four_followups_20261021/results/kummer-ambiguity.jsonl)
  checked 21/21 exact-order groups: the same unsigned input K(2P)=K(-2P)
  yields different K(P+2P) and K(P-2P).
- **Observed:** [The sign-safe reference gate](../research/kummer_walk_20261022/REPORT.md)
  checked all 1,260 frozen source targets, 8,820 labelled differential
  quotient cases and 630 exceptional/identity/two-torsion cases on 21 models.
  Every test had one quotient output and one signed lift, with source labels
  checked; the implementation uses Jacobian addition to enumerate possible
  quotient lifts, not fast Kummer coordinates.
- **Observed:** [The separate Rosenhain setup gate](../research/kummer_walk_20261022/ROSENHAIN_GATE.md)
  found and exactly checked an `Fp` Rosenhain normalization on all 21
  models. Each had 10 square-scale choices among 20 ordered root pairs.
  This establishes the normalized equation, not theta constants or Kummer
  projection coordinates.
- **Derived or proved:** Unsigned Kummer classes alone cannot define the
  binary ordinary addition used by this arbitrary-jump walk; the witness is
  exact for each transferred subgroup of order greater than four.
- **Derived or proved:** `K(A), K(B), K(A-B)` uniquely fix `K(A+B)`;
  `K(T), K(T+G)` and a signed `G` of order greater than two uniquely fix the
  signed `T`. These identities prove the local differential/recovery rule,
  not a closed state update under changing arbitrary jumps.
- **Restricted obstruction:** For a state consisting only of fixed-offset
  classes `{K(T+s):s in S}`, updating each class after a jump `J` by one xADD
  with `K(J)` and a stored difference requires `S-J` to be contained in `S`.
  If `J` has order `m`, every nonempty such `S` contains at least `m` offsets.
  This rules out a constant-size sidecar of that exact form for a large-order
  jump; it does not rule out other recurrences or walks.
- **Suspected explanation:** A differential walk with a known difference or
  signed Jacobian anchors may allow fast Kummer pseudogroup steps, but its
  relation labels and total cost are unknown.
- **Missing capability:** Explicit Kummer coordinates and differential
  formulas for these models, an efficient walk invariant that updates the
  needed difference/anchor sidecar as the arbitrary jump changes, and a
  checked lift of every collected relation in a full collector. The local
  signed-lift rule is reference-checked, but the required sidecar recurrence
  is not implemented or proved efficient.
- **What this evidence does not establish:** It does not rule out all Kummer
  collectors or yield a performance ratio to rho.

## What would unblock it

- [x] Prove and reference-check local differential quotient and signed-anchor
  recovery, including negation, identity and two-torsion cases.
- [ ] Specify and prove an efficiently updatable walk state with the needed
  difference/anchor sidecar under changing arbitrary jumps and cycle closure.
- [ ] Implement and cross-check the Kummer formulas against the Jacobian on
  all 21 models, then run and independently replay the 1,260 frozen targets
  with complete cold pricing and matched source rho.

The first condition makes the proposed collector mathematically usable; the
second measures it. The full rational two-torsion gate alone is insufficient.

## Next discriminating step

- **Action or proof obligation:** Derive a finite sidecar update rule for
  `K(T+G)` and each `K(T-J_i)` under the two prospective arbitrary jumps of
  the frozen collector, or substitute a fully specified differential walk
  whose candidate and relation distribution is measured against the control.
- **Coordinate prerequisite:** Starting from the checked p=739 Rosenhain
  normalization, derive theta constants and projection coordinates, then check
  Gaudry's xDBL/xADD and the signed recovery of Chung--Costello--Smith
  against the Jacobian, including all vanishing-denominator branches. The
  rational-root and Rosenhain gates do not establish this projection.
- **Inputs and controls:** All 21 exact models; the signed Jacobian and
  source-curve oracles; the frozen 16-jump collector and its 1,260 target
  workload. Verify the sidecar after every jump, restart and cycle closure.
- **Execution limits and stop condition:** Gate every exceptional branch on
  the 16-bit model first, then run the full 1,260 paired collector if the
  recurrence and coordinate formulas pass. Preserve failures and unmeasured
  cost cells.
- **Outcomes and next actions:** A closed, verified recurrence permits a
  priced run; a counterexample requires a different state or walk. A missing
  recurrence is an implementation/proof gap, not a global impossibility.
- **Recheck trigger:** Explicit sign-safe differential recurrence or a new
  complete Kummer addition construction.

## Resolution and history

2026-10-21: Rational two-torsion and unsigned ordinary-addition gates complete.
Differential walk, signed relation labels and the full cohort unresolved.

2026-10-22: The local differential quotient and signed-anchor recovery gate
passed 1,260 target and 630 exceptional cases. Fast coordinates, a closed
arbitrary-jump state recurrence, and all full-collector cost ratios remain
unresolved.

2026-10-22 addendum: Exact `Fp` Rosenhain normalization passed on 21/21
models. Theta coordinates, fast Kummer operations, signed jump recurrence,
and full-collector costs remain unresolved.
