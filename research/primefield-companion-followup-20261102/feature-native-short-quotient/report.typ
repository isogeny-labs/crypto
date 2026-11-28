#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Direct constant and linear quotients
T-1837 · 2026-11-07 · Balanced Euclidean steps

The quotient kernel now reads the leading coefficients directly when the quotient has length one or two. General divrem and monic remainder-tree evaluation share the helper. Larger quotients and zero dividends keep Newton reversal. The same quotient then feeds the existing low remainder product and half-GCD row updates.

#image("flow.svg", width: 100%)

*Coefficient proof.* For equal degrees, q is the leading coefficient ratio. For a one-degree gap, q1=a_m/b_d and q0=(a_(m−1)−q1 b_(d−1))/b_d, taking a missing divisor coefficient as zero. These cancel the highest coefficients exactly. One nonmonic leading inverse is reused; monic scaling and zero cross terms are skipped.

*Exact quotient-only cost.* Let u flag a nonmonic divisor, h a nonzero subleading divisor coefficient, and ell a nonzero subleading dividend coefficient. The retained kernel and candidate have these field-product counts:

#table(columns: (1fr, 1.5fr, 1.5fr, 1fr), inset: 4pt,
 [Quotient length], [Prior Newton M], [Direct M], [Either I],
 [1], [1], [u], [u],
 [2], [3 + 3h + ell], [2u + h], [u])

Thus a monic constant quotient saves one product, a nonmonic constant quotient ties, and every nonzero linear quotient saves at least one product. Dispatch is a length/nonzero check, not a tuned benchmark threshold. Branches, reductions and allocation costs remain additional; no timing claim follows.

*Storage.* The direct path avoids reversed input copies, the inverse series and Newton/reversal temporaries. The output quotient, input normalization and low remainder storage remain. This is not a total-memory bound.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Exact software check], [Count], [Coverage],
 [Constant / linear quotients], [224 / 4,744], [Exhaustive small leading tuples; cost formulas],
 [Larger quotient controls], [640], [Same outputs and arithmetic counts],
 [Tree / zero-dividend controls], [256 / 8], [Evaluation and zero-path preservation],
 [General divisions / resultants], [2,592 / 432], [Reconstruction; complete half-GCD outputs],
 [Additional recoveries], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 calls])

*Evidence and scope.* Source 1,108-point/24-window checks and prior singleton, matrix, division-value, lifetime and preparation controls pass. Normal build pins seven native dependencies without audits. Original 4,563-case cohort unchanged; no cloud, timing benchmark, external comparison or full Cargo rerun. PCC-20261107-18740d is unsigned producer evidence. See DERIVATION.md and receipts for commands, assumptions and hashes.

*Sources and next work.* Primary FLINT division documentation and exact predecessor kernels; KB unavailable. Standard specialization, no novelty claim. Next derive a wider triangular-quotient cost envelope that charges coefficient inspection and temporary storage before extending dispatch.
