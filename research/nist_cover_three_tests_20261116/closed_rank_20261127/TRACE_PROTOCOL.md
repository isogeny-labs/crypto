# Signed secant residual-trace elimination, frozen 27 November 2026

Keep the original225columns and rank-six baseline. Use the existing429-column
matrix with its204auxiliary columns for the augmented diagnostic. This is a
necessary-condition computation for canonical lines through two rational curve
points with prescribed signed images in the fixed domain, with one aggregate
degree-two residual trace per row. It does not claim actual rational support
for every signed symbol or a geometric realization for every norm quartet.

Let v(a,b) be the coefficient vector for the two prescribed images, h the
unchanged reference-section image vector, and kappa its actual elliptic sum.
The line relation has projected form v(a,b)+t-h, where the residual aggregate
maps to kappa-a-b. Fold every nonzero auxiliary trace under sign, treating O
and existing-column aliases directly. Reusing the same signed trace between
two rows yields v(a,b)-v(c,d); reusing opposite signs yields
v(a,b)+v(c,d)-2h. These lie respectively in signed four-symbol target0 and
target2kappa norm domains. Existing-column/O aliases lie in targetkappa with
O padding. Reference terms remain on the original base; no column promotion.
[pcc:unverified]

Within each new trace group, eliminate its one column against a representative
row. Every resulting old-domain row is spanned by the differences/sums above.
Thus union of original rows and all target0,kappa,2kappa quartet rows gives
an upper bound on the closed rank obtainable from this single-aggregate-trace
family. This is an algebraic necessary bound: a null result rules out that
family at the fixed domain; a positive result is only a candidate and requires
actual geometry/KM plus independent replay. Multiple separately named residual
points, higher-degree functions and new anchor image columns are outside scope.
[pcc:unverified]

Run old225and augmented429modes separately. At each target enumerate every
unordered pair over O and both signs, including diagonals. Compare each sum
and target-complement between named OpenSSL and separate GMP affine arithmetic;
compare whole stream SHA256and grouped/sorted coefficient rows and minimal
witnesses. Reuse frozen norms-v2 helpers without changing their bytes. Retain
all rows, including the formal zero row when present, up to4096distinct rows
per domain; reaching the cap rejects completeness. Check each row sum; append
to frozen original matrix; report full, auxiliary and closed ranks and gain.

Controls: targetkappa must recover every original row up to sign. Separate
G,2G fixture at targetO must detect(-2,1); target8G with reference8G must
detect(-8,4), proving the twice-reference branch is not hardwired empty.
Changed coefficients must reject. Fixture rows never enter the actual matrix.
Retain source/input/binary hashes, commands, stage exits, stdout/stderr and
complete wrapper costs. One CPU,1GiB virtual memory,300s strict build and600s
per native worker. No new PCC, scalar recovery or performance comparison.

Prior art: the already-reviewed Diem2012canonical-line norm condition and
large-prime elimination; recorded_pool_20261123H-B/H-C/H-E. This extends the
finite domain by the necessary sign-folded2kappa case, without a novelty claim.
