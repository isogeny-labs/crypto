# Actual branch evaluator and norm transport

[pcc:unverified] This successor packet constructs the first two missing Lean-source bridges identified in afccdb7d86. It preserves that packet verbatim. No compiler, axiom audit, tests or trials were invoked; source construction is distinct from kernel acceptance.

## Exact maps

Let A be the existing AffineRing for y²=f(x), L its localization at the rational branch ideal, and R the actual weighted-Proj branch stalk. The existing branchRingEquiv maps R to L. Define rho:A→R as the localization algebraMap followed by branchRingEquiv.symm. Define E:K[X]→R using the existing projectiveCoeffHom and projectiveX.

The new eval_eq_rho_affine proves E=rho∘affineEval by the polynomial ring-homomorphism extensionality principle. On constants, the claim is exactly the existing algebra tower identity algebraMap K L = algebraMap A L ∘ algebraMap K A. On X, it is the definition of projectiveX. No equation between unrelated branch and nonbranch stalks is posited: only the affine evaluator is reused from Nonbranch.

The existing affine_curve is derived from quotient_affine_relation, so rho carries it to rho(ybar)²=E(f). By definition rho(ybar)=projectiveY. Hence curve_eval derives E(f)=Y² on the actual branch stalk, without a new curve-relation hypothesis.

## Mapped norm and orders

Apply the ring hom E to n²-u1u2U=C(b²)f. Constants and products map naturally, and curve_eval replaces E(f) by Y². The resulting polynomial equation is rearranged to (E(n)-BY)(E(n)+BY)=E(u1)E(u2)E(U), with B=projectiveCoeffHom(b). This is stalk_norm; the desired stalk equality is its conclusion, not a premise.

At a simple branch with b!=0 and a support root, the prior numerator_root_from_norm derives n(a)=0. The prior actual_branch_conjugate_orders supplies order1 for each conjugate factor and order2 for their product. Apply the actual DVR addVal to stalk_norm and use AddValuation.map_mul twice. This gives vP(Eu1)+vP(Eu2)+vP(EU)=2. The source-specific wrapper supplies simplicity from separability and the polynomial identity from ActualNonbranchNorm.source_norm_identity using coefficientData and the two support-divisibility premises.

This source uses no assumed valuation equation, no assumed principal-divisor relation, no new axioms and no holes. Its five declarations and imported predecessor source remain uncompiled in this task. Proof-script elaboration, exact instance alignment and imported source closure require the separate machine check.

## Scope and remaining obligation

The equality is in Nat-infinity at the rational branch stalk. Its right-hand value2 implies each summand is finite; this packet does not add the Nat conversion theorem. It also does not implement base root-multiplicity transport. The exact next obligation is: for nonzero g=(X-C(a))^m*t with t(a)!=0, prove E(t) is a unit via the branch residual evaluation criterion; combine vP(E(X-C(a)))=2 with valuation multiplicativity to obtain vP(E(g))=2m. Here m is the finite base root multiplicity. At the norm support the base sum is1, not2. The factor2 therefore does not assert doubled Mumford multiplicity.

The predecessor's Q fixtures remain nonvacuity witnesses by hand algebra, including f=X⁵-X, supports X²-X and X²+1, n=0,b=-2,U=-4(X+1). No new measurements are attached. Infinity, nonrational points and global divisor assembly are separate obligations.

## Binding and provenance

input-bindings.json pins Nonbranch from a51006010d and BranchConjugateOrders from afccdb7d86, plus inspected primary local source/API files. The latter prospective module supplies the branch product-order result; it has not yet been kernel-checked. The displayed input set is not advertised as a complete import closure. Future machine validation must bind the entire dependency closure and audit all five new declaration names.

Sources checked (KB): inherited unavailable status confirmed in prior packet: no research-kb MCP exposed; scripts/research-kb absent in sparse checkout. Fallback uses exact primary source paths rather than broad novelty claims. Mathematical normalization follows Stacks 00PD (https://stacks.math.columbia.edu/tag/00PD) and 0EXQ (https://stacks.math.columbia.edu/tag/0EXQ), already read for the predecessor. Main authority here is the explicit quotient/localization map construction and cited Mathlib algebra-tower API.
