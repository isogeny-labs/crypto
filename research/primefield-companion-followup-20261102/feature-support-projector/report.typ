#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Bounded projector extraction
T-1933 · Prime fields · 2026-11-08

The optional extractor obtains candidate images from the implicit three-mode constraints, without a supplied Y or runtime candidate scan. It constructs a support indicator, reads trace moments and applies Newton identities.

#image("flow.svg",width:100%)

*Derived construction.* In the quotient by Y^L−epsilon, the product of 1−Eᵢ^(r−1) is1 at common roots and0 elsewhere. If its coefficients are cₑ, the survivor count is Lc₀ and the j-th moment is L epsilon c(L−j), with the stated endpoint convention. Newton identities reconstruct the exact candidate polynomial. A singleton uses the existing rational scalar chart and an independent public-point check.

*Recorded qualification.* All528 subset projectors and22 dense singleton controls pass. All144 transcript annihilators match the direct oracle, with1,176 phase memberships checked. Maxima for those144 cases are67,512 entered monomial pairs,15,558 retained node-coefficient slots and20 temporary terms. These are separate dimensions, not total work or peak RAM.

*Public recovery.* On29 fixed public targets,18 recover from extracted singleton images and11 from verified identity/endpoint exits. None is unresolved or capped. No fixture Y is passed to the adapter. Nominal recovery uses332 API queries and689,538 projector pairs;27 passing call guards add27 queries (359 including guards). Source setup/calibration and other operations remain charged. PCC-20261108-80d4d8 is unsigned producer evidence.

*Preserved large refusal.* Fixture observations at prime998244353, m7 and L71,303,168 hit the64-term cap after14,621 entered pairs and5,724 retained slots. This is not a large public-point recovery. The refusal and four separate cap controls are retained; no dense fallback runs.

*Representation bound.* Multiplication by the indicator has rank d, the number of survivors. Its cyclic-shift columns each have s nonzero coefficients. A basis of d columns must collectively cover all L coordinates, hence sd≥L. In particular singleton support requires all L coefficients. This bounds the explicit indicator, not every implicit extractor.

*Next step and costs.* Obtain selected trace moments without materializing the indicator. Low-degree truncation alone is invalid: high exponents wrap into low coefficients. Charge source operations, point shifts, map lookups, copies, allocations, coefficient updates, Newton inversions and verification, in addition to recorded monomial pairs.

*Custody and sources.* Fresh overflow-checked build/check pass within90/60 seconds; ten copied files and15 dependencies are hash-bound. Primary source: Chen-Fan-Luo, arXiv2211.11204v3, finite-field support/rank discussion. DERIVATION.md gives the direct quotient proof. KB unavailable; no novelty/timing claim or cloud submission. Independent signed review pending.
