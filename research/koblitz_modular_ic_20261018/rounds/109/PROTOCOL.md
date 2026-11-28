# R109: actual N51 valuation audit on the four x=1 branches

Bind the exact R51 donor, R102 N51 parser, R107 lambda edge data, and
R108 order-128 base series by SHA-256. Parse every one of the 105
rational N51 coefficient terms. Substitute each base series in its
numerator, denominator, and y monomial; invert the denominator unit
after removing its s-order, keeping an explicit known-precision bound.
Translate `l=alpha+u` over F4 and combine all terms at each u power.

For the degree-7 lambda branches use weights `(v(s),v(u))=(7,23)`;
the leading u unit is 1. For the degree-1 branches use `(1,5)` at
y=0 and `(1,4)` at y=1, with leading u unit alpha from R107.
Report every combined coefficient's exact order or lower bound, all
weighted minima and leading F4 sums. Claim a function valuation only
when the minimum is below every precision bound and its summed leading
scalar is nonzero. Otherwise record the exact unresolved cancellation.
Frobenius conjugates the alpha branch to alpha+1.

Fresh replay must be byte-identical; changed donor hash must fail.
Deterministic exact source-bound F2/F4 arithmetic, no cloud resources.
Commit source/protocol before data, retain failed attempts. The result
concerns the actual function `n=-H=H` in characteristic two, not the
coefficientwise clearing bound alone.

Sources: R51 donor, R102 N51 coefficient audit, R107 normalized branches,
R108 base series, [Stacks normal-curve DVRs](https://stacks.math.columbia.edu/tag/0B8Y),
and [Newton polygon theorem](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf).
