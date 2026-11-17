# Feature-coefficient Schur updates and sparse witnesses

2026-11-03. Construction and static source audit; numerical trials: zero.

## Source identity and scope

Retain the public prime r>3, window 1<=q<r, gamma in F_r excluding 0,1,
and all fixed nonzero weights of `direct-cyclic-companion/generators.py`.
Prime certification is the caller's responsibility. This component works on its
symmetric Gram B, before the invertible reversal/scaling used by the companion
compiler. It does not invoke that compiler or allocate an r-row array.

Let phi_S(x)=product_{d in S} chi(x+d), including phi_empty=1.
The preserved labels consist of every S subset of [0,q) with |S|<=3, exactly
once: subtract min(S) to obtain its unique pattern P, then shift back by min(S).
Thus m=1+q+binom(q,2)+binom(q,3). The initial coefficient matrix C is diagonal,
with entries w_empty and w_P gamma^a for the preserved pattern/shift ordering.
All are nonzero. B=Phi C Phi^T and b(x)=x. No original weight is retuned.
The representation is O(m^2), which is O(q^6), and need not be smaller than r.

## Closed residual representation

Represent a residual on remaining coordinates T by
B_T(x,y)=phi(x)^T C phi(y), h(x)=x-phi(x)^T u, initially u=0.
For supplied distinct unused coordinates I of size k in {1,2}, put
F=Phi_I, W=C F^T, E=F W. Require E invertible; otherwise reject that block.
Compute H=W E^-1 and

    C_new = C - H W^T
    u_new = u + H h_I.

Substitution gives the exact principal Schur matrix and reduced RHS:
Phi_T C_new Phi_T^T = B_TT-B_TI E^-1 B_IT, and
x-Phi_T u_new = h_T-B_TI E^-1 h_I.
Symmetry is preserved. No assertion of positive definiteness applies over F_r.
The implementation accepts any invertible symmetric two-point block, including
zero-diagonal pairs. Failed attempts charge arithmetic but leave immutable state
unchanged; there is no implicit fallback or search over the field.

Each accepted block reduces rank(C) by k. Indeed the bordered block
[[C,CF^T],[FC,E]] has rank(C) by elimination using the bottom rows' dependence
on the top rows, and is congruent to diag(C_new,E). Hence sum k<=m and also<=r.
This does not imply rank(Phi_T C Phi_T^T)=rank(C): the restricted feature rows
may be dependent or isotropic. Failure of a proposed block certifies only that
block's singularity, not zero residual or rank exhaustion.

## Sufficient terminal and exact sparse lift

The implemented terminal takes a supplied unused x. It explicitly checks
C phi(x)=0 and h(x)!=0, then sets w=1/h(x), support={(x,w)}, moment mu=w phi(x).
This is sufficient for a residual kernel witness with h-pairing one. It can
succeed before C=0. Failure of C phi(x)=0 is not proof that a residual Gram
column is nonzero, because the remaining feature rows may annihilate that image.

Given a tail witness with moment mu and normalized residual pairing, a frame
lifts it by y_I=-H^T mu, keeping its tail coefficients, and updates
mu_old=mu+F^T y_I. Then C_old mu_old=C_new mu=0, so the stronger coefficient
annihilation invariant is preserved. Also
h_I^T y_I+h_tail^T y_tail=h_reduced^T y_tail=1.
At the root C is invertible diagonal, hence mu_root=0 and sum_x y_x*x=1.
The certificate has at most 1+sum k<=m+1 support entries. Zero coefficients may
remain in the support; all coordinates are distinct.

The producer uses immutable state/frame/witness records and internal identity
tokens to reject out-of-order lifts. Tokens are an internal provenance guard,
not a security boundary against callers forging records. The final independent
checker imports no producer code, ignores cached moments and recomputes every
feature moment directly from public support. It also checks canonical residues,
distinct coordinates and normalized pairing. It establishes annihilation of all
original features without enumerating r output coordinates. The checker has
been implemented but not executed on a certificate in this round.

## Derived arithmetic schedule, not measurements

Meter.dot of length l uses l products and l additions, including addition to
zero. Inversions are counted separately; Python integer work, modulus, copies,
shape/symmetry checks, indexing and object allocation are additional native work.
Let M=1+q(q-1)/2 and J=q+2*binom(q,2)+3*binom(q,3).
Initial diagonal weights use (m-1)-M products; assigning the m-by-m diagonal
array uses no field operations. For h=(r-1)/2 and
L=popcount(h)+bit_length(h)-1, one feature evaluation uses q*L+J products.

For k supplied points, excluding k feature evaluations and inversion-block work,
the pivot uses:

- products: 2*k*m^2 + 2*k^2*m + 2*k*m;
- additions/subtractions: (2*k+1)*m^2 + 2*k^2*m + 2*k*m + m+k.

The scalar inverse adds one inversion. The two-point inverse adds six products,
three subtractions and one inversion. These counts include full C update,
W, E, H, pivot residual RHS and u update; no triangular optimization is used.
A successful terminal adds m^2+2*m products, m^2+m+1 additions/subtractions
and one inversion, in addition to one feature evaluation.
A lift uses 2*m*k products and 2*m*k+m+k additions/subtractions.
The independent checker separately uses native modular powers, up to s*J of
them for support size s; its arithmetic is not attributed to producer Meter.

Persistent state has m^2+m coefficient/RHS field entries, plus immutable labels
and used indices. A frame holds F and H (2*m*k field entries), k coordinates
and tokens; it retains no parent matrix or source. Accepted frames together
hold at most 2*m^2 field entries. Peak pivot storage includes old and new C,
W,H,F and other temporaries; O(m^2) describes order, not measured peak heap.
Callers retaining previous states accumulate additional memory. All-point
search, feature evaluations of rejected proposals and witness verification must
be charged in any complete algorithm. No timing or total-cost gain is measured.

## Literature and next construction

The block elimination identity is established Schur-complement algebra; see
[Higham, What Is the Schur Complement of a Matrix? (2023)](https://nhigham.com/2023/06/01/what-is-the-schur-complement-of-a-matrix/),
sections on elimination and block inverses. The factorization uses only field
operations and invertibility, so applies here in prime characteristic. Real
positive-definite and inertia statements from that source are not used.
The local preserved constructor establishes the exact character-feature family.
This construction specializes the identity to that family and carries its RHS
and sparse dual certificate. It claims no new general Schur identity.

Next investigate point selection using the restricted feature span and the
residual affine functional, including when h belongs to that span. This is the
missing search mechanism; the present API accepts supplied public points.
The link from an annihilating feature certificate to the original target's
extraction/exclusion conditions must remain explicit in a complete driver.
No elliptic collision search, index calculus, GHS or characteristic change is
introduced. The earlier root-pivot implementation and all cohorts remain frozen.

## Validation status

Static audit checks syntax, exact preserved constructor bytes, pattern AST,
normalized binary-power AST, absence of direct range(r) loops in producer/checker,
and the independent checker's import separation. It does not prove runtime
correctness, counters, lifetime behavior, point availability or recovery.
Numerical, corruption, heap and performance trials are zero; fresh execution
admission and a frozen protocol are required before running them. Required
controls include scalar and general pair pivots, singular pairs, wrong/used
coordinates, failed terminals, out-of-order frames, forged moments/support,
normalization, full direct Schur/RHS comparisons, rank and operation counts.
