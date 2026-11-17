# R108: both marked-23 base series through s^128

The source-bound R106 Hensel solver was extended from precision 32 to
128, with R105 support and R106 solver/result pinned by SHA-256. Both
new series satisfy the original f equation modulo s^128, preserve
`(v_s(z),v_s(t))=(2,3)` and `(3,4)`, and agree coefficient-for-coefficient
with R106 below s^32. The y=0 series has 71 nonzero z coefficients
below 128; the y=1 series has 80. The full supports are in `result.json`.

R102's largest coefficient denominator has x+1 order 9. Its pullback
has s-order 27 on the y=0 base place and 36 on y=1, so order 32 was
insufficient for the latter. Precision 128 supplies exact base-series
input for leading N51 rational-coefficient calculations. Cancellations
could still demand more precision; that must be checked from actual
weighted terms before asserting a divisor valuation.

The producer exited zero. A fresh process reproduced the JSON
byte-for-byte and a changed R106 hash was rejected. An independent
bit-polynomial evaluator checked both source residuals modulo s^128,
the low-bit match, and equality of the embedded Lean constants with the
committed source/result JSON. Lean 4.34.1 compiled the two 128-bit
residual and leading-bit theorems. Formal existence and uniqueness of
the full series follows from the simple-root [Hensel lemma](https://stacks.math.columbia.edu/tag/04GE)
used in R106; this round certifies the finite truncation.

Raw producer/replay SHA-256:
`99dcbbc8daaecdb718715b7ae978431d6723ada3108f94d3e99f63f1cab79a87`.
Result JSON SHA-256:
`a58481e36b510f06a8bb1c2aeb92fce61d49cf7c1ba2eb1ce872b584b1379b33`.
Next substitute into the actual N51 rational coefficients and use
R107's lambda edge weights to test whether a unique leading term
determines each degree-7 branch valuation; lift the degree-1 lambda
roots if their leading terms cancel.

Sources: R102 coefficient audit, R105 full base support, R106 local
uniformizers, and [Stacks Henselian lifting](https://stacks.math.columbia.edu/tag/04GE).
