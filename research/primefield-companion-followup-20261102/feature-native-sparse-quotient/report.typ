#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Linear recurrence for sparse quotient bands
T-1839 · 2026-11-07 · Arbitrary quotient length

Quotient construction now recognizes a sparse relevant leading band of the divisor. For quotient length n, only the top n divisor coefficients affect the quotient. With at most one nonzero subleading coefficient in that band, a delayed recurrence replaces inverse-series construction for any n. Lower divisor coefficients still participate in the remainder.

#image("flow.svg", width: 100%)

*Identity.* Write the reversed divisor modulo X^n as alpha + beta X^j, with alpha nonzero. The reversed quotient satisfies Cᵢ=(Aᵢ−beta C(i−j))/alpha, using zero for negative indices. Induction gives B C=A modulo X^n and therefore the same quotient as Newton. An absent beta gives independent coefficient scaling.

*Exact cost.* Let u indicate alpha≠1. Skipping zero products gives M=u Z(C)+Zⱼ(C), where Z counts nonzero quotient coefficients and Zⱼ counts those in the first n−j reversed positions. Thus M≤u n+n−j≤2n−1, with u inversions. Without a subleading term, M≤u n. Scan costs at most min(deg(b),n−1) comparisons; output reversal is linear. The routine keeps one n-entry output buffer and constant scan state, while input/remainder storage remains separate.

*Fallback.* A second relevant nonzero coefficient stops inspection and uses the prior Newton arithmetic. Its added scan cost is explicitly charged. Length-one/two formulas and zero dividends retain their prior paths. No universal timing advantage or source-workload frequency is asserted.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Exact software check], [Count], [Coverage],
 [Sparse leading-band cases], [1,536], [Outputs; exact products; scan bounds],
 [Lower / equal / higher products], [1,536 / 0 / 0], [Fixed quotient-stage panel],
 [Dense fallback controls], [384], [Identical field counts; two inspections],
 [Lower-coefficient controls], [1,536], [Same quotient; changed remainder],
 [Source points / windows], [1,108 / 24], [Direct products, covariance, signs],
 [Additional recoveries], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 calls])

*Evidence and scope.* Retained short/wider quotient, division, matrix, resultant, singleton and tree controls pass. Normal build pins seven native inputs without audits. Original 4,563-case cohort unchanged. No cloud, timing benchmark, external comparison or full Cargo rerun. PCC-20261107-44be6f is unsigned producer evidence. DERIVATION.md and receipts contain assumptions, commands and hashes.

*Sources and next work.* Standard sparse series recurrence and FLINT binomial-inverse documentation, with exact predecessor code; KB unavailable. No novelty claim. Next trace support sizes and quotient lengths in the actual source/recovery path before choosing a larger support cap. Historical reports and canonical graphs remain unchanged.
