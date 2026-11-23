# Finite nonbranch order construction — T-1613

[pcc:unverified] Source construction only, no compiler or experiment. Existing actual norm theorem is an ENat equality and does not itself establish nonzero germs. Prior local search examined weighted-global-class-reconciliation/NEXT.md, actual-nonbranch-norm-independent-review, current Nonbranch source, WeightedProjNonbranchDVR and mathlib addVal/ENat APIs. No later finite-order closure was found in scoped packet search.

Prior derivation: selected branch n(x0)=b*y0 and b,2,y0 nonzero imply affine n-b*y is nonzero by evaluating at the conjugate actual point (x0,-y0). If it vanished in the affine quotient, that evaluation would be2*b*y0=0. Separable actual affine domain then gives injective localization, and actual pointRingEquiv transfers nonvanishing. addVal_eq_top_iff supplies finite multiplicity; the existing norm-order equality forces each summand finite. ENat.toNat_add gives natural equality. No new norm identity or abstract valuation is introduced.

Attack joints: b=0 can make zero numerator; characteristic2 coalesces signs; y0=0 makes conjugate evaluation inconclusive; lack of domain can kill a nonzero element on localization; blindly applying toNat to top can erase infinity. Preserve these explicit hypotheses and separate the all-closed-points/global-divisor task.

Primary literature read before source construction: Stacks00PD for nonzero DVR unit-times-uniformizer form and02RJ for global rational-function orders; local pinned mathlib exact definitions control prospective Lean names. Producer here authors new source and cannot supply its own independent fidelity acceptance. Future separate reviewer and finite authorized compiler/readback required.
