# Complete projective arithmetic for the Scholten genus-2 cohort

- **Status:** `open`
- **Kind:** proof/generalization gap; frozen cohort measurement is complete
- **Affected work:** [five-leads follow-up](../research/five_leads_e2e_20261019/STATUS.md), L5; [four follow-ups](../research/four_followups_20261021/REPORT.md)
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-22

## Problem

Two projective-law collector arms now have full, source-verified 1,260-target
operation-count comparisons. What remains open is a universal completeness
proof for the implemented weighted law on every divisor/input branch, beyond
the finite branch checks and successful frozen cohort; a claim that the law
is complete for arbitrary curves or divisors cannot be made from those runs.

## Evidence and uncertainty

- **Observed, 2026-10-19:** The [native model gate](../research/five_leads_e2e_20261019/correctness/genus2-model-gate-v1.jsonl) converts all 42 recorded models to monic quintics without an x^4 term, validates 4,200 transferred divisor samples and checks 4,200 generator-addition pairs commute with the coordinate map. The cited [complete group law](https://link.springer.com/article/10.1007/s13389-024-00351-7) covers that equation form in odd characteristic. That gate ran no projective additions.
- **Observed, 2026-10-22:** The [weighted-law follow-up](../research/projective_law_followup_20261022/REPORT.md)
  ran a normalized wrapper and a retained weighted-state collector on all
  1,260 original targets each. Every candidate and same-process source-rho
  scalar passed independent source multiplication. At the representative
  price, the pooled candidate/rho ratios are 3.793783 and 4.179297; the
  retained-state/wrapper ratio is 1.101617. The arithmetic gate checked
  24,192 weighted 1+2 cases, 26,880 sampled 2+2 pairs, 2,310 targeted
  exceptional 2+2 cases, 1,680 doubles, and 21,504 consecutive additions
  against independent Cantor arithmetic. The [native validator](../research/projective_law_followup_20261022/results/independent-validation.json)
  recomputed 576 arm/size/price cells and cold-phase sums.
- **Observed, 2026-10-22 proof audit:** A deterministic [42-model finite audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  compared 1,020,000 sampled 2+2 and 204,000 sampled 1+2 weighted
  additions with Cantor arithmetic. No mismatch or unresolved return was
  observed. The audit also records five algebraic branch-selection lemmas;
  output-formula identities remain unproved.
- **Derived or proved:** For a quintic with leading coefficient L and x^4 coefficient a4, `x=L*u-a4/(5L)` and `y=L^3*v` give a monic quintic without u^4 when L and 5 are invertible. All 42 finite fields meet these conditions in the executed gate.
- **Derived or proved, 2026-10-22 symbolic follow-up:** The
  [proof audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  now gives an exact integer-polynomial certificate and written divisor
  argument for the generic weighted-input 1+2 branch with `K≠0`. This
  establishes that branch's formula identity over the stated family,
  but not the remaining exceptional or 2+2 branches.
- **Derived or proved, 2026-10-22 exceptional follow-up:** The same
  [proof audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  now has exact integer-polynomial certificates and written divisor
  arguments for the `K=0` cancellation and coincident-positive
  nontripling 1+2 branches. Tripling and the 2+2 outputs remain open.
- **Suspected explanation:** None needed for the measured cost. Branch behavior
  outside the tested finite cases remains a proof obligation.
- **Missing capability:** A checked universal case proof for the implemented
  weighted law on the claimed model family. Hardware speed additionally
  requires isolated timing and the applicable external arithmetic comparator.
- **What this evidence does not establish:** Finite branch checks and successful
  cohort solves do not prove universal formula completeness or wall speed.

## What would unblock it

- [ ] Prove the implemented projective addition/doubling case split covers
  every divisor pair on the stated model family, and independently check the
  remaining exceptional branches on the exact 42 recorded models.
- [x] Replay the original 1,260 exact-order one-target workloads with unchanged identities/seeds and matched source rho; retain primitive/native counts and source-answer checks for both measured arms.
- [ ] If claiming wall speed, run matched L2/A/A timing and the fastest available external arithmetic comparator.

The first condition establishes correctness. The second establishes complete
operation-model cost. The third qualifies a hardware-speed claim.

## Next discriminating step

- **Action or proof obligation:** Formalize the actual implemented case split,
  including zero denominators and degree drops, and prove its outputs equal
  Cantor addition for every permitted divisor pair; use an exhaustive small
  model as a separate regression, not as the proof.
- **Inputs and controls:** Existing affine Jacobian, all converted quintics, origin and exceptional divisors, equal-cost source rho, unchanged target cohort.
- **Execution limits and stop condition:** Preserve the cohort cap, 12 arithmetic prices and native sensitivity band from the [frozen protocol](../research/five_leads_e2e_20261019/protocols/genus2-choice2-v1.md); stop promotion on any mismatched class or scalar.
- **Outcomes and next actions:** A valid proof would close universal
  completeness; a counterexample requires a corrected branch and renewed
  cohort validation. The existing finite-cohort cost result remains scoped to
  its recorded inputs unless a correction changes them.
- **Recheck trigger:** Complete projective arithmetic and matching rho source implementation.

## Resolution and history

2026-10-19: Formula-model compatibility checked; arithmetic and cohort replay unresolved.

2026-10-21: [The branch gate](../research/four_followups_20261021/results/complete-law-branches.jsonl)
now matches Cantor on 13,608 sampled 1+1, 12,096 sampled 1+2, and 6,551
sampled 2+2 cases across the 21 exact-order models. These include 1,175
targeted exceptional 2+2 cases, with a byte-identical second run. The
formulas are still a standalone arithmetic gate: exceptional branch coverage
is not exhaustive, the inversion-free law is not integrated into the
collector, and zero of the 1,260 required DLP solves used it. An affine
normalization with inversions is used only at the comparison boundary. The
original completion conditions remain open.

2026-10-22: The wrapper and retained-state collectors each solved and
source-verified all 1,260 original targets, with independent 36-cell pricing.
The measured-cohort and integration gaps are closed. Universal completeness
of the implemented law remains unproved, as stated in the
[follow-up report](../research/projective_law_followup_20261022/REPORT.md).

2026-10-22: All 42 mapped models received the additional finite audit.
Five branch-selection conditions were discharged
algebraically for valid reduced inputs in the linked proof audit. No
counterexample to the implemented law was found. The universal
output-formula proof remains open.

2026-10-22: The generic weighted-input 1+2 output received exact
symbolic polynomial and homogeneity certificates plus a written
divisor-class proof. Every other output branch remains open;
the universal-completeness status is unchanged.

2026-10-22: The `K=0` 1+2 cancellation and coincident-positive
nontripling paths received exact symbolic certificates and written
divisor-class arguments. Tripling and every 2+2 output identity remain
unproved, so universal completeness is still open.
