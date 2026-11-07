# Nonblind exploratory mathematical feedback: same-support zero-slope implication

Reviewed derivation SHA256 `92e73a4caafaae304585147bdba5c8db6a593a95ee2a80dda10af1d2121157eb`. Independently read the exact normalization and branch snippets in projective_law_gate.rs and classifier in RuntimeSemantics.lean. Their current full-file hashes match the source index: Rust `5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0`; Lean `9e015c3c79f465b2b16f6e0dae5c744f9de3400f51646f746c61c54c4722c318`.

**Exploratory assessment:** the written field-polynomial argument is coherent with its stated hypotheses; no missing algebraic assumption was identified within that bounded implication. This is nonblind read-only source and derivation feedback. There is no precommitted blind-review plan or published new input here; this is not a completed review-evidence workflow or a validated_scoped record and must not promote formal research status. A formal workflow requires its own prospective plan, immutable published candidate, machine controls and report. This feedback supplies no Lean kernel check, compiled Rust test, input-invariant proof or complete addition-law theorem.

## Checked identities and powers

Normalization really reads q/z², r/z⁴, s/(z³w), t/(z⁵w), with a monic degree-two u. With Z=z1*z2 and W=w1*w2, the literal Rust cross expressions are Z²*qbar_i, Z⁴*rbar_i, Z³*W*sbar_i, and Z⁵*W*tbar_i. Both z weights and both w weights must be nonzero. This is sufficient for every cancellation used; no stronger relation between the weights is needed. Equal cross q/r gives identical normalized support and equal cross s gives identical normalized slope.

The X-coefficient product scales by (Z⁵W)(Z³W)=Z⁸W²; the constant product scales by (Z⁵W)²=Z¹⁰W². The note's two powers and W exponents are correct.

## Divisibility to zero

For the same f and common u, subtracting u|(f-v2²) from u|(f-v1²) yields u|(v2²-v1²). Setting the slopes equal cancels the degree-two coefficient. The remaining polynomial is exactly (t2-t1)(s1+s2)X+(t2-t1)(t2+t1). A nonzero multiple of a degree-two polynomial over a field cannot have degree at most one. Thus this difference polynomial is zero and its two displayed coefficients vanish. The zero-polynomial case must be split if the implementation uses natDegree, as the note already says. The monic quadratic has degree exactly two even if q or r vanishes; no squarefreeness or distinct-root assumption is used.

These coefficient facts require neither odd characteristic nor curve smoothness. The particular existing classifier theorem explicitly requires (2:K) != 0, so retain that assumption when invoking it. Its section also has Field K and DecidableEq K. Its inverse-before-equal-before-zero-slope precedence matches the displayed Rust branch. For the odd-characteristic reasoning, unequal constants force s1+s2=0, hence common slope zero, and t2+t1=0; the inverse branch therefore precedes the error. Equal constants instead satisfy the equality branch (or an earlier inverse branch). This checks the intended exclusion only.

## Boundaries that must remain explicit

- The zero-slope condition is essential to the stated low-degree divisibility argument. Arbitrary equal-support valid inputs can have nonzero degree-two difference, so coefficientwise vanishing cannot be promoted to all equal-support cases.
- Both inputs must be valid against the same f and lie in this monic quadratic, linear-ordinate chart with nonzero weights. No claim here establishes that all collector inputs do so.
- Rust u64 equality, negation, multiplication, subtraction and inverses must faithfully represent field operations with canonical equality; reading the source snippets does not prove that representation property or modulus assumptions. The runtime entry shown does not itself establish all required input hypotheses.
- Smoothness and the appropriate curve model remain needed for an intended Jacobian/group-law interpretation, although they are unused for this polynomial lemma. For a model involving a nonzero h in y²+h*y=f, the supplied validity equation would require adjustment; this note explicitly uses u|(f-v²).
- Draft Lean statements and suggested lemma names/imports remain uncompiled. The source-to-Lean mapping is manual; branch-body hashes are not a translation theorem. No returned divisor-class validity, totality, Picard bridge, timing or cohort conclusion follows from this review.

No builds, kernel runs, cloud calls, scientific trials or source edits were performed. Only this private review artifact was written.
