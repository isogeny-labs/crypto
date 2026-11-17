# R108: extend both marked-23 base-completion series to s^128

Bind R105's full f-support result and R106's accepted solver/result by
SHA-256. Reuse the committed R106 coefficient-by-coefficient Hensel
solver at precision 128; verify both original f residuals modulo s^128,
the `(2,3)` and `(3,4)` initial valuations, and exact agreement of every
coefficient below s^32 with R106. Reject a changed source hash and
freshly replay to byte-identical JSON.

The point is to cover R102's maximum x+1 denominator order 9, which
becomes s-order 27 or 36 on the two base places, before evaluating the
actual N51 coordinate. This is a finite-precision base-completion
calculation, not yet an N51 divisor or lambda-branch valuation.
One deterministic local Python process, no cloud resources or timing
comparison. Commit source/protocol before data; retain failures.

Sources: R102 coefficient denominator audit, R105 full base support,
R106 Hensel solver and [Stacks complete-local lifting](https://stacks.math.columbia.edu/tag/04GE).
