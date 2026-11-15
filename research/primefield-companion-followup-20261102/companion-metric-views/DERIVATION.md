# Immutable inverse-metric views — 2026-11-02

Status: derived validation reuse, implemented immutable root/views and separate
complete-driver integration; static checks only. Numerical trials: zero.

## Preserved mathematical scope

The existing public feature compiler supplies DA=A^T D. Its symmetric principal
pivot/Schur route inherits permuted and restricted diagonal metrics. This
component establishes only the coordinate-wise relation d_i*di_i=1 and its
preservation under index selection. It does not establish weighted symmetry
for an arbitrary source, certify source generators, or change the pivot rule.

The unchanged exact-field scalar/pair elimination context is described in
[Dumas and Pernet (2018)](https://arxiv.org/pdf/1802.10453). The local references
are the metric-pivot selector and row reconstruction, metric-solver block
cache, and complete paired-transport driver. No new factorization theorem or
numerical stability claim is introduced by this provenance representation.

## Root and view invariant

`MetricRoot(d,di,meter)` copies both arrays to tuples and runs the existing
canonical/nonzero/inverse validation once. It captures the coefficient prime
and meter identity. The frozen dataclass prevents ordinary attribute writes;
its stored metric entries and labels are immutable integers/tuples.

`MetricView(root,labels)` requires a nonempty distinct sequence of canonical
in-range integer indices. For every label l, root.d[l]*root.di[l]=1 already
holds. A permutation selects the same labels in a checked new order; a
nonempty tail discards a prefix. Neither operation changes any field entry,
so both preserve the inverse relation without field multiplication.

`require_view` checks exact view/root types, the same meter and captured prime,
dimensions, integer types and equality of every supplied d/di pair to the
indexed root values. This prevents an accidentally permuted, stale, altered
or cross-meter metric from silently bypassing inverse checks. A different
validated root with identical appropriate values is mathematically equivalent;
this is not an attestation of a unique source. Source symmetry remains a
caller-established invariant. Deliberate reflection/object mutation, forged
Python instances and concurrent meter mutation are outside the internal
provenance contract; frozen Python objects are not public certificates.

## Integration and exact preservation

Prepend companion-metric-views and companion-paired-transport to the sibling
module paths listed in companion-metric-solver/DERIVATION.md, then import
`solve_or_witness` from `driver_views`. Keep `transport` bound to the metric-pivot
module and `block_frame` to the metric-solver module; frozen modules are reused.

The driver retains its original root metric arrays, labels, gathered tuples,
selection, paired transport, block equations and root verification. It now
constructs one validated root and carries a current view in parallel with
its existing labels. After a simultaneous pivot permutation it permutes the
view; after nonterminal Schur descent it takes the corresponding tail.

The selector replaces only its inverse validation with `require_view`, taking
an extra view argument. Row reconstruction makes the same replacement and
performs all the same 2*n*cut reconstruction products. The block-cache profile
therefore records 2*n*cut row-setup products instead of n+2*n*cut. Its source
applications, arithmetic and release behavior are unchanged. The compact
block frames and paired transport remain the original implementations.

`integration.json` enumerates every declared substitution against four
preserved source fragments: selector, row helper, block cache and full driver.
The audit reconstructs each candidate using those substitutions and compares
complete ASTs. It also confirms exactly one root-validation call site and no
field multiplication, inversion or source-application call sites in view
construction/derivation/checking. This is a static audit, not execution.

## Derived accounting

Let N be root dimension, S the nonzero nodes that call the selector, and C the
nonterminal nodes that construct a row cache. Let n_v be a node's dimension.

    Previous inverse-check products = N + sum_{v in S} n_v + sum_{v in C} n_v
    New inverse-check products      = N
    Products removed                = sum_{v in S} n_v + sum_{v in C} n_v.

No scalar inversion, source query, row-reconstruction product, transport
product, pivot choice or lift product is removed by this change. For full
scalar descent S={N,...,1}, C={N,...,2}, so the removed count is N(N+1)-1,
including N=1. For full pair-only descent with even N, it is N(N+2)/2-2.
For an initially zero source there are no selector/cache calls and no removed
products. Singular cases use their actual node sums; do not substitute a
full-rank formula. These are derived schedules, not measured counts.

Each reused check still tests n coordinate pairs, consisting of canonical
integer checks, root-index reads and equality comparisons. Native counters
record coordinate-pair checks, label tests, permutation-index tests and roots/
views created. `metric_inverse_products_reused` is incremented only after
successful full coordinate matching. Set construction for uniqueness, tuple
copying, hashing, allocation and ordinary Python overhead are additional host
work; counters are not complete CPU instruction or bit-cost measurements.

Root storage remains 2N metric field entries. The driver still gathers current
d/di tuples as before. Views hold a reference to the same root plus an index
tuple, not a parent-view chain or duplicate field arrays. The current view can
add O(n) index slots while old/new views coexist during derivation. Frames do
not retain views; their previous metric/label storage is unchanged. Certificates,
trace, source operators and meter history remain separate memory costs.

## Validation obligation and next work

All numerical, corruption-control, heap and performance trials remain zero
under the existing execution-cost gate. Preserve the full upstream cohort and
same pivot sequences. Include root-zero, scalar and pair terminals; singular
compatible/incompatible RHS; all permutations and nonconstant metrics; malformed
root inverse pairs; changed metric values; wrong-order views; duplicate/out-of-
range/bool indices; wrong meter or changed prime; attempts at ordinary frozen
attribute writes; full root solution/witness equations; counters and heap aliasing.
Compare the actual removed product count with the node-sum formula, independently
of the new reuse counter. The global weighted-symmetry precondition is unchanged.

Next expose selector-established zero-prefix columns with explicit provenance,
so paired transport can avoid querying columns already proved zero. Derive
how that prefix transports through simultaneous permutations before changing
query counts. No such saving is included in this component.
