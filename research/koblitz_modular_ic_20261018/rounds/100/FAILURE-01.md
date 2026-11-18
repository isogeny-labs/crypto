# R100 attempt 01: off-by-one term count in preparation

The source-bound Python preparation exited 1 with `unexpected numerator terms`.
The R99 numerator has 73 positive-exponent x terms plus its constant term,
for 74 terms total. The frozen checker expected 73. No Singular factorization
was launched and no factor result was produced. Amendment 02 corrects the
expected count and preserves the same R99 source hash and factorization
acceptance checks.
