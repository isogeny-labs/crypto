# Pre-run independent square witnesses

The frozen search completed all 24 source/carrier cells with zero exact
degree-five identities. Every retained degree-five numerator is to be
reparsed and checked separately. The witness source recomputes
`P=B(A³+A²B+bB³)/f`; it requires exact divisibility and coprimality.
For each degree-eight `P`, it either saves a nonsquare leading coefficient
in `F243` or solves the five high coefficients for the only possible
degree-four square root and saves a nonzero lower-coefficient remainder.
It does not call `P.is_square()`.

Freeze this source before execution. The witness validates the final
square-test rejection; the complete numerator enumeration remains in
the original screen and must be audited separately.
