#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Direct projected geometry residual
2026-11-07 | PCC-20261107-51c5f9

#image("flow.svg", width: 100%)

*Derived construction.* Even-power recurrence factors vanish in the order-two image. In the order-four image only their quadratic characters remain. The recurrence reduces modulo squares to next = difference times previous; a parity-weighted accumulator supplies the required character. Resultants still use full-field polynomial arithmetic. [pcc:unverified]

*Observed development checks.* Seven selected native methods pass. Of 2500 deterministic synthetic geometry cases, 1349 valid outputs agree with projection of the original homogeneous residual and 1151 are rejected by both; two malformed shape/order cases are also rejected. The paired source retains 499 subgroup rows, 100 splits, 1375 advances and 450 skips, with prior predicate/cache/closure checks. Each invocation filters 75 methods. Unsigned producer evidence. [pcc:unverified]

*Domain.* Every original zero-difference check remains. A zero resultant product is rejected. The canonical projected pair is compared homogeneously with the old result. Synthetic grids are algebra tests, not an expanded curve cohort. The full-value validator remains.

*Next.* Investigate direct image computation of the root resultant product; then complete-cost accounting. No scalar recovery, timing panel or cloud job ran; original cohort intact.

Full formula and sources: DERIVATION.md, original multiseed-factor residual and prior projection derivation. Evidence: receipt.json, source-lock.json, residual.log and other logs. KB unavailable. Shared polynomial code means this is not independent review; no novelty claim.
