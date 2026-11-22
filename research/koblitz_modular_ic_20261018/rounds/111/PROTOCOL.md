# R111: x=0 Newton support for the degree-112 source

This deterministic exact computation binds the R51 donor and R105 parser by SHA-256. Parse all eight f51 y-coefficients and all 29 G51 terms. At x=0, verify f51(0,y)=y^4(y+1)^3. Translate y=z and y=1+z; expand every F2 monomial and compute the full (z exponent, x exponent) support, lower hull, rational slopes, and residual edge-polynomial supports. For each edge, form source-bound lower bounds on the five lambda coefficients using actual x numerator exponents, y exponents, and x denominator powers. Keep these as lower bounds until cancellation along a lifted base root is checked. Fresh byte-identical replay, changed donor rejection, independent recomputation, Lean edge arithmetic, and a report are acceptance checks. Commit source and protocol before data.

The result is a normalization entry point, not a complete x=0 branch census or N51 divisor. Exact base and lambda lifts and residue factorization follow. Local exact F2 arithmetic; no cloud or timed claim.

Sources: R51 accepted donor, R97 x=0 base-order fiber, R105 parser, [Stacks integral closure](https://stacks.math.columbia.edu/tag/00GH), [Newton polygon background](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf).
