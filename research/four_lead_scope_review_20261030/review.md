# Published genus-2 law: bounded source-to-claim review

Reviewed 2026-10-30. This is independent static inspection of retained source, declarations and saved evidence. No compiler, Magma, sampled census, solver, replay, benchmark or cloud job was run. No repository files were changed. This review establishes the boundaries below, not a universal proof of the compiled law or a new performance result.

## Source identity and correction

Historical source root: `/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022` (H below). Current repository tracked HEAD contains identical `src/bin/projective_law_gate.rs` SHA256 `5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0` and `formal/RuntimeSemantics.lean` SHA256 `9e015c3c79f465b2b16f6e0dae5c744f9de3400f51646f746c61c54c4722c318`. The latter is sparse/unmaterialized in the current worktree; comparison used `git show`, not checkout.

I downloaded, but did not execute, the actual author file at `https://raw.githubusercontent.com/ozbayelif/jac-on-jac/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m`. Private copy `pinned-jac-on-jac.m` adjacent to this review has expected SHA256 `7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a`. In this exact file, `DBL2` lines 428–449 defines `WW1:=W1^2` and uses `WW1` in N6. The production Rust wrapper at lines 824–826 selects the squared-W variant. There is **no demonstrated published W-versus-W² defect**. The W¹ variant is a deliberate mutation test; older attribution must remain withdrawn. The independently certified N6 homogeneity identities concern this correction, not all branch semantics.

## Common hypotheses and coordinate interpretation

The intended model is a smooth odd-characteristic curve `y²=x⁵+a3*x³+a2*x²+a1*x+a0`, with valid reduced Mumford input divisors, nonzero projective weights, and consistent curve/field parameters. These are mathematical input hypotheses, not all checked by the runtime public entry point. A degree-one tuple means `x=X/Z²`, `y=Y/(Z⁵W)`; degree two means `u=x²+(Q/Z²)x+R/Z⁴`, `v=(S/(Z³W))x+T/(Z⁵W)`. Identity has a separate Rust variant.

The branch formulas inspected contain field additions/multiplications/squares and comparisons, without explicit field inversion. `normalize` at lines 533–562 explicitly calls inverse twice for either nonidentity output (Z and W); identity uses none. Therefore the formula-level inversion-free property does not make normalization, conversion, or the entire collector inversion-free. `add_affine` normalizes, whereas `add_projective` retains weighted output. Other helpers, including rescaling at line 445, also contain inversion; they must remain separate in cost statements.

## Branch obligation table

Line references below are to H/src/bin/projective_law_gate.rs. All rows inherit the common hypotheses. Certificate counts denote retained exact-polynomial outputs, not freshly rerun checks. Every row still requires a compiled-source-to-proof bridge and concrete divisor-class semantics; the table records the additional local obligations.

| Runtime / author branch | Predicate and returned coordinates | Certificate / finite evidence | Precise proof obligation not discharged by that evidence alone |
|---|---|---|---|
| `add11` 511; author ADD11/ADD11DBL/ADD11ADD | Inverse tested first, including y=0: identity. Equal point next: degree two, Z unchanged and W=2Y. Otherwise distinct-x degree two with cross-difference in W. | Point formulas used by exceptional certificates; finite author/Rust/Cantor comparison. | Show distinct-x and nonzero doubled Y after preceding guards, output valid, and tangency/chord divisor including multiplicity and infinity. |
| generic `add12_weighted_generic` 232; ADD12G | K nonzero; degree two with Z proportional to K times input weights/Z factors, W=1. | 16 identities (7 affine, 9 weighted). | Interpolant congruences, residual reduction and output sign; prove guards imply nonzero output weights and concrete principal divisor. |
| complete 1+2 at 828; ADD12 | K=0: opposite shared ordinate returns other support point; equal ordinate and repeated support calls triple; equal ordinate/distinct support doubles then generic-adds other point; unmatched ordinate returns None. | Exception 12 identities; triple 16 (5 affine, 11 weighted). | Exclude mismatch from valid curve points; prove non-Weierstrass tripling guard and second generic K nonzero; account repeated-root multiplicities rather than only support coordinates. |
| same-u inverse in `add22_weighted` 896; ADDDIVISOR | Both ordinate coefficients opposite: identity before equal-v branch. | Finite comparisons and branch reasoning. | Concrete inverse-class representation, including 2-torsion and repeated support. |
| same-u mixed; ADD22PPMQ | Neither equal nor opposite v; nonzero slope difference extracts point then doubles it. Zero slope difference returns None. | Mixed 7 identities. RuntimeSemantics has conditional zero-slope exclusion. | Discharge validity premises in actual weighted input and identify canceled common component/divisor multiplicity. |
| equal-v doubling 727; DBL2 | B=0,S=0 gives identity; B=0,S!=0 extracts point then doubles. B!=0,C=0 gives degree one Z=Z*B*W,W=1; B!=0,C!=0 degree two Z=Z*C,W=B*W. N6 uses W². | Double 30 identities (11 affine,19 weighted), N6 bridge. | Justify every extraction, branch completeness and nonzero weights, repeated/tangent multiplicities, including degenerate B=0. Author top-level dispatch intercepts inverse before DBL2, so its direct DBL2B0 need not itself be a universal entry point. |
| distinct-u shared support; ADD22B0 | B=0 requires nonzero q difference; extracts shared and remaining points. Opposite shared y calls add11 on remainder; equal y doubles shared point then adds both remaining points with degree-aware Rust dispatch. Mismatch/zero-q/failed secondary composition return None. | Shared 27 identities, conditional classifier exclusions. | Show all None paths unreachable on valid inputs, including secondary compositions; prove composed degree-aware dispatch faithfully implements the author/class law when intermediate degree drops. Source algebra parity alone does not establish global closure. |
| coprime degree drop; ADD22C0 | B!=0,C=0; degree one Z=B*W1*W2*Z1*Z2,W=1. | Drop 18 identities (11 affine,7 weighted), with C=0 parametrization. | Prove parametrization exhaustive under field/guard hypotheses; norm factor multiplicities, correct residual point/sign, no missing infinity contribution. |
| coprime common; ADD22G | B!=0,C!=0; degree two Z=Z1*Z2*C,W=B*W1*W2. | Common 19 identities (9 affine,10 weighted); strongest later local formal chain described below. | Full runtime numerator/coefficient correspondence; nonbranch/infinity orders and global principal divisor/Pic⁰ equality remain. |

`add_projective` 606 calls `expect` on complete 1+2 and 2+2. Thus unproved unreachability of None is a completeness obligation, not a silent fallback to Cantor. This review found no concrete valid-input counterexample and ran no search for one.

The eight symbolic result files total 145 identities: generic12 16; exception12 12; triple16; common22 19; drop22 18; mixed22 7; double22 30; shared22 27. They are separately transcribed polynomial calculations. They do not mechanically prove correspondence of every compiled Rust statement, predicate or return value.

## Formal declarations: stronger than the old completion checklist, narrower than a complete law theorem

The older `NEXT_STEPS.md` and earlier PROOF_AUDIT sections are not the latest formal boundary. Later source now includes:

* `WeightedProjSmoothFinal.lean:64`, `smooth`: actual weighted-Proj structural smoothness under its perfect-field, characteristic and squarefree hypotheses.
* `WeightedProjCrossInjective.lean:100`, `gluedToProj_isIso`: actual global glued-model-to-Proj isomorphism. Calling this bridge still wholly missing would be stale.
* `WeightedProjInfinityDVR.lean`: actual infinity local DVR, unit/irreducibility facts, and `projectiveZ_order_two`. This is local geometry, not yet the interpolation function's full pole divisor.
* `Common22BranchDomain.lean` and subsequent branch transport: finite simple branch DVR and local orders.
* `WeightedProjSourceBranchActualQuintic.lean:23,60`: `source_style_actual_quintic_branch_zero_order` and `_of_separable`. For the proof-side common-2+2 coefficient data, valid input Mumford congruences, nonzero determinant B, rational branch root, and a root of input-support product times residual support, the actual weighted-Proj stalk order of `n(x)-B*y` is one. The actual-quintic specialization removes an earlier separately supplied-polynomial mismatch; the separable corollary derives the simple-root premise. It still does not identify a compiled Rust execution with n or cover all places/branches.
* `RuntimeSemantics.lean` proves conditional classifier exclusions and cleared polynomial congruences, but explicitly states no translation theorem. Its `residual_represents_input` at line292 assumes a `PrincipalClassInterface` and the proposition that input+residual is principal. It is an abstract group consequence of that premise, not a construction of the required divisor/Pic⁰ bridge.

Saved logs/PROOF_AUDIT report successful Lean compilation and axiom audits without sorryAx. I inspected declarations and retained claims but did not rerun Lean; this review must not be described as fresh kernel checking.

## Author execution and finite evidence boundary

`public_source_parity/README.md` reports 34,704 translated-author/Rust and 34,704 Rust/Cantor comparisons on `y²=x⁵+x+2` over F7 and F11: 66 and114 reduced divisors, all ordered pairs, two weight selections,108 joint path cells,28 author paths and16 Rust labels. The author functions were translated by a restricted source evaluator. **Magma was unavailable and the author Magma program was not actually executed.** Source-hash matching and this translation comparison must remain labeled separately from native author-tool execution. The retained 1,260-target cohort is another empirical boundary; neither finite comparison nor successful cohort rows imply universal group-law correctness.

## Concrete next non-execution work

1. Write a branch-indexed semantic specification with exact input validity, degree tags, scale equivalence and field hypotheses; connect the actual runtime expressions and predicates to existing certificate polynomials. Start with common22, where the strongest formal chain already exists. Avoid reproving the now-completed Proj isomorphism.
2. Prove valid-input unreachability of each runtime None/expect failure, nonzero returned weights and closure under each exceptional composition. Explicitly include intermediate degree drops in shared-positive dispatch.
3. Complete local orders for the actual interpolation function at nonbranch points, repeated supports and infinity; handle nonsplit closed points/base extension if the intended theorem quantifies over arbitrary fields. Compose into its global principal divisor, then instantiate the concrete divisor-class interface and reduction/involution sign.
4. Extend the compiled-output correspondence and principal-divisor argument to degree-drop, doubling, mixed, shared-support, tripling and point branches. Existing identity checks remain useful proof inputs, not substitutes for these obligations.
5. Maintain separate completion labels: published-source transcription parity; optional actual Magma parity; finite cohort validation; universal mathematical law proof; implementation completeness; full pipeline cost. An actual Magma run would require an available tool and separately bounded execution; it is not needed to honestly report the existing translation evidence.

These are concrete mathematical/implementation obligations, not a verdict against the published law. No new timing, speedup, cost ratio, extra cohort authorization or requirement to restart completed experiments follows from this review. This report supersedes only the stale formal-gap portion of my earlier four-lead completion audit; it leaves retained empirical results untouched.
