# R128: Newton expansion at the degree-7 double base root

Exact local calculation. Bind the tracked R51 `f23` donor and the R127
specialization by SHA-256. In `F128=F2[a]/(a^7+a^6+a^3+a+1)`, expand the
actual equation at `x=a+s`, `y=90+u` using characteristic-two binomial
parity. Record every nonzero coefficient, the first `s`-order at each
`u`-degree, the lower Newton hull, all edge supports and residual
polynomials. Check the `s=0` specialization against R127's exact factor
and the derivative/gcd pattern.

This locates the next local uniformizer; a Newton edge by itself does not
establish a normalized branch count or N51 valuation. Controls are a
separate direct coefficient evaluator, byte replay, changed-source
rejection, and finite Lean checks of any displayed edge arithmetic.
Local exact process with zero cloud cost; source/protocol pushed before data.

Prior work: R97 discriminant, R127 degree-7 factorization. Use the
[Stacks normal-curve DVR criterion](https://stacks.math.columbia.edu/tag/0B8Y)
to distinguish source-order multiplicity from normalized valuation.
