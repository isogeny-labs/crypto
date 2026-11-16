# Structured solve-or-witness with explicit scalar pivots

2026-11-02. Derived implementation, syntax checked only. Numerical trials: zero.

## Connection to the extractor

The Schur lift already transports a reduced normalized left-null witness to
the original public linear system, but it requires a leading solve. The
balanced recursive inverse supplies that solve only when its fixed pivot tree
succeeds. This component implements a separate deterministic route covering
singular and inconsistent square systems without a supplied solve oracle.
It is not inserted into either frozen numerical program.

Read local recursive-inverse, pivot-transport, generator-compression and
Schur-lift derivations and source. Prior work: Bostan et al. (2017), Section 6,
especially the rank-profile and solve discussion on printed pp. 764–765:
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf
Structured elimination, rank profiles and solve/inconsistency decisions are
established techniques. This deterministic scalar path does not inherit that
paper's randomized regularization probability or fast complexity bound.

The source is Z_a A-A Z_b=GK^T over a caller-certified prime F_r, r>3,
square dimension n>0 and a!=b. Entries and RHS are canonical residues.
The source elliptic scope remains prime fields, with the original exclusions.
No hidden scalar, elliptic collision table or extension-field substitution is
used. This is the linear-system component, not a scalar-label input interface.

## Column transport

For u=e_i-e_j, let Q=I-uu^T exchange columns i,j. Then

    Z_a (A Q)-(A Q)Z_b = G (Q K)^T + (A u)(Z_b^T u)^T - (A Z_b u)u^T.

This follows from Z_b Q-Q Z_b=u u^T Z_b-Z_b u u^T. Therefore use

    Gc=[G | A u | -A Z_b u], Kc=[Q K | Z_b^T u | u].

Two forward source queries, width s+2, unchanged field/wraps. Outside queries,
the implementation charges two field products and n+1 subtractions. An
identity swap costs no query or field arithmetic. The existing row transport
then produces B=P A Q, again using two transpose queries when nontrivial.
For solutions x=Q y, while left witnesses w=P^T v. RHS is P b.

## Algorithm and termination

At each level, exactly compress the generators. If certified displacement rank
is zero, A=0: the Sylvester operator is invertible because X^n-a and X^n-b
are coprime. If b=0, return the zero solution. Otherwise choose the first i
with b_i!=0 and return w=e_i/b_i, giving A^T w=0 and w^T b=1.

Otherwise scan source columns A e_j in order and choose the first nonzero
entry. A nonzero source must have one. Move it to (0,0) with column and row
transpositions. Its known nonzero value is the scalar leading block E.
Invert only this scalar. For n=1, solve directly. For n>1, construct the
existing Schur generators with cut=1 and distinct child wraps 2,1; reduce the
RHS, record the transport frame and continue at dimension n-1. Control flow
is iterative, so no Python recursion-depth assumption is required.

Unwind solution frames with lift_primal and the column transposition. Unwind
witness frames with lift_dual and the row transposition. Schur identities give

    B y-Pb = [0; S z-h],
    B^T v = [0; S^T w],    v^T(Pb)=w^T h.

Consequently induction yields a solution or normalized inconsistency witness
for every square input under the field/operator and primitive-correctness
assumptions. Each nonterminal step reduces dimension by one. Rank(A)=1+rank(S)
for a nonzero pivot, and the zero terminal has rank zero; the pivot count is
therefore the derived matrix rank. The implementation labels it
`rank_independently_checked=false` until independent transcript verification.

Before returning, the routine charges an original-source forward application
to check Ax=b, or a transpose application and dot product to check A^T w=0,
w^T b=1. It raises on a failed check. It does not infer a left-null witness
from a right-null column, or infer feature annihilation from Gram nullness.
The latter remains the separate extractor condition `F^T w=0` and is explicitly
reported unchecked. No nonzero nullspace basis is promised for consistent
homogeneous singular systems; the returned zero solution is valid.

## Work accounting and limits

All arithmetic uses the existing Meter, including compression and scalar solves.
Column search counts queried columns and inspected entries separately. A node
uses j+1 column queries for its first nonzero column j. Every nonsingular node
has a nonzero first column, so that case uses one search query. Singular nodes
may require up to n; no average-case bound is assumed.

After compression to width t, let c and p indicate nontrivial column and row
transpositions. Transport width is w=t+2c+2p and costs 2c forward plus 2p
transpose queries. The scalar Schur construction then adds two columns.
Before the next compression its width is at most t+6. If the actual displacement
rank grows, compression cannot remove that growth.

Using the existing Schur schedule at width w, generator setup uses 2w+6 source
queries and 2w+4 scalar solve applications. RHS reduction adds one source query
and one scalar solve. Unwinding either result adds one source query and one
scalar solve; the dual route uses transpose queries. One scalar inversion is
charged per pivot. The n=1 terminal omits Schur/RHS/lift work. Root verification
adds one original-source application and, for a witness, one length-n dot.
Compression's full setup is additional, as specified in its own derivation.

The trace retains all compression certificates; unwind frames retain sources,
generators and RHS values. Aggregate storage may grow quadratically or worse
with rank/width growth and is not just the final vector size. This is a complete
derived scalar-pivot route, not a balanced or measured fast solver. Improving
pivot discovery, structured block selection and certificate storage are concrete
next optimization targets. No free inverse, uncharged fallback or random retry
is hidden in its schedule.

## Validation required

Freeze a full contract covering nonsingular matrices with failed leading blocks,
singular compatible/incompatible systems, zero matrices, scalar/odd dimensions,
late nonzero columns, both swap directions, cancellation/zero compression,
all root equations, exact counters and corrupted transport signs. Retain the
full upstream kernel/integration cohorts and failures. Independently verify
rank with dense references and witness feature-annihilation when the extractor
requires it. All numerical execution remains behind the recorded cost gate.
