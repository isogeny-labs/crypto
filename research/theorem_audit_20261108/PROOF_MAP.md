# Proof map

Every declaration below is in namespace `TheoremAudit`. Local compilation and
transitive axiom audit are development evidence; PCC standing is unsigned until
an authorized verifier signs it. The final gate receipt is in
`evidence/proof-gate-final/`. [pcc:unverified]

| Module / declaration | Exact content and hypotheses | Boundary |
| --- | --- | --- |
| `S3Completion.S3_zero_iff_exists_points` | Arbitrary field K, algebraically closed, decidable equality, 2≠0; actual Weierstrass W with a1=a2=a3=0 and Δ≠0; all x1,x2,x3. S3=0 iff actual affine P,Q,R have those x coordinates and P+Q+R=0. | General S3 equivalence. Algebraic closure permits lifts; rational-only equivalence is not asserted. No binary/general-recursion result. |
| `S3_pointX_iff` | Same short coefficients and 2≠0; two nonsingular affine points over any field K. Root iff x of P+Q or P−Q equals the third coordinate, including equal x. | Does not require K algebraically closed when the first two lifts are given. |
| `S3_repeated_mathlib_iff` | Short coefficients, 2≠0, nonsingular affine P. S3(x,x,z)=0 iff x(P+P)=z, including the infinity case. | Actual tangent/2-torsion bridge. |
| `S3_signed_zero_sum_iff` | Distinct first two x, short coefficients, nonsingular affine points. Signed actual group sums with a third point. | Extends the existing distinct-x root factorization's interpretation. |
| `pointX_neg`, `pointX_double`, `zero_sum_pointX_iff` | Actual Mathlib points; double lemma additionally has short coefficients, 2≠0, y≠0. | Structural bridges for the main theorem. |
| `S3_repeated_identity`, `S3_repeated_factor`, `S3_repeated_root_iff`, `S3_two_torsion_no_root` | Any field; explicit curve equation. Factor/root need 2≠0 and y≠0; no-root case has y=0 and nonzero derivative 3x²+a. | Algebraic intermediate lemmas; not standalone claims about arbitrary singular models. |
| `FrobeniusCore.triangular_iterate`, `geometric_sum_identity` | Any field K, q,b∈K, any k. Exact iterate of (x,y)↦(x+b y,q y) and finite geometric-sum identity. | No elliptic curve object is introduced. |
| `triangular_iterate_identity_iff`, `unipotent_not_identity` | First: q≠1, all vectors, exact kth iterate identity iff q^k=1. Second: b≠0, q=1, map is not identity. | The actual Frobenius/E[r]/basis identification remains an explicit gap. |
| `StructuralAudit.minus_one_embedding_order`, `trace_zero_prime_embedding_degree` | r prime, r>2. First q∈ZMod r with q=−1; second q∈Nat with r dividing q+1, yielding orderOf(q mod r)=2. | Neither assumes or proves supersingularity, Hasse's theorem or the point-count trace formula. |
| `two_subgroup_embedding_degree` | Any odd natural q; orderOf(q mod2)=1. | Exact r=2 exception; no curve-existence claim. |
| `cyclic_restriction_trivial` | Additive commutative groups G,H, biadditive e, P with e(P,P)=0, natural m,n: e(mP,nP)=0. | Applies to alternating pairings after writing their target additively. It is not full-domain degeneracy. |
| `sum_confined`, `outside_subgroup_not_smooth` | Any additive commutative group, subgroup H, base F⊆H. Finite sums stay in H; no outside point is such a sum. | No concrete Jacobian or smoothness-density family provided. |
| `branch_collision_mass` | Finite index type, any rational weights p: double sum of diagonal iid-label weights equals Σpᵢ². | No positivity/normalization needed for identity; no fixed-partition walk or mixing theorem follows alone. |
| `stable_branch_locus_excludes_diem` | Any set/predicate stable under σ, n≥2: cannot have DiemCondition's branched R with unbranched other orbit points. | Branch locus interpretation of an actual cover remains a geometry bridge. |
| `short_branch_locus_stable` | Field endomorphism fixes a,b; it preserves roots of x³+a x+b. | Finite part of the short x-projection branch set, not the whole scheme proof. |
| `MultiplicityControl.double_root_quotient_dimension` | Any field K: finrank K(K[X]/(X²))=2. | Actual quotient algebra; no general intersection-theory theorem. |
| `double_root_support_size`, `double_root_multiplicity_mass` | Any field, decidable equality for finite support: support1 and root multiset size2. | Small nonreduced control, not a census. |
| `DegreeControl.control_X0_in_degree3_span`, `control_X1_in_degree3_span` | Concrete F2[x0,x1] fixture F={x0²+x0,x1²+x1,x0x1+x0+x1}; explicit nine degree≤3 rows. | Exact row-span inclusions. |
| `control_variables_are_gb`, `control_degree3_rows_suffice` | Same fixture, actual monomial order degLex and ideal.span F, actual IsGB definition from the census proof. | Exhibits a GB in degree3 rows. |
| `control_all_rows_degree_le_three`, `control_degree2_rejected` | Every named row has degree≤3; no GB of the fixture ideal lies in the degree2 generator span. | Exact known transition for the criterion. Production Rust tests exercise this same fixture. |

`S3Roots.lean` and `SolvingDegreeBound.lean` are unchanged copies of prior
repository modules. Their declarations are recompiled and included in the
repository proof audit but are not counted as new contributions. There are
33 new theorem declarations in the five modules listed above. No use of
`sorry`, custom research axioms, native evaluation or kernel bypass is admitted.
`lean4checker` was not installed, so that optional additional recheck is not
reported as run. [pcc:unverified]
