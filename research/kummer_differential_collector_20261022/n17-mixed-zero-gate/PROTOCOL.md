# N17 mixed-zero gate audit protocol

This is an exact algebra and source-path audit, not a new target cohort.
Inputs are the frozen 21 Rosenhain models and the N16 exact C1/C2 matrices.
For each model, form the valid reduced divisor given by the two rational
branch points `1` and `lambda`, with `u=(x-1)(x-lambda)` and `v=0`.
Compute the direct C2 coordinate vector, invert C1, and check the proposed
linear basis change exactly in `Fp`. Check all 21 distinct matrices and
their nonzero determinants. The N16 verifier independently reconstructs
the matrices from the frozen model constants.

Inspect the actual N11 recurrence, N15 lift and cached theta xADD source.
Record the mixed-zero condition, conditional fallback path, and missing
proof and implementation gates. Do not launch a new 1,260-target collector
unless the all-state geometric projection and xADD/fallback integration
are established and a complete cold-cost protocol is frozen first.
Retain the N15 finite replay unchanged. A null ratio is an unmeasured ratio,
not a zero cost or zero success rate.
