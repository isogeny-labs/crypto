#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Simple branch-point norm and multiplicity
[pcc:unverified] Hand derivation and prospective Lean source. Existing branch-stalk DVR and order-one source results are reused. No compiler, tests or trials were run.
#image("normalization.svg",width:100%)
== Derivation on the actual stalk
At a rational simple branch P=(a,0) of y²=f(x), write f=(X-a)q with q(a)=f′(a) nonzero. The actual local ring is a DVR; y is a uniformizer and x-a is a unit times y². For n(a)=0 and b nonzero, n=(X-a)m, so n±by=y(y q⁻¹m±b). Both parenthesized terms are units: each conjugate factor has order1 and their product order2. If n(a) is nonzero, both factors are units.
For H=n²-b²f=u1u2U, the curve-stalk support-order sum is twice the selected-factor order. The base polynomial multiplicity sum is instead one: H′(a)=-b²f′(a) is nonzero. This does not double Mumford multiplicity. Two support factors vanishing at a would force b=0, outside the generic determinant guard.
== Actual coefficient fixture and countermodel
Over Q: f=X⁵-X, a=0; u1=X²-X, u2=X²+1; both input ordinate polynomials0. Source coefficientData(-1,0,0,0,0,1,0,0)=(-2,0,0,-4,-4), hence n=0, b=-2, U=-4(X+1) and n²-u1u2U=4f. Input divisibilities hold. Separability follows from 5f-Xf′=-4X and f′(0)=-1. The factors ±2y have order1; support germs have orders (2,0,0), base multiplicities (1,0,0). The naive nonbranch stalk formula would give 1=2.
== Source delivered and exact remaining work
Three prospective declarations: actual_branch_conjugate_orders; numerator_root_from_norm; two_support_roots_force_b_zero. They use the actual branch stalk and derive conclusions without assuming a desired valuation identity. No holes or new axioms are introduced; elaboration is unverified.
The explicit missing formal bridge is E(f)=Y² for the branch evaluator, followed by mapping the polynomial norm identity to its stalk factorization. Base multiplicity transport requires the residual-unit criterion and finite factorization. Paper derivations are in DERIVATION.md; these bridges are not claimed as implemented. Infinity and global divisor assembly remain separate.
Sources: existing WeightedProjFiniteBranchDVR, WeightedProjNormBranchZeroOrder and source actual-quintic modules; Mathlib derivative and valuation APIs; Stacks 00PD (DVR normalization), 0EXQ (ramification). Exact hashes and KB-unavailability record are in sources.json. Review, compilation and named axiom audit remain required.

Additional hand fixture (coordinator-suggested, independently substituted): f=X(X²-1)(X²-4), u1=X²-1,u2=X²-4, zero ordinate polynomials yield coefficientData=(-9,0,0,-81,0), n=0,U=-81X. Points a=1,2,0 place the single simple base root in u1,u2,U respectively. Details: THREE-LOCATION-FIXTURE.md.
