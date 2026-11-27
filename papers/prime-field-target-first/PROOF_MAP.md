# Proof map

The numbered results below refer to `prime-field-summation-barrier.tex`. The manuscript contains written proofs. Every entry is open in Lean. Lean and Lake were unavailable in the publication runtime, so there is no declaration, pinned formal dependency revision, build receipt, or axiom report for these statements. The finite Python checks exercise selected examples and do not establish the general statements.

| Manuscript result | Formalization prerequisites still needed |
| --- | --- |
| Lemma 1, Dimension and length of a split tree | Connect projective Weierstrass point addition and the complete `S_3` incidence, including infinity and ramification. Formalize the finite image and dimension statements and the tower of monic quadratic algebras over the finite leaf grid. |
| Lemma 2, Finite support and representation size | Connect finite scheme length to geometric support and reduced elimination, prove multiplicativity for tensor products, and supply affine Bézout bounds with the stated distinction between support and scheme length. |
| Theorem 3, Full tuple expansion | Formalize signed multiset orbits and their invariant function algebra in arbitrary characteristic. Encode the full-tuple work charge, independent trials, and the acceptance and expected-work estimates. |
| Theorem 4, Explicit support and batching | Formalize support bounds for explicit lists, polynomials, finite algebras, and rational formulas. Connect them to the charged batch model, the probability bound, and the fixed-length elliptic-curve witness giving sharpness. |
| Lemma 5, Constant specialized fibers | Formalize the signed radix interval count and its reduction modulo the prime subgroup order. Connect the signed witnesses to the projective incidence and the reduced finite-grid quotient length. |
| Theorem 6, Opaque target-first barrier | Define adaptive oracle transcripts and formal affine handles, including equality up to sign. Prove first-deviation, affine collision, guessed-witness, and truncation estimates in that model. |
| Theorem 7, Hybrid coordinate barrier | Extend the transcript model with rational coordinate predicates, exceptional evaluations, and digit-valued output. Connect pole degree on the curve to zeros and poles and formalize the degree-budget probability bound. |
| Lemma 8, Weighted zeros on independent curve points | Formalize the reduced Weierstrass coordinate basis and its distinct pole orders, then apply the univariate divisor bound inductively under the stated product distribution. |
| Theorem 9, Independent-batch coordinate barrier | Combine the weighted zero lemma with affine comparisons on independent subgroup scalars, total predicate degree, output digit tests, and explicit accounting for expanded guarded formulas. |

Start with the finite probabilistic parts of Theorems 3 and 4 using existing finite-set and probability abstractions. A formal work bound must retain the manuscript’s representation and sampling hypotheses. The geometric and oracle connections are separate proof obligations.
