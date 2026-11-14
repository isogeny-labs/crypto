# Actual norm matrices and complete bounded polynomial families

2026-11-13. Unsigned producer evidence [pcc:unverified].

The preceding functional coefficient identity now has a faithful matrix
connection. This round defines the actual entry polynomials, moment matrices,
and coefficient matrices, identifies their quadratic expressions with the
assembled norm constraints, and instantiates matrix-pencil membership in the
full even-degree coordinate chart. It also supplies polynomial-family degree
bounds and completeness for every bounded polynomial pair. Actual branch
intersection/corank and a public-input finder remain separate [pcc:unverified].

## Actual matrices

For a field-linear polynomial functional mu and families Ai,Bi define

    E_ij = Ai Aj - f Bi Bj,
    D(mu)_ij = mu(E_ij),
    C_j(i,k) = coeff_j(E_ik) - coeff_d(E_ik)*coeff_j(G), d=natDegree G.

The matrices are defined using these exact formulas; no abstract form map or
numerical rank hypothesis is substituted. The actual moment matrix is
symmetric. For every finite-family coefficient vector w, with
A_w=sum wi Ai, B_w=sum wi Bi and N_w=A_w^2-f B_w^2,

    quadratic(D(mu),w) = mu(N_w),
    quadratic(C_j,w) = coeff_j(N_w)-coeff_d(N_w)*coeff_j(G).

The existing faithful NormLift.outerNorm supplies this connection. If G is
monic, mu(G)=0 and each E_ij has natDegree<=d, the earlier functional expansion
applied entrywise gives the exact matrix equality

    D(mu) = sum_(j<d) mu(X^j) * C_j.

Family pole-degree bounds imply the entry bound, rather than assuming it in
the final actual-witness instantiation [pcc:unverified].

## Actual witness and canonical full chart

Let F,H,G be monic, deg H>0, and suppose supplied U,V,alpha,beta,c obey

    UF+V(fH)=1,
    (alpha F)^2-f(beta H)^2=cG, c!=0.

The actual quotient moment mu(P)=coeff_(deg F-1)((PV) mod F) annihilates G
by the preceding checked functional theorem. Thus its matrix has the displayed
pencil expansion whenever the family bounds hold. No pencil membership or
rank conclusion is assumed [pcc:unverified].

For the full even chart set a=natDegree F, b=natDegree H, a=b+2,
natDegree G=2a, natDegree f<=3. The exact coordinate index is
Fin((a+1)+(b+1)). Use the product monomial basis of degreeLT K(a+1) and
degreeLT K(b+1). Its first/second components define the upper/lower polynomial
families. Each upper polynomial has natDegree<=a and each lower one<=b,
including zero components. Consequently all required entry degree bounds
follow in this actual chart [pcc:unverified].

The source also supplies: for every P,Q with natDegree P<=a and
natDegree Q<=b, there exists a common coefficient vector w with A_w=P and
B_w=Q. It constructs w from the genuine product basis coordinates and proves
both projected finite sums. Coordinate injectivity and transfer of the branch
radical to exact finite-matrix corank are still separate formal obligations;
the matrix-pencil equality alone does not establish them [pcc:unverified].

## Proof map

Source: [NormMomentMatrix.lean](../lean/NormMomentMatrix.lean), frozen
commitcc5c9651a. Exact declaration bindings are in
[matrix-evidence](../matrix-evidence/) [pcc:unverified].

| Declaration | Component |
|---|---|
| momentSymmetric | Actual entry symmetry |
| quadraticMoment | Actual matrix quadratic equals functional of norm |
| quadraticCoefficient | Actual coefficient matrix quadratic equals residual constraint |
| matrixPencil | Entrywise functional-to-matrix pencil equality |
| entryDegreeBound | Family bounds imply every actual entry degree bound |
| actualMatrixPencil | Witness-derived quotient matrix, without assumed mu(G)=0 |
| upperFamilyDegree | Actual upper product-basis family bound |
| lowerFamilyDegree | Actual lower product-basis family bound |
| familiesComplete | Every bounded polynomial pair has coefficient coordinates |
| canonicalMatrixPencil | Actual full even-degree chart satisfies the matrix-pencil identity |

All statements expose their hypotheses. Generic matrix entry statements do
not need finite index types; quadratic evaluations and the canonical chart
have their explicit finite index types. No characteristic or separability
restriction is needed in this round [pcc:unverified].

## Checks and failed attempts

Pinned Lean4.35.0 and unchanged CryptoLean/Mathlib revisions. Build006
passes3149 jobs. All ten printed axiom sets use only propext,
Classical.choice and Quot.sound. Actual-source audit covers120 declarations,
zero failing and zero additional trusted axioms. Import-inclusive fresh replay
exits0; both completed receipts are retained. Producer
standing is unsigned, not a signed PCC verdict [pcc:unverified].

Build001 failed on intro applied to matrix symmetry (a transpose equality)
and function-sum simplification of Matrix. Using ext and the actual Matrix
sum/smul APIs resolves those errors. Build004 used unrestricted simp on a
projected basis reconstruction equation, simplifying the supplied equation to
True before the intended projection; controlled product/submodule sum
rewrites resolve it. Successful intermediate builds and failures are retained.
No failed build is counted as proof [pcc:unverified].

The unchanged supporting native polarization record IDC1h345f3824264d232a is
copied byte-for-byte from functional-evidence. It checks only the universal
CommRing norm-polarization identity at the previously recorded32+32 points and
pins that earlier exact source. No new native run is claimed, and this record
does not check arbitrary finite sums, matrix-pencil equality or rank. Those
components have their own Lean proofs. No Rust implementation changed.

## Next dependencies

1. Establish coefficient-coordinate injectivity and the bilinear evaluation
   formula for arbitrary pairs of coordinate vectors, retaining bounds.
2. Define the actual branch evaluation functional, exhibit a radical vector
   on which it is nonzero, and instantiate branchTestsDetectFull.
3. Compute the scaled witness-line intersection and transfer its dimension to
   corank1; apply the guarded adjugate theorem with stationarity/nonzero guards.
4. Cover constant/degree-drop charts. Develop and account for a public-input
   moment finder at 2^{o(n)} bit cost, n=ceil(log2 r). The moments in the current
   construction still use supplied witness polynomials.

The original rank-directed jet completion motivates the certificate design.
Reachability through W_base+C(T)H(T) and an efficient selection of H remain
unestablished by these matrix identities [pcc:unverified].

## Sources and resource boundary

Reviewed exact existing NormLift/NormDualPencil/NormPencilFunctional source,
pinned Matrix.sum_apply/smul_apply, DegreeLT.basisProd/basis_repr and
Basis.sum_repr, polynomial degree and product/submodule sum declarations.
Primary pinned basis source:
https://github.com/leanprover-community/mathlib4/blob/cfc55fc58714a59dc06e6e79edc6d1d95140d3c2/Mathlib/RingTheory/Polynomial/DegreeLT.lean

KB and conductor unavailable; commands attempted and bounded source fallback
recorded. No priority/exhaustive-search claim for these classical ingredients.
No paid job, timing/scaling campaign or budget override. Monthly isolab budget
remains exhausted. No benchmark/leaderboard/ecdb record changes. Canonical
scoreboards, progress timeline, curve diagrams and historical study figures
gain no measured fact and remain unchanged.

Editable [DOT](matrix.dot), [SVG](matrix.svg), [diagram PDF](matrix.pdf),
[report PDF](report.pdf) and [LaTeX](report.tex) accompany this source-linked report.
