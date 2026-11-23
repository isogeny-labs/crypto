# Repository grounding and audit receipt

Audit date: 2026-11-22. The six Rust source files below were read at
`isogeny-labs/crypto` commit `ce396efc7129c8fc90409fbdb2720461272a366b`.
This is a bounded source inspection: no Rust tests, benchmarks, complete
ECDLP solves, or signed PCC verification were performed by this audit.
Existing test definitions are evidence of intended checks, not a passing
execution receipt. The catalog's literature bounds do not automatically
describe these implementations. Source observations and derived caveats
below remain `[pcc:unverified]`.

## Existing implementation to reuse

### Large-prime filtering and provenance

[`large_prime_filter.rs`](https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/large_prime_filter.rs)
exports `PartialRelation`, `CompleteRelation`, `LargePrimeFilterOptions`,
`filter_large_primes`, and `replay_complete_relation`. Input rows separate
factor-base columns (`u32`) from large-prime identities (`u64`), carry an
RHS, and require distinct source IDs. Normalization combines repeated keys
modulo the supplied modulus and drops cancellations before incidence
construction. Output rows retain sparse coefficients of original source IDs.
The replay helper reconstructs that combination; callers still compare the
reconstruction and check the original group relations.

The implementation already performs singleton peeling, light-row ordering,
and sparse elimination with a Markowitz-style score. Its column degrees come
from the initial core, so it is not the catalog's generic dynamically
maintained Markowitz algorithm. Fill/source limits can discard a candidate
row. Normalization, ordered deduplication, sorting, payload arithmetic, and
provenance growth must be charged beyond the abstract linear peeling bound.
The API accepts odd moduli with `3 <= modulus < 2^63`; it does not establish
primality. Prime subgroup order is a caller obligation, and noninvertible
pivots return an error.

### MST merging already exists

[`large_prime_filter_mst.rs`](https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/large_prime_filter_mst.rs)
is exported through the preceding module as `filter_large_primes_mst`,
`MstAttempt`, `MstLimits`, and `MstStats`. It peels, chooses a minimum-degree
large-prime column, normalizes its incident rows, and uses Prim selection
over actual modular row differences. The primary edge weight is remaining
small-plus-large support; source support and endpoint indices break ties.
This is a local tree objective, not global minimum fill.

Limits cover row count, column degree, edge evaluations, persistent
coefficient count, and row/source support. Infeasible edges are omitted;
failure to complete the projection returns `relations: None` atomically.
That is a refusal requiring a charged fallback, not a partial relation set.
The same modulus obligation applies. Whole-filter cost also includes repeated
column-map reconstruction and retained witness materialization. The next
experiment should compare this existing implementation with the existing
pivot filter on identical streams, rather than reimplement MST merging.

### Structural DSU bound and symbolic graph adapter

[`jv_large_prime_rank_bound.rs`](https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/jv_large_prime_rank_bound.rs)
provides `StructuralRankBound::{new, add_row, snapshot}`. It accepts bounded,
canonical homogeneous rows with zero, one, or two distinct nonzero LP
coefficients. Its forest and singleton anchors give a lower bound on LP rank
and an upper bound on possible small-only rank. Weighted-cycle contributions
are intentionally ignored. Union-by-size is present; path compression and
coefficient potentials are not. Therefore this module is neither the exact
gain-rank algorithm nor an implementation of the catalog's inverse-Ackermann
bound. It is a reuse point for instrumentation and conservative screening.

[`gaudry_cubic_graph.rs`](https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/gaudry_cubic_graph.rs)
already contains the internal `GraphEliminator` adapter to
`pencil_symbolic_graph::SymbolicGraph`. It protects target column `small`
and a constant-one column, translates the RHS with the appropriate sign,
and returns a relation on `Outcome::Constraint`; joins/anchors return none.
It records graph work and rejects out-of-range columns. The imported
`SymbolicGraph` implementation was not among these six files, so this audit
does not establish its complete complexity or supported input contract.

### Two different Krylov uses

[`pq_wiedemann.rs`](https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/pq_wiedemann.rs)
exports scalar `berlekamp_massey` and `wiedemann_solve` over `BigUint`
residues. It uses direct sparse products for square systems and implicit
`M^T M` products for rectangular systems, then checks `Mx = b` on the original
matrix before returning a solution. Block Wiedemann appears in commentary,
not as an implementation in this file. Modular inversions can fail.
The full-column-rank justification in its normal-equations comment needs
qualification: over `F_5`, the nonzero column `(1,2)^T` has zero Gram matrix.
This direct calculation is unformalized; changing a projection seed does not
remove that structural degeneracy. Preserve original-system checks and
audit preconditioning before reusing this branch.

[`gaudry_cubic_krylov.rs`](https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/gaudry_cubic_krylov.rs)
implements internal sparse-FGLM shape extraction from a multiplication
matrix, with recurrence/Hankel checks and a failure signal for dense fallback.
Here coefficients use the polynomial base field `p`, not automatically the
subgroup-order field of relation linear algebra. It constructs sparse edges
from a dense matrix and stores Krylov frames. Its delayed-reduction fast path
requires normalized coefficients, dimension at most 64, and `p < 2^20`.
These costs and restrictions must not be replaced by a generic linear-memory
Wiedemann claim.

## Related work and retrieval limits

The pinned [`docs/ACTIVE.md`](https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/docs/ACTIVE.md)
and [2026-11-21 session log](https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/docs/session-logs/2026-11-21.md)
already track graph-IC and hyperelliptic large-prime/sparse-algebra work.
They establish lane context here; their timing headlines are not imported
as audited results.

As inspected on 2026-11-22, [PR #4905](https://github.com/isogeny-labs/crypto/pull/4905)
was draft, on `claude/ecbench-graph-theory-ic`, head
`0b92b96c572d0f8dade30bb7319c94ba9a63a0b6`, implementing a gain-forest
harvester and coset-filtered MITM integration. This catalog, tracked by
[PR #4917](https://github.com/isogeny-labs/crypto/pull/4917), does not replace
that implementation work. Low-stretch, matching/DM, width, partitioning,
and block-solver candidates require further targeted repository discovery;
these six files cannot establish repository-wide absence.

**Sources checked (KB):** KB unavailable. No research-KB MCP tools were
exposed; `scripts/research-kb status` returned 127 because the executable
was unavailable. `conductor check` also returned 127. The fallback was the
pinned source-file, active-item, session-log, and related-PR review above,
plus the catalog's primary literature. This is not an exhaustive KB review
or an absence proof.
