# R104: leading cleared N51 residue at x=1

The source-bound R102 rows have exactly 22 coefficients of x+1 order -9.
Their leading F2 support is

    H = y^2 (y+1)^4 (l^2+l+1)^7
      = (y^6+y^2) (l^14+l^13+l^11+l^10+l^8+l^7+l^6+l^4+l^3+l+1).

The actual R101 specialized source equations are `f=y^3(y+1)^4` and
`G=(l^2+l+1)^8`. Pinned Singular checked that the quotient has dimension
112, H is nonzero and already in the standard monomial box, deleting one
term changes its reduction, and H has residue zero at both maximal ideals
`(y,l^2+l+1)` and `(y+1,l^2+l+1)`. The producer exited zero with no native
errors; a fresh process reproduced the input and output byte-for-byte.
The independent F2 support checker reconstructed the 22 terms directly
from the R102 rows and checked the factorization; Lean 4.34.1 compiled the
concrete F2 polynomial factor identities and dimension arithmetic.

The factorization strengthens the location statement: on the y=0
component, H is `y^2 q^7`, where `q=l^2+l+1`, and `(y,q)H=0` in the
source-order fiber. On the y=1 component, H is zero because `(y+1)^4=0`.
Thus the nonzero leading coefficient lies in a nilpotent socle. Since
`f,G` are monic and the G coefficients have only x-power denominators,
the local source order is free on `l^i y^j` for `i<16,j<7` at x+1;
H nonzero proves 9 is the least clearing power **in this order**.
The normalized local ring can be larger, so this is not a normalized pole
or branch valuation. The next construction is integral closure at the
y=0 point, followed by branch valuations and the other finite supports.

The raw producer/replay archive has SHA-256
`ac0d2364774167d2a79871484326308fd27d42b5c4b6d6fbd428a2a0aa3c3af4`.
The report and diagram are in `publication/`. The PDF was structurally
checked; this host lacks a rasterizer for visual page inspection.

Sources: R51 N51 donor, R101 fiber, R102 exact orders, and
[Stacks monic finite free quotient](https://stacks.math.columbia.edu/tag/00FD).
