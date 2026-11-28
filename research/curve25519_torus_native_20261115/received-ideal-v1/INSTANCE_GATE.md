# Lean instance gate before source edits

1. Structure above: Mathlib CommRing/Field, Polynomial, Polynomial.Monic,
   Polynomial.IsRoot and ordinary polynomial division. All ring/field/evaluation
   operations are inherited, not redefined. Native bounded matrices are a
   separate producer check, not the formal semantic object.
2. Existing laws used as stated: Polynomial.modByMonic_eq_zero_iff_dvd,
   Polynomial.modByMonic_add_div and Polynomial.eval_mul. Ring normalization
   and linear_combination discharge the curve family's coefficient equalities.
   No generic division/degree/group law is duplicated. New coefficients of the
   received-symbol polynomial are instance obligations; no new generic algebra
   law is needed below the study.
3. Object: monic quartic F in R[W] for coefficient expressions A,beta, and its
   denominator-cleared expression G. Raw branch point equations are explicit
   hypotheses. No custom syntax/wrapper stands in for a field or curve.
4. Assembly probe planned: type-check existing division equivalence with
   arbitrary monic polynomial and evaluate a multiple at its root using the
   library law. Then instantiate the coefficient expression and raw curve
   equations. Characteristic/denominator assumptions stay explicit; no native
   Fp implementation or actual prime theorem is inferred from generic rings.
5. Missing components only: curve-specific polynomial coefficient equality
   from u^2+A*u+beta and v^2=u^3+A*u^2+u; denominator clearing; chart relation
   over Field with 2*tau nonzero. Actual compiler/group/quotient rank/cost and
   divisor-selection proofs are not supplied by these identities.
6. Inheritance table follows. No limits raised, sorry/native_decide or new
   axioms. Pin CryptoLean5f2ca251cb34806f69fd5455175434cac3567dfa,
   Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2, Lean4.35.0, using
   matching existing read-only caches outside crypto-energy. No library
   checkout is edited, recompiled broadly or downloaded to fit a proof.

| Operation | Inheritance |
| --- | --- |
| Addition/multiplication/powers | CommRing/Field instances |
| Monic division equivalence | Polynomial.modByMonic_eq_zero_iff_dvd |
| Division reconstruction | Polynomial.modByMonic_add_div |
| Multiple evaluation | Polynomial.eval_mul |
| Raw curve coefficient identity | Ring normalization of explicit equations |
| Denominator cancellation | Field laws, nonzero2/tau hypotheses |
| Native matrix rank/actual field instance | OPEN, not inherited by algebra proof |

Proof order: assembly probe; received quartic instance; cleared identity;
guarded chart identity. Readable chain: reduce u^5+A*u^4+u^3 modulo
u^2+A*u+beta using quotient u^3+(1-beta)*(u-A); replace w^2 by the resulting
linear expression; use F(w)=(w^2-linear)*(w^2+linear+c2)+C^2*(u^2+A*u+beta).
Cleared identity is J^5*F after B=J*beta. Chart relation is ordinary field
cancellation at tau!=0/2!=0. No generic group law, hardness or curve-genus
theorem is re-derived. Fidelity remains conditional on these exact equations;
actual raw-map/prime/decoder/divisor bridges remain separate.
[pcc:unverified]

Assembly probe passed both arbitrary-monic and displayed-quartic monicity
instances, using degree_add_le/degree_C_mul_X_pow_le/degree_C_le and
monic_X_pow_add as stated. No new division law is introduced.
