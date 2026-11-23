# Version03: direct division-polynomial construction

Preserve qualifier01 u5/u7 and qualifier02 u7 setup timeouts, all at the
unchanged60-second limit. Qualifier02 constructs u5 but its coordinate-only
recognizer accepts only1/4 declared public inputs at every tested precision.
Do not count the other completed calls as successful maps.

Version03 changes only the symbolic construction. On y²=x³+1 it computes
the classical division polynomials in the coordinate algebra, with odd
terms real and even terms y times a polynomial. Initial values and
recurrences follow Mathlib4.24 DivisionPolynomial/Basic.lean and the
already frozen round02 normalized EDS implementation. Each division by
2y is implemented as an exact polynomial division by2(x³+1), then a y
factor; reject any remainder. Compress only polynomials whose nonzero
exponents have the required common residue modulo3. Final rational
functions use the unchanged primitive GCD from version02.

Require byte-identical u3 and u5 models and complete qualification output
against version02 before trying u7. Kernel numerator/quotient evidence
may have a different nonzero scalar normalization; exact monic kernels
and recognized beta/image certificates must agree. Sources live in v3/;
v1/v2 and their qualifiers remain immutable. Use new receipt/model names.

The mathematical construction, public inputs, precision lists, LLL bounds,
controls and60-second call cap remain unchanged. This is an implementation
remedy with correctness qualification, not a speed comparison or a new
transfer mechanism. A numerator/denominator Padé recognizer is a separate
future version; it is not part of this amendment.
