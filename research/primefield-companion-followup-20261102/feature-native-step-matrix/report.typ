#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Two-product half-GCD row updates
T-1833 · 2026-11-07 · Main resultant arithmetic

Half-GCD now specializes its Euclidean matrix action. Instead of constructing the step matrix and invoking general matrix multiplication, it moves the previous lower row and computes the new lower row with two polynomial products. Both Euclidean update sites use this path; genuinely general recursive compositions retain the previous implementation.

#image("flow.svg", width: 100%)

*Algebra.* Left multiplication by the matrix with rows (0,1) and (1,−q) maps rows (a,b),(c,d) to (c,d),(a−qc,b−qd). Exact matrix equality preserves transformed polynomial pairs, leading-factor records, parity, zero termination and resultant output through the recursion.

*Derived cost.* The old eight product calls contain two zero products, four products by one and two nontrivial products. The new path has two product calls and two subtractions. For canonical v, let L(v) be its vector length, or zero for the zero polynomial. Exactly L(a)+L(b)+L(c)+L(d) field products are removed per step. Negating one multiplication operand preserves this kernel's product counts. Internal matrices are unimodular, so the saving is at least two products per step. This does not change the asymptotic half-GCD bound.

*Ownership.* The c,d vector allocations move intact into the output row. New lower-row products and subtraction temporaries still allocate. General matrix composition, lifted applications, division and class powers remain charged. No total peak-memory or elapsed-time improvement is inferred.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Exact software check], [Count], [Coverage],
 [Local matrix updates], [288], [Dense equality; exact savings; buffer reuse],
 [Complete resultants], [432], [All exercise half-GCD; exact count delta],
 [Zero resultants], [47], [Common factors and cancellation],
 [Inherited singleton checks], [8,320], [Both orientations and operation bounds],
 [Source points / windows], [1,108 / 24], [Direct products, covariance, signs],
 [Additional recoveries], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 calls])

*Evidence and scope.* Normal build pins seven native sources with audit cfg disabled. Source, recovery and local matrix checks pass; predecessor is retained as the reference. Original 4,563-case cohort unchanged. No external benchmark, timing/cloud run or full Cargo rerun. PCC-20261107-a90b45 is unsigned producer evidence. DERIVATION.md and receipts specify assumptions, exact commands and hashes.

*Sources and next work.* Standard Euclidean matrix algebra and primary FLINT resultant/half-GCD documentation; KB unavailable. No novelty claim. Next examine low-remainder construction to avoid full quotient-times-divisor intermediates while preserving the quotient required by the row update. Historical reports and canonical graphs remain unchanged.
