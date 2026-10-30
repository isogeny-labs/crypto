# Published-law remaining obligations: source-focused follow-up

Read-only source and saved-review inspection, 2026-10-30. No Lean build, Rust build, experiment or new mathematical proof was performed. All paths below are relative to `/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/`. Adjacent `law-source-hashes.json` identifies the exact inspected files. This note proposes a bounded next proof obligation; it does not declare it proved.

## Already measured and independently rechecked

`independent_complete_law/COHORT_RECHECK_20261027.md:16` records seven 180-row files per arm, two equal 1,260-target key sets, candidate and same-process rho verification fields, complete cold phases and independent arithmetic repricing. Its own direct check compared recovered and planted public scalars; the separate native replay certificate states that 5,040 candidate/rho source multiplications were recomputed. Those are distinct checks. The matched operation-price cohort does not need rerunning merely to investigate universal law correctness.

`independent_complete_law/SOURCE_AUDIT.md:31` maps the implemented weighted branches to the published law and its incorporated references. It identifies no missing implementation of the requested law, but does not establish universal compiled-program semantics. The source audit corrects the historical N6 allegation: both the pinned author Magma and Rust use W². The W¹ regression is a synthetic mutation control, not evidence of an author-code defect.

## Smallest concrete next connection

Focus on **one error exit**, `same_u_mixed_zero_slope_delta`, before attempting the full group-law theorem.

- Actual Rust: `src/bin/projective_law_gate.rs:896` constructs cross-scaled q/r/s/t coordinates; lines 920–930 first exclude the inverse and equal cases, then return `None` if `s1c-s2c=0`.
- Existing Lean: `formal/RuntimeSemantics.lean:89` defines `classifySameU22`; line 131 proves `classifySameU22_ne_zeroSlope` **assuming** odd characteristic and the two supplied equations `hx: (t₂-t₁)(s₁+s₂)=0`, `hc: (t₂-t₁)(t₂+t₁)=0`.
- Actual normalization: `src/bin/projective_law_gate.rs:533` interprets the degree-two tuple as q/z², r/z⁴, s/(z³w), t/(z⁵w), with nonzero z,w.
- Binding limit: every row of `formal/runtime-bindings.json` says `binding_kind=manual_abstraction_hash_bound`, `semantic_equivalence=not_established`.

A narrowly reviewable next theorem should start with two valid weighted Mumford inputs (nonzero weights; normalized monic degree-two u; deg v<2; u divides f-v²), odd characteristic, and the **actual branch conditions** of equal cross-scaled q/r and equal cross-scaled slopes. It should derive hx and hc for the actual cross-scaled ordinates, then discharge the error branch through the existing Lean theorem. Derivation must be conditional on the zero-slope branch: hx/hc are not additional axioms imposed on arbitrary unequal-slope pairs. The remainder of v₂²-v₁² modulo the common support supplies the two coefficients after the slope equality is used.

Acceptance artifact: an exact source-derived expression/branch correspondence for the cross-scaling and decision order, a theorem deriving the hypotheses from the weighted validity predicate, its pinned kernel/axiom receipt, and an independent check of the theorem's scope. A hash association alone does not meet source correspondence. A theorem about arbitrary field arithmetic also needs the explicitly stated correspondence between Rust `Field` operations/equality and the modeled finite field before claiming compiled-Rust totality. This slice proves exclusion of one error exit; it does not prove the returned divisor is the sum.

If this proposed implication fails, a useful counterexample must provide odd characteristic, smooth normalized quintic, nonzero weights, valid reduced Mumford inputs, cross-scaled intermediate values and actual branch trace leading to `None` (or a verified incorrect normalized output). An invalid tuple or zero weight is not a counterexample to the stated law domain. This note does not execute such a search.

## Other totality and validity obligations remain separate

`formal/RuntimeSemantics.lean:109` already excludes `sharedZeroQ` for its abstract classifier from distinct supports, and line 41 excludes the 1+2 mismatch from a supplied equality of squares. Deriving the appropriate runtime validity hypotheses and source correspondence is still necessary.

The Rust shared-support positive path (`src/bin/projective_law_gate.rs:965–998`) constructs p1,p2,p3,p4, doubles p1, applies `add_point_to_result(p2,...)`, and then applies it again to p4. There is an explicit `shared_first_secondary_exception` and a final propagated optional result. `formal/branch-coverage.json` labels the first secondary exception `written_branch_argument_only`. The source-conditioned proof must cover **both sequential additions**, preserve valid reduced output and nonzero output weights through their degree changes, and show no propagated `None` reaches `add_projective`'s `expect` calls (lines 619 and 624). Showing only the first error unreachable does not prove this composition total.

`PROOF_AUDIT.md:66` gives a written support/doubling/tripling argument. That argument and finite zero-failure counts are evidence to review; they do not instantiate a source-faithful invariant for arbitrary runtime states. Ultimately the validity predicate must be preserved by every dispatcher branch, so it is available inductively for a collector's next operation.

## Common-branch identity and geometry are different connections

`formal/Common22SourceHead.lean:113` contains `source_curve_eq`: a proof-side cleared polynomial identity from valid-input divisibilities, nonzero determinant and monic quintic head assumptions. It is more specific than a bare finite arithmetic test. The coefficient/cross-scale/residual packets bind selected expressions, but a universal claim still needs identification of the complete executed output, its reduction and the intended divisor class.

`formal/RuntimeSemantics.lean:283` defines `PrincipalClassInterface`; `residual_represents_input` at line 292 takes the principal-divisor premise as an argument. It does not construct the concrete curve/divisor instance or prove the interpolation function has that divisor, including finite and infinity multiplicities, for every exceptional branch.

Do **not** repeat older reports' broad claim that all projective construction is missing. Later source files now include:

- `formal/WeightedProjProper.lean`: `weightedProj_proper` for the weighted Proj structural map to degree-zero spectrum.
- `formal/WeightedProjSmoothFinal.lean`: `smooth` over a perfect field, characteristic not two, with squarefree quintic.
- `formal/WeightedProjInfinityDVR.lean`: actual projective origin stalk ring equivalence and `projective_origin_dvr`.
- `formal/WeightedPic0Bridge.lean`: actual infinity point/stalk transport; its own opening comment explicitly says it constructs no Picard group and asserts no equality of divisor classes.

These source declarations were located and read, **not freshly built or independently qualified here**. The older `formal/STATUS.json` is dated 2026-10-25 and must not override later artifacts. The remaining geometric task should begin by reconciling the later exact build receipts, then supply the still-missing global principal-divisor/class identification rather than rebuilding already available chart and stalk components.

## Independent review boundary

Located reviews: source audit, proof audit, finite author-translation parity, and independent 2026-10-27 cohort recheck. Their stated scope does not include independent universal mathematical correctness of the complete compiled dispatcher. A future review should explicitly check the weighted-input domain, branch assumptions, preservation, finite-field implementation semantics, exact formula/source correspondence and the global divisor-class statement. New finite cohorts cannot substitute for that review or theorem.
