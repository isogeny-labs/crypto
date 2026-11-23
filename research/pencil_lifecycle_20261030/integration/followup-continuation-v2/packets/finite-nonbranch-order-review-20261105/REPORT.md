# Independent finite nonbranch order source review

The five new declarations match the announced finite local-order claim at source level. Exact source eaf3957bc766e9da64e7ba27efb6ef74b34c57705020d0ad2514dbe86bd75c1a is bound to published commit 1b7c04cd5f5ad6de6f662b84f24035ed756f6b5d. Six inspected announced dependency files matched their declared hashes. Two additional Mathlib API sources were hashed for this review.

No substantive mathematical counterexample to the stated hypotheses was found. The proof uses the actual affine quotient, actual rational-point stalk and a proved localization injection, rather than an arbitrary homomorphism. The nonzero proof evaluates at the conjugate point. The extended valuation is proved finite before conversion to Nat. The source-specific final theorem derives its polynomial norm identity from coefficientData and the two support-divisibility premises; it does not assume the desired valuation equation.

The source-level finding is faithful_scoped. Machine status is not_verified and overall review outcome is inconclusive: the new module has not been compiled or axiom-audited in this review. Reading source cannot certify elaboration, import resolution, absence of a shadow module or the transitive kernel boundary.

## API and object bindings

- Nonbranch rho composes the affine localization algebraMap with pointRingEquiv.symm. ProjPointRing is the actual weighted-Proj presheaf stalk, not a named scalar stand-in.
- Common22SeparableAffineDomain derives domain status using a simple prime factor of the separable monic quintic and the inherited affine domain construction.
- Mathlib Localization/AtPrime/Basic.lean:116–123 defines the FaithfulSMul theorem and instance for prime localization of a ring without zero divisors. Thus the intended injection is not an extra arbitrary hypothesis.
- WeightedProjNonbranchDVR.projective_point_dvr derives the actual stalk DVR from separability and 2*y0≠0. The instance declaration explicitly matches the derived domain instance.
- DiscreteValuationRing/Basic.lean:442 supplies addVal_eq_top_iff; ENat/Basic.lean:240 supplies toNat_add with both finite hypotheses. The finite_sum_toNat declaration supplies those hypotheses before conversion.
- Existing concrete_norm_order supplies the additive extended-order identity after proving the selected factor nonunit and its conjugate unit. The source-specific wrapper passes source_norm_identity, with no new valuation hypothesis.

## Attempts to break the boundary

The preread DERIVATION.md gives explicit b=0, y0=0 and characteristic-two cases where numerator order is one and norm order is two. A kernel-map attack demonstrates why evaluation at P cannot replace localization. Zero U would contradict the nonzero norm product under valid premises. These are hand derivations; no checked-control label is assigned.

The generic theorem is nonvacuous over Q with f=X^5+1, P=(0,1), n=b=1, u1=u2=X and U=-X^3: n²-u1*u2*U=f. After this initial derivation, the coordinator supplied a prior source-specific fixture. Independent hand substitution into Common22SupportHead.coefficientData yields (-9,0,0,-81,0), n=-9,U=-81X for f=X^5-5X³+4X+1, supports X²-1,X²-4 and P=(1,1). Both divisibilities, selected sign and norm identity follow explicitly; separability follows by squarefree reduction mod2. SOURCE-FIXTURE.md records the calculation and its nonblind provenance.

## Remaining review work

Bind an immutable complete import closure, compile the exact new declarations, inspect named elaborated types and transitive axioms, and obtain checked controls. The result's scope is rational nonbranch finite local order; global closed-point/branch/infinity divisor assembly remains separate. No runtime, cost or cohort claim is reviewed here.

The reviewer did not author the proof. The plan was written before reading this producer source, but the coordinator had disclosed the conjugate-point strategy; the review is context-informed and explicitly not blind. No sibling verdict was read. Source reads and hashes are retained in sources-read.json.
