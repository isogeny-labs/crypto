# R98: marked-23 lambda denominator audit

The accepted R51/R63 degree-112 source equation `G51` has exactly 28 division
tokens. All are `x^e`, with `e` in the recorded `result.json` list and maximum
161. The source SHA-256 is bound in `denominator_audit.py`; the checker matches
every slash position in the unique `G51` assignment. Its planted `x+1`
denominator is rejected, and an independent process produced byte-identical
JSON. Lean 4.34.1 compiled `lean/Denominator98.lean`, proving the list's count,
bound, occurrence of 161, and clearing exponents. `publication/report.pdf`
contains the recorded exponent histogram and claim scope.

Attempt 01 failed because its negative-control mutation targeted an earlier
assignment on the donor's single line; see `FAILURE-01.md`. Amendment 02 was
committed before the accepted attempt. This is an exact audit of the displayed
equation's coefficient poles: on `D(x)` the coefficients are regular after
clearing by `x^161`. It does not identify normalized branch points.

Next: compute `Res_y(f51, Res_l(G51,dG51/dl))` with the accepted donor on
`D(x)`, factor its numerator over F2[x], and combine those supports with R97's
base-order discriminant and infinity before local normalization. Bound the
native computation, freeze inputs before execution, and preserve failures.

Sources: R51 `runs/base23-struct51-03/marked23-triangular51.sing`, R63 `CM.md`,
R97 `CM.md`, and [Stacks 53.2](https://stacks.math.columbia.edu/tag/0BXX)
for passage from a function field to its normal projective curve.
