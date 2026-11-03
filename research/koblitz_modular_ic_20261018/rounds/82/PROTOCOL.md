# R82 candidate auxiliary fractions, frozen before data

The installed Singular `normal.lib` `computeRing` routine states that
its unreduced `T(k)` is module generator `J[k+1]/J[1]`. R50's producer
serialized the module with `J[1]` moved to the last slot, so the
corresponding saved fraction is `module50[k]/module50[27]` in the
selected degree-32 source field. `substpartSpecial` removed some `T`
variables but retained their original names. Check all 19 surviving
`T(k)` names/indices against R81, map each saved numerator and the
common denominator through the literal `normap`, and reduce the full
identity `denom*T(k)+numerator` modulo the complete candidate ideal.
Require the denominator nonzero in the selected source field, source
ideal containment, 19 zero residuals and a corrupted numerator control
whose residual is nonzero. This is a source-fraction compatibility
test of the rejected R50 candidate, not a normalization certificate.

Run locally in Singular, one Singular/FLINT thread, seed 82, 1200
seconds and 4 GiB virtual. Copy source/protocol/driver bytes from
committed Git, verify donor hashes, and accept only exit 0, exact
marker, zero native whitespace-`?` errors and every assertion.
Retain failures and timeouts; freeze a repair before a new attempt ID.
No timing comparison.
