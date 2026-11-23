# R102: exact x=1 coefficientwise clearing bound for N51

The SHA-256-bound R51 `N51` assignment is `n` plus 105 rational coefficient
terms with distinct `(lambda exponent,y exponent)` monomials. R102 parsed
every numerator and denominator over F2[x] and divided by `x+1` exactly
to obtain its coefficient order. All 105 orders are negative; their
histogram is in `result.json`. The minimum is -9, attained by 22 terms;
the maximum is -1. Therefore `(x+1)^9` is the least **coefficientwise**
clearing power for this displayed N51 relation. `(x+1)^8` fails for those
22 terms. A separate process reproduced the full JSON byte-for-byte,
and a planted non-x coefficient was rejected. Lean 4.34.1 compiled the
105-element bound, occurrence, and clearing arithmetic.
The separately committed `verify102.py` used Hasse coefficients of
`p(1+t)` instead of repeated polynomial division and independently
matched all 105 numerator, denominator, and net orders, including the
22 worst terms.

This says that the displayed coordinate `n` has no coefficientwise regular
expression at x=1 without clearing. It does not prove that the associated
function has normalized pole order 9: reductions modulo `f51,G51` or a
different integral basis may cancel poles. Combined with R101's two
source-order points above x=1, the next focused construction is to reduce
`(x+1)^9 n` in the local y/lambda algebra and compute its valuations on
the normalization.

The one-process exact audit and fresh replay are in the committed raw
archive, SHA-256
`554e9e6fab5c216b3f4964e9c505970544cac5a9ad0059fc6694c23c79ae38e4`.
No run failure occurred. The one-page PDF is generated from `result.json`
with a dependency-free renderer; its PDF structure and recorded values
are checked. This host lacks PDF rasterization, so no visual raster
inspection is claimed.

Sources: R51 actual triangular donor, R63 degree-112 source, R101 x=1
source-order fiber, and [Stacks finite integral extensions](https://stacks.math.columbia.edu/tag/00GH)
for the distinction between a displayed coordinate order and its integral
closure.
