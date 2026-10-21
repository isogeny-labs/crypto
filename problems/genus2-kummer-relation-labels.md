# Sign-safe Kummer relation labels for a genus-2 collector

- **Status:** `open`
- **Kind:** implementation and proof gap, with a proved ordinary-addition obstruction
- **Affected work:** [four-follow-up protocol](../research/four_followups_20261021/PROTOCOL.md), Kummer walk arithmetic
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-21

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
- **Derived or proved:** Unsigned Kummer classes alone cannot define the
  binary ordinary addition used by this arbitrary-jump walk; the witness is
  exact for each transferred subgroup of order greater than four.
- **Suspected explanation:** A differential walk with a known difference or
  signed Jacobian anchors may allow fast Kummer pseudogroup steps, but its
  relation labels and total cost are unknown.
- **Missing capability:** Explicit Kummer coordinates and differential
  formulas for these models, a walk invariant that carries the required
  difference, and a checked lift of every collected relation.
- **What this evidence does not establish:** It does not rule out all Kummer
  collectors or yield a performance ratio to rho.

## What would unblock it

- [ ] Specify and prove a walk invariant mapping each Kummer state and known
  difference to a signed Jacobian relation label, including negation,
  exceptional points and cycle closure.
- [ ] Implement and cross-check the Kummer formulas against the Jacobian on
  all 21 models, then run and independently replay the 1,260 frozen targets
  with complete cold pricing and matched source rho.

The first condition makes the proposed collector mathematically usable; the
second measures it. The full rational two-torsion gate alone is insufficient.

## Next discriminating step

- **Action or proof obligation:** Attempt a differential step and signed
  label recovery on the first 16-bit model, with the known-difference invariant
  written explicitly before timing.
- **Inputs and controls:** Exact p=739 transferred generator and relation
  labels; affine Jacobian addition and the source-curve scalar oracle.
- **Execution limits and stop condition:** Check all 60 frozen targets on the
  first model only as a correctness gate; retain every exceptional failure.
  Full-cohort execution remains required after this gate.
- **Outcomes and next actions:** Correct labels allow a timed full collector;
  a counterexample identifies the invariant to repair. Failure to construct
  the invariant is an implementation/proof gap, not an impossibility theorem.
- **Recheck trigger:** Explicit sign-safe differential recurrence or a new
  complete Kummer addition construction.

## Resolution and history

2026-10-21: Rational two-torsion and unsigned ordinary-addition gates complete.
Differential walk, signed relation labels and the full cohort unresolved.
