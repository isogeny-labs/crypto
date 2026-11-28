# Verified sparse solve after relation collection

The selected factor base has `B=2447` columns. If a relation producer
supplies at least `B−1` independent verified rows, a null vector gives
factor-base logs up to scale. The native verifier previously used dense
`O(B³)` elimination only on its toy fixture. The new
[dependency-free sparse module](native/src/sparse_wiedemann.rs) provides a
black-box solve and a rank-certified tall-row compression over a
trusted prime modulus such as `r=329406144237565159`.

After choosing `B−1` rows and fixing a column with nonzero null-vector
coordinate to 1, delete that column to obtain `A x=b` of dimension
`n=B−1`. `SparseMatrix` stores sorted nonzero row entries and uses
matrix-vector multiplication. Its Wiedemann-style sequence takes
`2n` products, Berlekamp–Massey finds a scalar recurrence, and the
candidate is reconstructed from Krylov powers of `b`. Every returned
candidate is checked by a full `A x=b` multiplication. Bad scalar
projections are retried; exhaustion returns `Inconclusive`, not a false
singularity claim. The `solve_certified` mode also requires a full-degree
recurrence with nonzero constant. Such a recurrence has degree `n` and
forces the matrix characteristic polynomial to have nonzero constant,
certifying the square minor is nonsingular. A smaller recurrence is
inconclusive even when the minor is invertible.

For `nnz` stored coefficients, each probe costs
`O(n·nnz+n²)` prime-field operations: `O(n)` sparse matrix products plus
quadratic scalar recurrence and vector accumulation. Live memory is
`O(nnz+n)` for the matrix, Krylov vectors, sequence, and one probe.
With seven nonzeros per selected row, `nnz=O(n)` and the solve fits the
proposed `Õ(B²)` scale **conditional on already having verified rows**.
Input validation requires sorted, distinct in-range columns and
nonzero coefficients; the caller is responsible for providing a prime
modulus and group-verified relation rows.

Focused tests recover the exact null vector of the repository's
19-row toy relation fixture after fixing column 19, solve a 64×64 sparse
recurrence over the selected 61-bit modulus, reject a bad projection
before a successful retry, and keep consistent singular and invalid
inputs inconclusive or rejected. These are meaningful solver checks,
not a benchmark or rank proof for M61.

`TallSparseRows::rank_certified_nullspace` handles an overdetermined
verified row system. In each bounded attempt it hashes each source row
into one of `n=B−1` buckets with a nonzero field weight, forming sparse
linear combinations. It fixes a free column to 1 and calls
`solve_certified` on the resulting square minor. It then checks the
reconstructed vector against **every original row**. A successful
full-degree square certificate proves the original row rank is at least
`B−1`; the checked nonzero null vector proves it is at most `B−1`.
Thus a returned result certifies exact rank `B−1`, independently of the
random bucket heuristic. Exhausted compression or scalar probes return
`Inconclusive`. A test with eight copies of the 19-row toy fixture
recovers its exact 20-column null vector; deficient and inconsistent
tall systems are rejected as inconclusive.

## Remaining integration boundary

The selected-instance heuristic predicts about 20,078 seven-term rows,
but no M61 producer exists. If `m=Õ(B)` verified rows each have `O(1)`
nonzeros, compression uses `O(m)` sparse storage and a compressed minor
with at most `O(m)` nonzeros. Each solver probe then costs
`O(Bm+B²)=Õ(B²)` field work. The number of compression and probe retries
needed for a growing family is not established; bounded failure is
reported, not hidden. For the selected `m≈20,078`, seven-term rows give
an upper bound of about 140,546 stored nonzeros before cancellations,
so practical constants may be substantial. The caller must still
verify each elliptic relation, obtain both generator and target descents,
and check the recovered log. The missing six/seven-term relation locator
remains the dominant open step.

## Selected-dimension synthetic cost probe

The [native scale example](native/examples/sparse_rank_scale.rs) plants a
one-dimensional kernel in a sparse unit-upper-triangular system over the
selected 61-bit prime modulus. It makes eight scaled copies of each
independent row, with at most seven nonzeros per source row, then asks
the tall-row routine to recover and certify the planted vector. These
are **synthetic linear equations**, not elliptic relations. One local
release run gave:

| `n=B−1` | Source rows | Source nonzeros | Compressed nonzeros | Total ms | Compression attempts |
|---:|---:|---:|---:|---:|---:|
| 64 | 512 | 3,464 | 2,140 | 4 | 1 |
| 256 | 2,048 | 14,216 | 11,022 | 76 | 1 |
| 512 | 4,096 | 28,552 | 23,253 | 285 | 1 |
| 1,024 | 8,192 | 57,224 | 47,978 | 1,215 | 1 |
| 2,446 | 19,568 | 136,856 | 116,251 | 7,162 | 1 |

The selected-dimension run shows that this implementation can carry a
roughly 20,000-row sparse system through a certified solve in seconds on
the current workspace. Its duplicated, planted row structure makes it
unsuitable for estimating rank or compression success on actual M61
relations. Timings vary by machine, and no relation-generation cost is
included.

```sh
cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example sparse_rank_scale
```
