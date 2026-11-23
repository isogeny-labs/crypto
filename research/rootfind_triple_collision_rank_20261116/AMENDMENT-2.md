# Before-data amendment 2: independent row replay and rank-stop prefix

Frozen before the triple-collision test has executed. The original protocol and
Amendment 1 remain intact. This amendment changes validation and reporting,
not the registered curve cells, triple stream, collision rule, or rank target.

After generating all rows, independently decode the serialized witness and row
records. Recompute each triple sum with the general binary-curve point-addition
implementation rather than the fast single-word implementation used by the
producer. Reconstruct each signed column row from its indices and coefficients,
compare it with the serialized row, and replay the independent/dependent flags
in stream order. A mismatch fails the test. Report the number of independently
replayed rows. This is an implementation-independent arithmetic check within
the same repository, not a second-host or independently sourced reproduction.

Record the first unique-triple prefix at which homogeneous rank reaches
`columns - 1`, if it does. At that instant snapshot the direct generator
additions, cached prefix and tail additions, cache hits and retained entries,
collision verification additions, hash lookups, rows, and rank. Label this a
deterministic *early-stop work projection*: the test still processes its full
frozen triple budget and does not time an early-stopping collector. If the rank
ceiling is not reached, emit `null` and preserve the final censored trajectory.
The projection excludes pair-table setup, generator/challenge descent, rank
time, allocation, memory and verification, so it cannot support a speed claim.
