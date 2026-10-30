# Direct projective witness comparison

The CPU oracle only needs to decide whether its independently accumulated sum
matches the target. In Jacobian coordinates, x=X/Z^2 and y=Y/Z^3; compare
X=xZ^2 and Y=yZ^3 without an inversion. Infinity is handled separately.
Every input point and the target remain validated. The solver relation check
and both complete affine certificate replays remain unchanged.

Source: [EFD Jacobian coordinate definition](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html), reviewed 2026-10-30.
This removes one final field inversion per finite returned witness. It does
not alter decomposition search, first witnesses, the algorithm or its complexity.

All 31 release tests and strict Clippy pass. The checked-sum tests now compare
positive results with independent affine sums and reject a different valid
point and the opposite-y point, over every small-group pair and 64x64 pairs
on the 40-bit fixture. Empty sums, identity, inverse pairs, off-curve and
noncanonical tuple points and targets are checked. Archived certificates remain
byte-identical in the existing regression tests.

Performance is unmeasured for this candidate. Next freeze the same five-arm
complete panel, with original baseline2bd4588fba60 and previous308bf1a6a5bc.
All setup, discarded trials, checks and serialization remain charged. This
change alone is not expected to close the full remaining 10x gap. Shared
solver profiling overhead remains a separate opportunity, currently reserved
by T-906 and not edited here.
