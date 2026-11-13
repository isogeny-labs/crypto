# Next test: quotient-moment certificate completeness

Status: **unproved construction hypothesis; not an algorithm or an executed
experiment**. No novelty/priority asserted. The input to this proposed
completeness proof includes an already valid norm witness A,B. It is not a
procedure for finding A,B from public data [pcc:unverified].

For the even-degree case d=2a, a>=2, suppose deg A=a, deg B=a-2,
deg f<=4, gcd(A,fB)=1, and A²-fB²=cG with c!=0. Let Ahat be the monic
associate of A, and define a candidate functional on polynomials P by

    mu(P) = coeff_(a-1)( P * inverse(fB modulo Ahat) modulo Ahat ).

The inverse exists under the stated coprimality. Test the two Hankel blocks

    H_A(i,j) = mu(X^(i+j)),          0<=i,j<=a,
    H_B(i,j) = -mu(f X^(i+j)),       0<=i,j<=a-2.

The proposed conclusions are: H_A has radical span(A), H_B has radical span(B),
and the block-diagonal form has exactly a two-dimensional radical. This needs
a proof of the coefficient pairing's nondegeneracy and a quotient/residue
reciprocity identity, not a numerical rank assumption [pcc:unverified].

If those statements hold, restrict the form to the actual target branch
A(q)-yB(q)=0, with A(q),B(q),y nonzero. Its radical should become the line of
the original witness. Also prove mu(G)=0, so the restricted form belongs to the
actual coefficient-constraint pencil. The branch restriction must not create
additional radical vectors. Odd degrees, degree drops, repeated roots and
finite-characteristic details remain separate cases [pcc:unverified].

## Exact development-control prediction, not yet checked

On F17 take f=X³+2X+2, A=5X²+11X+6, B=1, target Q=(0,6), and support
points (5,1),(6,3),(3,1). The hand-derived prediction is

    A²-fB² = 8*X*(X-5)*(X-6)*(X-3),
    mu(X^j), j=0..4: (7,11,15,15,0).

After substituting A=tX²+sX+6u, B=u, predict the three quadrics

    2ts-u²-3t²,   s²+12tu-12t²,   12su-2u²-12t².

Their pencil parameters (15,15,11) are predicted to give

    S = [[15,15,15],[15,0,5],[15,5,14]],
    adj(S) = [[9,1,7],[1,2,14],[7,14,13]].

The normalized third adjugate column is predicted to recover (s,t,u)=(11,5,1).
All these are **test inputs/predictions, not recorded outcomes**. First check
the actual norm, moment construction, coefficient matrices, exact rank and
cofactor pairings in Lean/native development controls before citing them as
evidence. Host timing is irrelevant to their truth [pcc:unverified].

Sources reviewed for formulating the next test: pinned Mathlib
RingTheory/AdjoinRoot.lean, modByMonicHom and powerBasisAux' (quotient coordinates);
Matrix.Adjugate/Matrix.Rank; current norm/recovery proofs. The quotient-coordinate
source does not prove the required nondegeneracy/reciprocity lemma. A search hit
for coalgebra Frobenius equations is not treated as that missing result.
KB unavailable; primary review and formalization of the proposed pairing remain
owed before promoting the hypothesis [pcc:unverified].

Even a successful completeness proof supplies no fast public-input finder for
mu or the pencil parameters. That remains the essential algorithmic step.
