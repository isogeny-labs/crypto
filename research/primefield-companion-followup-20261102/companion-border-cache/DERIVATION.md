# Leading-border reuse in scalar Schur reduction

2026-11-02. Derived construction and static source checks. Numerical trials 0.

## Purpose and source review

At a nonterminal scalar-pivot node, the existing generator routine repeatedly
applies the same source to scalar multiples of e_0. The transported pivot
column is already known, and one transpose query supplies the first row.
This version replaces those applications by explicit scalar-vector products.
Unlike zero-prefix column transport, it also applies to nonsingular nodes.

Reviewed local Schur, primal/dual lift, recursive Operator/SourceSchur and
cached-pivot source, including the complete fused multiplication routine.
Primary literature: Bostan et al. (2017), Section 6.2 and Figure 4, supplies
the established structured Schur/inverse context:
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf
This cache specialization is derived from the local scalar-cut schedule;
it does not import the paper's fast matrix multiplication or probabilistic
regularization claims. Source elliptic scope remains prime fields with the
original method exclusions.

## Correctness and internal provenance

Write B=P A Q for the pivoted source. The search provides c_j=A e_j and
Q e_0=e_j, so B e_0=P c_j. The caller obtains this first column by the
already recorded row permutation, with no new source query. One explicit
query computes B^T e_0. Store these two length-n borders, c and r.

For every scalar alpha in the same field,

    B(alpha e_0)=alpha c,    B^T(alpha e_0)=alpha r.

BorderSchur specializes cut=1. It scans the tail of each source argument.
If the tail is zero while borders are retained, it returns a length-n scaling
of the appropriate border, charging every product even when alpha is zero
or one. Otherwise it delegates to the unchanged source operator. Consequently
all inherited Schur generators, RHS reductions and residual identities remain
the same, assuming the internal cached-column provenance and source correctness.

The constructor checks canonical border entries, nonzero leading pivot, and
agreement of the two corner entries. A matching corner is not a proof of the
entire supplied column. Its validity comes from the immediate search and
permutation path; it is not a public certificate. Source generators must remain
unchanged throughout the object's lifetime. Existing root equation checks and
the separate feature-annihilation condition remain unchanged.

## Cache lifetime

After generating the Schur pair and reducing the RHS, the integration calls
release_borders before descending. It drops both tuples and records zero
retained border entries. The stack keeps the source/Schur frame, but not these
border vectors. On unwinding, lift_primal and lift_dual query [0;z] or [0;v].
If that input is zero, the released object returns the known zero product
without field multiplications. If it is nonzero, it delegates. For completeness,
any later nonzero alpha e_0 request also delegates after release, so releasing
does not invalidate the operator interface.

Only the current node retains its two borders: their contribution is O(N)
peak field entries, not a sum over every frame. Other frame generators,
compression certificates and cached-column audit receipts can still accumulate
quadratic or larger storage. Profile peak counts exclude temporary query/list
copies; no total peak-memory claim is made.

## Derived per-node schedule

Let n>=2 and w be the pivoted source generator width. Define ell=1 if the
eventual lift input is nonzero, else ell=0. Normalized witness lifts always
have ell=1. The table excludes pivot search/transports, compression, child work,
root verification, and scalar/Schur arithmetic common to both versions.

| Phase | Original full source applications | Cached full source applications | New cache field products |
|---|---:|---:|---:|
| Build first row | 0 | 1 | 0 |
| Schur generators | 2w+6 | 2 | n(2w+4) |
| Reduce RHS | 1 | 0 | n |
| Unwind one lift | 1 | ell | 0 |
| Total | 2w+8 | 3+ell | n(2w+5) |

In generator setup, the loop contributes 2w scalar-e_0 requests, the two
boundary solves contribute two, and the inner corrections in the two Schur
applications contribute two. Only the two outer boundary-vector queries
remain full applications. Existing scalar solves are still charged normally.
Zero-source and n=1 terminal branches do not construct BorderSchur.

Support scanning counts are recorded separately. Full applications are counted
in source_forward_calls/source_transpose_calls; semantic requests, hits and
post-release zero results have separate counters. The object profile's
build_queries is a subset of transpose_queries, not another query to add.
The trace retains the live profile dictionary so later unwinding updates are
included in the final returned record. Intermediate snapshots must not be
mistaken for final counts.

## Connection to the fused native-product schedule

The preserved fused apply implementation performs seven instrumented bound
products, four instrumented native products per generator column (two
convolutions and two folds), and one final scale product: 4w+8 native products
per successful full application. check_kernel_schedule.py checks these syntax
counts and binds the source hash without importing or executing the kernel.
Successful-path interpretation also uses the inspected source structure.

For the table's phases, the derived instrumented native-product counts are

    original: (2w+8)(4w+8)
    cached:   (3+ell)(4w+8).

The difference is (2w+5-ell)(4w+8) instrumented native products, accompanied
by n(2w+5) new field products. These units must not be added or exchanged at
unit price. Removed source applications also each include one field inversion,
one field subtraction and n coefficient reductions. Bit-length volumes and
elapsed time depend on operands and remain unmeasured. This is a schedule
derivation, not a measured total-cost or runtime result.

## Static checks and validation next

solve_border.py is a separately named version of solve_cached.py. A normalized
AST audit permits only the BorderSchur import, construction from the permuted
search column, one profile trace record and release after RHS reduction.
Every other executable statement matches the reference. Prior source and
frozen numerical callers remain unchanged.

Future full validation must retain the original cohort and cover all source
widths, nonzero/zero scalar requests, both directions, n=2 boundary cases,
zero lift inputs, normalized witnesses, wrong columns/corners, reuse after
release, double-release rejection, root equations, feature-annihilation where
required, exact per-node ledgers, aggregate storage and operand bit-lengths.
Compare inherited generator outputs and solves with the unchanged reference.
No numerical execution or performance measurement occurred under the cost gate.
