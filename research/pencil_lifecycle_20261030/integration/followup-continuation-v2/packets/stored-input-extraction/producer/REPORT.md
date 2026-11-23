# Stored inputs to native fast-call hypotheses

Five main declarations and seven fixed controls compiled, with twelve separate named axiom readbacks. The construction connects actual stored-array coefficients to the existing fast polynomial-call endpoint, deriving the required coefficient identities and residual hypotheses from the stored contract.

## Definitions and claims

`StoredInput.WellFormed` requires degree at most two, exact U/V capacities three/two, canonical coefficients in every slot, active U slot equal to one, and residual divisibility of the decoded accessor polynomials. It deliberately allows nonzero inactive padding. `extract` reads the eight actual low coefficients in native qa,ra,sa,ta,qb,rb,sb,tb order. Its total getD formulation is used only after array capacities have been established.

- `lists_three_two`: capacities imply exact list decomposition into three U and two V words.
- `accessor_shape`: derives actual up length, leading one, normality of both accessors, strict capacity-compatible V length, and canonicality. It handles degree zero, one and two separately; vp uses the actual trimming function.
- `stored_inputs`: derives nonempty support, decoded support degree equal to stored degree, and Mumford validity. Requires prime p>1.
- `degree_two_inputs`: derives canonical FastPrelude.Inputs, exact upA/upB/vpA equalities, decoded vpB, and both fast-support residual divisibilities directly from actual slots and the WellFormed predicates. It does not assume these extraction equalities.
- `stored_fast_calls`: for prime 2<p<2^31, canonical degree-five f, degree-two wellformed inputs and the actual nonzero norm/s1 guards, invokes the existing fast endpoint and returns successful modeled quotient/remainder call equations, canonical normalized output lists, length3/leading1 U, length<=2 V, and decoded validity.

Native source is the immutable e62d035ab852be5bbfd27bbe8f11c8f6bfe25f2e snapshot `three_leads.rs`, SHA256 26e99b56dace8f848d055e8709bb989622e4caea65aec26d63e00805153dc3c6. Read constructor/accessors lines943–964, validity lines972–982, and addition lines1004–1061. Native correspondence is the n=1/no-table branch; Field::validate admits odd p>=5, while the algebra helper theorem keeps p>2. The construction is value correspondence, not physical slice/copy/compiler/counter semantics. Literature/source basis is the existing constructor, trim, polynomial degree and fast-call constructions, plus the exact primary source; no new mathematical method is claimed.

## Controls and receipts

Seven controls: degree-zero identity with nonzero inactive padding, degree-one zero-ordinate trimming, degree-two zero-ordinate trimming, rejection of wrong U capacity/leading coefficient/degree3, literal extraction and successful guards, actual p17 fixture input validity, and complete composition from that actual stored fixture to the fast polynomial endpoint. Fixture A has U=[0,0,1], V=[1,0]; fixture B has U=[1,0,1], V=[1,1]; fwords=[1,0,0,16,16,1]. Existential output lists are kernel-checked, not newly measured Rust trajectories.

Successful receipts:
- StoredInput-attempt-RJ87Cu3V
- StoredDecode-attempt-46JCfsjp
- StoredFast-attempt-ZDL6RRa6
- StoredControls-attempt-MznpDxfD
- StoredReadback-attempt-xFpGLcD2

All exit0 with unchanged bound inputs. Twelve readbacks use only subsets of propext, Classical.choice and Quot.sound. ExpectedFailure-attempt-SmBsB4ll rejects 0=1 with exit1 and unchanged bindings. Dependency/compiler caches are reused and bound; no independent kernel checker or independent source review was run here.

Failures are preserved: StoredInput-attempt-WD9Zyz6s used an original equality where case elimination had rewritten the goal to reflexivity. StoredControls-attempt-id9zamPh contained a redundant tactic after a solved degree-one goal, attempted decidable equality for Inputs without that instance, used incorrect B ordinate [0,0] instead of [1,1], and referenced scaled in the wrong namespace. Repairs change only the private producer source; final controls prove the corrected explicit fixture. An unused subst_vars linter warning remains harmlessly recorded. The first harness bound live native source; before subsequent work it was switched to the immutable pinned snapshot because root was merging its worktree. No binding failure occurred.

## Next joint

Independent source-fidelity review should check leading-slot and padding scope, coefficient ordering, actual source guard assumptions and trimmed ordinate semantics. The next implementation is the source-ordered addition dispatch relation, composing these input facts with fast and strict-fallback output constructors, then a stored Jacobian::valid wrapper. No full dispatch, representative equality, scientific cohort, runtime qualification or cost ratio is asserted here. The original scope and campaign limits are unchanged.
