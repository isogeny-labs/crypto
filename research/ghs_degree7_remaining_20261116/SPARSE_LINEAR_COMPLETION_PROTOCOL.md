# Trace +22 sparse and rational-square completion

Frozen 16 November 2026 UTC before counted runs. This is a parameter-only
adaptation of the already committed trace-`+26` degree-seven sparse and
linear-square sources in `../ghs_odd5_carrier_20261115/` to the fixed
trace-`+22` carrier and source-orbit representatives. The base field,
branch-factor pattern, and CRT method are unchanged; the carrier equation,
elliptic cubic coefficient, rational branch factor, source masks, and target
order are changed exactly as shown in the source diff.

`sparse_trace22.sage` covers `B=1`, `B=Q_i`, `B=Q_i³`, and `B=Q_iQ_j`.
The first three have unique CRT numerator candidates; the last has
`A=(Bh mod M)+λM` for every `λ∈K×`. Expected branch assignment counts are
243, 405, 405, and 270 respectively; the last checks 65,340 numerator
candidates.

`linear_square_trace22.sage` covers `B=(x-c)^m` with `m=2,4,6` and
`B=Q_i(x-c)^m` with `m=2,4`, for every nonzero `c∈K`.
It solves the exact coefficient equations `deg((Bh mod M))≤7` in `c` by
univariate polynomial gcd, including the all-zero fallback. Expected
logical assignments are 58,806 per first-group shape and 98,010 per
second-group shape. Every candidate receives coprimality and the exact
`f C²=B(A³+A²B+bB³)` square check.

Together with the prior quadratic-square screen and the three newly run
denominator classes, these arms cover all normalized `deg(B)≤6` even
denominators if the branch-parity classification in the forthcoming theorem
is correct. A zero-hit result remains conditional on the checked exact
source, complete receipt, and independent audit of that classification.
Only the fixed carrier and two source orbit representatives are in scope.

Sources checked: the published local trace-`+26` protocols and sources,
the fixed trace-`+22` carrier receipts, and Howe's [Theorem 4.17](https://arxiv.org/pdf/2110.04221).
