#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= General dyadic quotient-class source
T-1824 · 2026-11-07 · derived algebra and native producer checks

The public sign source now carries multiplicative classes for arbitrary dyadic q, including numerator/denominator pairs throughout its seed and block recurrence. It retains the prior projected source as a full-value reference. Prime p and odd prime subgroup order r are caller-certified; gcd(r,p−1)=1, r≠p, word-size and geometry-lane bounds remain explicit.

#image("flow.svg", width: 100%)

*Derived invariant.* The quotient Fp*/(Fp*)^q has exponent q. Nonzero powers reduce modulo q, and a class inverse is its (q−1)-st power. Zero powers are handled before reducing exponents. The resultant accumulator uses this quotient while all Euclidean remainders and their actual degrees remain in Fp. For beta=W(r+1)²W(2)/W(r+2), e=(r²)⁻¹ mod q, the seed pair is (W(r+2)^e, (W(r+1)²W(2))^e). Multiplicative recurrence steps commute with projection. Calibration compares raw output to the base value or its negative.

#table(columns: (2fr, 1fr, 3fr), inset: 5pt,
 [Exact check], [Count], [Evidence],
 [Class multiplication pairs], [114,751], [validation-02/check.stdout],
 [Projected root resultants], [1,792], [Seven prime fields, degrees 1–16],
 [Source points], [1,108], [Full reference, direct products, covariance, signs],
 [Identity-crossing windows], [24], [Prepared window checks],
 [New-domain recovery targets], [617], [5,929 source calls; zero unresolved],
 [Inherited public comparisons], [63], [511 source calls])

*Controls.* Projection orders include 8,16,32,64,256. Large exponents, zero inversion and cross-field combination are checked. The wrong exponent e=1 changes 266 point-function values. The initial passing source and generator are preserved under validation-01; validation-02 follows removal of obsolete copied test includes. Final native source and recovery builds pass; warnings remain in the receipts. A normal metadata build has five dependencies and excludes derivative jets and prior source implementations.

*Costs.* Class powers/inverse cost O(log q) field multiplications; final projection costs O(log((p−1)/q)). Seed formation, seed inversion, final source division and calibration no longer require their full-field inverses. Geometry, division-polynomial cache and remainder arithmetic still have inversion costs. Classes store field representatives, not short logarithm encodings. Square-root geometry remains. No timing comparison or cloud launch; original 4,563-case cohort unchanged.

*Evidence and next step.* PCC-20261107-8013b5 is an unsigned producer statement, not an independent signed verdict. See DERIVATION.md and receipts for assumptions, commands, hashes and replay. Prior Ward normalization uses Lauter–Stange2008; FLINT half-GCD documentation informed field-valued remainder handling (generated-prose notice recorded). KB unavailable. Next account total field operations against the full representative route before selecting a runtime backend.
