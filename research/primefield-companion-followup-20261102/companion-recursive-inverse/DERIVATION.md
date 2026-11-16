# Two-query principal generators and reusable recursive inverse

2026-11-01. Derived identities and unexecuted source. Syntax checks only;
no numerical trial, benchmark, frozen execution protocol or success-rate claim.

## Question and sources

Replace supplied leading solves in the companion Schur construction by an
explicit same-prime-field algorithm. Source inputs are public displacement
generators, distinct wraps and a prime r>3, not hidden elliptic scalar labels.
The source elliptic scope remains prime fields; no index calculus, GHS,
generic elliptic collision search or extension-field substitution is introduced.

Read the existing companion-blocks/operators.py principal and inverse formulas,
companion-blocks/SCHUR_NEXT.md, companion-schur/schur.py and the frozen fused
operator source. Bostan, Jeannerod, Mouilleron and Schost (2017), Section 6.2,
provides prior art for recursive leading inverse generators. Its conventions
and rank-profile handling differ. Its randomized regularization guarantee
requires a sufficiently large sampling set; this prototype imports no such
guarantee. This is a convention-specific implementation route, not a novelty
claim about structured inversion.
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf

## Derived leading-block identity

Let Z_(N,a) A - A Z_(N,b) = G K^t, width s, and let E=A[:n,:n]
for 0<n<N. All coordinate indices below are zero-based. Write e_i for an
N-dimensional coordinate vector and u_i for an n-dimensional one. Define

    q = (A (e_n - d e_0))[:n]
    t = (A^t (c e_(n-1) - a e_(N-1)))[:n].

Then, for distinct c,d in the same field,

    Z_(n,c) E - E Z_(n,d)
      = G[:n] K[:n]^t + u_0 t^t + q u_(n-1)^t.

Proof: the leading block of the source displacement is
N_n E - E N_n + a u_0 A[N-1,:n] - A[:n,n] u_(n-1)^t.
Replacing the two nilpotent shifts with wraps c,d adds
c u_0 E[n-1,:] - d E[:,0] u_(n-1)^t. Group the two terms
with left factor u_0 and the two with right factor u_(n-1).

Thus principal.py returns generators of constructed width s+2 with exactly
one forward and one transpose source query. Forming the two sparse inputs
uses two field subtractions. No scalar products are required outside the
source applications. This combines the historical four-column formula;
it does not change the frozen original source or its numerical evidence.
The count is the uncached source-query schedule, not a wall-time comparison
against an already populated historical BoundaryCache.

## Recursive construction and correctness argument

Split n=floor(N/2), m=N-n. Build E generators by the formula above. Recursively
prepare E^-1. Construct S=D-C E^-1 B using the existing Schur generator
formula with its two correction columns. Recursively prepare S^-1.
Both child source widths are s+2; their wraps are fixed c=2,d=1.

For each right-hand side b, the internal block solve is

    p=E^-1 b1; h=b2-(A[p;0])2;
    z=S^-1 h; x1=E^-1 (b1-(A[0;z])1); x=[x1;z].

The transpose path substitutes A^t,E^-t,S^-t throughout. Block elimination
proves the result assuming both recursive inverses exist. Each solve uses
two source applications, two leading inverse applications, one trailing
inverse application and N field subtractions. No dense block is stored.

Apply this solve to every original G column, and its transpose to every K
column. Return U=-A^-1 G and V=A^-t K with SWAPPED wraps b,a, since

    Z_(N,b) A^-1 - A^-1 Z_(N,a) = U V^t.

This returns width s, not s+2. A later inverse query is one fused application
to these returned generators; it does not replay the recursive solve tree.
The tree's temporary children are not retained in the returned operator.

At N=1, obtain the scalar A by one source query, reject zero, invert the
nonzero field scalar, and return Ginv=[[(b-a)/A]], Kinv=[[1]]. This has width
one and swapped wraps. Induction proves conditional correctness in exact
arithmetic; numerical implementation checks remain to be frozen and run.

## Derived cost recurrence, not a measured speedup

For a successful nonleaf with source width s, excluding recursive child setup:

| Phase | A/A^t queries | E^-1/E^-t queries | S^-1/S^-t queries |
|---|---:|---:|---:|
| Principal generators | 2 | 0 | 0 |
| Schur generators | 2s+6 | 2s+4 | 0 |
| Compile inverse generators via 2s block solves | 4s | 4s | 2s |
| Total | 6s+8 | 6s+4 | 2s |

Totals split equally between forward and transpose. Outside these kernels,
there are 2+m(2s+5)+3Ns field additions/subtractions and m field products:
two principal input negations, inherited Schur correction work, 2sN block
solve subtractions and Ns inverse-generator negations. Leaf overhead beyond
its source query is one inversion, one subtraction and one product.

Let K(N,s) bound a generated-operator query in one explicitly chosen unit.
Then setup is the sum of child setup plus
(6s+8)K(N,s)+(6s+4)K(n,wE)+2sK(m,wS) and the stated scalar work,
where wE,wS are s+2 except that scalar children return width one.
Along a depth-d path, source width is at most s+2d. After setup, a root query
costs K(N,s), or K(1,1) at a scalar root. Keep native products, coefficient
reductions, field operations, bit lengths and byte counts separate; no
numeric speedup, time or peak-memory bound is claimed by this recurrence.

Operator.events and Meter.blocks are audit histories and can grow with all
calls; the retained inverse generator size alone is not total memory usage.
Include setup, intermediate generators, encodings and audit storage in tests.

## Failure domain and next validation

A zero leaf raises PivotTreeObstruction with its E/S path. It says that this
fixed recursion cannot proceed; it does not classify the root rank. The
invertible swap matrix with a zero leading scalar must be retained as an
obstruction control. No randomization, pivot retries, matrix permutation or
rank-revealing completion is silently inserted. Singular and bad-profile
inputs are part of the future cohort, not filtered successes.

After the existing fused and Schur/lift prerequisites are reconciled, freeze
full tests for both query directions, all principal cut positions, zero/equal
wrap guards, odd dimensions, scalar leaves, corrupted correction columns,
all success/obstruction branches and exact per-node schedules. Compare each
returned inverse with independently computed dense references and require
both A*inverse and inverse*A identities. Preserve the full source cohorts;
no selected pilot or replacement of failed original rows.

Remaining algorithmic extension: charged rank-profile handling and singular
system/witness extraction, followed by testing whether the resulting inverse
representation lowers the actual marked-feature extraction cost.
