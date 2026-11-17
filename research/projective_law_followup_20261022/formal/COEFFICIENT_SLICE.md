# Source-derived common 2+2 coefficient slice

Schema: `projective-law-coefficient-slice/v1`. The restricted parser extracts
seven straight-line field expressions from `add22_weighted_common` and the
seven corresponding `let` expressions in `generic22Runtime`. It substitutes
earlier assignments, expands each expression over the integers, and requires
exact polynomial equality. The checked mapping is `q1c` to `q₁`, etc.; the
weighted-to-dehomogenized interpretation is outside this check.

All seven coefficient pairs match. Subtraction-to-addition mutations in
Rust `bcoef` and Lean `B` are rejected. The [CSV](coefficient-slice.csv) records each pair,
monomial count, and equality result; the [graph](coefficient-slice.svg) shows
the counts. [Certificate](coefficient-slice.json) records both source hashes
and every expanded monomial with its integer coefficient.

This is a source-derived algebraic check for a narrow fragment. It does not
model Rust control flow, field representation, weighted-coordinate validity,
the remaining addition branches, or equality of the returned divisor class.
The existing full-cohort prices and their relation to matched rho are
unchanged. Run `python3 check_coefficient_slice.py --check` to verify the
frozen certificate; run `python3 check_coefficient_slice.py` and
`python3 render_coefficient_slice.py` to reproduce these artifacts.
