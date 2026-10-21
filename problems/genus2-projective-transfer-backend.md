# Complete projective arithmetic for the Scholten genus-2 cohort

- **Status:** `open`
- **Kind:** implementation and measurement gap
- **Affected work:** [five-leads follow-up](../research/five_leads_e2e_20261019/STATUS.md), L5; [four follow-ups](../research/four_followups_20261021/REPORT.md)
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-21

## Problem

The requested projective genus-2 comparison on all 1,260 original exact-order
targets cannot yet be made because the repository has no complete projective
divisor addition for the 21 Scholten models, with normalization and root
extraction integrated into the collector. The matched source rho also needs
an equally optimized coordinate arm.

## Evidence and uncertainty

- **Observed:** The [native model gate](../research/five_leads_e2e_20261019/correctness/genus2-model-gate-v1.jsonl) converts all 42 recorded models to monic quintics without an x^4 term, validates 4,200 transferred divisor samples and checks 4,200 generator-addition pairs commute with the coordinate map. The cited [complete group law](https://link.springer.com/article/10.1007/s13389-024-00351-7) covers that equation form in odd characteristic. No projective additions were run.
- **Derived or proved:** For a quintic with leading coefficient L and x^4 coefficient a4, `x=L*u-a4/(5L)` and `y=L^3*v` give a monic quintic without u^4 when L and 5 are invertible. All 42 finite fields meet these conditions in the executed gate.
- **Suspected explanation:** Unknown whether saved inversions exceed added multiplications, normalization, and source-rho gains; the cohort was not measured in projective coordinates.
- **Missing capability:** complete weighted-coordinate group law with degenerate cases, conversion at transfer/root/verification boundaries, independent arithmetic oracle and source-rho projective/batched control.
- **What this evidence does not establish:** Model applicability is not an arithmetic implementation or a speed result.

## What would unblock it

- [ ] Implement and verify all projective addition/doubling/exceptional cases against the existing affine Jacobian on the exact 42 models, including input/output conversion and every extracted root.
- [ ] Replay the original 1,260 exact-order one-target workloads with unchanged identities/seeds and matched source-rho optimization; retain all primitive/native counts and source-answer checks.
- [ ] If claiming wall speed, run matched L2/A/A timing and the fastest available external arithmetic comparator.

The first condition establishes correctness. The second establishes complete
operation-model cost. The third qualifies a hardware-speed claim.

## Next discriminating step

- **Action or proof obligation:** Port the complete case split from the published formulas, first checking mixed and degenerate divisor additions on all 42 converted models.
- **Inputs and controls:** Existing affine Jacobian, all converted quintics, origin and exceptional divisors, equal-cost source rho, unchanged target cohort.
- **Execution limits and stop condition:** Preserve the cohort cap, 12 arithmetic prices and native sensitivity band from the [frozen protocol](../research/five_leads_e2e_20261019/protocols/genus2-choice2-v1.md); stop promotion on any mismatched class or scalar.
- **Outcomes and next actions:** Passing formulas permit the full paired replay; an exception failure requires a corrected case; a slower complete result is retained as a measured loss.
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
