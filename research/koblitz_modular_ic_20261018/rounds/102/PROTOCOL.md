# R102: source-bound x+1 valuations of N51 coefficients

Bind the actual R51 triangular donor by SHA-256. Extract the unique `N51`
assignment and require it to be `n` plus exactly 105 rational coefficient
terms with distinct `(l exponent,y exponent)` monomials in the 16-by-7
standard range. Parse every numerator and denominator as a nonzero F2[x]
polynomial, compute exact `(x+1)` valuations by polynomial division, and
report each coefficient order, histogram, minimum, and the least
coefficientwise clearing exponent. Reject a planted non-x term through the
same parser. A separate process must produce byte-identical JSON.

This is an exact source audit, with no timing/statistical comparison and no
CAS job. Its clearing exponent is sufficient for the displayed N51
coefficients; reductions modulo f51/G51 or integral closure may improve it.
It is not a pole valuation of the corresponding function on the normalized
curve. Commit source and protocol before the first run; retain failures.

Sources: R51 donor; R63 degree-112 source field; R101 x=1 y/lambda
source-order fiber; [Stacks finite and integral extensions](https://stacks.math.columbia.edu/tag/00GH)
for the distinction between a coordinate order and integral closure.
