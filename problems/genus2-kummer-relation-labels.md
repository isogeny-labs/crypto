# Sign-safe Kummer relation labels for a genus-2 collector

- **Status:** `partly resolved`; fixed-difference collector measured and fixed-offset orbit bound formalized, arbitrary-sidecar construction open
- **Kind:** restricted implementation and proof gap, with a proved ordinary-addition obstruction
- **Affected work:** [four-follow-up protocol](../research/four_followups_20261021/PROTOCOL.md), Kummer walk arithmetic
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-24

## Problem

A fixed-difference differential Kummer walk now preserves checked signed
relation labels through a full 1,260-target genus-2 collector. The original
arbitrary-jump collector still lacks a constant-size signed sidecar update;
its distinct recurrence and cost claim remain open.

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
- **Observed:** [The matched full collector](../research/kummer_differential_collector_20261022/MATCHED_REPORT.md)
  solved and source-verified all 1,260 frozen targets and ran 1,260 fresh
  source rho controls in the same native process. Its independent replay and
  validation gate passed. Across 36 full-cold price cells its candidate/rho
  ratio was 8.959687–55.935424, with zero cells and zero of 45,360
  target/cell comparisons below rho. At `S/M=0.5`, `I/M=10`, native charge
  `0.1`, the ratio was 14.356662. The exact rows, costs, graph and PDF are
  in that study; the [cross-lead packet](../research/four_followups_review_20261022/REPORT.md)
  provides a common schema.
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
- **Formalized restricted obstruction:** The
  [fixed-offset orbit packet](../research/kummer_differential_collector_20261022/orbit-proof/REPORT.md)
  proves the closure-to-orbit step in Lean and checks 336 frozen cyclic
  witnesses (21 prime-order groups by 16 nonzero jumps). Every exact orbit has
  size `r`; the explicit current `[u64;4]` state bound ranges from 1,092,704 to
  17,668,176,138,464 bytes. These are bounds for the stated explicit
  one-xADD-per-entry model, not for compressed or different sidecars.
- **Remaining gap:** The fixed-difference implementation supplies explicit
  coordinates, checked differential formulas, signed recovery and a full
  priced collector. It does not supply the sidecar recurrence for the frozen
  arbitrary-jump walk or a cost measurement for that specific walk.
- **What this evidence does not establish:** The measured loss of this
  fixed-difference collector does not bound every Kummer collector.

## What would unblock it

- [x] Prove and reference-check local differential quotient and signed-anchor
  recovery, including negation, identity and two-torsion cases.
- [ ] Specify and prove an efficiently updatable walk state with the needed
  difference/anchor sidecar under changing arbitrary jumps and cycle closure.
- [x] Mechanize the fixed-offset closure theorem and instantiate all 336
  frozen prime-order group/nonzero-jump orbit witnesses.
- [x] Implement and cross-check Kummer formulas against the Jacobian on all
  21 models, then run and independently replay 1,260 frozen targets with
  complete cold pricing and matched source rho for a fixed-difference walk.

The completed measurement belongs to the fixed-difference walk. A separate
arbitrary-jump measurement requires the still-open recurrence.

## Next discriminating step

- **Action or proof obligation:** Derive a finite sidecar update rule for
  `K(T+G)` and each `K(T-J_i)` under the two prospective arbitrary jumps of
  the frozen collector, or substitute a fully specified differential walk
  whose candidate and relation distribution is measured against the control.
- **Coordinate basis:** Reuse the checked theta projection, xDBL/xADD and
  signed Recover implementation from the fixed-difference collector. Check
  any coordinate or vanishing-denominator branches newly reached by the
  arbitrary-jump sidecar against the Jacobian before pricing that walk.
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

2026-10-22 matched collector: Explicit fixed-difference arithmetic and signed
recovery passed; all 1,260 candidate and same-process rho controls were
source-verified and independently replayed. Full-cold candidate/rho ratios
were 8.959687–55.935424 across 36 cells. The arbitrary-jump sidecar remains
unimplemented.

2026-10-24 fixed-offset theorem: Lean checked the conditional closure-to-orbit
theorem, and the native validator checked 336 exact cyclic instances with zero
failures. This closes the proof obligation for the direct explicit
fixed-offset state model. It does not establish an arbitrary-sidecar
impossibility or provide the still-missing recurrence.
