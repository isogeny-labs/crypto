# R84: first direct reduction inconclusive

R84 corrected R83's ring-scope error by serializing all 27 source
module polynomials before loading the candidate ring. The frozen run
reached the first denominator-cleared identity, for `T(2)`. Singular
reported that the saved `norid` generators are not a standard basis
and returned a nonzero remainder. Under the frozen protocol this is
**inconclusive**, not a false-identity certificate. It printed no
terminal success marker, and the strict gate rejected the attempt.
No positive fraction membership was certified. The raw attempt and
checkpoint are retained.

The combined R82–R84 evidence narrows the useful next construction.
The full 22-variable standard basis is the known expensive path;
direct division by nonstandard generators cannot decide membership
when its remainder is nonzero. A proper next verifier needs an
explicit syzygy/lift witness from the saved `normal.lib` ring
construction, or a bounded local standard basis with a justified
bridge to the source field. The accepted rank-32 RR algebra and
diamond-transport chart remain an independent normalization route.
