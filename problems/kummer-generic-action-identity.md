# Generic Kummer two-torsion action identity

- **Status:** `partial` — explicit-chart action identities proved on all reduced-form branches; geometric chart interpretation and collector integration open
- **Kind:** proof gap
- **Affected work:** [N16 action certificate](../research/kummer_differential_collector_20261022/n16-action-certificate/REPORT.md) and a complete fast Kummer law for the genus-2 collector
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-25

## Problem

The five recorded rational two-torsion matrices now satisfy exact
projective action identities for the declared degree-two and degree-one
charts, including `u(r)=0` and `O`/torsion cases, on all valid reduced
classes of the 21 frozen Rosenhain models. A proof that these particular
charts give the intended geometric Kummer embedding, and complete collector
integration preserving signed relation labels, remain open. The exact
chart identity does not by itself supply a new end-to-end cost ratio.

## Evidence and uncertainty

- **Observed:** [N16 raw evidence](../research/kummer_differential_collector_20261022/n16-action-certificate/results/summary.json) checks 105 invertible involutions, 210 projective pair commutations, 21 five-action scalar products, and 27,470 native finite exceptional action comparisons without mismatch.
- **Derived or proved:** [The cancelled chart identity](../research/kummer_differential_collector_20261022/n16-action-certificate/CHART_NONVANISHING.md) proves nonvanishing for every degree-two polynomial input and every degree-one candidate input over each recorded field, using 21 nonzero conversion determinants. It also gives the exact generic translation rational map on `u(r)!=0` and the two Mumford ideal equations.
- **Exact generic result:** [The generic ideal packet](../research/kummer_differential_collector_20261022/n16-action-certificate/GENERIC_IDEAL.md) retains 630/630 zero-remainder six-cross-product certificates across 105 model/root pairs on `U=u(r)!=0`, with `U^6` clearing denominator, explicit ideal quotients, and a separate native checker. The degree-two chart matrix `C2` has its own exact nonzero determinant check.
- **Exact exceptional result and correction:** [The exceptional packet](../research/kummer_differential_collector_20261022/n16-action-certificate/EXCEPTIONAL_IDEAL.md) retains 630/630 zero univariate cross-products on `U=0`, 105 exact `O`/torsion checks, nonzero determinants for both the actual degree-one matrix `C1` and `C2`, and the involution derivation for degree-one classes. The earlier chart note incorrectly said `C1=C2`; the [failed wrong-basis diagnostic](../research/kummer_differential_collector_20261022/n16-action-certificate/results/exceptional-ideal-wrong-basis.json) preserves 105/105 mismatched pairs with that assumption.
- **Still missing:** A source-independent proof that both declared charts are the intended geometric Kummer projection, and a complete signed collector using this action with cold, matched-rho accounting. The finite N15/N16 cohorts remain finite checks and do not supply that collector ratio.

## What would unblock it

- [x] Produce a reproducible exact polynomial-ideal certificate for all 105 model/root pairs on the `u(r)!=0` generic degree-two locus. Retain fixed constants, matrices, cleared denominators, Mumford generators, zero remainders, and an independent certificate checker.
- [x] Prove the `u(r)=0` branch and degree-one/zero **declared-chart** action identities, including the zero-denominator cases, relative to reduced-form Jacobian translation and the recorded involutions.
- [ ] Establish the declared charts as the intended geometric Kummer projection independently of these action identities.
- [ ] Check that the resulting all-state law is used by a signed, relation-label-preserving collector before attributing any new end-to-end rho ratio to it.

The first two checked conditions establish the algebraic action identities
for the declared charts on the named model family. They do not establish
that those charts are the intended geometric Kummer embedding, a lower
collector cost, or a security claim.

## Next discriminating step

- **Action or proof obligation:** Give a source-independent geometric derivation of the degree-one `C1(0,1,x,x²)` and direct degree-two `C2(1,-a1,-a0,a0*a1+H)` charts from the Kummer embedding, then exercise an all-state signed collector with intact relation labels.
- **Inputs and controls:** Use the frozen theta constants, recorded matrices, Rosenhain quintics, and an independent Jacobian addition oracle. Check source and output charts without defining expected coordinates through the action matrix.
- **Execution limits and stop condition:** Preserve the recorded finite and exact packets; do not launch another external job or infer the missing chart theorem from bounded samples.
- **Outcomes and next actions:** A geometric chart derivation would close the embedding gap; a labeled collector cohort would determine its cold cost against matched rho.
- **Recheck trigger:** A checked chart derivation or a complete signed collector replay.

## Resolution and history

2026-10-25: Exact chart nonvanishing established on all 21 recorded fields.
The later generic and exceptional exact packets establish all reduced-form
projective identities for the declared charts and correct the false `C1=C2`
statement. Geometric chart interpretation and collector cost remain open.
