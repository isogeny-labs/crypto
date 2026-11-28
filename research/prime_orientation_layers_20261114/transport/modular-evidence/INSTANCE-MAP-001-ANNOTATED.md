# Fourth-power assembly probe: instance map

Remote new lean-instance-gate instructions read at bd990c086 before any successor Lean edit. The preceding432 statement snapshot remains immutable and was authored before this policy was fetched. No retroactive inheritance audit is inferred.

1. Structure above:Mathlib CommSemiring R,Polynomial R;existing coefficient linear maps,composition by C(s)*X and binomial coefficients. No new syntax or primary computational representation.
2. Existing theorem as stated:Polynomial.coeff_one_add_X_pow R n b and Polynomial.comp_C_mul_X_coeff. They give the complete scaled binomial coefficient by composition;do not re-prove binomial expansion. The remaining candidate law is generic four-divisibility at odd b,from actual4.choose b. Its owner is crypto-lean,not a curve-specific module.
3. Instance:the polynomial (1+C(s)*X)^4 for arbitrary s:R. Later R=Q[X] and s=C(actual half-slope);the actual VBound instance obligation is still separate.
4. Assembly probe:instance-probe-001.lean.txt is a type-checked-intent probe over arbitrary CommSemiring,using only named Mathlib laws and finite numeric coefficient reduction. A successful probe is NOT the actual denominator/profile application nor an owned-library promotion.
5. Missing components:generic owner law for odd coefficient divisible4;transport from scalar factor4 to Content2 for actual half-slopes;actual fourth-power denominator decomposition;global profile degree/schedule and shared-W compatibility. No missing group/curve fact is assumed.
6. Inheritance:coefficients from Polynomial.coeff;substitution from Polynomial.comp;substitution coefficient from comp_C_mul_X_coeff;binomial from coeff_one_add_X_pow;out-of-range vanishing from Nat.choose_eq_zero_of_lt;finite4.choose values by kernel-checked norm_num. All ring arithmetic from existing CommSemiring instance. No maxHeartbeats/recDepth raise,no composite internals opened. [pcc:unverified]

Proof chain:rewrite polynomial as ((1+X)^4).comp(C(s)*X);apply composition coefficient theorem;apply binomial coefficient theorem;for odd b<=4 retain b=1 or3,where4.choose b=4;for b>4 use choose=0. Witness is s^b,or0 out of range. Numeric controls b1,b3 and b>4 are inside the same universally quantified probe,not separate copied certificate families.

Statement table:scaled coefficient equality is an assembly of two Mathlib laws;odd four-divisibility is an upstream candidate over CommSemiring R. Its first consumer is the proposed actual fourth-power parity-content endpoint. Do not publish it as a new study-specific arithmetic discovery.

KB unavailable as previously diagnosed,no tool exposed. Exact primary sources Coeff.lean292--307 and Eval/Degree.lean107 reviewed at pinned Mathlib. Lessons brief must use exported exact remote entries with explicit --dir;initial /tmp script default used wrong root and its empty brief is discarded. No native/paid/cloud/attack run.
