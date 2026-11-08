#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Powered mode evaluation in public recovery
T-1905 · Capacity-bounded optional adapter · 2026-11-08

Section powering is now integrated into the canonical signed-recovery path, including its terminal scalar phase. Eligible branches compute all modes in one binary power. Branches exceeding an explicit component capacity retain the existing streaming method.

#image("flow.svg",width:100%)

*Constraint.* With all sections available, use A²·V·F₁(V)·F[m−1](V)−f₊f₋ directly. This avoids reciprocal evaluation and its scale inverse in the powered branch. Mode0 is applied first; old-quotient representatives remain valid after its gcd shrinks the modulus. Existing phase/orientation state and final point verification remain intact.

*Qualification.* Validation03 passes 1,008 quadratic section equalities, 7,632 canonical state comparisons and all 2,529 public target/capacity cases. Outcomes, routes and recorded source stats match streaming. There are 2,205 subsequent image updates, 700 powered terminal phase calls and 350 phase fallbacks. These are finite correctness controls over 281 targets at three lift caps and three power capacities, not a timing comparison. Producer evidence awaits independent signed review.

*Capacity and work.* The preflight cap is 2·m·deg(g) represented output coefficients; it is not process peak memory. Each algebra multiplication uses m² quadratic products plus m(m−1)/2 wrap products. The powered signed equation adds two quadratic products. Mode0 rejection can discard sections already computed. Terminal scalar phase uses the same general evaluator and can be specialized later. No speed threshold has been selected.

*Remaining initial cost.* ReciprocalMode.build still constructs the initial mode0 representation through its coefficient stream. Source signs, radius normalization, canonical filtering, lifting/replay and point checks remain charged. The preceding large-prime evaluator control is not a large-prime recovery run.

*Evidence and proofs.* 113 copied source records and 128 compiler dependencies are hash-bound; inherited stdout is byte-identical. Three successful qualification revisions cover branch integration, terminal phase and explicit phase coverage. Lean4.34.1 checks the conditional direct-product identity; full quotient/runtime refinement remains open. No compile/native failure or cloud run.

*Sources.* Predecessor residue-section derivation and signed-image canonical/adapter modules. KB unavailable; exact-source fallback recorded. See DERIVATION.md, Product.lean and validation03/receipt.json. Next compare full work under a common frozen protocol before choosing a default policy.
