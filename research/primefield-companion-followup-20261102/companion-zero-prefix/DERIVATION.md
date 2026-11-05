# Proven-zero columns in paired transport

2026-11-02. Derived construction and static integration audit; numerical,
corruption, heap and performance trials: zero. Prime-field source scope unchanged.

## Invariant and proof

The selector queries columns in increasing order until the first nonzero
column j. All columns with original indices in [0,j) have been checked zero.
Its secondary-diagonal branch may select i>j instead. The new Selection stores
zero_prefix=j in every nonzero branch, and n only at the all-zero terminal.
It never infers a zero prefix from the selected index or query count.

If A e_t=0 and P is a transposition, then (P A P)e_{P(t)}=P A e_t=0.
Thus after each simultaneous swap the zero set is permuted by that same
transposition. This argument needs no symmetry; the unchanged paired generator
formula and pivot logic still require caller-established D A=A^T D.

When a requested front column is absent from the selected-column cache but
belongs to the proven-zero set, materialize a zero tuple instead of querying
it. Keep the exact cache key/vector transformation, generator formula, metric
transport, block inverse, pivot order, descent and root verification.
The zero set is local to one move; it is not propagated across a Schur step.

## Derived query schedule

For a nonidentity swap at input width w let h be the previous missing-column
count (0 or 1) and z the reused-zero count (0 or 1, z<=h).
Previous source queries: 2+h. Candidate: 2+h-z, all at the same input width w.
Transpose queries remain 1; forward queries become 1+h-z. Output width w+4,
outside-query products 3n+4 and additions 4n+1 are unchanged. Under the
preserved successful source-kernel schedule, removed instrumented native
products are z(4w+8). This is a derived count, not a runtime measurement.

For j>0 a scalar move saves exactly one query: front 0 was checked zero.
For a pair (j,i), weighted symmetry and prior zero columns imply i>j.
Both swaps are nonidentity. After the first, front 1 is either original 0
(if j=1) or original 1 (if j>1); it is zero in both cases. Thus the pair saves
two queries, at widths w and w+4, removing 8w+32 instrumented native products.
For j=0 the zero set is empty and no query is saved. Identity swaps stay skipped.
The all-zero selection has no pivot; move rejects it as before.

## Ownership and remaining native work

Selection remains an internal mutable producer record with source identity,
signature and one-use guard. An exact Selection type guard rejects the old
record; integer/range guards reject malformed prefix metadata. These guards
do not certify forged prefix values, mutated generators, concurrent mutation
or global weighted symmetry. Such inputs remain outside this internal API's
provenance contract, matching the prior cached-column design.

The set initially contains j indices, is permuted once per nonidentity swap,
and is discarded on return. Each missing cache entry incurs a membership test.
Each reused column allocates n zero entries; caches still retain at most
cut+swaps<=4 columns. Counters record retained indices, transported indices,
membership tests, reused columns and materialized entries. Set hashing,
allocation, canonical cache checks and vector permutations remain native work;
no heap or wall-time improvement is asserted.

## Integration and validation obligations

Three full AST comparisons against explicitly recorded substitutions preserve
the selector search, paired transport arithmetic and complete metric-view
driver. These are static checks, not execution. Load the sibling paths from
companion-metric-solver/DERIVATION.md plus companion-metric-views,
companion-paired-transport and this directory; import driver_zero.solve_or_witness.
Never overwrite the frozen references.

The funded validation panel must retain all upstream inputs, both outcomes,
root verification and separate feature-annihilation controls. Add j=0,1,>1;
first-diagonal, secondary-diagonal and pair branches; both swaps; nonconstant
metrics; cache hits; all-zero/no-pivot; wrong source/signature/consumed record;
malformed prefix metadata; exact generator, source query and counter comparison;
zero-set membership after each permutation; object lifetime and allocation.
Changing a plausible prefix in a forged internal record is not a supported
certificate-validation test: independent source-column replay is required
if this metadata is ever accepted from an untrusted boundary.

Next opportunity: a proved zero column in a weighted-symmetric source is also
a zero row. A solver could remove such coordinates, immediately produce a
normalized witness for nonzero RHS there, or retain free zero coordinates.
That larger change needs a displacement representation for coordinate deletion
and exact primal/dual reinsertion; it is not implemented or counted here.

## Sources reviewed

Local companion-metric-views/selection_view.py and driver_views.py;
companion-paired-transport/paired.py and DERIVATION.md. Established exact-field
scalar/pair pivot context: Dumas and Pernet (2018), introduction and Section 2,
https://arxiv.org/pdf/1802.10453. This cache construction claims neither their
rank-profile guarantee nor a new factorization theorem.
