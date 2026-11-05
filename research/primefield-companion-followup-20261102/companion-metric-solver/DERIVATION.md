# Complete conditional weighted-symmetry driver — 2026-11-02

Status: implemented route and derived identities/accounting; syntax and static
preservation checks. Numerical, mutation, heap and performance trials: zero.

## Domain and provenance

A is square with distinct companion wraps over a caller-certified prime p>3.
The caller supplies nonzero diagonal entries D and their inverses and establishes
DA=A^T D. The public weighted Gram compiler supplies this invariant through
A=D^{-1}JBJ and B=B^T. Checking inverse metric entries and reciprocal pivot
entries does not establish global symmetry for arbitrary generator inputs.

Dependencies remain unmodified. Put these sibling directories on the module
path: companion-metric-solver, companion-metric-pivot, companion-rank-solver,
companion-generator-compression, companion-recursive-inverse,
companion-pivot-transport, companion-schur-lift, companion-schur,
companion-blocks, companion-fused, structured-block-kernel. The imported
`solve` module must be companion-rank-solver/solve.py, `selection` and
`transport` must be the metric-pivot versions. No external solve oracle,
field extension, or unknown scalar label is used.

## Complete descent and unwind

`driver.solve_or_witness(source,rhs,metric,inverse_metric)` validates canonical
inputs, copies the root metric into two immutable tuples, then iterates:

1. Compress generators exactly. Distinct wraps imply zero displacement means
   zero source. With zero RHS choose zero solution; otherwise choose the
   normalized coordinate left witness e_i/b_i. This branch matches the prior
   solver's AST exactly.
2. Select a nonzero principal scalar or antidiagonal 2x2 pivot and move it by
   simultaneous permutations using the published metric-pivot primitives.
   Permute the RHS and metric labels consistently. A zero-column selection
   after nonzero displacement is an explicit contradiction error.
3. For a full-size pivot, apply its explicit inverse and undo this level's
   permutation. Otherwise generate the Schur source, reduce RHS and store a
   compact block frame; the next metric is the trailing permuted metric.
4. Unwind in reverse. Every frame restores its original local coordinate order.
   The final original-system solution or normalized-left-witness check is
   identical to the prior solver's AST.

Nonzero nodes reduce dimension by cut k in {1,2}. With the primitive
correctness and weighted-symmetry assumptions, a nonzero source always has
such a pivot. Compression preserves the matrix, simultaneous permutations
preserve weighted symmetry, and a principal Schur complement inherits its
trailing metric. Thus the route reaches a full-size inverse or zero terminal.
The sum of pivot sizes is the derived rank; no independent rank check is
claimed. Feature annihilation remains a separate extractor requirement.

## Shared block borders during setup

Write the pivoted source as A=(E,F;C,H), with cut k and q=n-k. The selected
columns already supply the leading n-by-k block. Weighted symmetry gives
row j at coordinate i as d_j^{-1}d_i A_ij. `BlockCache` reconstructs these
k full rows using the published helper, paying n+2nk field products including
metric validation, with no source query.

For an argument supported in the first k coordinates, `_source` computes
its product from cached columns, or cached rows in the transpose direction.
Every call charges nk products and nk additions, even for zero coefficients.
Other arguments delegate to the unchanged source. The inherited Schur builder
has 2w+4 supported calls (2w generator loop, two border terms, two inner Schur
corrections), and RHS reduction adds one. Its two outer boundary calls have
nonzero trailing support and therefore remain full source applications.
The cache is released before descent. Use after release raises; the driver
never relies on its post-release operator behavior.

## Compact weighted block lifts

The frame retains only C as k column tails, b_head, the explicit small inverse,
references to the two root metric tuples, metric labels and local permutation,
shared meter, profile and consumed flag. No source, Schur object, full RHS,
reconstructed F or leading full columns are retained by frames.

For S=H-CE^{-1}F and h=b_tail-CE^{-1}b_head:

    solution: x_head=E^{-1}(b_head-F z),       S z=h;
    witness:  y_head=-E^{-T} C^T v,          S^T v=0, v^T h=1.

The solution lift evaluates Fz=D1^{-1}C^T(D2 z), reusing C. The witness lift
uses C^T v directly. In both cases the existing block identities establish
A x=b or A^T y=0 and y^T b=1. Both vectors undo the same simultaneous
permutation. Unlike a transposition, a composed permutation can have order
higher than two; `undo_vector` explicitly inverts the full order mapping.

A frame reserves one-use ownership before arithmetic. Failed arithmetic leaves
it consumed; successful use releases field data, metric references and index
tuples. Profiles are retained by reference in the trace and updated on unwind.
They contain no source or metric data arrays. Metric labels refer to immutable
root entries, so inherited metrics need not stay geometric after permutations.

## Derived arithmetic and storage

All counts below exclude pivot search/transport, compression, original-system
verification and common inherited Schur arithmetic unless expressly included.
Let n>k, q=n-k and w be the transported generator width.

| Phase | Full source applications | Additional field products | Additional field additions |
|---|---:|---:|---:|
| Construct cached rows, including validation | 0 | n+2nk | 0 |
| Supported setup + RHS calls | 0 | nk(2w+5) | nk(2w+5) |
| Outer Schur boundary calls | 2 | source kernel work | source kernel work |
| Primal lift, including small inverse | 0 | kq+q+2k | kq+k |
| Dual lift, including small inverse | 0 | kq+k | kq+k |

The unmodified general SourceSchur generator/RHS/one-lift route makes 2w+8
source calls over these phases. The new route makes two. This is a schedule
comparison for the same n,k,w block, not a full-run comparison to the scalar
route, whose pivot sequence can differ. For the fused source the two calls
record 2(4w+8) native products. Native and scalar field products are not
interchangeable cost units. Zero lift inputs still pay the listed arithmetic.

Root metric validation costs N products. Every nonzero selector also validates
n inverse pairs, and its offdiagonal branch adds two reciprocal-check products.
Selection and simultaneous transports retain their previously derived costs,
including different intermediate widths. Small inverse setup costs k inversions.
Constructing the root geometric metric, if not already available, is separate.

Each live frame stores kq+2k field slots: C, k RHS entries and k inverse
coefficients. The shared root metric/inverse costs 2N field slots once. Each
frame also stores two length-n integer tuples (labels and order), besides
profile/scalar/object metadata. For all-scalar nonterminal frames, summed
frame field slots are (N^2+3N-4)/2; the extra root metric contributes 2N.
The current setup cache transiently retains 2nk border values; copied frame
C coexists with it during capture. Local gathered metric tuples, source
operators, compression certificates, root source, meter blocks and trace
history remain additional memory. No total memory or measured peak bound is
inferred from the frame-slot formula.

## Static checks and remaining validation

`check_preparation.py` compares the complete zero-terminal branch and final
root-equation/normalization branch ASTs to the preserved rank solver, parses
all Python files, records dependency hashes, and inspects frame slots. It
executes no arithmetic producer. Its checks do not establish new driver,
cache or lift correctness and are not numerical validation.

Full funded validation must use the frozen upstream cohort plus every scalar
selection branch, antidiagonal pair, full-size pair/scalar, zero source,
singular compatible/incompatible RHS, nontrivial composed permutations,
permuted nonconstant metrics, both lift directions, zero/nonzero tails,
corrupted metric/borders/inverse/counters, source identity and frame lifetime.
Independently compare reconstructed matrices, Schur metrics, solution/witness
equations and native/field ledgers; measure heap aliases and bytes separately.
The existing execution-cost gate remains in force; no numerical run is launched.

## Literature and next technical step

[Dumas and Pernet (2018), introduction and Section 2](https://arxiv.org/pdf/1802.10453)
provides established exact-field symmetric block-pivot context. This local
route uses its elementary 1x1/2x2 principle, not its rank-profile strategy or
fast recursive bound. The companion/Schur machinery is the preserved
[Bostan et al. (2017)](https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf)
context. No new symmetric factorization theorem is claimed.

Next remove redundant inverse-metric validation through a provenance-bound
metric object, and use the metric to derive paired row/column transport that
reuses queried columns. Price those savings against generator-width growth.
The full driver is now supplied; numerical acceptance and public-feature
extraction conditions remain their own gates.
