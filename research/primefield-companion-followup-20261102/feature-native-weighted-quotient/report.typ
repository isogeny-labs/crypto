#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Linear quotient work from weighted support
T-1841 · 2026-11-07 · Native prime-field polynomial kernel

The quotient dispatcher now accepts several subleading divisor terms when their total contribution count fits a linear work certificate. It strictly contains the preceding one-term class. This broadens the usable structure without assuming that every sparse polynomial is cheap or that the source workload frequently has this structure.

#image("flow.svg",width:100%)

*Derived identity.* With reversed dividend A and divisor B, leading alpha nonzero and relevant nonzero lags J, coefficient division gives Cᵢ = (Aᵢ − sum of Bⱼ C(i−j))/alpha. Induction establishes B C = A modulo X^n. Reversing C returns the exact quotient; lower original divisor coefficients still enter the remainder.

*Work certificate.* Accept W = sum over J of (n−j) ≤ n−1. Skip zero products and monic scaling. Exact field products are u Z(C) + sum of Zⱼ(C), at most u n + W ≤ 2n−1, with u inversions; u indicates a nonmonic divisor. Traverse sorted support only through the first future lag: exactly W productive visits, at most W+n total visits. Scan and output reversal are linear. Support size s satisfies s(s+1)/2 ≤ n−1, giving O(sqrt(n)) pairs plus the n-entry output. Input, remainder and recursive storage remain additional.

#table(columns:(2fr,1fr,2fr),inset:4pt,
 [Deterministic software check],[Cases],[Result],
 [Accepted weighted bands],[2,080],[Exact products, visits and quotient],
 [Multiple-term accepted bands],[896],[Up to 15 subleading terms],
 [Exact work-cap boundary],[640],[Accepted at W = n−1],
 [Rejected bands],[4,192],[Same Newton field counts],
 [Inherited source checks],[1,108 points],[24 windows; all pass],
 [Additional recoveries],[617],[5,929 calls; zero unresolved],
 [Inherited public comparisons],[63],[511 calls; all pass])

*Scope and controls.* Exhaustive support masks for lengths up to eight accompany named and longer controls. Counts include deliberate overlap. Single-term, dense, short-quotient, remainder, matrix, resultant and singleton controls remain passing. Normal build has seven input pins and excludes audits. This is software validation, not elapsed-time evidence. Original 4,563-case cohort unchanged; no cloud or full Cargo rerun. PCC-20261107-98493f is unsigned producer evidence.

*Sources and next step.* Standard coefficient division recurrence: FLINT nmod_poly series-division documentation; exact predecessor source. KB unavailable. DERIVATION.md proves the bound and records assumptions. Next trace support lengths and all source-phase costs in actual public-point recovery before claiming an aggregate reduction. Historical evidence and canonical research graphs are preserved.
