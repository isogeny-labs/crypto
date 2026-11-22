# Fresh F631 principal functions, normalization and local chart

The separately frozen normalization gate f832dd0203f4/checkpoint20261116T013358Z and local-chart admission 8b627c787346/checkpoint013724Z each executed once. Both status0/stderr empty; no scientific rerun or omitted input. Inputs are the fresh F631kernel serialization and certified public source, not F127functions, normalization, images or logarithms.

## Function construction and field-port control

All52nonzero kernel points have freshly constructed principal functions satisfying exact norm, divisibility and lifted-curve identities. All52corrupted functions are rejected. The F127Hensel constant64is not the inverse of2overF631; the correct value is316. The stale64formula fails all52tested curve lifts, while the corrected formula passes. This is an explicit negative field-port control, not an unexpected candidate failure.

Eight fixed split-point triples give432independent product/determinant-versus-norm evaluations, all equal, with no unsupported exclusions. Zero functions for the two identity points are treated separately from the52nonzero principal identities.

## Complete normalization checks

The first four source multiples1..4are regular; no sample rejection. All54normalization coefficients are constructed fresh without root extraction and serialized as ALPHA lines in RUN.txt. The same coefficients are recovered across all four samples.

| Exact check | Passing comparisons |
|---|---:|
| Cocycle constancy |4374|
| Cocycle symmetry |1458|
| Cocycle associativity |39366|
| Normality |5832|
| Symmetry values |216|
| Frobenius scalars |54|
| Theta translation actions |216|

All eight sampled theta values are nonzero and descend to F631. Corrupted weights produce152normality failures and208translation-action failures, validating those controls. These are finite exact checks, not complete geometry/implementation formalization or a runtime measurement.

## Rational 2-primary chart admission

The source order is4ell. First16degree-one places projected byell generate a subgroup of order4, certifying the complete rational2-primary group. Its2-torsion has two elements: zero and the divisor u=[436,451,7,1],v=0. The nonzero divisor has degree3and divides the source polynomial, admitting the current local quotient chart.

The first projected point has order4, so the later geometry must reduce it by one doubling before using order2local formulas. This corrects the old shortcut that assumed any nonzero ell projection already had order2. It does not change the source subgroup or target specification.

## Formal scalar identity and scope

RootRatio.lean checks the conditional root-free identity: if a2≠0 and a3²=a2³, then a=a3/a2 satisfies a²=a2 and a³=a3. This formalizes only scalar algebra, not the curve functions, cocycle geometry, sample completeness or code. LEAN.txt/EXIT-LEAN.txt retain the result in the pinned offline Lean4.19/mathlib environment.

This advances actual larger-field construction to normalized theta sections, not a quotient map or DLP recovery. Source kernel data, function identities and normalization are specific to the registered F631case. All construction and verification costs remain chargeable. No timing, exponent, novelty or tenfold gain is claimed. Full objective and genus2/4remain open; earlier failed charged cold-cost result unchanged.
