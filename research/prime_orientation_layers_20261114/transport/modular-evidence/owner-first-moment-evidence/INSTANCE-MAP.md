# Generic product first coefficient

Structure:CommSemiring R,Polynomial R,arbitrary finite index set S,slope s and natural multiplicity m. Owning floor:CryptoLean with Mathlib structures,not a curve study.

Existing laws as stated:Polynomial.coeff_derivative and coeff_zero_eq_eval_zero identify coefficient1 with derivative evaluated0;derivative_prod_finset gives product rule;derivative_pow gives multiplicity factor;eval_finsetSum/eval_prod/eval_mul/eval_pow preserve the result. Missing composed law is the resulting product first coefficient,first consumer actual shared matching-port denominator in crypto PROTOCOL015.

Objects:product_(j in S) (1+C(s_j)*X)^(m_j). Empty S and m_j0 included by inherited finite-product and power laws;no positivity or favorable coefficient witness.

Probe:actual generic module compiles on pinned Lean4.35/Mathlibcfc55fc. Proof chain restructures coefficient into eval0 derivative,replaces derivative with product rule,then applies eval/power/linear-factor laws. No copied induction or limit raise. Zero coefficient endpoint from coeff_zero_eq_eval_zero and eval_prod directly.

Inheritance table:coeff from Polynomial.coeff,derivative from Polynomial.derivative,finite products from Finset,eval as evalRingHom,all algebra from CommSemiring;existing Mathlib laws above justify each rewrite. New names are composed laws in shared owning floor;study must apply them as stated.

Sources:Mathlib Derivative.lean59,478,618 and Eval/Coeff.lean56,pinned cfc55fc. CryptoLean docs/DESIGN.md three-floor policy reviewed. No AGENTS.md in owning pinned tree. New isolated worktree,consumer dependency untouched. No shared gate/fresh claim until actual affected validation.
