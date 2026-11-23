# Frozen independent source semantic review

Commit 957dd4c86b2a8b9416e4c62c6ad1f2eb51dcbee5. Read five project modules from git show, plus toolchain and dependency manifest. No producer logs or compilation outputs read. Preceded by frozen blind record /var/tmp/weighted-fidelity-blind-FJBciYGF/BLIND.md (8699ade31d43826df9903a178fc9b72111774adc341c1eaf1f8ae7433d618723).

Verdict: semantic PASS for the literal field-polynomial branch-exclusion claim, subject to subsequent kernel replay. No mathematical flaw found.

WeightedSameSupport theorem statement contains precisely all four nonzero weight premises, characteristic other than two, two actual polynomial divisibilities for the same f, normalized q/r/s/t with powers 2/4/3/5, cross Q/R equalities, and the literal cross-scaled classifier output exclusion. Equal S is a proof case split, not a global assumption. No desired coefficient identities appear as theorem premises.

Source map: WeightedSameSupport.cross_scale is exact clearing x*z2^n*w2=((z1*z2)^n*(w1*w2))*(x/(z1^n*w1)); field cancellation uses hz1,hw1. The weighted theorem derives normalized q/r equality by div_eq_div_iff and normalized slope equality by the same cancellation with associativity. It rewrites valid2 to the same u before calling validity_coefficients. It scales the normalized linear identity by the product of common T and S scales, and the constant identity by the square of common T scale. The opposite input scales use explicit commutation, with correct orientation and powers. Non-equal S excludes the forbidden third branch directly.

SameSupport.validity_coefficients first substitutes slope equality, subtracts the SAME f validity assertions, and identifies their difference as a linear polynomial. Its degree argument uses monic quadratic degree exactly two, linear degree at most one, and divisibility to force zero. It then extracts coefficients 1 and 0. This is a direct specialization of the blind remainder derivation, not an assumed invariant. No root distinctness, irreducibility, squarefreeness, genus, or degree-of-f hypothesis is needed.

RuntimeSemantics.classifySameU22 has exactly the standalone inverse-first, double-second, zero-slope-third order. classifySameU22_ne_zeroSlope uses both coefficient identities and ProjectiveLawBoundary.same_u_zero_slope_delta_is_inverse to force inverse when equal slopes and unequal intercepts. That boundary lemma correctly cancels the nonzero intercept difference and 2. SameSupportClassifier is an imported unweighted corollary; the weighted proof calls the coefficient theorem and runtime classifier lemma directly.

The blind controls remain applicable. In particular the valid S=0,T1=-T2 unequal case is handled by inverse, and zero-weight totalized-division inputs can falsify the conclusion without the nonzero hypotheses. Characteristic restriction is sufficient although stronger than necessary for the isolated statement.

Scope: this establishes field-polynomial validity to exclusion in a manually defined Lean classifier. No Rust source was reviewed and no Rust translation, compiled execution equivalence, divisor-class addition, or concrete geometric invariant is established. Comments in both boundary/runtime files explicitly preserve this distinction. Imported unrelated formula and abstract divisor-interface theorems do not supply missing semantics here.
