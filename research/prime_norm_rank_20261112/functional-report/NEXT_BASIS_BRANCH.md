# Actual basis, branch and corank assembly

Unproved next plan; this note does not complete the matrix bridge
[pcc:unverified].

Source lookup for the next phase found the exact pinned APIs in
Mathlib/RingTheory/Polynomial/DegreeLT.lean: degreeLT.basis gives monomials,
basis_repr gives coefficient coordinates, and degreeLT.basisProd supplies a
basis of the two bounded spaces. Use bounds a+1,b+1 for deg<=a,deg<=b.
This avoids an assumed bijection between arbitrary coefficient arrays and
admissible polynomials. Review the actual Matrix/BilinearForm APIs before
choosing the matrix conversion; an initial guessed BilinearForm/Matrix.lean
path did not exist. The correct nearby file is LinearAlgebra/Matrix/BilinearForm.lean.

Define the bilinear form on the product bounded submodules as

    B((A,B),(Q,T)) = moment F V (A*Q - f*(B*T)).

Construct its two linear-map structures from polynomial multiplication and
moment linearity, then prove symmetry. Use basisProd for the full coefficient
matrix; identify its diagonal quadratic form and actual coefficient-pencil
matrices via normCoefficientPencil and normPolarization. If dividing by2,
require characteristic not2 explicitly. Then the finite matrix is the actual
one, not a numeric rank hypothesis [pcc:unverified].

The evaluation branch is l(A,B)=A(q)-y B(q). If F(q)!=0, the right-radical
vector k=(F,0) has nonzero branch value. Apply branchTestsDetectFull. In the
known full radical A=cF,B=dH, branch-zero means

    c*F(q) = y*d*H(q).

The supplied actual witness satisfies alpha*F(q)=y*beta*H(q). With alpha!=0
and F(q)!=0, solve the two scalar equations to obtain

    (c,d)=t*(alpha,beta).

This gives the witness line inside the branch. Prove both containments and
that the witness vector is nonzero. Use a basis of the branch kernel and
rank-nullity to obtain exactly corank1, then invoke guarded adjugate recovery.
Do not infer this matrix conclusion solely from the abstract test lemma
[pcc:unverified].

Resource/mathematical boundaries: constant H and degree drops remain separate;
KB unavailable, bounded pinned-source review only. No public-input search
procedure or cost follows from any of the preceding certificate identities.
