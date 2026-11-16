# Amendment 1: replace the unavailable K17 fixture

Recorded after the first source-committed release test, before changing the
test list. That run exited 101: both added tests panicked at
`KoblitzCurve::new(0, 17).unwrap()`. For the E0 trace recurrence,
`#E(F_{2^17}) = 130972 = 4 · 137 · 239`; the constructor requires its
largest prime factor to exceed the cofactor, and `239 < 548`. K17 is
therefore unavailable as a `KoblitzCurve` under this repository's existing
admission rule. The full stdout, stderr and exit are retained as
`test-first.*` once copied from the test path. This is a fixture failure,
not a result about the batched lift. [pcc:unverified]

Replace the K17/d6 fixture with the already supported K19/d7 E0 fixture in
both added tests and the source-derived inversion-call chart. Keep K5/d2,
K7/d3, K13/d5 and K23/d8, the same cache modes, point-order and metadata
checks, and the same failure rule. The replacement raises the fourth
subspace from 64 to 128 abscissae; no timing comparison or method selection
depends on this choice. Do not change the original protocol text. Recompile
and rerun the fixed correctness screen, retaining every attempt.
