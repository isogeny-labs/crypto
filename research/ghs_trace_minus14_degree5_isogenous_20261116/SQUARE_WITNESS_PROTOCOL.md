# Pre-run square-obstruction replay

The twelve source-bound degree-five screen receipts each retain one
degree-five, coprime numerator whose full equation failed the square test.
Before interpreting the zero-hit count, `square_witness.sage` will parse
each retained `(A,B)` independently, recompute
`P=B(A³+A²B+bB³)/f`, and verify exact divisibility.

For `deg P=8`, a square must have square leading coefficient. If that
coefficient is nonsquare, save its quadratic character in `F243`. If it
is square, solve uniquely for the five coefficients of a degree-four
square root from the coefficients of `x⁸,...,x⁴`, then save the nonzero
remainder in degrees below four. This gives an explicit finite-field
obstruction for every survivor without calling `P.is_square()`.

Freeze the source before execution. A failure or missing cell prevents a
twelve-carrier exclusion claim. The witness checks the final square
condition independently, while the original search still supplies the
exhaustive candidate enumeration.
