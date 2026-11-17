# Reuse the first nonzero column in pivot transport

2026-11-02. Derived implementation and static source checks; numerical trials 0.

## Scope and prior work

The explicit scalar-pivot solver queries columns in increasing order until it
finds a nonzero entry. Its general column-transport primitive then makes two
additional source queries even though the pivot column and zero prefix have
already been established. This version retains that search information.

Read `companion-rank-solver/solve.py` and DERIVATION.md, the existing pivot and
companion displacement formulas, and Bostan et al. (2017), Section 6:
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf
The literature supplies structured-solve context. The calculation here is a
direct specialization of our existing column-swap identity, not a new rank
theory or an imported fast-complexity claim. Field and source elliptic scope
remain prime fields, with the original exclusions. No frozen source is edited.

## Identity

Let c_j=A e_j be the first nonzero column, j>0. The completed search establishes
c_0=...=c_(j-1)=0. Put u=e_0-e_j and Q=I-uu^T. The original generators are

    Gc=[G | A u | -A Z_b u], Kc=[Q K | Z_b^T u | u].

The cached first correction is A u=-c_j. Since n>=2,

    -A Z_b u = c_(j+1)-c_1       if j<n-1,
               -c_1+b c_0       if j=n-1.

If j=1, c_1 is the cached pivot column. If j>1, c_1=0 is already established.
Also c_0=0. Thus an interior pivot needs only c_(j+1), obtained with one new
unit-column query. A last-column pivot needs no new query. For n=2,j=1 the
last correction equals -c_j and reuses the first correction. For n>2,j=n-1
the last correction is zero. The implementation retains both correction
columns to preserve exact generator width and downstream schedule structure.

Z_b^T u has -1 at index j-1 and b at index n-1, with zeros elsewhere.
These indices are distinct. Assigning the already canonical residue b does
not require multiplication by one. The original positive-width generator
representation and wraps are otherwise unchanged.

## Derived work at the transport call

Search itself uses the identical j+1 source queries and entry-test order.
The following table excludes source-query internals and counts scalar work
using the existing Meter's convention:

| Pivot column | Old extra queries | New extra queries | Old products | New products | Old add/sub | New add/sub |
|---|---:|---:|---:|---:|---:|---:|
| j=0 | 0 | 0 | 0 | 0 | 0 | 0 |
| j=1<n-1 | 2 | 1 | 2 | 0 | n+1 | 2n+1 |
| 1<j<n-1 | 2 | 1 | 2 | 0 | n+1 | n+1 |
| j=n-1>0 | 2 | 0 | 2 | 0 | n+1 | n+1 |

The new n+1 includes constructing -1 and negating c_j. For j=1 interior,
forming c_(j+1)-c_j adds n subtractions. The n=2 last-column case reuses
the already negated vector. Copies, cache checks and allocations are native
work, not field operations. The cache retains one length-n column; it does
not retain every preceding zero column. Receipts retain the cached column,
so audit storage is additional to temporary cache storage.

This is an exact query-count schedule, not a timing result. The removed
queries used different input vectors, so bit-length volumes and runtime
must be measured or separately accounted; fewer calls alone is not a total
cost claim. Nonsingular nodes always have a nonzero first column and take
the unchanged j=0 branch. The optimization targets singular intermediate
systems with a zero prefix, relevant to the witness route.

## Cache contract and integration

`find_pivot` creates an internal one-use SearchCache tied by object identity
and dimension/wrap/prime signature to the source it queried. The cached column
is a tuple. `swap_from_search` refuses a different source, changed signature,
reuse, or malformed index/column. The source generators must not be mutated
between search and immediate consumption. Identity/signature checks do not
detect in-place generator mutation, and a caller can forge a Python object;
the cache is not an independently verified public certificate or security
boundary. Correctness relies on the internal search provenance and the source
operator's correctness. Future controls must test this contract explicitly.

`solve_cached.py` is a separately named copy of the original solver with one
additional import and exactly two changed call sites: search returns the cache,
and transport consumes it. A normalized AST comparison confirms every other
executable statement is unchanged. The original search/transport functions
remain in the copy as unused references; the live calls use the new module.
The original source file remains byte-preserved. Zero-source handling, scalar
inverses, row swaps, Schur construction, lifts and root checks are unchanged.

## Validation next

No numerical or performance trial has run. Preserve the complete original
solver cohort. Add all branch boundaries, n=2,j=1, j=1 interior, later interior,
last-column prefixes, nonzero wraps and zero wrap, all pivot rows, cache
reuse/wrong-source/malformed mutations, both solution/witness outcomes and
exact source/scalar counts. Compare generated pairs against the original
two-query transport and independently check the represented matrices. Include
unchanged j=0 controls and separately charge cache/audit storage. Do not omit
singular or failed cases. Execution stays behind the recorded cost gate.
