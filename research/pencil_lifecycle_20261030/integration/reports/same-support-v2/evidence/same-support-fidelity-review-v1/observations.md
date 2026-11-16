# Same-support coefficient fidelity review

The narrow algebraic implication holds over every field. The source declaration faithfully states the two coefficient consequences of common monic quadratic support, same polynomial f, divisibility validity, and equal slopes. This is a source-level mathematical review, not a compiler or kernel verdict.

## Independent derivation

Before reading the producer source, the review plan recorded the degree argument. Put u=X²+qX+r and v_i=s_i X+t_i. Subtract the two supplied divisibilities to obtain u | (v₂²-v₁²). Under s₁=s₂, expand the difference as A X+B, where A=(t₂−t₁)(s₁+s₂) and B=(t₂−t₁)(t₂+t₁). Since u is monic of degree two, a nonzero multiple cannot have degree below two over a field. Hence A X+B=0 and A=B=0. Consequently v₁²=v₂² as polynomials. The candidate exports the coefficient conjunction, not a separately named square-equality theorem; its proof uses the vanishing linear difference internally.

## Actual source signature and proof

SameSupportCandidate.validity_coefficients quantifies over an arbitrary field K, an arbitrary polynomial f, and six arbitrary coefficients q,r,s1,t1,s2,t2. Its only explicit propositions are the two divisibility statements by the identical literal monic quadratic and hs:s1=s2. The conclusion is exactly A=0 ∧ B=0. Monicity and degree two are encoded in the displayed support expression; linear ordinate degree is encoded in the displayed v expressions. Nontriviality and no-zero-divisors follow from Field. No hypothesis of odd characteristic, squarefree support, separability, smoothness, genus two, finiteness, or degree five for f is necessary for this implication. The proof text implements divisibility subtraction, expansion, the degree bound, and coefficient extraction. Whether the chosen mathlib lemmas and tactic expressions elaborate remains unverified here.

## Relation to the helper and error exclusion

ProjectiveLawRuntime.classifySameU22_ne_zeroSlope takes the two exact coefficient equations as hypotheses, together with Field K, DecidableEq K and (2:K)≠0. Given equal slopes, the candidate supplies its hx and hc arguments directly. If slopes differ, the third predicate s1-s2=0 of classifySameU22 is false, so the sameUMixedZeroSlope constructor is unreachable without those coefficient hypotheses. Thus a wrapper can split on equal slopes: invoke the candidate and the existing helper in the equal-slope case, and eliminate the branch condition in the other case. This wrapper is not present in the reviewed candidate.

ProjectiveLawBoundary.same_u_zero_slope_delta_is_inverse additionally uses t2≠t1, obtained in the decision tree from failure of the equal-input branch, to conclude s1=0 and t2=−t1. Failure of the inverse branch then contradicts this result. The existing helper requires odd characteristic, while the candidate does not. That helper precondition is sufficient for the current composition, not evidence that the coefficient theorem needs it. In characteristic two, hc=0 already implies t1=t2 in a field, which excludes the error under equal slopes; a separate proof could remove the odd-characteristic restriction from the branch theorem.

## Boundary attacks, checked by symbolic reasoning only

1. Equal slope is load-bearing for equality of squares. Over Q, take u=X²−1, f=X², v1=X and v2=1. Both validity divisibilities hold, but v2²−v1²=1−X² is nonzero and the proposed constant coefficient equation becomes 1=0. This control is not a counterexample to the stated theorem because slopes differ.
2. Same curve polynomial is load-bearing. Over Q with u=X²+1, v1=0, v2=1, f1=u and f2=u+1, each separate-curve validity holds and slopes agree, but squared ordinates differ. The theorem correctly shares one f.
3. Quadratic degree is load-bearing. With linear u=X, f=1, v1=X+1, v2=X−1 over Q, both validity statements hold and slopes agree, but their squared difference is −4X, a nonzero multiple of u. The theorem encodes a monic quadratic.
4. Assumptions are satisfiable beyond equal ordinates. Over Q take u=X²+1, f=X⁵+X³+1 and v1=1, v2=−1. Then f−v_i²=X³u, slopes are zero, and both coefficient equations hold. This illustrates the inverse case; no claim of smoothness of this illustrative f is needed.

## Exact outstanding bridges

The narrow polynomial implication lacks no mathematical assumption. Applying it to the compiled arithmetic law still requires establishing the runtime inputs yield coefficients in the intended field; modulus/primality and faithful modular arithmetic; nonzero projective scaling and correct dehomogenization or an equivalent cleared polynomial statement; common normalized support and shared f; both Mumford divisibility invariants; correspondence of zero slope, equal-input and inverse predicates to the manually defined Lean classifier; and source/compilation refinement connecting the Lean classifier to the actual Rust execution. Field validity and these bridges are supplied neither by this candidate nor by a source hash.

For a claim about addition of divisor classes, further obligations are concrete smooth-curve Mumford/divisor semantics, principal interpolation divisor, reduction correctness, and output class equality. The reviewed files explicitly leave these as interfaces or comments. No runtime code, compiled binary, binding manifest, curve constructor, parser, or compiler receipt was examined in this review.

## Verdict dimensions

- Mathematical implication: holds.
- Source-level fidelity to the narrow coefficient claim: faithful_scoped.
- Machine status: not_verified (builds and compiler receipts excluded by assignment).
- Review outcome: inconclusive for a formal gate; no gate promotion performed.
- Fully composed error-exclusion theorem from validity: mathematical composition explained, declaration absent in the reviewed candidate.
- Compiled arithmetic correctness: not established by these sources.

This report is scoped to the hashes in review-plan.json. No source edits, builds, experiments, external services, sibling reports, or producer compiler outputs were used. Review-plan.json was written before opening the three source files. The pre-source derivation used the mathematical statement provided in the assignment; full formal definitions were then checked against source, so this is a limited statement-first derivation, not a claim of isolated blind reconstruction of all definitions.
