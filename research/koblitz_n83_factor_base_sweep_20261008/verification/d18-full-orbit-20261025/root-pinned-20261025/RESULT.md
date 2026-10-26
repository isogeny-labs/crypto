# Full-D18 planted relation passes with selector and phase pins

The [preregistered constructed control](../ROOT_PIN_PROTOCOL.md) ran from
clean commit `9e3edd9786a30c954abad96ab2fc858ab6d0fe65`, binary SHA-256
`457fce83a960729f2d4a33077198ff49710ff0bb3837db4c886515123c16bad7`.
It used the same previously fresh-fetched, locally rehashed full-D18 S3
archive as the [shared-root comparison](../shared-root-20261025/RESULT.md).
The worker imported all 5,418,738 points in 59.098 seconds and built the
shared-root circuit plus 498 x-coordinate and 32 selector/phase pin clauses
in 0.178 seconds. The actual model had **329,726 variables, 2,807,784 total
CNF clauses and 3,901 XOR rows**.

SAT returned in **0.165 seconds with zero conflicts**. The worker checked the
original-field S3 model, lifted the four x coordinates to the archived points,
reconstructed a signed-orbit row, rejected a changed target and inserted a
rank-one row matching the planted witness. Its status is
`PASS_CONSTRUCTED_CONTROL`, with 59.766 seconds whole-process worker wall.
The external supervisor recorded 61.20 seconds real, 3,887,513,600 bytes
maximum resident set size and exit zero on a shared L0 Mac.

The [independent Python verifier](verify.py) rehashed the source binary and
full 160,747,529-byte S3 object; implemented GF(2^83) multiplication,
inversion and binary-curve addition separately from Rust; checked the four
archived points and generator have the exact subgroup order; recomputed the
four-point target sum and sparse-row BLAKE3 digest; and rejected a changed
target. It returned [PASS_INDEPENDENT_GROUP_AND_ROW_REPLAY](independent-replay.json).
Four [tamper controls](tamper-controls.json) rejected altered target, row
digest, source commit and S3 GET length.

This is a correctness control with the orbit selectors and Frobenius phases
provided to SAT. The earlier shared-root control with those choices free
reached its 300-second cap. The contrast shows that the full base, model and
group verifier can support the planted row, while the free-choice search
remains unresolved. It does not measure natural relation yield or solve time.
Its rank-one row is constructed and excluded from natural useful-rank counts.
`selected_best_total_runtime` remains `null`; full cold IC accounting and
matched same-target rho are still open.
