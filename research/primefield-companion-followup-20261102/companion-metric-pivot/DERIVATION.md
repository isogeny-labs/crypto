# Weighted-symmetry pivots for the public feature compiler — 2026-11-02

Status: derived algebra and implemented primitives; numerical trials zero.
No complete symmetric solver driver or accepted performance result is supplied.

## Actual source identity

The public constructor in `direct-cyclic-companion/generators.py` represents
A=D^{-1} J B J, where B is the weighted feature Gram matrix in its derivation,
J reverses coordinates, and D=diag(1,gamma,...,gamma^{r-1}). The source already
returns `inverse_diagonal`, built with one inverse of gamma and successive
multiplications. Since B=B^T, DA=JBJ is symmetric. The weights need not have
any positivity property in F_r. Prime coefficient field r>3 and gamma not
0 or1 remain the original domain; there is no field extension.

This identity is inherited from the constructor proof, not from inspecting a
few source entries. `validate_metric` verifies inverse entries only. A caller
providing arbitrary G,K must independently establish DA=A^T D.

## Pivot selection with at most one extra column query

Scan source columns until the first nonzero column j. If A_jj!=0, choose the
scalar principal pivot j. Otherwise select an i with A_ij!=0 and query column
i. Weighted symmetry gives d_i A_ij=d_j A_ji, so A_ji!=0. Check this local
identity (two charged field products). If A_ii!=0 choose scalar pivot i;
otherwise choose principal block on ordered indices (j,i):

    E = [ 0  u ]     u=A_ji !=0,
        [ v  0 ]     v=A_ij !=0.

Its determinant is -uv!=0 and E^{-1}=[[0,v^{-1}],[u^{-1},0]]. No square root
or positive-definite assumption is needed. Its transpose inverse exchanges
u^{-1} and v^{-1}. If all queried columns are zero, the source is exactly
zero. That terminal observation alone says nothing about RHS compatibility.

The local reciprocal check does not certify global symmetry. Selection caches
are internal, source-identity/signature bound and one-use at transport;
in-place generator mutation or forged selections are outside the contract.
Invalid source output or the checked reciprocal identity raises explicitly.

## Simultaneous permutation and Schur closure

Move each chosen index into its leading position using the existing general
row transport followed by the general column transport for the same swap.
The final source is A'=P A P^T. Its metric is D'=P D P^T, and
D'A'=P DA P^T is symmetric. Intermediate row-only sources need not be
weighted symmetric; no symmetry-dependent operation is applied to them.

Returned `order` satisfies A'_{ij}=A_{order[i],order[j]}. Use the same order
on the RHS; invert it on both solutions and left witnesses. The cached selected
columns are reordered without new arithmetic or source queries. They give E
and all leading column borders in final coordinates.

Partition the final source A'=(E,F;C,H) and metric D'=(D1,0;0,D2).
D1 E=E^T D1 and D1 F=C^T D2. The Schur complement S=H-C E^{-1}F obeys

    D2 S = D2 H - F^T D1 E^{-1} F.

D1 E^{-1} is symmetric, since E^T D1=D1 E. Thus D2 S is symmetric. The child
metric is just the trailing portion of the permuted metric, not a fresh
geometric progression. The inherited inverse entries are sliced identically.
Consequently the 1x1/2x2 selection can be repeated on nonzero Schur sources;
rank(A')=cut+rank(S). This is a derived closure/completeness argument, not an
executed driver, independent rank certificate or feature-annihilation proof.

## Weighted row reconstruction

For a known leading column c_j=A'e_j, row j has entry

    A'_{ji} = inverse_metric[j] * metric[i] * c_j[i].

`transpose_from_columns` reconstructs all cut leading rows with no source
query. It allocates them explicitly. The helper validates the metric first,
then charges 2*n*cut products, including zero/one factors. It supplies a
concrete replacement for querying each leading transpose basis vector.
Total helper arithmetic is n+2*n*cut products because inverse validation is
charged. This implementation does not yet provide the shared metric ownership
needed to claim a whole-driver memory reduction.

## Implemented interfaces and exact schedules

- `metric.py`: geometric metric and inverse, inverse validation, leading-row
  reconstruction. Building length n uses one inversion and 2(n-1) products.
- `selection.py`: deterministic first-nonzero-column search; scalar or
  antidiagonal pair; all-zero terminal. Selection first validates n inverse
  entries (n products). Search needs j+1 queries for the first-diagonal branch,
  j+2 otherwise, and n for zero. A valid weighted-symmetric zero prefix implies
  i>j in the offdiagonal branch, so its j+2 is also at most n. Entry scans and
  diagonal tests are separate counters. Offdiagonal branches add two products
  for their local consistency guard, regardless of final cut.
- `transport.py`: at most cut nonidentity simultaneous swaps. Each uses four
  existing full source queries (two transpose then two forward), width +4,
  four field products and 2(n+1) additions outside queries. Thus at most eight
  queries and width +8 for cut2. The intermediate widths differ and must be
  priced separately. No cached transport saving is assumed. SmallInverse
  construction uses cut inversions; each application uses cut products and
  no additions. Inversion is never delegated to a matrix solve oracle.
- `check_preparation.py`: syntax and dependency hashes only. Confirms the
  preserved constructor exposes its inverse metric; it does not prove the
  algebra or run the new code.

The transport result is directly shaped for the existing
`SourceSchur(result['source'], result['cut'], result['inverse'])`. If cut equals
n, apply the small inverse directly; otherwise use the inherited generator
builder and pass the trailing metric to the child. Existing general Schur
calls are not automatically redirected through the new reconstructed rows.
A complete driver, cached block-query path, compact block lifts, metric storage
ownership and full trial cohort are next implementation work.

## Prior work and scope

[Dumas and Pernet, *Symmetric indefinite triangular factorization revealing
the rank profile matrix* (2018), introduction and Section 2](https://arxiv.org/pdf/1802.10453)
explicitly discusses exact finite-field symmetry and 1x1/2x2 pivot blocks.
The local construction specializes that established pivot principle to this
compiler's diagonal symmetrizer and existing companion-generator transports.
It does not inherit their rank-profile-revealing property or fast recursive
complexity. [Bostan et al. (2017)](https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf)
is the preserved companion/Schur context. Numerical stability thresholds from
real LAPACK routines are irrelevant to this exact-field selection.

## Required validation and next step

Freeze and run the full existing upstream cohort plus: both scalar-selection
branches; zero-diagonal pair; zero/singular sources; all simultaneous swaps;
full-size2 terminal; scalar terminal; nonconstant and permuted metrics;
metric/reciprocal corruption; forward/transpose small inverses; independently
recomputed field/query/width counts; source matrices and inherited Schur
metrics. Check feature exclusion separately when required by the extractor.
No test or reduced sample was launched under the existing cost gate.

Next implement the complete symmetry-preserving driver and shared weighted
block borders. It must retain the full singular solve/witness behavior of the
reference route, use explicit 1x1/2x2 inverses, avoid treating diagonal zeros
as rank failure, and price metric metadata and all transport queries.

Packaging: the public constructor is included byte-for-byte under dependencies/
so the syntax/dependency audit runs in the published follow-up snapshot. The
audit uses the original sibling when present and otherwise this exact copy.
