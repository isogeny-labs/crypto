# Word traversal equals affine polynomial composition

T-1171, 2026-10-31. Producer proof packet; independent source-fidelity review pending.

The decoded polynomial reconstructed from the complete word-array transform equals C(s^-1) times f composed with aX+b. The proof identifies the ordered list of binomial terms with the earlier symbolic outer-loop polynomial, identifies every stored output coefficient, and proves all higher coefficients vanish. Under the additional equality decode(s)=decode(a)^m, it equals the earlier normalization map T_m.

The generic list/binomial identity is proved for arbitrary coefficient count over a field. The machine-array theorem retains length<=6, prime 1<p<2^31, canonical input coefficients and a,b,s, and nonzero scale. The supplied scale equality in the T_m corollary is explicit; computing the scale and subsequent trimming/copying remain separate source bindings.

Twelve main declarations and ten controls passed, with 22 named standard-axiom readbacks. Controls include empty and constant polynomials, rational quadratic coefficients 17/2,10,3 for f=1+2X+3X^2 under (a,b,s)=(2,3,4), negative-alpha linear coefficient -10, omitted-scale mismatch, quintic leading coefficient, high-coefficient zero, and identity transformation for arbitrary coefficient count. These are fixed formal controls, not cohort measurements.

The first identification attempt exposed broad list rewrites, function-composition simplification and coefficient simplification. Subsequent attempts corrected a closed-goal tactic, compact notation parsed as a principal-segment token, and equality orientation in a high-coefficient conditional. Attempt 4 passed with deprecation warnings. The first reconstruction lacked the explicit prime-field instance required by sourcePoly; native exit 1 rejected that build and its elaborator-generated sorry warnings. Reconstruct2 passed, and final named axiom readback contains no sorryAx. Controls and readback passed first attempt. All failures remain recorded.

Sources: projective_law_gate.rs transform; pinned Loop.lean, Schedule.lean, Traversal.lean, Storage.lean, WordCoefficient.lean and Inversion.lean. Primary reference: https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Coeff.html, checked through existing pinned modules. Standard binomial/coefficient identities are reused without novelty claims.

The remaining source bindings are trailing-zero trimming, fixed-size divisor copying, and the scale computation for the T_m corollary. This is a logical source model, not compiled-Rust equivalence or a geometric class transport proof. Existing compiled dependencies remain within the trust boundary. No scientific execution, timing or cost ratio; cohort and campaign limits unchanged.
