# Exact two-sided generator compression

2026-11-02. Derived identities and implementation; syntax validation only.
Numerical trials, corruption controls and performance measurements: zero.

## Purpose and prior work

The pivot primitive adds two columns and a scalar Schur step can add another
two. The balanced inverse construction likewise carries boundary columns.
This component removes algebraic redundancy before subsequent generated
operator applications. It preserves the same field and displacement operator;
it does not choose pivots in the underlying matrix or resolve singular solves.

Read local `companion-pivot-transport/DERIVATION.md`,
`companion-recursive-inverse/DERIVATION.md`, `companion-blocks/SCHUR_NEXT.md`
and the existing `structured-block-kernel/kernel.py` Meter implementation.
Primary literature checked: Bostan, Jeannerod, Mouilleron and Schost (2017),
*On Matrices with Displacement Structure: Generalized Operators and Faster
Algorithms*, DOI 10.1137/16M1062855, introduction, generator-compression passage
on printed page 756, and Section 6.
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf

Generator compression and minimal displacement rank are established methods.
This deterministic classical implementation supplies explicit certificates and
operation accounting for our conventions. It does not implement the paper's
fast matrix multiplication bound or its randomized regularization procedure.
The source elliptic scope remains prime fields with the original exclusions.

## Construction and proof

Let G,K be n-by-s matrices over a caller-certified prime field F_r, r>3,
n>0. Width s=0 is allowed. All entries are canonical residues. Define D=GK^T,
but do not form D.

1. Compute a full-rank factorization G=UR. U is n-by-g, R is g-by-s,
   and g=rank(G).
2. Form H=KR^T, of size n-by-g.
3. Compute a full-rank factorization H=VS. V is n-by-t, S is t-by-g,
   and t=rank(H).
4. Return G'=US^T and K'=V, both n-by-t.

Then G'K'^T=US^TV^T=UH^T=URK^T=GK^T. Since U is injective on F_r^g,
rank(UH^T)=rank(H^T)=t. Consequently t is the least possible generator
width: any n-by-w pair has product rank at most w. This detects product
cancellation, not just dependent columns on one side. The proof also covers
t=0 with the usual empty-product conventions.

Applied to Z_a A-A Z_b=GK^T, the same identity and wraps are preserved.
For square equal-size cyclic shifts with a!=b, the two characteristic
polynomials X^n-a and X^n-b are coprime, so the Sylvester operator is invertible
and the represented A is unchanged. Compression itself does not need this
invertibility assumption to preserve D. It never changes the base field.

## Deterministic factorization and certificate

Scan original columns in order. Reduce each against existing normalized basis
columns. Each basis column has value one at its pivot row and zero at all
earlier pivot rows. Subtract its current pivot-row coefficient times that
basis column. For a nonzero residual, choose its first nonzero entry as pivot,
normalize it, and append it to the basis. Retain all reduction coefficients
and the nonzero normalization coefficient in the corresponding coordinate row.

The invariant after each processed column is A_processed=U R_processed.
Selected basis rows, in pivot-discovery order, form a unit lower-triangular
minor. Selected original columns of R form an upper-triangular minor with
nonzero diagonal. Thus both factors have rank equal to the number of pivots.
Pivot rows need not be sorted by their numeric index.

The independent `certificate.py` imports no producer or Meter code. It checks
canonical shapes, both factor identities, both pairs of triangular minors,
H=KR^T and the final output identities using skinny products. These checks
prove preservation and minimal width under the prime-field precondition.
It does not trust the producer's rank integer alone, and does not construct
the n-by-n displacement. Checker work and storage must be charged separately.

No primality test is included: a composite modulus is outside the contract.
Zero divisors invalidate the rank proof. Width-zero output is supported by
the compressor and certificate, but existing kernels require positive width.
`nonempty_generators` explicitly encodes verified zero-rank output by one zero
column on each side. Mathematical rank remains zero; encoding width is one.
No frozen caller has been modified to insert this adapter automatically.

## Derived exact arithmetic schedule

Let q_G be the sum of current ranks before processing each of G's s columns,
and q_H the analogous sum for H's g columns. Equivalently, for each factor,
q=sum(width-1-j) over original pivot-column indices j.

The implementation performs every elimination update even when its coefficient
is zero, and the Meter dot product charges its initial-zero addition. Therefore
the producer's derived totals are:

| Counted unit | Exact expression |
|---|---|
| Field products | n(q_G+g+q_H+t+sg+gt) |
| Field additions/subtractions | n(q_G+q_H+sg+gt) |
| Field inversions | g+t |
| Source operator queries | 0 |

Pivot entry tests are counted separately in `compression_pivot_entry_tests`;
at most n(s+g) are made. Canonical-input validation, allocations, copies, list
operations and certificate storage are not field arithmetic. This is an
O(ns^2) classical field-operation bound, not the faster literature bound.
Retained certificates include U,R,H,V,S and outputs, so output size alone is
not total memory. When s approaches n, skinny factors themselves can be dense.

The checker recomputes the arithmetic schedule from certificate pivot columns,
but deliberately reports `producer_runtime_counts_verified=false`: equality
to actual Meter invocation counts needs the future controlled validation.

## Use decision and remaining dependency

If C is compression setup cost and K(n,w) is the cost of a generated query in
the same counted unit, q future queries benefit only when
C+q K(n,t)<q K(n,s). A smaller rank alone does not establish a net saving.
No bound t=O(1), no balanced-pivot guarantee and no measured speedup follows.
If both factors have full column rank s<=n, their product already has rank s
and no exact compression is possible. Genuine displacement-rank growth survives.

Next prepare a frozen full validation contract covering ranks zero through
min(n,s), s>n, cancellation between nonzero factors, nonmonotone pivot rows,
both query directions after reconstruction, legacy zero encoding and mutations
of every identity/minor/rank field. Include dense references, unchanged-width
controls and exact producer/checker costs. Execution remains subject to the
existing cost gate; no new trial or kernel invocation was run here.

See `reports/design/report.pdf` and its editable diagram/renderer. Historical
reports, frozen sources, protocols and numerical acceptance gates are unchanged.
