# Standalone CPU relation finder and complete recovery

This package runs the checked hashed-ECDLP reduction with a reusable CPU
meet-in-the-middle oracle. It includes the repository reduction directly and
builds without compiling the root `crypto` package or any CUDA code.

The oracle accepts the reduction's ordered signed-point bases. Explicit
x-coordinate lists can be lifted to both signed points, sorted and deduplicated
per slot before calling `find`; the shared ecbench plug-in remains separate.

```
cargo test --locked --manifest-path crates/hashed-ecdl-cpu/Cargo.toml
cargo build --locked --release --manifest-path crates/hashed-ecdl-cpu/Cargo.toml
# Public inputs only; Q must be an affine point on this prime-order curve.
hashed-ecdl-cpu cached p a b r Px Py Qx Qy m tau seed max_rows batch [max_columns] > certificate.json
hashed-ecdl-cpu --verify certificate.json
```

`reference` selects the original exhaustive oracle. Both modes verify the
complete certificate independently before emitting JSON. A limit is an error,
never evidence that no relation exists. The included shared reduction validates
the full prime order with the generator witness and Hasse interval, supporting
fields below 2^60. Optional `max_columns` defaults to 512, at most 16384. These
are admission limits, not evidence that all admitted instances are feasible.

## Work and memory

For each new curve/base set, choose a contiguous split minimizing the combined
prefix/suffix tuple counts, subject to `max_rows`. Build both sum tables once,
index suffix sums by exact compressed point keys, and retain the tables across
queries. The open-addressed index keeps the first tuple ID for duplicate sums. Visit prefix tuples lexicographically and select the smallest matching
suffix index. Thus duplicate sums still produce exactly the original oracle's
first full tuple, making whole certificates directly comparable.

Compute target-minus-prefix residuals with Montgomery batch inversion. `batch=1`
is the per-residual inversion control. If distinct suffix sums cover at least
one eighth of the group, batches grow geometrically from one up to the configured
maximum; otherwise full batches are used immediately. This scheduling heuristic
does not assume an early match or change exhaustive negative answers.
The first match in a batch wins; all
arithmetic already performed in that batch is charged. Every returned witness
is replayed by a checked projective sum: validate every point, accumulate, then
compare directly with the affine target using cross multiplication, avoiding
the final inversion. Full certificate verification independently checks all relations
using the shared affine group operations.

`max_rows` and `max_work` cap the combined *retained tuple count*, checked before
preparation and again on warm queries. They do not cap total RSS, arithmetic or
wall time. The original exhaustive oracle uses a different work unit.
Preparation also stores the original bases and four curve parameters and uses
temporary sum vectors; queries use O(batch) scratch. `peak_table_bytes` counts only retained
prefix capacity and suffix slot capacity (including empty slots), excluding these
other allocations. Suffix slots use at most half occupancy; their count is the
next power of two above twice the suffix tuple count. All counters include
discarded trials. `clear()` drops tables without resetting cumulative counters.

This is generic decomposition (bounded by square-root group-order methods;
cannot establish an advantage over rho), not the paper's conditional Boolean
tensor contraction algorithm. It is an engineering optimization with exact
reference-output preservation, not an asymptotic claim.

The implementation applies standard affine formulas and batch inversion; see
[Bernstein and Lange, Analysis and optimization of elliptic-curve single-scalar
multiplication](https://www.hyperelliptic.org/EFD/precomp.pdf). The unchanged
shared reduction and its independent verifier define certificate semantics.
