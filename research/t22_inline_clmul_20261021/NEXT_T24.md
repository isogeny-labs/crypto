# T24 next card: shorten the dependent chains of the probe loop (from T22)

Evidence (T22): -45% instructions in the multiply/reduce path, -15% of the K = 40 profile, scan ns per probe unchanged within a ~5% noise floor. Hypothesis: the K = 600 scan is latency-bound, not instruction-bound.

Test 1 (same-binary switch, default off): `RootBatch128::solve` runs the Montgomery prefix and back-substitution as 4 interleaved sub-batches of 64 (4 independent serial chains instead of one of 256); results identical to the single chain, unit-tested on random and exceptional inputs, n = 73 and 83.
Test 2: Bloom-line prefetch depth (`KIC_PREFETCH` currently prefetches a whole state's partners): sweep a lookahead of 1, 2, 4 states.

Gate: the T21/T22 scan gate (K = 600, 20,000 states, 7 rotated rounds, A/A arm, ratio <= 0.8333 and upper bound <= 0.90) with callgrind Ir as the guard (Ir must not rise by more than a few percent: the cut is about ILP, not work). Kill: under 1.2x on the scan, or any change in relations/recovered scalar. Pre-register an A/A that includes a rebuild if two builds are compared.
