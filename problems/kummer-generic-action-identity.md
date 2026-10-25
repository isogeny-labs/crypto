# Generic Kummer two-torsion action identity

- **Status:** `open`
- **Kind:** proof gap
- **Affected work:** [N16 action certificate](../research/kummer_differential_collector_20261022/n16-action-certificate/REPORT.md) and a complete fast Kummer law for the genus-2 collector
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-25

## Problem

The five recorded rational two-torsion matrices on each of 21 frozen
Rosenhain Jacobians have not been proved to act as translation on every
Jacobian class. This blocks an all-state action law and any claim that the
matrix-assisted fallback is complete on arbitrary collector states. It does
not invalidate the finite checked cohort or its reported costs.

## Evidence and uncertainty

- **Observed:** [N16 raw evidence](../research/kummer_differential_collector_20261022/n16-action-certificate/results/summary.json) checks 105 invertible involutions, 210 projective pair commutations, 21 five-action scalar products, and 27,470 native finite exceptional action comparisons without mismatch.
- **Derived or proved:** [The cancelled chart identity](../research/kummer_differential_collector_20261022/n16-action-certificate/CHART_NONVANISHING.md) proves nonvanishing for every degree-two polynomial input and every degree-one candidate input over each recorded field, using 21 nonzero conversion determinants. It also gives the exact generic translation rational map on `u(r)!=0` and the two Mumford ideal equations.
- **Missing:** An exact reduction of the six projective cross-products for `K(D+T_r)` and `M_r K(D)` modulo both Mumford equations, for each root and model. The exceptional `u(r)=0` branches and degree-one/zero geometric chart identities require separate proof.
- **What this evidence does not establish:** Matrix group relations and a finite sample do not establish the geometric action on untested divisors. The N15 bounded signed-lift result and N16 finite action replay do not provide a complete cold collector ratio for a new action-based implementation.

## What would unblock it

- [ ] Produce a reproducible exact polynomial-ideal certificate for all 105 model/root pairs on the `u(r)!=0` generic degree-two locus. Retain fixed constants, matrices, cleared denominators, Mumford generators, zero remainders, and an independent certificate checker.
- [ ] Prove the `u(r)=0` branches and degree-one/zero chart/action identities, including their zero-denominator cases, against Jacobian translation.
- [ ] Check that the resulting all-state law is used by a signed, relation-label-preserving collector before attributing any new end-to-end rho ratio to it.

The first two conditions establish the geometric action on the named model
family. They do not alone establish a lower collector cost or a security
claim.

## Next discriminating step

- **Action or proof obligation:** Symbolically reduce the six cleared generic cross-products for the branch `r=0` of the first frozen model modulo both Mumford equations. If all vanish, generalize the exact checker to all 105 model/root pairs; preserve a nonzero remainder as a counterexample candidate and independently evaluate it on valid divisors.
- **Inputs and controls:** Use the frozen theta constants, recorded matrices, Rosenhain quintics, and an independent Jacobian addition oracle. Check source and output charts without defining expected coordinates through the action matrix.
- **Execution limits and stop condition:** Start with the single exact model/root certificate; do not launch another external job or infer a theorem from bounded samples. Expand only after the exact reduction and checker pass.
- **Outcomes and next actions:** Zero remainders advance the generic branch proof; a verified nonzero remainder changes the proposed action/chart; an unfinished reduction remains a proof gap.
- **Recheck trigger:** A checked symbolic reduction or a proven alternative geometric construction.

## Resolution and history

2026-10-25: Exact chart nonvanishing established on all 21 recorded fields.
The generic and exceptional geometric action identities remain unresolved.
